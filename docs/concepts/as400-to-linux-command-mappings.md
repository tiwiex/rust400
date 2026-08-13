# AS/400-style to Linux command mappings

Rust/400 uses these mappings as a learning bridge, not as a claim of compatibility. Linux examples are displayed as educational examples only. Rust/400 does not execute them automatically.

## PATH

- Rust/400 concept: Library list
- Rust/400 command: `DSPLIBL`
- Linux analogy: ordered search path
- Shared idea: both resolve names by checking locations in order
- Where the analogy breaks: `PATH` mainly locates executables; a library list resolves typed objects across libraries
- Rust/400 example: `DSPLIBL`
- Linux example: `printf '%s\n' "$PATH" | tr ':' '\n'`
- Behavior label: Historically inspired

## Process

- Rust/400 concept: Job
- Rust/400 command: `DSPJOB`
- Linux analogy: process plus session metadata
- Shared idea: both describe active work and its owner
- Where the analogy breaks: a job carries richer operational context than a single Linux process
- Rust/400 example: `DSPJOB`
- Linux example: `ps -ef`
- Behavior label: Emulated

## User

- Rust/400 concept: User profile
- Rust/400 command: `DSPUSRPRF`
- Linux analogy: user account
- Shared idea: both represent an identity associated with work and permissions
- Where the analogy breaks: Rust/400 user profiles are emulator identities and do not claim host authentication
- Rust/400 example: `DSPUSRPRF`
- Linux example: `id && whoami`
- Behavior label: Intentionally different

## Filesystem

- Rust/400 concept: Libraries and the IFS analogy
- Rust/400 command: `WRKLIB`
- Linux analogy: filesystem hierarchy with directories
- Shared idea: both organize named resources
- Where the analogy breaks: a library is a managed catalog concept, not just a directory
- Rust/400 example: `WRKLIB`
- Linux example: `find . -maxdepth 2 -type d`
- Behavior label: Historically inspired

## Print spool

- Rust/400 concept: Spooled files
- Rust/400 command: `WRKSPLF`
- Linux analogy: print spool or queued text report
- Shared idea: output can be retained for later inspection
- Where the analogy breaks: Rust/400 spooled files are managed emulator records rather than arbitrary files
- Rust/400 example: `WRKSPLF`
- Linux example: `lpq`
- Behavior label: Planned

## Message queue

- Rust/400 concept: Operational messages
- Rust/400 command: `DSPMSG`
- Linux analogy: mailbox, notification queue, or log stream
- Shared idea: information can be sent now and inspected later
- Where the analogy breaks: Rust/400 messages are tied to emulator workflows rather than a single Linux facility
- Rust/400 example: `DSPMSG`
- Linux example: `journalctl -n 20`
- Behavior label: Planned
