use std::env;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use rust400::commands::{
    CommandRequest, build_request, find_command, registered_commands, render_help,
    validate_registry_metadata,
};
use rust400::libraries::{create_library, find_library};
use rust400::menus::{
    FunctionKeyAction, MenuAction, find_footer_hint, find_menu, find_option, registered_menus,
    validate_menu_registry,
};
use rust400::parser::parse_command;
use rust400::session::SessionContext;
use rust400::ui::{INPUT_PROMPT, MenuScreen, render_green_screen};
use rust400::workspace::Workspace;

const USAGE: &str = "Usage: rust400 (--workspace <absolute-path> | --temporary-workspace)";

fn main() -> ExitCode {
    if let Err(error) = validate_registry_metadata(registered_commands()) {
        eprintln!("Could not start Rust/400: invalid command metadata: {error}");
        return ExitCode::FAILURE;
    }

    if let Err(error) = validate_menu_registry(registered_menus()) {
        eprintln!("Could not start Rust/400: invalid menu metadata: {error}");
        return ExitCode::FAILURE;
    }

    match parse_mode(env::args_os().skip(1)) {
        Ok(Mode::Permanent(path)) => start_permanent(path),
        Ok(Mode::Temporary) => start_temporary(),
        Err(message) => {
            eprintln!("{message}\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn start_permanent(path: PathBuf) -> ExitCode {
    match Workspace::initialize(path) {
        Ok(workspace) => {
            run_interactive_session(&workspace, io::stdin().lock(), io::stdout().lock())
        }
        Err(error) => {
            eprintln!("Could not initialize Rust/400: {error}");
            ExitCode::FAILURE
        }
    }
}

fn start_temporary() -> ExitCode {
    match Workspace::temporary() {
        Ok(workspace) => {
            run_interactive_session(&workspace, io::stdin().lock(), io::stdout().lock())
        }
        Err(error) => {
            eprintln!("Could not initialize Rust/400: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_interactive_session(
    workspace: &Workspace,
    mut input: impl BufRead,
    mut output: impl Write,
) -> ExitCode {
    let session = SessionContext::new();

    if let Err(error) = write_startup(&mut output, workspace, &session) {
        eprintln!("Could not start Rust/400: {error}");
        return ExitCode::FAILURE;
    }

    match command_loop_with_session(workspace, &session, &mut input, &mut output) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Could not continue Rust/400: {error}");
            ExitCode::FAILURE
        }
    }
}

fn write_startup(
    output: &mut impl Write,
    workspace: &Workspace,
    session: &SessionContext,
) -> io::Result<()> {
    render_active_screen(output, workspace, session, "MAIN")
}

#[cfg_attr(not(test), allow(dead_code))]
fn command_loop(
    workspace: &Workspace,
    input: &mut impl BufRead,
    output: &mut impl Write,
) -> io::Result<()> {
    let session = SessionContext::new();
    command_loop_with_session(workspace, &session, input, output)
}

fn command_loop_with_session(
    workspace: &Workspace,
    session: &SessionContext,
    input: &mut impl BufRead,
    output: &mut impl Write,
) -> io::Result<()> {
    let mut line = String::new();
    let mut current_menu = "MAIN";
    let mut last_direct_input: Option<String> = None;

    loop {
        write!(output, "  {INPUT_PROMPT}")?;
        output.flush()?;

        line.clear();

        if input.read_line(&mut line)? == 0 {
            writeln!(output)?;
            return Ok(());
        }

        let command = line.trim();

        if command.is_empty() {
            continue;
        }

        if let Some(result) =
            try_function_key(command, current_menu, &mut last_direct_input, output)?
        {
            match result {
                FunctionKeyResult::Continue => continue,
                FunctionKeyResult::Exit => return Ok(()),
            }
        }

        if let Some(result) = try_menu_selection(command, current_menu, output)? {
            match result {
                MenuSelectionResult::Continue => continue,
                MenuSelectionResult::Render(menu_id) => {
                    current_menu = menu_id;
                    render_active_screen(output, workspace, session, current_menu)?;
                    continue;
                }
                MenuSelectionResult::Exit => return Ok(()),
            }
        }

        last_direct_input = Some(command.to_string());

        match parse_command(command) {
            Ok(parsed) => {
                let Some(definition) = find_command(&parsed.name) else {
                    writeln!(
                        output,
                        "Command '{}' is not registered yet. Use HELP for available commands.",
                        parsed.name
                    )?;
                    if let Some(suggestion) = combined_command_hint(command) {
                        writeln!(output, "{suggestion}")?;
                    }
                    continue;
                };

                match build_request(&parsed, definition) {
                    Ok(CommandRequest::Exit) => {
                        writeln!(output, "Session ended.")?;
                        return Ok(());
                    }
                    Ok(CommandRequest::Help(request)) => {
                        if let Some(command_name) = request.command {
                            if let Some(help_definition) = find_command(&command_name) {
                                writeln!(output, "{}", render_help(help_definition))?;
                            } else {
                                writeln!(
                                    output,
                                    "Command '{command_name}' is not registered yet."
                                )?;
                            }
                        } else {
                            writeln!(
                                output,
                                "Available commands: {}",
                                registered_commands()
                                    .iter()
                                    .map(|definition| definition.name)
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )?;
                        }
                    }
                    Ok(CommandRequest::CreateLibrary(request)) => {
                        match create_library(workspace, &request.library, request.text.as_deref()) {
                            Ok(record) => {
                                writeln!(
                                    output,
                                    "CRTLIB created library {}{}.",
                                    record.name,
                                    record
                                        .text
                                        .as_deref()
                                        .map(|text| format!(" with text '{text}'"))
                                        .unwrap_or_default()
                                )?;
                            }
                            Err(error) => writeln!(output, "{error}")?,
                        }
                    }
                    Ok(CommandRequest::DisplayLibrary(request)) => {
                        match find_library(workspace, &request.library) {
                            Ok(Some(record)) => {
                                writeln!(output, "Library: {}", record.name)?;
                                writeln!(
                                    output,
                                    "Text: {}",
                                    record.text.as_deref().unwrap_or("*NONE")
                                )?;
                                writeln!(output, "Created: {}", record.created_at_epoch_seconds)?;
                            }
                            Ok(None) => {
                                writeln!(output, "Library {} does not exist.", request.library)?;
                            }
                            Err(error) => writeln!(output, "{error}")?,
                        }
                    }
                    Ok(CommandRequest::DisplayJob) => {
                        writeln!(output, "Job: {}", session.job_name())?;
                        writeln!(output, "User: {}", session.current_user())?;
                        writeln!(output, "Status: {}", session.status())?;
                        writeln!(output, "Started: {}", session.started_at_epoch_seconds())?;
                        writeln!(output, "Workspace: {}", workspace.root().display())?;
                    }
                    Ok(CommandRequest::DisplayUserProfile) => {
                        writeln!(output, "User profile: {}", session.current_user())?;
                        writeln!(output, "Profile status: ENABLED")?;
                        writeln!(output, "Current job: {}", session.job_name())?;
                        writeln!(
                            output,
                            "Linux analogy: similar to the signed-in shell user, but contained inside Rust/400."
                        )?;
                    }
                    Ok(CommandRequest::SendMessage(request)) => {
                        writeln!(
                            output,
                            "SNDMSG would send '{}' to {} recipient(s).",
                            request.message,
                            request.recipients.len()
                        )?;
                        if request.recipients.is_empty() {
                            writeln!(
                                output,
                                "Hint: add TO(name) for a recipient, for example SNDMSG MSG('Hello') TO(QSYSOPR)."
                            )?;
                        }
                    }
                    Ok(CommandRequest::WorkObject(request)) => {
                        writeln!(
                            output,
                            "Command 'WRKOBJ' is mapped to handler {:?} with LIB({}) OBJ({}).",
                            definition.handler,
                            request.library.as_deref().unwrap_or("*NONE"),
                            request.object.as_deref().unwrap_or("*NONE")
                        )?;
                    }
                    Err(error) => {
                        writeln!(output, "Validation error: {error}")?;
                    }
                }
            }
            Err(error) => {
                writeln!(output, "Syntax error: {error}")?;
                if let Some(suggestion) = combined_command_hint(command) {
                    writeln!(output, "{suggestion}")?;
                }
            }
        }
    }
}

fn render_active_screen(
    output: &mut impl Write,
    workspace: &Workspace,
    session: &SessionContext,
    menu_id: &str,
) -> io::Result<()> {
    let menu = find_menu(menu_id).expect("validated registry should contain rendered menu");
    let screen = MenuScreen {
        menu,
        system_name: "RUST400",
        current_user: session.current_user(),
        job_name: session.job_name(),
    };

    write!(output, "{}", render_green_screen(&screen))?;
    writeln!(output, "  Workspace: {}", workspace.root().display())?;
    writeln!(
        output,
        "  Enter EXIT in the command line to end the session."
    )?;
    Ok(())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MenuSelectionResult {
    Continue,
    Render(&'static str),
    Exit,
}

fn try_menu_selection(
    input: &str,
    current_menu: &str,
    output: &mut impl Write,
) -> io::Result<Option<MenuSelectionResult>> {
    let menu = find_menu(current_menu).expect("validated registry should contain current menu");

    if let Some(option) = find_option(menu, input) {
        return Ok(Some(match option.action {
            MenuAction::RunCommand("EXIT") => {
                writeln!(
                    output,
                    "Menu selection {} -> {}",
                    option.selector, option.label
                )?;
                writeln!(output, "Session ended.")?;
                MenuSelectionResult::Exit
            }
            MenuAction::RunCommand("DSPLIB") => {
                writeln!(
                    output,
                    "Menu selection {} -> {}. Type DSPLIB LIB(name) to inspect one library.",
                    option.selector, option.label
                )?;
                MenuSelectionResult::Continue
            }
            MenuAction::RunCommand("DSPJOB") => {
                writeln!(
                    output,
                    "Menu selection {} -> {}. Type DSPJOB to inspect the current session job.",
                    option.selector, option.label
                )?;
                MenuSelectionResult::Continue
            }
            MenuAction::RunCommand("DSPUSRPRF") => {
                writeln!(
                    output,
                    "Menu selection {} -> {}. Type DSPUSRPRF to inspect the current Rust/400 profile.",
                    option.selector, option.label
                )?;
                MenuSelectionResult::Continue
            }
            MenuAction::RunCommand("SNDMSG") => {
                writeln!(
                    output,
                    "Menu selection {} -> {}. Type SNDMSG MSG('Hello') TO(QSYSOPR). Rust/400 uses single-token command names, not 'SND MSG'.",
                    option.selector, option.label
                )?;
                MenuSelectionResult::Continue
            }
            MenuAction::RunCommand("CRTLIB") => {
                writeln!(
                    output,
                    "Menu selection {} -> {}. Type CRTLIB LIB(name) TEXT('description') to create a library.",
                    option.selector, option.label
                )?;
                MenuSelectionResult::Continue
            }
            MenuAction::RunCommand(command_name) => {
                writeln!(
                    output,
                    "Menu selection {} -> {} maps to command '{}'.",
                    option.selector, option.label, command_name
                )?;
                MenuSelectionResult::Continue
            }
            MenuAction::OpenMenu(target_menu) => {
                writeln!(
                    output,
                    "Menu selection {} -> {} opens menu '{}'.",
                    option.selector, option.label, target_menu
                )?;
                MenuSelectionResult::Render(target_menu)
            }
        }));
    }

    if input.chars().all(|character| character.is_ascii_digit()) {
        writeln!(
            output,
            "Selection '{input}' is not valid on menu {}.",
            menu.id
        )?;
        return Ok(Some(MenuSelectionResult::Continue));
    }

    Ok(None)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum FunctionKeyResult {
    Continue,
    Exit,
}

fn try_function_key(
    input: &str,
    current_menu: &str,
    last_direct_input: &mut Option<String>,
    output: &mut impl Write,
) -> io::Result<Option<FunctionKeyResult>> {
    if !looks_like_function_key(input) {
        return Ok(None);
    }

    let menu = find_menu(current_menu).expect("validated registry should contain current menu");
    let Some(hint) = find_footer_hint(menu, input) else {
        writeln!(
            output,
            "Function key '{input}' is not supported on menu {}.",
            menu.id
        )?;
        return Ok(Some(FunctionKeyResult::Continue));
    };

    Ok(Some(match hint.action {
        FunctionKeyAction::Exit => {
            writeln!(output, "Function key F3 -> Exit")?;
            writeln!(output, "Session ended.")?;
            FunctionKeyResult::Exit
        }
        FunctionKeyAction::Prompt => {
            writeln!(
                output,
                "Function key F4 -> Prompt. Type a command such as HELP, CRTLIB LIB(MYLIB), DSPLIB LIB(MYLIB), or 90."
            )?;
            FunctionKeyResult::Continue
        }
        FunctionKeyAction::Retrieve => {
            if let Some(previous) = last_direct_input.as_deref() {
                writeln!(output, "Function key F9 -> Retrieve '{previous}'")?;
            } else {
                writeln!(
                    output,
                    "Function key F9 -> No previous command or selection to retrieve."
                )?;
            }
            FunctionKeyResult::Continue
        }
        FunctionKeyAction::Cancel => {
            writeln!(output, "Function key F12 -> Cancel and remain on MAIN.")?;
            FunctionKeyResult::Continue
        }
        FunctionKeyAction::Help => {
            writeln!(
                output,
                "Function key F13 -> Information Assistant. Use HELP for command help or choose a menu option."
            )?;
            FunctionKeyResult::Continue
        }
        FunctionKeyAction::SetInitialMenu => {
            writeln!(
                output,
                "Function key F23 -> Set initial menu is not implemented yet."
            )?;
            FunctionKeyResult::Continue
        }
    }))
}

fn combined_command_hint(input: &str) -> Option<String> {
    let mut parts = input.split_whitespace();
    let first = parts.next()?;
    let second = parts.next()?;
    let second_prefix: String = second
        .chars()
        .take_while(|character| character.is_ascii_alphanumeric())
        .collect();

    if !first
        .chars()
        .all(|character| character.is_ascii_alphanumeric())
    {
        return None;
    }

    if second_prefix.is_empty() {
        return None;
    }

    let combined = format!("{first}{second_prefix}").to_ascii_uppercase();

    find_command(&combined).map(|_| {
        format!(
            "Hint: Rust/400 uses single-token command names. Try {} instead of '{} {}'.",
            combined,
            first.to_ascii_uppercase(),
            second_prefix.to_ascii_uppercase()
        )
    })
}

fn looks_like_function_key(input: &str) -> bool {
    let Some(stripped) = input.strip_prefix(['F', 'f']) else {
        return false;
    };

    !stripped.is_empty() && stripped.chars().all(|character| character.is_ascii_digit())
}

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Permanent(PathBuf),
    Temporary,
}

fn parse_mode(arguments: impl Iterator<Item = std::ffi::OsString>) -> Result<Mode, &'static str> {
    let arguments: Vec<_> = arguments.collect();

    match arguments.as_slice() {
        [flag, path] if flag == "--workspace" => Ok(Mode::Permanent(PathBuf::from(path))),
        [flag] if flag == "--temporary-workspace" => Ok(Mode::Temporary),
        [] => Err("A workspace mode is required."),
        _ => Err("Invalid workspace arguments."),
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::io::Cursor;
    use std::path::PathBuf;

    use rust400::menus::{
        FooterHint, FunctionKeyAction, MenuAction, MenuDefinition, MenuOption,
        validate_menu_registry,
    };
    use rust400::workspace::Workspace;

    use super::{Mode, command_loop, parse_mode, run_interactive_session};

    #[test]
    fn parses_permanent_workspace_mode() {
        let arguments = [
            OsString::from("--workspace"),
            OsString::from("/tmp/rust400"),
        ];

        assert_eq!(
            parse_mode(arguments.into_iter()),
            Ok(Mode::Permanent(PathBuf::from("/tmp/rust400")))
        );
    }

    #[test]
    fn parses_temporary_workspace_mode() {
        let arguments = [OsString::from("--temporary-workspace")];

        assert_eq!(parse_mode(arguments.into_iter()), Ok(Mode::Temporary));
    }

    #[test]
    fn blank_input_is_ignored_without_ending_the_session() {
        let mut input = Cursor::new("\nEXIT\n");
        let mut output = Vec::new();

        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.starts_with("  ===> "));
        assert!(transcript.contains("  ===> Session ended."));
    }

    #[test]
    fn exit_command_closes_the_session_cleanly() {
        let mut input = Cursor::new("exit\n");
        let mut output = Vec::new();

        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Session ended."));
    }

    #[test]
    fn end_of_input_closes_the_session_cleanly() {
        let mut input = Cursor::new("");
        let mut output = Vec::new();

        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert_eq!(transcript, "  ===> \n");
    }

    #[test]
    fn interactive_session_reports_workspace_and_exit_guidance() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut output = Vec::new();

        let status = run_interactive_session(&workspace, Cursor::new("EXIT\n"), &mut output);

        assert_eq!(status, std::process::ExitCode::SUCCESS);

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("IBM i Main Menu"));
        assert!(transcript.contains("Selection or command"));
        assert!(transcript.contains("F3=Exit"));
        assert!(transcript.contains("Workspace: "));
        assert!(transcript.contains("Enter EXIT in the command line to end the session."));
        assert!(transcript.contains("Session ended."));
    }

    #[test]
    fn startup_fails_if_shared_menu_metadata_is_invalid() {
        let invalid = [MenuDefinition {
            id: "BROKEN",
            title: "",
            prompt_label: "Selection or command",
            system_label: "System",
            options: &[MenuOption {
                selector: "1",
                label: "Anything",
                action: MenuAction::OpenMenu("NEXT"),
            }],
            footer_hints: &[FooterHint {
                key: "F3",
                label: "Exit",
                action: FunctionKeyAction::Exit,
            }],
        }];

        assert_eq!(
            validate_menu_registry(&invalid),
            Err("registered menu is missing a title")
        );
    }

    #[test]
    fn help_lists_registered_commands_from_shared_metadata() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("help\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains(
            "Available commands: EXIT, HELP, CRTLIB, DSPLIB, DSPJOB, DSPUSRPRF, SNDMSG, WRKOBJ"
        ));
    }

    #[test]
    fn command_specific_help_comes_from_the_same_metadata_as_validation() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("help cmd(crtlib)\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Command: CRTLIB"));
        assert!(transcript.contains("Summary: Create an emulated library definition."));
        assert!(transcript.contains("- LIB (required): Names the library to create."));
    }

    #[test]
    fn malformed_syntax_reports_a_parser_error() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("crtlib lib(\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Syntax error:"));
    }

    #[test]
    fn invalid_parameters_report_a_validation_error() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("crtlib text('only text')\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Validation error: missing required parameter LIB"));
    }

    #[test]
    fn shared_metadata_builds_a_typed_command_request_for_crtlib() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("crtlib lib(mylib) text('Learning library')\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("CRTLIB created library MYLIB"));
        assert!(transcript.contains("with text 'Learning library'"));
    }

    #[test]
    fn dsplib_displays_a_persisted_library() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input =
            Cursor::new("crtlib lib(mylib) text('Learning library')\ndsplib lib(mylib)\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Library: MYLIB"));
        assert!(transcript.contains("Text: Learning library"));
    }

    #[test]
    fn dsplib_reports_missing_libraries_cleanly() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("dsplib lib(missing)\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Library MISSING does not exist."));
    }

    #[test]
    fn duplicate_library_creation_reports_a_stable_message() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("crtlib lib(mylib)\ncrtlib lib(mylib)\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("CRTLIB created library MYLIB."));
        assert!(transcript.contains("Library MYLIB already exists."));
    }

    #[test]
    fn numeric_main_menu_selection_is_handled_from_menu_metadata() {
        let mut input = Cursor::new("1\nEXIT\n");
        let mut output = Vec::new();

        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Menu selection 1 -> User tasks opens menu 'USR'."));
        assert!(transcript.contains("User Tasks"));
    }

    #[test]
    fn invalid_numeric_selection_reports_menu_specific_error() {
        let mut input = Cursor::new("77\nEXIT\n");
        let mut output = Vec::new();

        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Selection '77' is not valid on menu MAIN."));
    }

    #[test]
    fn direct_commands_still_work_from_the_main_menu_input_field() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("help cmd(crtlib)\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Command: CRTLIB"));
        assert!(transcript.contains("Summary: Create an emulated library definition."));
    }

    #[test]
    fn split_command_names_show_a_helpful_hint() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("snd msg('Hello')\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains(
            "Hint: Rust/400 uses single-token command names. Try SNDMSG instead of 'SND MSG'."
        ));
    }

    #[test]
    fn user_tasks_menu_offers_command_guidance() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("1\n1\n2\n90\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("User Tasks"));
        assert!(transcript.contains("Type DSPUSRPRF to inspect the current Rust/400 profile."));
        assert!(transcript.contains("Type DSPJOB to inspect the current session job."));
        assert!(transcript.contains("Return to main menu"));
    }

    #[test]
    fn dspjob_shows_current_session_identity() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("dspjob\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Job: "));
        assert!(transcript.contains("User: "));
        assert!(transcript.contains("Status: ACTIVE"));
        assert!(transcript.contains("Started: "));
    }

    #[test]
    fn dspusrprf_shows_current_profile_details() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("dspusrprf\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("User profile: "));
        assert!(transcript.contains("Profile status: ENABLED"));
        assert!(transcript.contains("Linux analogy:"));
    }

    #[test]
    fn sign_off_selection_exits_consistently_from_the_menu() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("90\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Menu selection 90 -> Sign off"));
        assert!(transcript.contains("Session ended."));
    }

    #[test]
    fn function_key_f3_exits_the_session() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("F3\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Function key F3 -> Exit"));
        assert!(transcript.contains("Session ended."));
    }

    #[test]
    fn function_key_f4_prompts_for_supported_input() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("F4\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Function key F4 -> Prompt."));
    }

    #[test]
    fn function_key_f9_retrieves_the_previous_direct_input() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("help\nF9\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Function key F9 -> Retrieve 'help'"));
    }

    #[test]
    fn function_key_f12_cancels_gracefully() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("F12\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Function key F12 -> Cancel and remain on MAIN."));
    }

    #[test]
    fn function_key_f13_offers_information_assistant_guidance() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("F13\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Function key F13 -> Information Assistant."));
    }

    #[test]
    fn unsupported_function_key_fails_gracefully() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("F5\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Function key 'F5' is not supported on menu MAIN."));
    }
}
