# Build Log: Define commands from shared metadata

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Codex |
| Story | `US-002-03` |
| Epic | `EPIC-002` |
| PRD requirements | `FR-004`, `FR-005` |
| Branch | `story/us-002-03-shared-command-metadata` |
| Pull request | Not opened |
| Status | Verified |

Use the same stable story ID in the branch name, pull-request title, and build-log filename whenever the work belongs to a numbered story.

## Intended outcome

Replace the temporary per-command validation bridge with one shared command registry that defines command metadata once and lets validation, help, registry checks, and typed handler requests consume the same source.

## Starting context

`US-002-02` introduced a parser and lightweight validation, but the shell still chose validation rules with a local `match` in `main.rs`. That approach would eventually let syntax, help, and execution drift apart, which is exactly what `FR-005` and `US-002-03` are meant to prevent.

## Plan

1. Introduce a shared typed command registry and move validation rules into it.
2. Build one typed request path from shared metadata and prove help uses the same definitions.
3. Verify the registry with tests, update the ADR, and record the evidence.

## Story context

- Story title: Define commands from shared metadata
- Acceptance criteria:
  - A command definition describes its name, summary, parameters, validation rules, and handler identity.
  - Validation and help consume the same definition.
  - A test fails if a registered command lacks required documentation metadata.
- Definition of Done checks:
  - Implementation added
  - Tests added
  - Documentation updated
  - ADR updated
  - Build log indexed

## Work performed

Added `src/commands.rs` as the shared registry for Rust/400 commands. Each `CommandDefinition` now records command name, summary, parameter metadata, mutually exclusive parameter pairs, and a typed `HandlerId`. The registry currently includes `EXIT`, `HELP`, `CRTLIB`, `SNDMSG`, and `WRKOBJ`.

Moved validation to consume those definitions instead of a local `match` in `main.rs`. Added help rendering from the same metadata, plus typed request-building so a parsed `CRTLIB` command becomes a `CreateLibraryRequest` for the handler path. The shell still uses placeholder execution text for now, but it now reports that text from the typed request rather than from duplicated string logic.

Added registry metadata checks so tests fail if a command or parameter is missing required documentation fields. Updated ADR-0004 from Proposed to Accepted because the typed static approach now has working evidence for one end-to-end command path.

### Commands and observations

```text
$ ./scripts/check.sh
Formatting, clippy, and tests passed.

$ cargo run -- --temporary-workspace
R400> help cmd(crtlib)
Command: CRTLIB
Summary: Create an emulated library definition.
Parameters:
- LIB (required): Names the library to create.
- TEXT (optional): Supplies a descriptive text for the library.
Handler: CreateLibrary
R400> crtlib lib(mylib) text('Learning library')
Command 'CRTLIB' is mapped to handler CreateLibrary for library 'MYLIB' with text 'Learning library'.
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Accept typed static Rust command definitions for the MVP | Keep ADR-0004 open; move to declarative runtime files; use macros | The typed registry is small, explicit, easy to test, and now proves non-drifting validation/help/request-building with low complexity | `ADR-0004` |
| Build a typed request for `CRTLIB` now | Limit the story to metadata plus validation only | The typed request closes the prototype loop and gives the ADR the end-to-end evidence it required | `ADR-0004` |

## Problems, failed approaches, and recovery

The main risk was accidentally pushing too far into full command execution or a richer help system, which belongs to later stories. The recovery was to keep this story focused on the registry contract: definitions, validation, help rendering, typed requests, and metadata quality checks.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1 | `CommandDefinition`, `ParameterDefinition`, `HandlerId`, and registry entries in `src/commands.rs` | Pass |
| AC 2 | `help_and_validation_consume_the_same_command_definition`; shell test for `help cmd(crtlib)` | Pass |
| AC 3 | `registry_metadata_check_fails_for_missing_summary`; `shipped_registry_has_required_metadata` | Pass |

## Material changes

- `src/commands.rs`: added the shared command registry, help rendering, validation bridge, typed requests, and metadata tests.
- `src/lib.rs`: exported the commands module.
- `src/main.rs`: replaced local placeholder validation with shared metadata lookup and typed requests.
- `README.md`: documented the new metadata-driven help and request flow.
- `docs/adr/0004-define-commands-from-shared-metadata.md`: accepted the typed static registry decision.
- `build-logs/index.md`: added this entry.

## Deviations and remaining risks

Execution is still placeholder behavior, and summary help is intentionally minimal. `US-002-04` should build richer discovery and command help on top of the accepted registry. `US-002-05` should layer stable message IDs and recovery guidance on top of the same command path.

## Lessons learned

This was the first story where the codebase clearly benefited from a central source of truth. Once help, validation, and handler request-building shared the same metadata, the architecture became easier to explain and safer to extend.

## Next action

Move to `US-002-04` so the accepted command registry can power discoverability and richer built-in help instead of only serving tests and placeholder execution.

## Correction history

None.
