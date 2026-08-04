# EPIC-006: Spooled output and batch execution

## Outcome

Users can retain report output for later inspection and execute repeatable command files with dependable exit behavior.

## Stories

### US-006-01: Create text spooled files

**As a** returning operator, **I want** designated reports retained as spooled files **so that** I can inspect output after the command completes.

- Requirements: FR-015
- Priority: Must
- Dependencies: US-004-01, US-005-01

Acceptance criteria:

1. A designated report command creates one spooled record containing text, job identity, owner, creation time, status, and unique number.
2. Creation is atomic with the command's successful result.
3. A failed report command does not leave an incomplete spooled file.

### US-006-02: List and display spooled output

**As a** user, **I want** to list and view spooled files **so that** I can retrieve prior report output.

- Requirements: FR-015
- Priority: Must
- Dependencies: US-006-01

Acceptance criteria:

1. `WRKSPLF` lists available files with enough metadata to select one unambiguously.
2. `DSPSPLF` renders the selected text without interpreting it as a host command.
3. Missing, inaccessible, and ambiguous selections return distinct stable messages.

### US-006-03: Execute one command non-interactively

**As an** automation user, **I want** to run one command without opening a prompt **so that** Rust/400 can participate in scripts and tests.

- Requirements: FR-002, FR-018
- Priority: Must
- Dependencies: US-002-03

Acceptance criteria:

1. A documented CLI option executes exactly one Rust/400 command and then exits.
2. Success returns `0`; parsing, validation, domain, and system failures return documented nonzero exit codes.
3. Normal output and diagnostics use separate streams suitable for automation.

### US-006-04: Execute a UTF-8 command script

**As a** systems learner, **I want** commands read from a text file **so that** demonstrations and experiments are repeatable.

- Requirements: FR-002, FR-018, FR-023, FR-026
- Priority: Must
- Dependencies: US-006-03, US-005-04

Acceptance criteria:

1. Commands execute in file order with documented handling of blank lines and comments.
2. On failure, output identifies the source line, command, and stable message ID.
3. The documented stop/continue policy is applied consistently and reflected in the final exit code.
4. Script contents can invoke only registered Rust/400 commands, never arbitrary host shell commands.

