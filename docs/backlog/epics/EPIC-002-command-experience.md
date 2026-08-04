# EPIC-002: Command experience

## Outcome

Users can enter structured AS/400-inspired commands from a shared command line, understand their syntax, receive useful results, and recover from mistakes.

This epic now serves the command engine beneath the full-screen experience. Screen layout, menu flow, and function-key behavior should be handled by a dedicated presentation epic so command semantics and terminal rendering can evolve without drifting apart.

## Stories

### US-002-01: Run an interactive command loop

**As a** returning operator, **I want** an interactive command line within the emulator session **so that** I can issue commands in a familiar workflow alongside menu navigation.

- Requirements: FR-001, FR-012
- Priority: Must
- Dependencies: US-001-01, US-001-03

Acceptance criteria:

1. Given an initialized workspace, when the application starts interactively, then it accepts successive commands through the emulator's command-entry path.
2. `EXIT` and end-of-input close the session cleanly.
3. Empty input does not fail or mutate state.

### US-002-02: Parse CL-like command syntax

**As a** returning operator, **I want** case-insensitive commands with keyword parameters **so that** commands resemble the OS/400 interaction model.

- Requirements: FR-003, FR-004
- Priority: Must
- Dependencies: US-002-01

Acceptance criteria:

1. The parser accepts documented command names, parentheses, quoting, whitespace, and keyword parameters.
2. Equivalent unquoted identifiers in different letter cases normalize identically.
3. Missing, duplicate, mutually exclusive, and unknown parameters produce structured validation errors.
4. Parser tests cover the documented syntax branches and malformed-input boundaries.

### US-002-03: Define commands from shared metadata

**As a** maintainer, **I want** execution, validation, help, and completion driven by shared command definitions **so that** they cannot silently disagree.

- Requirements: FR-004, FR-005
- Priority: Must
- Dependencies: US-002-02

Acceptance criteria:

1. A command definition describes its name, summary, parameters, validation rules, and handler identity.
2. Validation and help consume the same definition.
3. A test fails if a registered command lacks required documentation metadata.

### US-002-04: Discover commands and help

**As a** systems learner, **I want** summary and command-specific help **so that** I can learn command behavior without external documentation.

- Requirements: FR-017, FR-019
- Priority: Must for help; Should for completion
- Dependencies: US-002-03

Acceptance criteria:

1. `HELP` explains how to discover and run commands.
2. `WRKCMD` lists all registered commands with concise descriptions.
3. Command-specific help displays syntax, parameters, defaults, examples, and related concepts.
4. Interactive completion suggests registered commands and valid parameter names where terminal support permits it.

### US-002-05: Receive actionable command errors

**As a** user, **I want** stable and helpful error messages **so that** I can correct input without losing work.

- Requirements: FR-013, FR-014
- Priority: Must
- Dependencies: US-002-02, US-002-03

Acceptance criteria:

1. Unknown commands and invalid parameters return stable message IDs, severity, cause, and corrective guidance.
2. Validation failures leave persistent state unchanged.
3. Expected input errors do not produce an unhandled panic or developer backtrace in normal output.
