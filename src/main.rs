use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use rust400::workspace::Workspace;

const STARTUP_MESSAGE: &str =
    "Rust/400 development shell: initialization complete; no commands are available yet.";
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
            println!("{STARTUP_MESSAGE}");
            println!("Workspace: {}", workspace.root().display());
            ExitCode::SUCCESS
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
            println!("{STARTUP_MESSAGE}");
            println!("Temporary workspace: {}", workspace.root().display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Could not initialize Rust/400: {error}");
            ExitCode::FAILURE
        }
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
    use std::path::PathBuf;

    use super::{Mode, STARTUP_MESSAGE, parse_mode};

    #[test]
    fn startup_message_identifies_the_placeholder_state() {
        assert!(STARTUP_MESSAGE.starts_with("Rust/400 development shell:"));
        assert!(STARTUP_MESSAGE.contains("no commands are available yet"));
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
}
