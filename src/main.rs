use std::env;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use rust400::workspace::Workspace;

const STARTUP_MESSAGE: &str = "Rust/400 interactive shell: initialization complete.";
const PROMPT: &str = "R400> ";
const EXIT_COMMAND: &str = "EXIT";
const USAGE: &str = "Usage: rust400 (--workspace <absolute-path> | --temporary-workspace)";

fn main() -> ExitCode {
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
    if let Err(error) = write_startup(&mut output, workspace) {
        eprintln!("Could not start Rust/400: {error}");
        return ExitCode::FAILURE;
    }

    match command_loop(&mut input, &mut output) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Could not continue Rust/400: {error}");
            ExitCode::FAILURE
        }
    }
}

fn write_startup(output: &mut impl Write, workspace: &Workspace) -> io::Result<()> {
    writeln!(output, "{STARTUP_MESSAGE}")?;
    writeln!(output, "Workspace: {}", workspace.root().display())?;
    writeln!(
        output,
        "Type {EXIT_COMMAND} to end the session. Additional commands will arrive in later stories."
    )?;
    Ok(())
}

fn command_loop(input: &mut impl BufRead, output: &mut impl Write) -> io::Result<()> {
    let mut line = String::new();

    loop {
        write!(output, "{PROMPT}")?;
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

        if command.eq_ignore_ascii_case(EXIT_COMMAND) {
            writeln!(output, "Session ended.")?;
            return Ok(());
        }

        writeln!(
            output,
            "Command '{command}' is not available yet. Type {EXIT_COMMAND} to close the session."
        )?;
    }
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

    use rust400::workspace::Workspace;

    use super::{
        EXIT_COMMAND, Mode, PROMPT, STARTUP_MESSAGE, command_loop, parse_mode,
        run_interactive_session,
    };

    #[test]
    fn startup_message_identifies_the_interactive_shell() {
        assert_eq!(
            STARTUP_MESSAGE,
            "Rust/400 interactive shell: initialization complete."
        );
    }

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

        command_loop(&mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.starts_with(PROMPT));
        assert!(transcript.contains(&format!("{PROMPT}Session ended.")));
    }

    #[test]
    fn exit_command_closes_the_session_cleanly() {
        let mut input = Cursor::new("exit\n");
        let mut output = Vec::new();

        command_loop(&mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains("Session ended."));
    }

    #[test]
    fn end_of_input_closes_the_session_cleanly() {
        let mut input = Cursor::new("");
        let mut output = Vec::new();

        command_loop(&mut input, &mut output).expect("session should complete");

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert_eq!(transcript, format!("{PROMPT}\n"));
    }

    #[test]
    fn interactive_session_reports_workspace_and_exit_guidance() {
        let workspace = Workspace::temporary().expect("temporary workspace should exist");
        let mut output = Vec::new();

        let status = run_interactive_session(&workspace, Cursor::new("EXIT\n"), &mut output);

        assert_eq!(status, std::process::ExitCode::SUCCESS);

        let transcript = String::from_utf8(output).expect("session output should be utf-8");
        assert!(transcript.contains(STARTUP_MESSAGE));
        assert!(transcript.contains("Workspace: "));
        assert!(transcript.contains(&format!("Type {EXIT_COMMAND} to end the session.")));
        assert!(transcript.contains("Session ended."));
    }
}
