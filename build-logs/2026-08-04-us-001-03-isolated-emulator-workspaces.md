# Build Log: Create isolated emulator workspaces

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Project owner and Codex |
| Story | `US-001-03` |
| Epic | `EPIC-001` |
| PRD requirements | FR-006, FR-021, NFR-SEC-001, NFR-SEC-002, NFR-SEC-003 |
| Branch | `story/us-001-03-isolated-emulator-workspaces` |
| Pull request | Not opened |
| Status | Verified |

## Intended outcome

Initialize permanent or disposable Rust/400 state inside one validated Linux workspace without allowing traversal or symbolic-link inputs to write outside it.

## Starting context

`US-001-01` provided the executable baseline. `US-001-02` added local and hosted quality automation and was merged through PR #1. The application had no mutable state, workspace abstraction, or startup configuration.

The `US-001-02` build log still awaits independent hosted-CI evidence because GitHub CLI is installed but not authenticated in the agent environment. This does not block this story, whose dependency is `US-001-01`.

## Plan

1. Introduce a standard-library workspace boundary with explicit root validation.
2. Support permanent absolute paths and automatically cleaned temporary workspaces.
3. Reject traversal and symbolic links before state mutation.
4. Expose safe relative resolution and directory creation for later persistence stories.
5. Add integration tests for containment and update startup documentation.

## Work performed

- Added a library module for workspace initialization and containment.
- Added permanent and temporary startup modes without introducing a CLI dependency.
- Added a schema marker beneath each initialized workspace.
- Added safe relative-path resolution and workspace directory creation.
- Added traversal, root-symlink, child-symlink, persistence, and temporary-cleanup tests.
- Updated startup documentation.

### Commands and observations

```text
$ ./scripts/check.sh
Formatting: PASS
Clippy: PASS
Tests: PASS (8 passed; 0 failed)

$ cargo run -- --workspace /tmp/rust400-us00103-cli-evidence
Rust/400 development shell: initialization complete; no commands are available yet.
Workspace: /tmp/rust400-us00103-cli-evidence

$ find /tmp/rust400-us00103-cli-evidence -maxdepth 1 -type f
system.meta

$ cargo run -- --temporary-workspace
Rust/400 development shell: initialization complete; no commands are available yet.
Temporary workspace: /tmp/rust400-<unique-id>

$ cargo run -- --workspace relative/path
Could not initialize Rust/400: unsafe workspace path relative/path:
workspace root must be an absolute path
Process exit status: 1
```

The temporary path no longer existed after process exit. The permanent CLI evidence directory was inspected and then removed from `/tmp`.

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Require an absolute permanent root | Resolve relative paths against current directory | Explicit roots avoid accidental state placement and simplify the trust boundary | Future workspace ADR after ADR process exists |
| Reject all symlinks in workspace paths | Canonicalize and allow links that resolve within root | A strict initial policy is easier to explain and test; it can be relaxed only with evidence | Future workspace ADR |
| Use standard library only | Add `tempfile`, CLI, or path-security crates | Current behavior is small enough to implement clearly without dependencies | N/A |
| Remove temporary workspace on `Drop` | Retain it for debugging; cleanup command | Disposable mode should leave no state after a clean run; failure residue risk is documented | N/A |
| Add a library target within one package | Keep everything in `main.rs`; create multiple crates | A library exposes the workspace boundary to integration tests while preserving the PRD's single-package preference | N/A |

## Problems, failed approaches, and recovery

- The first full quality run passed. A documentation comparison found that the startup string had lost “initialization complete” during CLI refactoring while `README.md` still documented it. The original verified wording was restored and the complete gate was rerun.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1: mutable state remains beneath configured workspace | `permanent_workspace_contains_all_initialized_state`; permanent CLI inspection | Pass |
| AC 2: disposable workspace can be selected | `temporary_workspace_is_removed_when_released`; temporary CLI run | Pass |
| AC 3: traversal and symlink escape cannot write outside | `parent_traversal_is_rejected_without_outside_write`, `symlinked_workspace_root_is_rejected_without_outside_write`, and `symlinked_child_is_rejected_without_outside_write` | Pass |

## Material changes

- `src/lib.rs`: Exposes the workspace module to the binary and integration tests.
- `src/workspace.rs`: Implements workspace validation, initialization, resolution, and cleanup.
- `src/main.rs`: Selects permanent or temporary startup mode.
- `tests/workspace_isolation.rs`: Proves expected behavior and containment boundaries.
- `README.md`: Documents workspace startup and safety behavior.
- `build-logs/2026-08-04-us-001-03-isolated-emulator-workspaces.md`: Records story decisions and evidence.

## Deviations and remaining risks

- The standard-library check is designed for the MVP's single-process model and does not claim race-proof containment against a concurrent hostile process changing filesystem components between validation and creation.
- Temporary cleanup on abnormal process termination is not guaranteed.
- Stable message IDs arrive in the later command/message stories.
- Stable message IDs and an atomic persistent catalog remain later-story work.

## Lessons learned

- Canonicalization alone is insufficient for a teaching-friendly security boundary; rejecting symlink components makes the initial policy explicit.
- Adversarial tests should assert both the returned error and absence of the outside artifact.
- A disposable resource can use `Drop` for normal cleanup while still documenting that abnormal termination may leave residue.

## Next action

Perform the branch review, then request owner authorization to commit, push, open a PR, and obtain hosted CI evidence.

## Correction history

No corrections recorded.
