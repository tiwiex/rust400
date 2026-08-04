# Build Log: Support function-key actions

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Codex |
| Story | `US-008-03` |
| Epic | `EPIC-008` |
| PRD requirements | `FR-019A` |
| Branch | `story/us-008-03-support-function-key-actions` |
| Pull request | Not opened |
| Status | Verified |

Use the same stable story ID in the branch name, pull-request title, and build-log filename whenever the work belongs to a numbered story.

## Intended outcome

Make the main menu respond to a documented subset of AS/400-style function-key actions so the interface feels more like an operator screen instead of just a menu with typed commands.

## Starting context

The main menu already displayed a footer legend with function-key hints, but those hints were only visual. The input field accepted direct commands, yet entries such as `F3` or `F12` had no behavior behind them.

## Plan

1. Extend shared menu metadata so footer hints map to explicit function-key actions.
2. Interpret typed function-key inputs before normal command parsing.
3. Add tests and documentation for supported and unsupported function keys.

## Story context

- Story title: Support function-key actions
- Acceptance criteria:
  - A documented subset of function-key actions such as Exit, Cancel, Prompt, Refresh, and Help is supported.
  - Function-key hints render in the footer legend for the active screen.
  - Unsupported keys fail gracefully with a documented message.
- Definition of Done checks:
  - Implementation added
  - Tests added
  - Documentation updated
  - Build log indexed

## Work performed

Extended `src/menus.rs` so each footer hint now carries an explicit `FunctionKeyAction`. Updated the main input loop in `src/main.rs` to interpret typed function-key entries such as `F3`, `F4`, `F9`, `F12`, `F13`, and `F23` before normal command parsing.

Implemented the current subset as follows:

- `F3` exits the session.
- `F4` prints prompt guidance.
- `F9` retrieves the most recent direct input.
- `F12` cancels and remains on the main menu.
- `F13` prints Information Assistant guidance.
- `F23` reports that setting the initial menu is not implemented yet.

Unsupported function keys such as `F5` now fail gracefully with a menu-specific message. Updated the README to document the supported subset.

### Commands and observations

```text
$ ./scripts/check.sh
Formatting, clippy, and tests passed.

$ cargo run -- --temporary-workspace
  ===> F4
Function key F4 -> Prompt. Type a command such as HELP, CRTLIB LIB(MYLIB), or 90.
  ===> help
Available commands: EXIT, HELP, CRTLIB, SNDMSG, WRKOBJ
  ===> F9
Function key F9 -> Retrieve 'help'
  ===> F5
Function key 'F5' is not supported on menu MAIN.
  ===> F3
Function key F3 -> Exit
Session ended.
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Treat typed `F*` entries as the MVP function-key mechanism | Wait for raw terminal key capture first | This keeps the feature testable and useful now without taking on full terminal input complexity yet | N/A |
| Support a documented subset and graceful placeholders | Pretend every footer legend key already works fully | Honest partial behavior matches the story and avoids overstating emulator completeness | N/A |

## Problems, failed approaches, and recovery

The main challenge was making function keys feel real without terminal key-capture infrastructure. The recovery was to scope the MVP to typed function-key tokens, which still exercises the interaction model and shared footer metadata cleanly.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1 | `function_key_f3_exits_the_session`, `function_key_f4_prompts_for_supported_input`, `function_key_f9_retrieves_the_previous_direct_input`, `function_key_f12_cancels_gracefully`, `function_key_f13_offers_information_assistant_guidance` | Pass |
| AC 2 | Footer hints still render from shared metadata in `src/ui.rs` and are mapped in `src/menus.rs` | Pass |
| AC 3 | `unsupported_function_key_fails_gracefully`; README function-key section | Pass |

## Material changes

- `src/menus.rs`: added explicit function-key actions to footer hints.
- `src/main.rs`: added function-key input handling before normal command parsing.
- `README.md`: documented the current supported function-key subset.
- `build-logs/index.md`: added this entry.

## Deviations and remaining risks

This story supports typed function-key tokens, not raw terminal key-capture events. A later terminal-input story can replace or augment this with actual key handling if desired.

## Lessons learned

Attaching behavior to the footer metadata turned the footer from decoration into a real contract. That makes future terminal-input upgrades much easier because the meaning of each key already lives in one place.

## Next action

Decide whether to merge this branch directly or first reapply the mixed menu-selection behavior so the stacked UI stories stay in the order you prefer.

## Correction history

None.
