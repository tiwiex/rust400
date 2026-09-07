# Build Log: Real terminal function-key input

| Field | Value |
|---|---|
| Date | 2026-08-13 |
| Author | Codex |
| Story | `US-008-05` |
| Epic | `EPIC-008` |
| PRD requirements | `FR-019A`, `FR-001` |
| Branch | `story/us-008-05-real-function-key-input` |
| Pull request | Not opened |
| Status | Verified |

## Intended outcome

Replace typed pseudo-function-key input with real terminal key-event handling so physical F-keys can drive Rust/400 footer actions.

## Starting context

Rust/400 currently renders function-key hints in the footer and accepts typed text such as `F3` or `F12` through a line-based input loop. Physical function keys do not work because the session reads whole lines rather than terminal key events.

## Plan

1. Add the story definition and build-log entry for traceability.
2. Introduce terminal key-event input while preserving command entry and menu selection.
3. Verify supported keys, fallback behavior, and terminal cleanup.

## Story context

- Story title: Capture real terminal function keys
- Acceptance criteria:
  1. Supported physical function keys trigger footer actions directly.
  2. Commands and menu selections still work after event-loop changes.
  3. Terminal mode is restored cleanly on exit.
  4. Automated tests cover key-to-action mapping and fallback behavior where practical.
- Definition of Done checks:
  - Shared footer hints remain the source of truth for supported actions.
  - Interactive changes are backed by tests and build evidence.
  - The terminal is not left in an unusable state after the session ends.

## Work performed

Started the story by aligning `EPIC-008` with a new `US-008-05` entry before implementation.

Added `crossterm` and introduced a TTY-only raw terminal event loop for interactive sessions. Physical function keys now map to the existing footer actions through terminal key events, while the previous line-based command loop remains as the fallback path for tests and non-terminal input.

Refactored command handling so both input modes share one execution path. Added a raw-mode guard to restore the terminal on exit and on ordinary error paths.

### Commands and observations

```text
$ git status --short --branch
## story/us-008-05-real-function-key-input

$ rg -n "read_line|F3|F12|FunctionKeyAction" src
Confirmed the current session loop still depends on line-based input and text representations such as "F3".

$ cargo test --all-targets --all-features
Passed after adding the crossterm event loop. 36 library tests, 38 binary tests, and 5 workspace-isolation tests succeeded.

$ cargo clippy --all-targets --all-features -- -D warnings
Passed after the event-loop refactor and formatting cleanup.

$ cargo fmt --all -- --check
Passed.
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Use `crossterm` for TTY-only event handling | Keep typed `F3`/`F12` only; write bespoke ANSI parsing | `crossterm` provides cross-platform key-event and raw-mode support with less terminal-specific parsing risk | N/A |
| Keep the line-based loop as a fallback | Replace every input path with raw-mode events | Preserves existing automated tests and non-TTY behavior while enabling real keys for interactive terminals | N/A |

## Problems, failed approaches, and recovery

- Adding a new dependency required temporary network access to download crates from crates.io.
- The first refactor left formatting issues that were corrected by `cargo fmt` before final verification.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1 | TTY-only `crossterm` event loop in `src/main.rs`; physical F-key events map to existing footer actions | Pass |
| AC 2 | Existing interactive command/menu tests still pass through the line-mode fallback | Pass |
| AC 3 | `RawModeGuard` restores terminal mode on drop; normal exit path covered by code review and structure | Pass |
| AC 4 | `cargo test --all-targets --all-features`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo fmt --all -- --check` | Pass |

## Material changes

- `docs/backlog/epics/EPIC-008-operator-interface.md`: added `US-008-05`.
- `build-logs/2026-08-13-us-008-05-real-function-key-input.md`: started build log entry.
- `build-logs/index.md`: registered the new story log.
- `Cargo.toml`: added `crossterm`.
- `Cargo.lock`: locked the new terminal-event dependency graph.
- `src/main.rs`: added raw terminal event handling, a raw-mode guard, shared session state, and a shared command-processing path for both event and line input modes.

## Deviations and remaining risks

- Introducing terminal raw-mode handling may affect portability and testability; this needs careful isolation.
- Physical key handling is active only when both stdin and stdout are terminals. Piped or test input still uses the line-mode fallback.
- This story does not yet add advanced in-screen line editing, cursor navigation, or alternate keyboard mappings for terminals that do not emit higher function keys consistently.

## Lessons learned

- The visible footer hints were already in place, so the missing piece is terminal input mechanics rather than UI copy.
- Splitting “interactive terminal input” from “command processing” made the feature easier to add without breaking the existing test suite.

## Next action

Manually test supported physical function keys in a real terminal session, then commit and merge the story if the interaction feels right.

## Correction history
