# Build Log: Define menu screens from shared metadata

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Codex |
| Story | `US-008-02` |
| Epic | `EPIC-008` |
| PRD requirements | `FR-005A`, `FR-017A` |
| Branch | `story/us-008-02-shared-menu-metadata` |
| Pull request | Not opened |
| Status | Verified |

Use the same stable story ID in the branch name, pull-request title, and build-log filename whenever the work belongs to a numbered story.

## Intended outcome

Move menu content out of the renderer and into one shared metadata registry so titles, options, footer hints, and future navigation targets cannot silently drift between code paths.

## Starting context

`US-008-01` added a main-menu renderer, but the menu title, options, and footer hints still lived directly inside `src/ui.rs`. That meant rendering worked, but later navigation logic would have had to duplicate the same screen structure somewhere else.

## Plan

1. Add a shared menu registry with menu definitions, options, footer hints, and target actions.
2. Make the renderer consume shared menu metadata instead of hard-coded values.
3. Add metadata validation tests and update the docs and build log.

## Story context

- Story title: Define menu screens from shared metadata
- Acceptance criteria:
  - A shared menu definition describes title, options, footer hints, and target actions.
  - Rendering and navigation consume the same menu definition.
  - A test fails if a registered menu lacks required display metadata.
- Definition of Done checks:
  - Implementation added
  - Tests added
  - Documentation updated
  - Build log indexed

## Work performed

Added `src/menus.rs` as the shared menu registry. Each menu definition now carries its ID, title, prompt label, system label, options, footer hints, and target actions. The `MAIN` menu is registered there with numbered options and footer-key hints.

Updated `src/ui.rs` so the screen renderer now consumes a `MenuDefinition` through `MenuScreen` instead of owning the menu content directly. That means the renderer and future navigation logic can operate from the same menu metadata. Added menu-registry validation so missing IDs, titles, prompt labels, options, or footer hints fail fast in tests and at startup.

Updated `main.rs` to validate the menu registry before starting and to render the `MAIN` menu from the registry. Updated the README to explain that menu content now comes from shared metadata.

### Commands and observations

```text
$ ./scripts/check.sh
Formatting, clippy, and tests passed.

$ cargo run -- --temporary-workspace
MAIN                         IBM i Main Menu                  System: RUST400
...
  Selection or command
  ===>
  ----------------------------------------------------------------------------
  F3=Exit   F4=Prompt   F9=Retrieve   F12=Cancel   F13=Information Assistant
  F23=Set initial menu
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Add a dedicated `menus` module parallel to the `commands` module | Leave menu metadata embedded in `ui.rs` until selection handling exists | Keeping metadata centralized now prevents renderer and navigation drift later and mirrors the command-registry pattern already proven in the codebase | N/A |
| Include target actions in the menu options now | Defer navigation targets until option selection is implemented | The target actions make the metadata complete enough for future navigation stories and satisfy the acceptance criteria cleanly | N/A |

## Problems, failed approaches, and recovery

The main design concern was not overbuilding navigation behavior before the next story. The recovery was to define target actions declaratively in the menu registry without implementing menu selection handling yet.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1 | `MenuDefinition`, `MenuOption`, `FooterHint`, and `MenuAction` in `src/menus.rs` | Pass |
| AC 2 | `src/ui.rs` renders `MenuScreen` from a shared `MenuDefinition` found through `find_menu("MAIN")` | Pass |
| AC 3 | `metadata_validation_fails_when_a_menu_title_is_missing` in `src/menus.rs`; startup validation path in `src/main.rs` | Pass |

## Material changes

- `src/menus.rs`: added the shared menu registry, metadata validation, and tests.
- `src/lib.rs`: exported the menu module.
- `src/ui.rs`: renders menus from shared metadata instead of hard-coded options.
- `src/main.rs`: validates the shared menu registry and renders the registered `MAIN` menu.
- `README.md`: documented the shared menu metadata approach.
- `build-logs/index.md`: added this entry.

## Deviations and remaining risks

The menu registry does not yet drive actual numbered selection handling or function-key input. That is intentional and belongs to later `EPIC-008` stories.

## Lessons learned

The same design discipline that helped command metadata also helps screen work. Once menu structure is described separately from rendering, the UI roadmap becomes much easier to extend safely.

## Next action

Move to `US-008-03` for function-key behavior or `US-008-04` for mixed menu selection and direct command entry, depending on which interaction path you want to feel real first.

## Correction history

None.
