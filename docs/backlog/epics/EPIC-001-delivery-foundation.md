# EPIC-001: Delivery foundation

## Outcome

Contributors can build, test, change, review, and release Rust/400 through a repeatable workflow with traceability from requirement to evidence.

## Scope and success

This epic establishes the repository, Rust project, automated checks, architectural boundaries, documentation conventions, and disposable workspace needed by later vertical slices. It succeeds when a new contributor can follow the documented workflow without undocumented setup.

## Stories

### US-001-01: Initialize the Rust project

**As a** Rust contributor, **I want** a minimal buildable Rust application with documented toolchain expectations **so that** I can begin from a reproducible baseline.

- Requirements: NFR portability and maintainability; MVP acceptance criteria 1 and 8
- Priority: Must
- Dependencies: None

Acceptance criteria:

1. Given the documented supported Rust toolchain, when a contributor runs the standard build and test commands, then both complete successfully.
2. The application starts and returns a deliberate placeholder response without panicking.
3. Repository documentation states how to build, test, format, and lint the project.

### US-001-02: Establish automated quality checks

**As a** maintainer, **I want** formatting, linting, tests, and dependency checks automated **so that** pull requests receive consistent evidence.

- Requirements: NFR reliability and maintainability; MVP acceptance criterion 8
- Priority: Must
- Dependencies: US-001-01

Acceptance criteria:

1. A single documented local command runs the required quality checks.
2. CI runs formatting, linting, and tests for every proposed change.
3. A failed check prevents the workflow from reporting success and identifies the failing stage.

### US-001-03: Create isolated emulator workspaces

**As a** user or tester, **I want** Rust/400 state contained in an explicit workspace **so that** experiments cannot alter unrelated host data.

- Requirements: FR-006, FR-021, NFR-SEC-001, NFR-SEC-002, NFR-SEC-003
- Priority: Must
- Dependencies: US-001-01

Acceptance criteria:

1. Given a configured workspace, when Rust/400 initializes, then all mutable emulator state is created beneath it.
2. A disposable workspace can be selected for tests and demonstrations.
3. Path traversal and symlink-escape tests prove a crafted input cannot write outside the workspace.

### US-001-04: Record architecture decisions

**As a** contributor, **I want** significant technical decisions recorded as ADRs **so that** later contributors understand the context and tradeoffs.

- Requirements: NFR maintainability and observability
- Priority: Must
- Dependencies: None

Acceptance criteria:

1. The repository contains an ADR template, index, and documented status lifecycle.
2. The persistence and command-definition choices are recorded before their implementations are merged.
3. Superseded decisions link to their replacements rather than being silently rewritten.

### US-001-05: Trace work through branches and build logs

**As a** business-analysis learner, **I want** stories linked to branches, pull requests, evidence, and build logs **so that** I can study the complete delivery process.

- Requirements: Product goal 6 and PRD section 15
- Priority: Must
- Dependencies: None

Acceptance criteria:

1. Story, branch, pull-request, and build-log templates request the same stable story ID.
2. A worked example demonstrates navigation from a PRD requirement to implementation evidence.
3. The Definition of Done prevents closing a story without tests, documentation, and build-log evidence where applicable.

