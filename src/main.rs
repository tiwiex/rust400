use std::env;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use rust400::commands::{
    CommandRequest, build_request, find_command, registered_commands, render_help,
    validate_registry_metadata,
};
use rust400::libraries::{create_library, find_library, list_libraries};
use rust400::linux_views::{
    display_linux_path, list_linux_directory_entries, read_host_linux_users,
};
use rust400::mappings::{find_mapping, find_mapping_for_menu, render_mapping};
use rust400::menus::{
    FunctionKeyAction, MenuAction, find_footer_hint, find_menu, find_option, registered_menus,
    validate_menu_registry,
};
use rust400::parser::parse_command;
use rust400::session::SessionContext;
use rust400::ui::{
    INPUT_PROMPT, MenuScreen, render_detail_screen, render_green_detail_screen,
    render_green_screen, render_menu,
};
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
    let session_linux_path = determine_session_linux_path(workspace.root());

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
                FunctionKeyResult::Render(menu_id) => {
                    current_menu = menu_id;
                    render_active_screen(output, workspace, session, current_menu)?;
                    continue;
                }
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
                        let mut lines = Vec::new();
                        if let Some(command_name) = request.command {
                            if let Some(help_definition) = find_command(&command_name) {
                                lines.extend(
                                    render_help(help_definition).lines().map(str::to_string),
                                );
                            } else {
                                lines.push(format!(
                                    "Command '{command_name}' is not registered yet."
                                ));
                            }
                        } else {
                            lines.push(format!(
                                "Available commands: {}",
                                registered_commands()
                                    .iter()
                                    .map(|definition| definition.name)
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ));
                        }
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Command help",
                            &lines,
                        )?;
                    }
                    Ok(CommandRequest::LinuxMapping(request)) => {
                        if let Some(mapping) = find_mapping(&request.term) {
                            let lines = render_mapping(mapping)
                                .lines()
                                .map(str::to_string)
                                .collect::<Vec<_>>();
                            render_command_result(
                                output,
                                workspace,
                                session,
                                current_menu,
                                "Linux mapping result",
                                &lines,
                            )?;
                        } else {
                            render_command_result(
                                output,
                                workspace,
                                session,
                                current_menu,
                                "Linux mapping result",
                                &[format!(
                                    "Linux mapping term '{}' is not registered yet.",
                                    request.term
                                )],
                            )?;
                        }
                    }
                    Ok(CommandRequest::DisplayWorkspacePath) => {
                        let lines = display_linux_path(&session_linux_path)
                            .lines()
                            .map(str::to_string)
                            .collect::<Vec<_>>();
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Linux directory view",
                            &lines,
                        )?;
                    }
                    Ok(CommandRequest::DisplayWorkspaceListing) => {
                        let mut lines = vec![
                            "Linux host view: entries inside the current session directory"
                                .to_string(),
                            format!("Directory: {}", session_linux_path.display()),
                            "Rust/400 note: this lists host Linux files and directories, not emulated libraries or objects.".to_string(),
                        ];
                        match list_linux_directory_entries(&session_linux_path) {
                            Ok(entries) if entries.is_empty() => {
                                lines.push(
                                    "No files or directories exist in this Linux directory."
                                        .to_string(),
                                );
                            }
                            Ok(entries) => {
                                for entry in entries {
                                    lines.push(format!("- {:<7} {}", entry.kind, entry.name));
                                }
                            }
                            Err(error) => {
                                lines.push(format!(
                                    "Could not list Linux directory entries: {error}"
                                ));
                            }
                        }
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Linux directory view",
                            &lines,
                        )?;
                    }
                    Ok(CommandRequest::DisplayLinuxUsers) => {
                        let mut lines = vec![
                            "Linux host view: user account summary".to_string(),
                            "Rust/400 note: Linux accounts and Rust/400 user profiles are related learning concepts, not the same authority model.".to_string(),
                        ];
                        match read_host_linux_users() {
                            Ok(users) if users.is_empty() => {
                                lines.push("No Linux user accounts were discovered.".to_string());
                            }
                            Ok(users) => {
                                for user in users.into_iter().take(12) {
                                    lines.push(format!(
                                        "- {name} uid={uid} gid={gid} home={home} shell={shell}",
                                        name = user.username,
                                        uid = user.uid,
                                        gid = user.gid,
                                        home = user.home,
                                        shell = user.shell
                                    ));
                                }
                                lines.push(
                                    "Displayed the first 12 entries from /etc/passwd for learning purposes."
                                        .to_string(),
                                );
                            }
                            Err(error) => {
                                lines.push(format!("Could not read Linux users: {error}"))
                            }
                        }
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Linux user summary",
                            &lines,
                        )?;
                    }
                    Ok(CommandRequest::CreateLibrary(request)) => {
                        let lines = match create_library(
                            workspace,
                            &request.library,
                            request.text.as_deref(),
                        ) {
                            Ok(record) => vec![format!(
                                "CRTLIB created library {}{}.",
                                record.name,
                                record
                                    .text
                                    .as_deref()
                                    .map(|text| format!(" with text '{text}'"))
                                    .unwrap_or_default()
                            )],
                            Err(error) => vec![error.to_string()],
                        };
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Command result",
                            &lines,
                        )?;
                    }
                    Ok(CommandRequest::DisplayLibrary(request)) => {
                        let lines = match find_library(workspace, &request.library) {
                            Ok(Some(record)) => {
                                vec![
                                    format!("Library: {}", record.name),
                                    format!("Text: {}", record.text.as_deref().unwrap_or("*NONE")),
                                    format!("Created: {}", record.created_at_epoch_seconds),
                                ]
                            }
                            Ok(None) => {
                                vec![format!("Library {} does not exist.", request.library)]
                            }
                            Err(error) => vec![error.to_string()],
                        };
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Command result",
                            &lines,
                        )?;
                    }
                    Ok(CommandRequest::WorkLibrary) => {
                        let mut lines = Vec::new();
                        match list_libraries(workspace) {
                            Ok(libraries) if libraries.is_empty() => {
                                lines.push(
                                    "No libraries exist in the current Rust/400 workspace."
                                        .to_string(),
                                );
                                lines.push(
                                    "Try next: CRTLIB LIB(MYLIB) TEXT('Learning library')"
                                        .to_string(),
                                );
                            }
                            Ok(libraries) => {
                                lines.push("Libraries in current workspace:".to_string());
                                for library in libraries {
                                    lines.push(format!(
                                        "- {}{}",
                                        library.name,
                                        library
                                            .text
                                            .as_deref()
                                            .map(|text| format!(" -- {text}"))
                                            .unwrap_or_default()
                                    ));
                                }
                            }
                            Err(error) => lines.push(error.to_string()),
                        }
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Command result",
                            &lines,
                        )?;
                    }
                    Ok(CommandRequest::DisplayJob) => {
                        let lines = vec![
                            format!("Job: {}", session.job_name()),
                            format!("User: {}", session.current_user()),
                            format!("Status: {}", session.status()),
                            format!("Started: {}", session.started_at_epoch_seconds()),
                            format!("Workspace: {}", workspace.root().display()),
                        ];
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Command result",
                            &lines,
                        )?;
                    }
                    Ok(CommandRequest::DisplayUserProfile) => {
                        let lines = vec![
                            format!("User profile: {}", session.current_user()),
                            "Profile status: ENABLED".to_string(),
                            format!("Current job: {}", session.job_name()),
                            "Linux analogy: similar to the signed-in shell user, but contained inside Rust/400.".to_string(),
                        ];
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Command result",
                            &lines,
                        )?;
                    }
                    Ok(CommandRequest::SendMessage(request)) => {
                        let mut lines = vec![format!(
                            "SNDMSG would send '{}' to {} recipient(s).",
                            request.message,
                            request.recipients.len()
                        )];
                        if request.recipients.is_empty() {
                            lines.push("Hint: add TO(name) for a recipient, for example SNDMSG MSG('Hello') TO(QSYSOPR).".to_string());
                        }
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Command result",
                            &lines,
                        )?;
                    }
                    Ok(CommandRequest::WorkObject(request)) => {
                        let lines = vec![format!(
                            "Command 'WRKOBJ' is mapped to handler {:?} with LIB({}) OBJ({}).",
                            definition.handler,
                            request.library.as_deref().unwrap_or("*NONE"),
                            request.object.as_deref().unwrap_or("*NONE")
                        )];
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Command result",
                            &lines,
                        )?;
                    }
                    Err(error) => {
                        render_command_result(
                            output,
                            workspace,
                            session,
                            current_menu,
                            "Validation error",
                            &[format!("Validation error: {error}")],
                        )?;
                    }
                }
            }
            Err(error) => {
                let mut lines = vec![format!("Syntax error: {error}")];
                if let Some(suggestion) = combined_command_hint(command) {
                    lines.push(suggestion);
                }
                render_command_result(
                    output,
                    workspace,
                    session,
                    current_menu,
                    "Syntax error",
                    &lines,
                )?;
            }
        }
    }
}

fn render_command_result(
    output: &mut impl Write,
    workspace: &Workspace,
    session: &SessionContext,
    menu_id: &str,
    section_title: &str,
    detail_lines: &[String],
) -> io::Result<()> {
    let menu = find_menu(menu_id).expect("validated registry should contain rendered menu");
    let screen = MenuScreen {
        menu,
        system_name: "RUST400",
        current_user: session.current_user(),
        job_name: session.job_name(),
    };

    write!(
        output,
        "{}",
        render_green_detail_screen(&screen, section_title, detail_lines)
    )?;
    writeln!(output, "  Workspace: {}", workspace.root().display())?;
    writeln!(
        output,
        "  Enter EXIT in the command line to end the session."
    )?;
    Ok(())
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

    let rendered_screen = if let Some(mapping) = find_mapping_for_menu(menu_id) {
        let detail_lines = render_mapping(mapping)
            .lines()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        render_detail_screen(&screen, "Linux mapping detail", &detail_lines)
    } else {
        render_menu(&screen)
    };

    write!(
        output,
        "{}",
        if find_mapping_for_menu(menu_id).is_some() {
            format!(
                "\x1b[2J\x1b[H\x1b[40m\x1b[92m{}{reset}",
                rendered_screen,
                reset = "\x1b[0m"
            )
        } else {
            render_green_screen(&screen)
        }
    )?;
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
            MenuAction::RunCommand("DSPPWD") => {
                writeln!(
                    output,
                    "Menu selection {} -> {}. Type DSPPWD to show the current Linux directory for this session.",
                    option.selector, option.label
                )?;
                MenuSelectionResult::Continue
            }
            MenuAction::RunCommand("DSPLS") => {
                writeln!(
                    output,
                    "Menu selection {} -> {}. Type DSPLS to list files and directories inside the current Linux directory.",
                    option.selector, option.label
                )?;
                MenuSelectionResult::Continue
            }
            MenuAction::RunCommand("DSPUSRS") => {
                writeln!(
                    output,
                    "Menu selection {} -> {}. Type DSPUSRS to show a read-only Linux user summary.",
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
    Render(&'static str),
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
            if find_mapping_for_menu(current_menu).is_some() {
                writeln!(
                    output,
                    "Function key F12 -> Cancel and return to Linux mappings."
                )?;
                FunctionKeyResult::Render("LNX")
            } else {
                writeln!(output, "Function key F12 -> Cancel and remain on MAIN.")?;
                FunctionKeyResult::Continue
            }
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

fn determine_session_linux_path(workspace_root: &Path) -> PathBuf {
    env::current_dir()
        .ok()
        .and_then(|path| path.canonicalize().ok())
        .unwrap_or_else(|| workspace_root.to_path_buf())
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
            "Available commands: EXIT, HELP, LNXMAP, DSPPWD, DSPLS, DSPUSRS, CRTLIB, DSPLIB, WRKLIB, DSPJOB, DSPUSRPRF, SNDMSG, WRKOBJ"
        ));
    }

    #[test]
    fn dsppwd_reports_the_session_linux_path() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("dsppwd\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Linux host view: current session directory"));
        let expected = std::env::current_dir()
            .expect("current dir should exist")
            .canonicalize()
            .expect("current dir should canonicalize");
        assert!(transcript.contains(expected.to_string_lossy().as_ref()));
    }

    #[test]
    fn dspls_lists_linux_directory_entries_without_claiming_library_equivalence() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("dspls\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(
            transcript.contains("Linux host view: entries inside the current session directory")
        );
        assert!(transcript.contains("not emulated libraries or objects"));
        assert!(transcript.contains("Directory: "));
        assert!(transcript.contains("Cargo.toml"));
    }

    #[test]
    fn dspurss_displays_linux_user_summary_from_host_data() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("dspusrs\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Linux host view: user account summary"));
        assert!(transcript.contains("not the same authority model"));
        assert!(transcript.contains("Displayed the first 12 entries from /etc/passwd"));
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
    fn wrklib_lists_existing_libraries() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new(
            "crtlib lib(mylib) text('Learning library')\ncrtlib lib(testlib)\nwrklib\nEXIT\n",
        );
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Libraries in current workspace:"));
        assert!(transcript.contains("- MYLIB -- Learning library"));
        assert!(transcript.contains("- TESTLIB"));
    }

    #[test]
    fn wrklib_reports_an_empty_catalog_cleanly() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("wrklib\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("No libraries exist in the current Rust/400 workspace."));
        assert!(transcript.contains("Try next: CRTLIB LIB(MYLIB) TEXT('Learning library')"));
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
    fn linux_mappings_menu_opens_from_main_menu() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("12\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Menu selection 12 -> Linux mappings opens menu 'LNX'."));
        assert!(transcript.contains("Linux Mappings"));
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
    fn lnxmap_displays_a_registered_mapping_card() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("lnxmap term(path)\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Linux term: PATH"));
        assert!(transcript.contains("Rust/400 concept: Library list"));
        assert!(transcript.contains("Comparable Linux example:"));
    }

    #[test]
    fn linux_mappings_menu_runs_a_predefined_lookup() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("12\n1\n90\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Linux Mappings"));
        assert!(
            transcript.contains("Menu selection 1 -> PATH and library lists opens menu 'LNXPATH'.")
        );
        assert!(transcript.contains("Linux term: PATH"));
        assert!(transcript.contains("Rust/400 command: DSPLIBL"));
        assert!(
            transcript.contains("Menu selection 90 -> Return to Linux mappings opens menu 'LNX'.")
        );
    }

    #[test]
    fn linux_mapping_detail_f12_returns_to_linux_mappings() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut input = Cursor::new("12\n1\nF12\nEXIT\n");
        let mut output = Vec::new();

        command_loop(&workspace, &mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Linux term: PATH"));
        assert!(transcript.contains("Function key F12 -> Cancel and return to Linux mappings."));
        assert!(transcript.contains("Linux Mappings"));
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
