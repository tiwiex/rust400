# Build Log: Linux mappings menu and lookup

| Field | Value |
|---|---|
| Date | 2026-08-13 |
| Author | Codex |
| Story | `US-003-03` |
| Epic | `EPIC-003` |
| PRD requirements | `FR-024`, `FR-025`, `FR-026`, `FR-027`, `FR-028`, `FR-017A` |
| Branch | `story/us-003-03-linux-mappings-menu` |
| Pull request | Not opened |
| Status | Verified |

## Intended outcome

Add a Linux mappings menu and a built-in mapping lookup so Linux-first learners and AS/400 enthusiasts can navigate between familiar Linux terms and Rust/400 concepts from inside the shell.

## Starting context

Rust/400 already had a growing command set and subordinate menus, but the Linux learning bridge mostly lived in documents. There was no dedicated menu or in-shell command for starting from Linux concepts such as `PATH`, process, or user.

## Plan

1. Add a read-only mapping command with a small curated mapping set.
2. Add a dedicated Linux mappings menu that can trigger those lookups directly.
3. Verify with tests and update the learning documentation.

## Story context

- Story title: Navigate from Linux to Rust/400
- Acceptance criteria:
  - lookup supports at least the core Linux terms delivered in this slice;
  - each Linux term links to a relevant Rust/400 concept;
  - analogies avoid claiming one exact match.
- Definition of Done checks:
  - menu and command behavior tested;
  - docs updated;
  - build log indexed;
  - full local verification run.

## Work performed

Added a new `LNXMAP` command for looking up Linux-to-Rust/400 mapping cards and a new `LNX` subordinate menu reachable from the main screen. The menu uses predefined safe inputs such as `LNXMAP TERM(PATH)` so a learner can browse mappings without memorizing command syntax first. Added a curated initial mapping set covering `PATH`, process, user, filesystem, print spool, and message queue concepts, along with a dedicated repository document that mirrors those concepts in prose.

## Commands and observations

```text
$ cargo fmt --all -- --check
pass

$ cargo clippy --all-targets --all-features -- -D warnings
pass

$ cargo test --all-targets --all-features
pass
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Add one generic mapping command plus predefined menu inputs | Separate command per Linux concept; docs only | Keeps the shell teachable without exploding the command surface or turning docs into the only bridge | N/A |

## Problems, failed approaches, and recovery

None yet.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| `FR-027` Linux term lookup | `lnxmap_displays_a_registered_mapping_card`; `linux_mappings_menu_runs_a_predefined_lookup` | Pass |
| `FR-026` Linux examples displayed only | mapping renderer tests and output review | Pass |
| Full baseline | `cargo fmt --all -- --check`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo test --all-targets --all-features` | Pass |

## Material changes

- `src/mappings.rs`: curated Linux-to-Rust/400 mapping set and renderer.
- `src/commands.rs`: new `LNXMAP` command definition.
- `src/menus.rs`: new Linux mappings menu and predefined menu inputs.
- `src/main.rs`: menu-triggered mapping lookup and command output.
- `docs/concepts/as400-to-linux-command-mappings.md`: first dedicated mapping reference.

## Deviations and remaining risks

- The first slice uses a small curated mapping set rather than the full glossary.
- Several mapped Rust/400 commands such as `DSPLIBL`, `WRKSPLF`, and `DSPMSG` are referenced as learning targets even where implementation is still planned.

## Lessons learned

The learning bridge becomes much more approachable when it is reachable through the same menu model as the rest of the emulator. A small curated mapping set is enough to prove the in-shell concept before investing in a larger glossary source.

## Next action

Verify the full baseline, then consider broadening the mapping set or linking `HELP` to the same content source.

## Correction history

- 2026-08-13: Initial entry.
