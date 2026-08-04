# Build Log: Parse CL-like command syntax

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Codex |
| Story | `US-002-02` |
| Epic | `EPIC-002` |
| PRD requirements | `FR-003`, `FR-004` |
| Branch | `story/us-002-02-parse-cl-like-command-syntax` |
| Pull request | Not opened |
| Status | Verified |

Use the same stable story ID in the branch name, pull-request title, and build-log filename whenever the work belongs to a numbered story.

## Intended outcome

Give the interactive shell a real CL-like parser that understands case-insensitive command names, keyword parameters, quoting, whitespace, and parameter-validation rules, while staying small enough to evolve in later metadata and help stories.

## Starting context

`US-002-01` added a working interactive prompt, but every non-`EXIT` line was treated as a generic placeholder string. There was no syntax model, no normalized identifiers, and no structured parameter-validation behavior yet.

## Plan

1. Add a parser module with a typed command representation and malformed-input tests.
2. Add lightweight validation rules for required, unknown, duplicate, repeated, and mutually exclusive parameters.
3. Integrate parser feedback into the interactive loop, update docs, and verify the quality gate.

## Story context

- Story title: Parse CL-like command syntax
- Acceptance criteria:
  - The parser accepts documented command names, parentheses, quoting, whitespace, and keyword parameters.
  - Equivalent unquoted identifiers in different letter cases normalize identically.
  - Missing, duplicate, mutually exclusive, and unknown parameters produce structured validation errors.
  - Parser tests cover the documented syntax branches and malformed-input boundaries.
- Definition of Done checks:
  - Implementation added
  - Tests added
  - Documentation updated
  - Build log indexed

## Work performed

Added `src/parser.rs` with a typed `ParsedCommand` model, parameter values for normalized identifiers and quoted strings, parse errors with structured variants, and validation errors for missing required, duplicate, unknown, and mutually exclusive parameters. The parser accepts CL-like forms such as `CRTLIB LIB(MYLIB) TEXT('Learning library')`, handles both single and double quoted strings, tolerates extra whitespace, and normalizes unquoted identifiers to uppercase ASCII.

Integrated the parser into the interactive shell so successful syntax now receives a "parsed successfully" placeholder, malformed input reports syntax errors, and a small temporary validation layer demonstrates the validation rules without forcing the command-definition ADR to close early. This keeps `US-002-02` focused on syntax while leaving `US-002-03` free to choose the longer-term shared command-definition structure.

Updated the README with current parser behavior examples so learners can see where the shell now stands.

### Commands and observations

```text
$ ./scripts/check.sh
Formatting, clippy, and tests passed.

$ cargo run -- --temporary-workspace
R400> crtlib lib(mylib) text('Learning library')
Command 'CRTLIB' parsed successfully, but execution is not available yet.
R400> crtlib text('Missing required LIB')
Validation error: missing required parameter LIB
R400> crtlib lib(
Syntax error: parameter LIB is missing a value
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Represent parsed commands as typed Rust structs now | Delay all parser work until the shared command-definition ADR is accepted | `US-002-02` needs syntax and validation behavior now, and a typed parse tree does not lock in the future registry shape by itself | `ADR-0004` remains Proposed |
| Use a small temporary validation helper in the interactive loop | Build the full shared registry in this story | This demonstrates `FR-004` while preserving the separate decision gate for `US-002-03` | `ADR-0004` remains Proposed |

## Problems, failed approaches, and recovery

The main design constraint was avoiding accidental closure of the still-pending command-definition ADR. The recovery was to keep the parser generic and keep the current validation examples deliberately lightweight and local to the shell loop.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1 | Parser tests in `src/parser.rs` for command names, whitespace, parentheses, and quoted values | Pass |
| AC 2 | `normalizes_unquoted_identifiers_case_insensitively` in `src/parser.rs` | Pass |
| AC 3 | Validation tests in `src/parser.rs`; interactive-loop validation test in `src/main.rs` | Pass |
| AC 4 | Malformed-input tests in `src/parser.rs`; `./scripts/check.sh` | Pass |

## Material changes

- `src/parser.rs`: added the CL-like parser, parse tree, validation types, and tests.
- `src/lib.rs`: exported the parser module.
- `src/main.rs`: integrated parser and validation feedback into the interactive loop.
- `README.md`: documented current parser behavior and examples.
- `build-logs/index.md`: added this entry.

## Deviations and remaining risks

The shell does not yet execute real commands from shared metadata, and validation rules are still temporary placeholders selected per recognized command name. `US-002-03` should replace that bridge with the accepted shared-definition design.

## Lessons learned

Separating syntax parsing from command-definition ownership keeps the incremental path clean: the parser can mature now, while help, validation, and execution can converge later around the same command metadata once the ADR is accepted.

## Next action

Move to `US-002-03` and close the command-definition decision with a shared metadata design that drives validation, help, and execution together.

## Correction history

None.
