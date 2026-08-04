# Build Log: Establish automated quality checks

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Project owner and Codex |
| Story | `US-001-02` |
| Epic | `EPIC-001` |
| PRD requirements | NFR reliability and maintainability; MVP acceptance criterion 8 |
| Branch | `story/us-001-02-automated-quality-checks` |
| Pull request | Not opened |
| Status | In review — hosted CI evidence pending |

## Intended outcome

Give contributors and continuous integration one shared quality command that checks formatting, treats Clippy warnings as errors, and runs all current tests with clear stage-level results.

## Starting context

`US-001-01` provided a dependency-free Rust binary and documented separate Cargo commands. Those commands passed on the story branch, after merge on `main`, and independently in the owner's Pop!_OS terminal. No CI workflow or unified local command existed.

The repository has no configured remote, so the GitHub workflow can be validated structurally and locally but cannot produce a hosted Actions run in this story session unless a remote is later configured and the branch is pushed with owner authorization. `prompts.zip` remains unrelated and untracked.

## Plan

1. Add a portable shell script that names, runs, and reports each quality stage.
2. Add a least-privilege GitHub Actions workflow that invokes the same script.
3. Document the unified local command.
4. Verify successful and deliberately failing script paths and inspect workflow syntax.

## Work performed

- Added `scripts/check.sh` using POSIX shell syntax and fail-fast stage reporting.
- Added formatting, Clippy-with-denied-warnings, and full test stages.
- Added a GitHub Actions workflow for `main`, `story/**`, and pull-request changes.
- Restricted the workflow token to read-only repository contents and set a ten-minute job timeout.
- Updated contributor documentation with the shared local/CI command.

### Commands and observations

```text
$ ./scripts/check.sh
Formatting: PASS
Clippy: PASS
Tests: PASS (1 passed; 0 failed)
All Rust/400 quality checks passed.

$ RUST400_CARGO=false ./scripts/check.sh
Formatting: FAIL (exit 1)
Process exit status: 1

$ ruby -e '<parse .github/workflows/quality.yml>'
YAML parsed successfully.

$ git diff --check
Completed successfully with no whitespace errors.
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Use a POSIX shell script | Makefile; cargo alias; task-runner dependency | Linux is the MVP platform and a small script gives local/CI parity without adding a tool dependency | N/A |
| Fail at the first red stage | Run all checks and aggregate failures | Fast, named failure keeps output focused and meets the story criterion; aggregation can be introduced if contributors need it | N/A |
| Use GitHub Actions | Provider-neutral CI only; another hosted provider | The project prompt suite and planned conventions are GitHub-oriented; the workflow remains a local reviewed file until a remote exists | N/A |
| Install through runner Rustup | Third-party Rust toolchain action | Avoids adding another marketplace dependency and respects `rust-toolchain.toml` | N/A |
| Use `actions/checkout@v5` | Older checkout major; pin full commit SHA | v5 is the selected official major for modern runners; immutable SHA pinning can be added once the project establishes an action-update policy | Future supply-chain policy decision |

## Problems, failed approaches, and recovery

- The official web lookup for the current checkout release returned no usable content in this environment. The workflow uses the known current major `actions/checkout@v5`; a hosted CI run will be the next operational validation.
- Initial workflow logic only installed the toolchain when no active toolchain was detected. That did not guarantee Clippy and Rustfmt components. It was corrected to run an idempotent Rustup installation with both required components.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1: one documented local command runs required checks | Successful `./scripts/check.sh`; `README.md` | Pass |
| AC 2: CI runs formatting, linting, and tests | Workflow parses and invokes the shared script; hosted run unavailable without remote | Not verified operationally |
| AC 3: failed check prevents success and identifies stage | `RUST400_CARGO=false ./scripts/check.sh` reported `FAIL: Formatting` and exited 1 | Pass |

## Material changes

- `scripts/check.sh`: Adds the shared named-stage quality gate.
- `.github/workflows/quality.yml`: Runs the gate in GitHub Actions.
- `README.md`: Documents the unified contributor command and CI triggers.
- `build-logs/2026-08-04-us-001-02-automated-quality-checks.md`: Records implementation and evidence.

## Deviations and remaining risks

- Hosted CI evidence requires a configured remote and owner-authorized push.
- The checkout action is major-version pinned rather than commit-SHA pinned pending a dependency-update policy.
- Local and structural verification pass; a hosted GitHub Actions run remains pending.

## Lessons learned

- Calling one script from both local development and CI eliminates duplicated command lists that can drift.
- Failure-path injection can prove shell gate behavior without corrupting source files or intentionally introducing lint errors.
- Toolchain presence and component presence are different CI prerequisites.

## Next action

Review the branch, then configure a GitHub remote or otherwise obtain a hosted Actions run before marking AC 2 fully verified and closing the story.

## Correction history

No corrections recorded.
