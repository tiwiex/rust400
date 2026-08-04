# EPIC-005: Jobs, messages, and history

## Outcome

Every session is an inspectable job that produces structured operational messages and a safe audit history.

## Stories

### US-005-01: Identify each session as a job

**As a** returning operator, **I want** each shell session to have a job identity **so that** work and output have an operational context.

- Requirements: FR-012
- Priority: Must
- Dependencies: US-002-01

Acceptance criteria:

1. A session receives a unique job identity, owner, start time, and status.
2. `DSPJOB` shows the current job's documented attributes.
3. The job reaches a terminal status and end time on clean session closure.

### US-005-02: Use a structured message catalog

**As a** user, **I want** consistent informational, warning, and error messages **so that** I can interpret and troubleshoot results.

- Requirements: FR-013
- Priority: Must
- Dependencies: US-002-05

Acceptance criteria:

1. Messages have stable IDs, severity, summary, and optional cause and recovery text.
2. Message IDs are unique and validated by automated tests.
3. Rendered messages remain readable without color and in non-interactive mode.

### US-005-03: Send and inspect local messages

**As a** returning operator, **I want** to send and inspect emulator messages **so that** I can learn the operational-message model.

- Requirements: FR-013; MVP scope messages
- Priority: Must
- Dependencies: US-005-01, US-005-02

Acceptance criteria:

1. `SNDMSG` creates a message for a supported local recipient or queue.
2. `DSPMSG` lists and displays messages in a documented order.
3. Missing recipients and invalid message text produce stable errors without creating a message.

### US-005-04: Review redacted command history

**As a** user, **I want** to review commands and outcomes **so that** I can reproduce work and understand failures.

- Requirements: FR-016
- Priority: Must
- Dependencies: US-005-01, US-005-02

Acceptance criteria:

1. `HISTORY` shows timestamp, job identity, normalized command, and outcome.
2. Secret-bearing parameters are redacted before persistence and rendering.
3. History retention is bounded or configurable, and large output is paginated or limited.

