# Build Log: Mix menu selection with direct commands

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Codex |
| Story | `US-008-04` |
| Epic | `EPIC-008` |
| PRD requirements | `FR-001`, domain rule 10 |
| Branch | `story/us-008-04-mix-menu-selection-and-direct-commands` |
| Pull request | Not opened |
| Status | Verified |

Use the same stable story ID in the branch name, pull-request title, and build-log filename whenever the work belongs to a numbered story.

## Intended outcome

Make the main menu input field behave like an AS/400-style "selection or command" area by accepting either a numbered menu choice or a direct command from the same prompt.

## Starting context

The main menu already rendered correctly and drew its content from shared metadata, but the input field still behaved like a plain command line. Numeric input was not interpreted as a menu selection yet, which meant the screen looked right without actually acting like a menu.

## Plan

1. Interpret numeric input through shared menu metadata before attempting command parsing.
2. Keep direct command entry working from the same field.
3. Add tests and documentation for valid selections, invalid selections, and sign-off behavior.

## Story context

- Story title: Mix menu selection with direct commands
- Acceptance criteria:
  - The main menu accepts either a numbered selection or a command in the documented input area.
  - The input-handling rules are documented and tested for ambiguous or invalid entries.
  - Navigation back to the main menu remains consistent after menu-driven and command-driven flows.
- Definition of Done checks:
  - Implementation added
  - Tests added
  - Documentation updated
  - Build log indexed

## Work performed

Added shared menu-option lookup to `src/menus.rs` and updated the interactive loop in `src/main.rs` to interpret numeric input through the active menu before trying command parsing. Valid menu selections now resolve from shared menu metadata. For the current `MAIN` screen, option `90` signs off consistently, while options such as `1` acknowledge their future target menu and return to `MAIN` until those subordinate menus exist.

Invalid numeric selections now return a clear menu-specific error instead of falling through into generic command parsing. Non-numeric input still follows the direct command path, which keeps `HELP`, `EXIT`, and the current command set working from the same input line.

Updated the README with examples showing mixed menu and command input from the same field.

### Commands and observations

```text
$ ./scripts/check.sh
Formatting, clippy, and tests passed.

$ cargo run -- --temporary-workspace
  ===> 1
Menu selection 1 -> User tasks would open menu 'USR'. Returning to MAIN until that menu is implemented.
  ===> help cmd(crtlib)
Command: CRTLIB
Summary: Create an emulated library definition.
  ===> 77
Selection '77' is not valid on menu MAIN.
  ===> 90
Menu selection 90 -> Sign off
Session ended.
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Treat purely numeric input as a menu-selection attempt first | Always try command parsing first | This matches the screen's documented "selection or command" model and keeps menu behavior intuitive | N/A |
| Acknowledge unimplemented target menus and return to `MAIN` | Block all non-`90` menu options until subordinate screens exist | This preserves navigation intent now and gives visible proof that shared menu metadata is driving the behavior | N/A |

## Problems, failed approaches, and recovery

The main design pressure was the dependency note that pointed toward `US-002-04`. The recovery was to keep this story focused on mixed input interpretation only, using the existing command engine for direct commands rather than waiting for richer discovery work.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1 | `numeric_main_menu_selection_is_handled_from_menu_metadata`; `direct_commands_still_work_from_the_main_menu_input_field` | Pass |
| AC 2 | `invalid_numeric_selection_reports_menu_specific_error`; README mixed-input example | Pass |
| AC 3 | `numeric_main_menu_selection_is_handled_from_menu_metadata`; `sign_off_selection_exits_consistently_from_the_menu` | Pass |

## Material changes

- `src/menus.rs`: added shared menu-option lookup.
- `src/main.rs`: added mixed menu-selection and direct-command input handling.
- `README.md`: documented the mixed-input behavior.
- `build-logs/index.md`: added this entry.

## Deviations and remaining risks

Subordinate menus are not implemented yet, so non-`90` menu selections currently acknowledge their targets and remain on `MAIN`. That is intentional and should be replaced by real navigation once those screens exist.

## Lessons learned

Once screen structure and command structure both come from metadata, the main challenge becomes deciding precedence rules clearly. Making "all-digit input means selection first" explicit keeps the interaction predictable and testable.

## Next action

Move to `US-008-03` for function-key actions or start implementing the first subordinate menu screen that `MAIN` can actually open.

## Correction history

None.
