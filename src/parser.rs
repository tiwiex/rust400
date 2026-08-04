use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParsedCommand {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

impl ParsedCommand {
    pub fn parameter_map(&self) -> BTreeMap<&str, Vec<&ParameterValue>> {
        let mut map = BTreeMap::new();

        for parameter in &self.parameters {
            map.entry(parameter.name.as_str())
                .or_insert_with(Vec::new)
                .push(&parameter.value);
        }

        map
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub value: ParameterValue,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParameterValue {
    Identifier(String),
    StringLiteral(String),
}

impl ParameterValue {
    pub fn as_display_text(&self) -> &str {
        match self {
            Self::Identifier(value) | Self::StringLiteral(value) => value,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseError {
    EmptyInput,
    MissingCommandName,
    UnexpectedCharacter { position: usize, found: char },
    UnterminatedString { position: usize },
    MissingParameterValue { parameter: String },
    MissingClosingParenthesis { parameter: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(formatter, "input is empty"),
            Self::MissingCommandName => write!(formatter, "missing command name"),
            Self::UnexpectedCharacter { position, found } => {
                write!(
                    formatter,
                    "unexpected character '{found}' at position {position}"
                )
            }
            Self::UnterminatedString { position } => {
                write!(
                    formatter,
                    "unterminated quoted string starting at position {position}"
                )
            }
            Self::MissingParameterValue { parameter } => {
                write!(formatter, "parameter {parameter} is missing a value")
            }
            Self::MissingClosingParenthesis { parameter } => {
                write!(formatter, "parameter {parameter} is missing a closing ')'")
            }
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValidationError {
    MissingRequiredParameter { parameter: String },
    DuplicateParameter { parameter: String },
    UnknownParameter { parameter: String },
    MutuallyExclusiveParameters { first: String, second: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredParameter { parameter } => {
                write!(formatter, "missing required parameter {parameter}")
            }
            Self::DuplicateParameter { parameter } => {
                write!(formatter, "duplicate parameter {parameter}")
            }
            Self::UnknownParameter { parameter } => {
                write!(formatter, "unknown parameter {parameter}")
            }
            Self::MutuallyExclusiveParameters { first, second } => {
                write!(
                    formatter,
                    "parameters {first} and {second} cannot be used together"
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommandValidation {
    pub required_parameters: Vec<&'static str>,
    pub optional_parameters: Vec<&'static str>,
    pub repeated_parameters: Vec<&'static str>,
    pub mutually_exclusive_pairs: Vec<(&'static str, &'static str)>,
}

pub fn parse_command(input: &str) -> Result<ParsedCommand, ParseError> {
    let mut parser = Parser::new(input);
    parser.skip_whitespace();

    if parser.is_eof() {
        return Err(ParseError::EmptyInput);
    }

    let name = parser
        .parse_identifier()
        .ok_or(ParseError::MissingCommandName)?;
    let mut parameters = Vec::new();

    loop {
        parser.skip_whitespace();

        if parser.is_eof() {
            break;
        }

        let parameter_name = parser
            .parse_identifier()
            .ok_or_else(|| match parser.peek() {
                Some(found) => ParseError::UnexpectedCharacter {
                    position: parser.position(),
                    found,
                },
                None => ParseError::MissingCommandName,
            })?;

        parser.skip_whitespace();

        if !parser.consume_if('(') {
            return Err(parser.unexpected_or_missing());
        }

        parser.skip_whitespace();

        let value = if let Some(quote) = parser.peek().filter(|c| *c == '\'' || *c == '"') {
            parser.parse_quoted_string(quote)?
        } else if let Some(identifier) = parser.parse_identifier() {
            ParameterValue::Identifier(identifier)
        } else {
            return Err(ParseError::MissingParameterValue {
                parameter: parameter_name,
            });
        };

        parser.skip_whitespace();

        if !parser.consume_if(')') {
            return Err(ParseError::MissingClosingParenthesis {
                parameter: parameter_name,
            });
        }

        parameters.push(Parameter {
            name: parameter_name,
            value,
        });
    }

    Ok(ParsedCommand { name, parameters })
}

pub fn validate_command(
    command: &ParsedCommand,
    validation: &CommandValidation,
) -> Result<(), ValidationError> {
    let parameter_map = command.parameter_map();
    let known_parameters: Vec<_> = validation
        .required_parameters
        .iter()
        .chain(validation.optional_parameters.iter())
        .copied()
        .collect();

    for parameter in &command.parameters {
        if !known_parameters
            .iter()
            .any(|known| *known == parameter.name)
        {
            return Err(ValidationError::UnknownParameter {
                parameter: parameter.name.clone(),
            });
        }
    }

    for required in &validation.required_parameters {
        if !parameter_map.contains_key(required) {
            return Err(ValidationError::MissingRequiredParameter {
                parameter: (*required).to_string(),
            });
        }
    }

    for (name, values) in &parameter_map {
        let allows_repetition = validation
            .repeated_parameters
            .iter()
            .any(|repeated| repeated == name);

        if !allows_repetition && values.len() > 1 {
            return Err(ValidationError::DuplicateParameter {
                parameter: (*name).to_string(),
            });
        }
    }

    for (first, second) in &validation.mutually_exclusive_pairs {
        if parameter_map.contains_key(first) && parameter_map.contains_key(second) {
            return Err(ValidationError::MutuallyExclusiveParameters {
                first: (*first).to_string(),
                second: (*second).to_string(),
            });
        }
    }

    Ok(())
}

struct Parser<'a> {
    chars: Vec<char>,
    index: usize,
    _source: &'a str,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().collect(),
            index: 0,
            _source: source,
        }
    }

    fn position(&self) -> usize {
        self.index
    }

    fn is_eof(&self) -> bool {
        self.index >= self.chars.len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.peek()?;
        self.index += 1;
        Some(character)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(character) if character.is_whitespace()) {
            self.index += 1;
        }
    }

    fn consume_if(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn parse_identifier(&mut self) -> Option<String> {
        let start = self.index;

        while matches!(self.peek(), Some(character) if is_identifier_character(character)) {
            self.index += 1;
        }

        if self.index == start {
            return None;
        }

        Some(
            self.chars[start..self.index]
                .iter()
                .collect::<String>()
                .to_ascii_uppercase(),
        )
    }

    fn parse_quoted_string(&mut self, quote: char) -> Result<ParameterValue, ParseError> {
        let start = self.position();
        self.advance();
        let mut value = String::new();

        loop {
            match self.advance() {
                Some(character) if character == quote => break,
                Some(character) => value.push(character),
                None => return Err(ParseError::UnterminatedString { position: start }),
            }
        }

        Ok(ParameterValue::StringLiteral(value))
    }

    fn unexpected_or_missing(&self) -> ParseError {
        match self.peek() {
            Some(found) => ParseError::UnexpectedCharacter {
                position: self.position(),
                found,
            },
            None => ParseError::MissingCommandName,
        }
    }
}

fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '_' | '#' | '$' | '@')
}

#[cfg(test)]
mod tests {
    use super::{
        CommandValidation, ParameterValue, ParseError, ParsedCommand, ValidationError,
        parse_command, validate_command,
    };

    #[test]
    fn parses_command_name_without_parameters() {
        assert_eq!(
            parse_command("dspjob").expect("command should parse"),
            ParsedCommand {
                name: "DSPJOB".to_string(),
                parameters: Vec::new(),
            }
        );
    }

    #[test]
    fn parses_keyword_parameters_with_whitespace_and_quotes() {
        let parsed = parse_command("crtlib  lib(mylib)  text('Learning library')  ")
            .expect("command should parse");

        assert_eq!(parsed.name, "CRTLIB");
        assert_eq!(parsed.parameters.len(), 2);
        assert_eq!(parsed.parameters[0].name, "LIB");
        assert_eq!(
            parsed.parameters[0].value,
            ParameterValue::Identifier("MYLIB".to_string())
        );
        assert_eq!(parsed.parameters[1].name, "TEXT");
        assert_eq!(
            parsed.parameters[1].value,
            ParameterValue::StringLiteral("Learning library".to_string())
        );
    }

    #[test]
    fn normalizes_unquoted_identifiers_case_insensitively() {
        let upper = parse_command("crtlib lib(MyLib)").expect("command should parse");
        let lower = parse_command("CRTLIB LIB(MYLIB)").expect("command should parse");

        assert_eq!(upper, lower);
    }

    #[test]
    fn supports_double_quoted_string_values() {
        let parsed = parse_command("sndmsg msg(\"Hello there\")").expect("command should parse");

        assert_eq!(
            parsed.parameters[0].value,
            ParameterValue::StringLiteral("Hello there".to_string())
        );
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(parse_command("   "), Err(ParseError::EmptyInput));
    }

    #[test]
    fn rejects_unterminated_string() {
        assert_eq!(
            parse_command("crtlib text('broken)").expect_err("command should fail to parse"),
            ParseError::UnterminatedString { position: 12 }
        );
    }

    #[test]
    fn rejects_missing_parameter_value() {
        assert_eq!(
            parse_command("crtlib lib()").expect_err("command should fail to parse"),
            ParseError::MissingParameterValue {
                parameter: "LIB".to_string()
            }
        );
    }

    #[test]
    fn rejects_missing_closing_parenthesis() {
        assert_eq!(
            parse_command("crtlib lib(mylib").expect_err("command should fail to parse"),
            ParseError::MissingClosingParenthesis {
                parameter: "LIB".to_string()
            }
        );
    }

    #[test]
    fn rejects_unexpected_trailing_character() {
        assert_eq!(
            parse_command("crtlib /").expect_err("command should fail to parse"),
            ParseError::UnexpectedCharacter {
                position: 7,
                found: '/',
            }
        );
    }

    #[test]
    fn validation_accepts_required_optional_and_repeated_parameters() {
        let parsed =
            parse_command("sndmsg msg('Hi') to(QSYSOPR) to(MYUSR)").expect("command should parse");
        let validation = CommandValidation {
            required_parameters: vec!["MSG"],
            optional_parameters: vec!["TO"],
            repeated_parameters: vec!["TO"],
            mutually_exclusive_pairs: Vec::new(),
        };

        assert_eq!(validate_command(&parsed, &validation), Ok(()));
    }

    #[test]
    fn validation_rejects_missing_required_parameter() {
        let parsed = parse_command("crtlib text('Hi')").expect("command should parse");
        let validation = CommandValidation {
            required_parameters: vec!["LIB"],
            optional_parameters: vec!["TEXT"],
            repeated_parameters: Vec::new(),
            mutually_exclusive_pairs: Vec::new(),
        };

        assert_eq!(
            validate_command(&parsed, &validation),
            Err(ValidationError::MissingRequiredParameter {
                parameter: "LIB".to_string(),
            })
        );
    }

    #[test]
    fn validation_rejects_unknown_parameter() {
        let parsed = parse_command("crtlib lib(MYLIB) owner(ME)").expect("command should parse");
        let validation = CommandValidation {
            required_parameters: vec!["LIB"],
            optional_parameters: vec!["TEXT"],
            repeated_parameters: Vec::new(),
            mutually_exclusive_pairs: Vec::new(),
        };

        assert_eq!(
            validate_command(&parsed, &validation),
            Err(ValidationError::UnknownParameter {
                parameter: "OWNER".to_string(),
            })
        );
    }

    #[test]
    fn validation_rejects_duplicate_parameter() {
        let parsed = parse_command("crtlib lib(MYLIB) lib(OTHER)").expect("command should parse");
        let validation = CommandValidation {
            required_parameters: vec!["LIB"],
            optional_parameters: Vec::new(),
            repeated_parameters: Vec::new(),
            mutually_exclusive_pairs: Vec::new(),
        };

        assert_eq!(
            validate_command(&parsed, &validation),
            Err(ValidationError::DuplicateParameter {
                parameter: "LIB".to_string(),
            })
        );
    }

    #[test]
    fn validation_rejects_mutually_exclusive_parameters() {
        let parsed = parse_command("wrkobj lib(MYLIB) obj(THING)").expect("command should parse");
        let validation = CommandValidation {
            required_parameters: Vec::new(),
            optional_parameters: vec!["LIB", "OBJ"],
            repeated_parameters: Vec::new(),
            mutually_exclusive_pairs: vec![("LIB", "OBJ")],
        };

        assert_eq!(
            validate_command(&parsed, &validation),
            Err(ValidationError::MutuallyExclusiveParameters {
                first: "LIB".to_string(),
                second: "OBJ".to_string(),
            })
        );
    }
}
