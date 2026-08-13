# Build Log: Controlled Linux read-only views

| Field | Value |
|---|---|
| Date | 2026-08-13 |
| Author | Codex |
| Story | `US-003-05` |
| Epic | `EPIC-003` |
| PRD requirements | `FR-001`, `FR-023`, `FR-024`, `FR-026`, `FR-027`, `FR-028` |
| Branch | `story/us-003-05-controlled-linux-readonly-views` |
| Pull request | Not opened |
| Status | Verified |

## Intended outcome

Add safe, read-only Linux-facing commands that help learners compare Rust/400 concepts with the host environment without introducing arbitrary shell execution or host mutation.

## Starting context

Rust/400 already provides menu-driven Linux concept mappings through `LNXMAP` and detail screens, but those screens are explanatory only. The next useful learning step is to expose a few controlled host views: current Linux directory, directory listing, and Linux user account summary.

## Plan

1. Add a backlog entry and build log for the new story so implementation remains traceable.
2. Implement read-only commands and menu access using Rust file and filesystem APIs rather than shell passthrough.
3. Verify behavior with automated tests and update learning documentation.

## Story context

- Story title: View controlled Linux read-only details
- Acceptance criteria:
  1. Display current Linux directory from a Rust/400 command.
  2. List files and directories inside the current Linux directory with Linux-analogy wording.
  3. Show a read-only Linux user summary with explicit authority-model differences.
  4. Do not execute arbitrary shell input or mutate host state.
- Definition of Done checks:
  - Shared command metadata and help stay aligned.
  - User-visible behavior is covered by tests or documented manual evidence.
  - Learning bridge wording remains explicit about analogy limits.

## Work performed

Started the story by aligning the backlog with a new `US-003-05` entry before changing code.

Implemented three new read-only commands:

- `DSPPWD` to display the current Linux directory for the session
- `DSPLS` to list files and directories inside the current Linux directory
- `DSPUSRS` to show a capped Linux user summary from `/etc/passwd`

Added menu guidance for these commands under both the user-tasks menu and the Linux mappings menu. Refined `DSPPWD` and `DSPLS` after user feedback so they use the real current Linux directory rather than only the Rust/400 workspace. Also changed command results to repaint the green-screen body area instead of appending an ever-growing transcript below the menu. Kept the implementation inside Rust code with direct file and filesystem reads so the emulator still does not execute arbitrary host shell commands.

### Commands and observations

```text
$ git status --short --branch
## story/us-003-05-controlled-linux-readonly-views

$ sed -n '1,260p' docs/PRD.md
Confirmed FR-023 forbids arbitrary host shell execution and FR-024/026/027/028 guide the learning bridge.

$ cargo clippy --all-targets --all-features -- -D warnings
Passed after the new read-only Linux view commands were added.

$ cargo test --all-targets --all-features
Passed after the Linux view and repainted result-panel changes. 36 library tests, 38 binary tests, and 5 workspace-isolation tests succeeded.

$ cargo fmt --all -- --check
Passed after formatting adjustments.
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Use Rust stdlib and direct file reads for host views | Shell out to `pwd`, `ls`, or `getent` | Keeps the feature read-only, testable, and inside the product guardrails | N/A |

## Problems, failed approaches, and recovery

None yet.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1 | `DSPPWD`; `tests::dsppwd_reports_the_session_linux_path` | Pass |
| AC 2 | `DSPLS`; `tests::dspls_lists_linux_directory_entries_without_claiming_library_equivalence` | Pass |
| AC 3 | `DSPUSRS`; `tests::dspurss_displays_linux_user_summary_from_host_data` | Pass |
| AC 4 / `FR-023` | Implementation uses `std::fs` reads in `src/linux_views.rs`; no shell execution path added | Pass |
| Baseline verification | `cargo fmt --all -- --check`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo test --all-targets --all-features` | Pass |

## Material changes

- `docs/backlog/epics/EPIC-003-learning-bridge.md`: added `US-003-05`.
- `build-logs/2026-08-13-us-003-05-controlled-linux-readonly-views.md`: started build log entry.
- `build-logs/index.md`: registered the new log entry.
- `src/linux_views.rs`: added controlled read-only Linux host views and tests.
- `src/commands.rs`: registered `DSPPWD`, `DSPLS`, and `DSPUSRS` in shared command metadata.
- `src/main.rs`: executed the new commands, rendered repainted result panels, and added interactive-session tests.
- `src/menus.rs`: exposed the new commands from user and Linux-learning menus.
- `src/lib.rs`: exported the Linux views module.
- `src/ui.rs`: added a green-screen detail wrapper for repainted command result screens.
- `docs/concepts/as400-to-linux-command-mappings.md`: linked the new controlled host-view commands into the learning bridge.

## Deviations and remaining risks

- `DSPUSRS` intentionally caps output to the first 12 `/etc/passwd` entries to keep the green-screen transcript readable.
- `DSPPWD` and `DSPLS` now reflect the session's current Linux directory, which is initialized from the process working directory and falls back to the Rust/400 workspace if needed.
- Directory navigation is still not interactive yet; a later story can add `CHGDIR` or a `WRKLNK`-style browser.
- Function keys shown in the footer are still typed-command hints rather than true terminal key events. Real keyboard function-key handling should move to a dedicated operator-interface story.

## Lessons learned

- For this repo, adding the story definition before coding keeps the branch, build log, and acceptance criteria aligned.
- Read-only host learning views are much easier to keep safe when they are implemented as narrowly scoped Rust functions rather than generic command passthrough.
- Early user feedback helped sharpen the intended behavior: “Linux view” was more useful when it meant the real current directory, not only the isolated Rust/400 workspace.
- Repainting command results instead of appending a transcript makes the emulator feel much closer to the intended AS/400-style interaction, even before full terminal key-event support exists.

## Next action

The next useful step is real terminal function-key support under `EPIC-008`, followed by richer read-only Linux browsing such as `CHGDIR` or a `WRKLNK`-style screen.

## Correction history
