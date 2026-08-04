# Build Log: Render a full-screen main menu

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Codex |
| Story | `US-008-01` |
| Epic | `EPIC-008` |
| PRD requirements | `FR-001`, `FR-017A` |
| Branch | `story/us-008-01-render-full-screen-main-menu` |
| Pull request | Not opened |
| Status | Verified |

Use the same stable story ID in the branch name, pull-request title, and build-log filename whenever the work belongs to a numbered story.

## Intended outcome

Make Rust/400 start with a recognizably AS/400-style full-screen text menu instead of a plain prompt so the emulator immediately feels closer to the target operator experience.

## Starting context

The project already had a command engine, parser, shared command metadata, and help output, but startup still looked like a normal line-oriented shell. The corrected PRD and backlog now make the menu-driven operator interface a first-class target, so the next slice needed to render that visible main menu structure.

## Plan

1. Add a dedicated UI renderer for a stable main-menu screen.
2. Integrate that renderer into startup while preserving the existing command engine.
3. Add snapshot-style tests and update the documentation.

## Story context

- Story title: Render a full-screen main menu
- Acceptance criteria:
  - Startup renders a full-screen text UI with a title area, system context area, main content area, input line, and footer legend.
  - The screen includes a "selection or command" style input affordance.
  - Automated snapshot or rendering tests verify the stable screen layout.
- Definition of Done checks:
  - Implementation added
  - Tests added
  - Documentation updated
  - Build log indexed

## Work performed

Added `src/ui.rs` with a dedicated `MainMenuScreen` model and `render_main_menu` function. The renderer produces a stable text-mode menu layout with a title area, system label, numbered options, a `Selection or command` section, and a footer legend listing function-key hints.

Integrated the rendered screen into startup so Rust/400 now opens with the menu layout before handing control to the existing command loop. Kept the rendering logic separate from input parsing so later stories can add shared menu metadata and screen navigation without tangling them into command handling.

Updated the README to show the new startup experience and documented the main UI regions now available.

### Commands and observations

```text
$ ./scripts/check.sh
Formatting, clippy, and tests passed.

$ cargo run -- --temporary-workspace
MAIN                            IBM i Main Menu              System: RUST400
...
  Selection or command
  ===>
  ----------------------------------------------------------------------------
  F3=Exit   F4=Prompt   F9=Retrieve   F12=Cancel   F13=Information Assistant
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Render the first screen as stable text output rather than true terminal control | Jump straight to `crossterm` or terminal-cursor management | A text-first renderer satisfies the story and is easy to snapshot-test while the screen model is still evolving | N/A |
| Keep command input handling separate from screen rendering | Build a menu-navigation engine in the same story | This keeps `US-008-01` focused on layout and buys a cleaner seam for `US-008-02` shared menu metadata | N/A |

## Problems, failed approaches, and recovery

The main challenge was balancing the full-screen feel with testability. The chosen recovery was to treat the main menu as a renderable text snapshot first, which keeps the structure stable and reviewable without depending on terminal control libraries yet.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1 | `main_menu_contains_expected_layout_regions` in `src/ui.rs`; startup integration test in `src/main.rs` | Pass |
| AC 2 | `Selection or command` line and `===>` input affordance in `src/ui.rs` snapshot and README example | Pass |
| AC 3 | `main_menu_matches_the_snapshot_layout` in `src/ui.rs`; `./scripts/check.sh` | Pass |

## Material changes

- `src/ui.rs`: added the main-menu screen model, renderer, and snapshot tests.
- `src/lib.rs`: exported the UI module.
- `src/main.rs`: renders the main menu at startup and continues command input from the screen's command line.
- `README.md`: updated the startup transcript and current UI status.
- `build-logs/index.md`: added this entry.

## Deviations and remaining risks

This story renders the main menu but does not yet add shared menu metadata, numbered selection handling, or true function-key input handling. Those belong to later EPIC-008 stories.

## Lessons learned

The screen model became much easier to reason about once it was treated as a renderable artifact in its own module. That same choice should make future snapshot reviews much easier as the UI grows.

## Next action

Move to `US-008-02` so menu definitions, labels, and navigation can be driven from shared metadata instead of being hard-coded in one renderer.

## Correction history

None.
