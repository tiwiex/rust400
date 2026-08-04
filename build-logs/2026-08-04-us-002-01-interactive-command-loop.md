# Build Log: Run an interactive command loop

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Codex |
| Story | `US-002-01` |
| Epic | `EPIC-002` |
| PRD requirements | `FR-001`, `FR-012` |
| Branch | `story/us-002-01-interactive-command-loop` |
| Pull request | Not opened |
| Status | Verified |

Use the same stable story ID in the branch name, pull-request title, and build-log filename whenever the work belongs to a numbered story.

## Intended outcome

Replace the one-shot placeholder startup with a real interactive shell loop that shows a prompt, accepts successive input lines, ignores blank input, and exits cleanly on `EXIT` or end-of-input.

## Starting context

Rust/400 already initialized isolated permanent and temporary workspaces, but the process ended immediately after printing a startup message. There was no persistent prompt and no session behavior to build future command work on top of.

## Plan

1. Convert startup into a testable interactive session loop without disturbing workspace initialization.
2. Add tests for repeated prompting, `EXIT`, and end-of-input behavior.
3. Update documentation and record evidence in the build log.

## Story context

- Story title: Run an interactive command loop
- Acceptance criteria:
  - Given an initialized workspace, when the application starts interactively, then it displays a prompt and accepts successive commands.
  - `EXIT` and end-of-input close the session cleanly.
  - Empty input does not fail or mutate state.
- Definition of Done checks:
  - Implementation added
  - Tests added
  - Documentation updated
  - Build log indexed

## Work performed

Updated `src/main.rs` so both permanent and temporary workspace modes now enter a shared interactive session instead of exiting immediately. The session writes startup guidance, displays the `R400> ` prompt, reads successive lines from standard input, ignores blank input, and exits on `EXIT` without error. End-of-input also exits cleanly so piped or scripted demos can terminate naturally.

Kept the implementation in `main.rs` for now because the loop remains small and story-local. The code separates startup output from the command loop so the behavior can be tested with in-memory input and output buffers instead of a real terminal.

Updated the README to describe the interactive shell’s current behavior and example startup transcript.

### Commands and observations

```text
$ ./scripts/check.sh
Formatting, clippy, and tests passed.

$ cargo run -- --temporary-workspace
Rust/400 interactive shell: initialization complete.
Workspace: <temporary workspace path>
Type EXIT to end the session. Additional commands will arrive in later stories.
R400>
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Keep the initial command loop in `src/main.rs` | Create a separate shell module immediately | The loop is still small; delaying extraction avoids premature structure before parser and command metadata work land | N/A |
| Return a friendly placeholder for unknown commands | Reject everything silently until parsing exists | A visible placeholder confirms the loop is working and teaches the current boundary of implemented behavior | N/A |

## Problems, failed approaches, and recovery

No material blockers occurred during implementation. The only special handling was keeping the loop testable with `Cursor` and `Vec<u8>` so prompt and exit behavior could be verified without interactive terminal dependencies.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1 | `command_loop` tests plus `run_interactive_session` test in `src/main.rs`; manual startup transcript in this log | Pass |
| AC 2 | `exit_command_closes_the_session_cleanly`; `end_of_input_closes_the_session_cleanly` | Pass |
| AC 3 | `blank_input_is_ignored_without_ending_the_session` | Pass |

## Material changes

- `src/main.rs`: replaced the one-shot placeholder with a testable interactive session loop.
- `README.md`: updated current behavior and startup example.
- `build-logs/index.md`: added this entry.

## Deviations and remaining risks

Unknown commands currently return a simple placeholder string instead of structured message IDs. That is intentional and should be refined by later command-parser and error-handling stories.

## Lessons learned

Even a minimal REPL becomes much easier to evolve when input and output are abstracted behind `BufRead` and `Write`. That keeps the next parser and help stories easy to test without depending on a real terminal.

## Next action

Move to `US-002-02` so the interactive loop can parse CL-like command syntax instead of treating every non-`EXIT` line as a placeholder.

## Correction history

None.
