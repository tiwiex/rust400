# EPIC-007: MVP hardening and release

## Outcome

Rust/400 v0.1 is safe, measurable, documented, and reproducible for its supported Linux audience.

## Stories

### US-007-01: Prove critical workflows end to end

**As a** maintainer, **I want** automated critical-path tests **so that** releases demonstrate the PRD acceptance criteria.

- Requirements: MVP acceptance criteria 1-12; NFR reliability
- Priority: Must
- Dependencies: All Must stories in EPIC-002 through EPIC-006

Acceptance criteria:

1. Disposable-workspace tests cover initialization, help, library creation, resolution, object inspection, error recovery, spooling, learning help, and batch execution.
2. Every MVP acceptance criterion links to passing automated evidence or a documented manual check.
3. Tests are deterministic and leave no state outside their disposable workspaces.

### US-007-02: Validate security boundaries

**As a** user, **I want** adversarial inputs contained **so that** using the emulator does not endanger my Linux account.

- Requirements: NFR-SEC-001 through NFR-SEC-005, FR-023, FR-026
- Priority: Must
- Dependencies: US-001-03, US-006-04

Acceptance criteria:

1. Tests cover traversal, absolute paths, symlink escape, unsafe overwrite, command injection, and malformed persisted data.
2. Rust/400 requires no root privileges and invokes no arbitrary host command from emulated input.
3. Any `unsafe` code is absent or linked to an approved ADR and targeted safety evidence.

### US-007-03: Establish a performance baseline

**As a** maintainer, **I want** repeatable performance measurements **so that** responsiveness targets are evidence-based.

- Requirements: PRD success measures; NFR performance
- Priority: Must
- Dependencies: US-004-05, US-005-04

Acceptance criteria:

1. A documented benchmark dataset contains 100 libraries, 10,000 objects, and 10,000 history entries.
2. Startup, metadata-operation latency, and large-list memory behavior are measured reproducibly.
3. Results and deviations from PRD targets are recorded in the release build log.

### US-007-04: Publish user and contributor documentation

**As a** new user or contributor, **I want** complete getting-started guidance **so that** I can use or change Rust/400 without hidden knowledge.

- Requirements: MVP acceptance criteria 1, 2, and 9
- Priority: Must
- Dependencies: All MVP behavior stories

Acceptance criteria:

1. Documentation covers installation, first session, commands, learning bridge, configuration, backup/recovery, known limitations, and troubleshooting.
2. Contributor guidance covers backlog flow, branches, tests, ADRs, build logs, and pull requests.
3. All documented commands and links are checked manually or automatically before release.

### US-007-05: Produce the v0.1 release candidate

**As a** product owner, **I want** a traceable release candidate **so that** I can decide whether the MVP is ready to publish.

- Requirements: All Must requirements and MVP acceptance criteria
- Priority: Must
- Dependencies: US-007-01 through US-007-04

Acceptance criteria:

1. The release candidate is built from a reviewed revision with all required checks passing.
2. Release notes list delivered stories, intentional limitations, known issues, migration/backup guidance, and supported environment.
3. A release build log links requirements, stories, evidence, artifacts, and unresolved risks.
4. Product-owner approval or rejection and its rationale are recorded.

