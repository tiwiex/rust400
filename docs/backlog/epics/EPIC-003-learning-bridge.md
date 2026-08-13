# EPIC-003: OS/400-to-Linux learning bridge

## Outcome

Returning operators and Linux-first learners can navigate between OS/400-inspired concepts and Linux analogies without mistaking those analogies for exact equivalence.

## Stories

### US-003-01: Browse the concept glossary

**As a** systems learner, **I want** a glossary of OS/400-inspired concepts **so that** I can build the vocabulary needed to use Rust/400.

- Requirements: FR-025
- Priority: Must
- Dependencies: US-002-03, US-002-04

Acceptance criteria:

1. The glossary lists every MVP domain concept with a plain-language definition.
2. A user can open a concept from built-in help without knowing its documentation path.
3. Related commands and concepts are linked from each entry.

### US-003-02: Compare a concept with Linux

**As a** Linux-first learner, **I want** the nearest Linux analogy and its limitations **so that** I can transfer existing knowledge accurately.

- Requirements: FR-024, FR-026, FR-028
- Priority: Must
- Dependencies: US-003-01

Acceptance criteria:

1. Every MVP concept includes a Linux analogy, a shared idea, and a “where the analogy breaks” explanation.
2. Each entry includes distinct Rust/400 and Linux examples.
3. Linux examples are displayed but never automatically executed.
4. Each entry labels behavior as emulated, historically inspired, or intentionally different where relevant.

### US-003-03: Navigate from Linux to Rust/400

**As a** Linux-first learner, **I want** to start from concepts such as `PATH`, processes, and permissions **so that** I can find the corresponding Rust/400 lessons.

- Requirements: FR-027; MVP acceptance criterion 12
- Priority: Should
- Dependencies: US-003-01, US-003-02

Acceptance criteria:

1. Lookup supports at least `PATH`, process, user, permissions, print spool, filesystem, service, and message queue terms.
2. Each Linux term links to one or more relevant Rust/400 concepts.
3. Ambiguous analogies present alternatives rather than claiming one exact mapping.

### US-003-04: Maintain learning content from one source

**As a** maintainer, **I want** one structured source for terminal help and published learning documentation **so that** examples and terminology stay consistent.

- Requirements: FR-005, FR-024, FR-025; NFR maintainability
- Priority: Should
- Dependencies: US-003-01

Acceptance criteria:

1. Concept content has a documented schema and validation rules.
2. Built-in help and generated documentation use the same source content.
3. Automated checks detect missing mappings, limitation notes, examples, or broken concept links.

### US-003-05: View controlled Linux read-only details

**As a** returning operator or Linux-first learner, **I want** safe read-only views of selected Linux state **so that** I can connect Rust/400 concepts to the host system without turning the emulator into a general shell.

- Requirements: FR-001, FR-023, FR-024, FR-026, FR-027, FR-028
- Priority: Should
- Dependencies: US-002-03, US-003-02, US-003-03

Acceptance criteria:

1. A user can display the current Linux directory context for the session through a documented Rust/400 command without invoking a host shell.
2. A user can list files and directories inside the current Linux directory through a documented Rust/400 command, and the output labels the view as a Linux analogy rather than an OS/400-equivalent object catalog.
3. A user can view a read-only summary of Linux user accounts from host data through a documented Rust/400 command, with clear wording that Rust/400 profiles and Linux accounts are not the same authority model.
4. The implementation does not accept arbitrary shell input or mutate host users, files, or system configuration.
