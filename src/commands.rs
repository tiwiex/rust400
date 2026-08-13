use crate::parser::{CommandValidation, ParsedCommand, ValidationError, validate_command};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HandlerId {
    Exit,
    Help,
    LinuxMapping,
    CreateLibrary,
    DisplayLibrary,
    WorkLibrary,
    DisplayJob,
    DisplayUserProfile,
    SendMessage,
    WorkObject,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ParameterMultiplicity {
    Required,
    Optional,
    Repeated,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ParameterDefinition {
    pub name: &'static str,
    pub summary: &'static str,
    pub multiplicity: ParameterMultiplicity,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CommandDefinition {
    pub name: &'static str,
    pub summary: &'static str,
    pub parameters: &'static [ParameterDefinition],
    pub mutually_exclusive_pairs: &'static [(&'static str, &'static str)],
    pub handler: HandlerId,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CommandRequest {
    Exit,
    Help(HelpRequest),
    LinuxMapping(LinuxMappingRequest),
    CreateLibrary(CreateLibraryRequest),
    DisplayLibrary(DisplayLibraryRequest),
    WorkLibrary,
    DisplayJob,
    DisplayUserProfile,
    SendMessage(SendMessageRequest),
    WorkObject(WorkObjectRequest),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HelpRequest {
    pub command: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinuxMappingRequest {
    pub term: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateLibraryRequest {
    pub library: String,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DisplayLibraryRequest {
    pub library: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SendMessageRequest {
    pub message: String,
    pub recipients: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WorkObjectRequest {
    pub library: Option<String>,
    pub object: Option<String>,
}

const NO_PARAMETERS: &[ParameterDefinition] = &[];
const NO_EXCLUSIONS: &[(&str, &str)] = &[];

const HELP_PARAMETERS: &[ParameterDefinition] = &[ParameterDefinition {
    name: "CMD",
    summary: "Show help for one registered command name.",
    multiplicity: ParameterMultiplicity::Optional,
}];

const LNXMAP_PARAMETERS: &[ParameterDefinition] = &[ParameterDefinition {
    name: "TERM",
    summary: "Names the Linux concept to compare with Rust/400.",
    multiplicity: ParameterMultiplicity::Required,
}];

const CRTLIB_PARAMETERS: &[ParameterDefinition] = &[
    ParameterDefinition {
        name: "LIB",
        summary: "Names the library to create.",
        multiplicity: ParameterMultiplicity::Required,
    },
    ParameterDefinition {
        name: "TEXT",
        summary: "Supplies a descriptive text for the library.",
        multiplicity: ParameterMultiplicity::Optional,
    },
];

const SNDMSG_PARAMETERS: &[ParameterDefinition] = &[
    ParameterDefinition {
        name: "MSG",
        summary: "Provides the message text to send.",
        multiplicity: ParameterMultiplicity::Required,
    },
    ParameterDefinition {
        name: "TO",
        summary: "Identifies one or more recipients.",
        multiplicity: ParameterMultiplicity::Repeated,
    },
];

const DSPLIB_PARAMETERS: &[ParameterDefinition] = &[ParameterDefinition {
    name: "LIB",
    summary: "Names the library to display.",
    multiplicity: ParameterMultiplicity::Required,
}];

const WRKOBJ_PARAMETERS: &[ParameterDefinition] = &[
    ParameterDefinition {
        name: "LIB",
        summary: "Filters objects by library.",
        multiplicity: ParameterMultiplicity::Optional,
    },
    ParameterDefinition {
        name: "OBJ",
        summary: "Filters results to one object.",
        multiplicity: ParameterMultiplicity::Optional,
    },
];

const WRKOBJ_EXCLUSIONS: &[(&str, &str)] = &[("LIB", "OBJ")];

const COMMANDS: &[CommandDefinition] = &[
    CommandDefinition {
        name: "EXIT",
        summary: "End the current interactive session.",
        parameters: NO_PARAMETERS,
        mutually_exclusive_pairs: NO_EXCLUSIONS,
        handler: HandlerId::Exit,
    },
    CommandDefinition {
        name: "HELP",
        summary: "Explain available Rust/400 commands or one command's syntax.",
        parameters: HELP_PARAMETERS,
        mutually_exclusive_pairs: NO_EXCLUSIONS,
        handler: HandlerId::Help,
    },
    CommandDefinition {
        name: "LNXMAP",
        summary: "Compare one Linux concept with the closest Rust/400 idea.",
        parameters: LNXMAP_PARAMETERS,
        mutually_exclusive_pairs: NO_EXCLUSIONS,
        handler: HandlerId::LinuxMapping,
    },
    CommandDefinition {
        name: "CRTLIB",
        summary: "Create an emulated library definition.",
        parameters: CRTLIB_PARAMETERS,
        mutually_exclusive_pairs: NO_EXCLUSIONS,
        handler: HandlerId::CreateLibrary,
    },
    CommandDefinition {
        name: "DSPLIB",
        summary: "Display one emulated library definition.",
        parameters: DSPLIB_PARAMETERS,
        mutually_exclusive_pairs: NO_EXCLUSIONS,
        handler: HandlerId::DisplayLibrary,
    },
    CommandDefinition {
        name: "WRKLIB",
        summary: "List emulated libraries in the current workspace.",
        parameters: NO_PARAMETERS,
        mutually_exclusive_pairs: NO_EXCLUSIONS,
        handler: HandlerId::WorkLibrary,
    },
    CommandDefinition {
        name: "DSPJOB",
        summary: "Display the current Rust/400 session job.",
        parameters: NO_PARAMETERS,
        mutually_exclusive_pairs: NO_EXCLUSIONS,
        handler: HandlerId::DisplayJob,
    },
    CommandDefinition {
        name: "DSPUSRPRF",
        summary: "Display the current Rust/400 user profile.",
        parameters: NO_PARAMETERS,
        mutually_exclusive_pairs: NO_EXCLUSIONS,
        handler: HandlerId::DisplayUserProfile,
    },
    CommandDefinition {
        name: "SNDMSG",
        summary: "Send a local emulator message.",
        parameters: SNDMSG_PARAMETERS,
        mutually_exclusive_pairs: NO_EXCLUSIONS,
        handler: HandlerId::SendMessage,
    },
    CommandDefinition {
        name: "WRKOBJ",
        summary: "Work with emulated objects using filters.",
        parameters: WRKOBJ_PARAMETERS,
        mutually_exclusive_pairs: WRKOBJ_EXCLUSIONS,
        handler: HandlerId::WorkObject,
    },
];

pub fn registered_commands() -> &'static [CommandDefinition] {
    COMMANDS
}

pub fn find_command(name: &str) -> Option<&'static CommandDefinition> {
    registered_commands()
        .iter()
        .find(|definition| definition.name == name)
}

pub fn validate_against_definition(
    command: &ParsedCommand,
    definition: &CommandDefinition,
) -> Result<(), ValidationError> {
    validate_command(command, &validation_for(definition))
}

pub fn render_help(definition: &CommandDefinition) -> String {
    let mut lines = Vec::new();
    lines.push(format!("Command: {}", definition.name));
    lines.push(format!("Summary: {}", definition.summary));

    if definition.parameters.is_empty() {
        lines.push("Parameters: none".to_string());
    } else {
        lines.push("Parameters:".to_string());

        for parameter in definition.parameters {
            let multiplicity = match parameter.multiplicity {
                ParameterMultiplicity::Required => "required",
                ParameterMultiplicity::Optional => "optional",
                ParameterMultiplicity::Repeated => "repeated",
            };

            lines.push(format!(
                "- {} ({multiplicity}): {}",
                parameter.name, parameter.summary
            ));
        }
    }

    lines.push(format!("Handler: {:?}", definition.handler));
    lines.join("\n")
}

pub fn build_request(
    command: &ParsedCommand,
    definition: &CommandDefinition,
) -> Result<CommandRequest, ValidationError> {
    validate_against_definition(command, definition)?;

    match definition.handler {
        HandlerId::Exit => Ok(CommandRequest::Exit),
        HandlerId::Help => Ok(CommandRequest::Help(HelpRequest {
            command: single_optional_identifier(command, "CMD"),
        })),
        HandlerId::LinuxMapping => Ok(CommandRequest::LinuxMapping(LinuxMappingRequest {
            term: required_value(command, "TERM")
                .expect("validated command should contain required TERM"),
        })),
        HandlerId::CreateLibrary => Ok(CommandRequest::CreateLibrary(CreateLibraryRequest {
            library: required_value(command, "LIB")
                .expect("validated command should contain required LIB"),
            text: single_optional_text(command, "TEXT"),
        })),
        HandlerId::DisplayLibrary => Ok(CommandRequest::DisplayLibrary(DisplayLibraryRequest {
            library: required_value(command, "LIB")
                .expect("validated command should contain required LIB"),
        })),
        HandlerId::WorkLibrary => Ok(CommandRequest::WorkLibrary),
        HandlerId::DisplayJob => Ok(CommandRequest::DisplayJob),
        HandlerId::DisplayUserProfile => Ok(CommandRequest::DisplayUserProfile),
        HandlerId::SendMessage => Ok(CommandRequest::SendMessage(SendMessageRequest {
            message: required_value(command, "MSG")
                .expect("validated command should contain required MSG"),
            recipients: repeated_values(command, "TO"),
        })),
        HandlerId::WorkObject => Ok(CommandRequest::WorkObject(WorkObjectRequest {
            library: single_optional_identifier(command, "LIB"),
            object: single_optional_identifier(command, "OBJ"),
        })),
    }
}

fn validation_for(definition: &CommandDefinition) -> CommandValidation {
    let mut required_parameters = Vec::new();
    let mut optional_parameters = Vec::new();
    let mut repeated_parameters = Vec::new();

    for parameter in definition.parameters {
        match parameter.multiplicity {
            ParameterMultiplicity::Required => required_parameters.push(parameter.name),
            ParameterMultiplicity::Optional => optional_parameters.push(parameter.name),
            ParameterMultiplicity::Repeated => {
                optional_parameters.push(parameter.name);
                repeated_parameters.push(parameter.name);
            }
        }
    }

    CommandValidation {
        required_parameters,
        optional_parameters,
        repeated_parameters,
        mutually_exclusive_pairs: definition.mutually_exclusive_pairs.to_vec(),
    }
}

fn required_value(command: &ParsedCommand, name: &str) -> Option<String> {
    command
        .parameters
        .iter()
        .find(|parameter| parameter.name == name)
        .map(|parameter| parameter.value.as_display_text().to_string())
}

fn single_optional_identifier(command: &ParsedCommand, name: &str) -> Option<String> {
    command
        .parameters
        .iter()
        .find(|parameter| parameter.name == name)
        .map(|parameter| parameter.value.as_display_text().to_string())
}

fn single_optional_text(command: &ParsedCommand, name: &str) -> Option<String> {
    single_optional_identifier(command, name)
}

fn repeated_values(command: &ParsedCommand, name: &str) -> Vec<String> {
    command
        .parameters
        .iter()
        .filter(|parameter| parameter.name == name)
        .map(|parameter| parameter.value.as_display_text().to_string())
        .collect()
}

pub fn validate_registry_metadata(commands: &[CommandDefinition]) -> Result<(), &'static str> {
    for definition in commands {
        if definition.name.trim().is_empty() {
            return Err("registered command is missing a name");
        }

        if definition.summary.trim().is_empty() {
            return Err("registered command is missing a summary");
        }

        for parameter in definition.parameters {
            if parameter.name.trim().is_empty() {
                return Err("registered parameter is missing a name");
            }

            if parameter.summary.trim().is_empty() {
                return Err("registered parameter is missing a summary");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        CommandRequest, HandlerId, build_request, find_command, registered_commands, render_help,
        validate_against_definition, validate_registry_metadata,
    };
    use crate::parser::{ValidationError, parse_command};

    #[test]
    fn help_and_validation_consume_the_same_command_definition() {
        let definition = find_command("CRTLIB").expect("CRTLIB should be registered");
        let parsed = parse_command("crtlib lib(mylib) text('Learning library')")
            .expect("command should parse");

        assert_eq!(validate_against_definition(&parsed, definition), Ok(()));

        let help = render_help(definition);
        assert!(help.contains("Command: CRTLIB"));
        assert!(help.contains("Summary: Create an emulated library definition."));
        assert!(help.contains("- LIB (required): Names the library to create."));
        assert!(help.contains("- TEXT (optional): Supplies a descriptive text for the library."));
    }

    #[test]
    fn typed_request_is_built_from_the_shared_definition() {
        let definition = find_command("CRTLIB").expect("CRTLIB should be registered");
        let parsed = parse_command("crtlib lib(mylib) text('Learning library')")
            .expect("command should parse");

        assert_eq!(
            build_request(&parsed, definition),
            Ok(CommandRequest::CreateLibrary(super::CreateLibraryRequest {
                library: "MYLIB".to_string(),
                text: Some("Learning library".to_string()),
            }))
        );
    }

    #[test]
    fn validation_rejects_unknown_parameters_through_registered_metadata() {
        let definition = find_command("CRTLIB").expect("CRTLIB should be registered");
        let parsed = parse_command("crtlib lib(mylib) owner(me)").expect("command should parse");

        assert_eq!(
            validate_against_definition(&parsed, definition),
            Err(ValidationError::UnknownParameter {
                parameter: "OWNER".to_string(),
            })
        );
    }

    #[test]
    fn registry_metadata_check_fails_for_missing_summary() {
        let invalid = [super::CommandDefinition {
            name: "BROKEN",
            summary: "",
            parameters: &[],
            mutually_exclusive_pairs: &[],
            handler: HandlerId::Help,
        }];

        assert_eq!(
            validate_registry_metadata(&invalid),
            Err("registered command is missing a summary")
        );
    }

    #[test]
    fn shipped_registry_has_required_metadata() {
        assert_eq!(validate_registry_metadata(registered_commands()), Ok(()));
    }
}
