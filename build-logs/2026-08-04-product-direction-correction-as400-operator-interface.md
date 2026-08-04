# Build Log: Correct product direction toward AS/400-style operator interface

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Codex |
| Story | `N/A` |
| Epic | `N/A` |
| PRD requirements | Product direction correction; `FR-001`, `FR-005A`, `FR-017A`, `FR-019A` |
| Branch | `story/us-002-03-shared-command-metadata` |
| Pull request | Not opened |
| Status | Verified |

Use the same stable story ID in the branch name, pull-request title, and build-log filename whenever the work belongs to a numbered story.

## Intended outcome

Correct the project documentation so Rust/400 clearly targets an AS/400-style, 5250-inspired operator experience instead of a plain line-oriented shell as the final destination.

## Starting context

The existing PRD and README described Rust/400 primarily as an OS/400-inspired shell. That direction fit the engine work completed so far, but it did not match the intended end-state shared by the project owner: a recognizably AS/400-like full-screen menu and command environment.

## Plan

1. Update the PRD to make the operator-interface target explicit.
2. Adjust the backlog so command-engine work and screen-presentation work are separated cleanly.
3. Record the scope correction in a build log for traceability.

## Story context

- Story title: N/A
- Acceptance criteria:
  - N/A
- Definition of Done checks:
  - PRD updated
  - Backlog updated
  - README updated
  - Build log indexed

## Work performed

Updated the PRD from an "OS/400-inspired shell" framing to an "AS/400-inspired emulator" framing with a 5250-style terminal interface over a command engine. Revised the MVP scope, product goals, user journeys, requirements, and risks so menu navigation, full-screen rendering, and function-key guidance are first-class product capabilities rather than later optional polish.

Adjusted `EPIC-002` to clarify that it now represents the command engine beneath the full-screen experience. Added a new `EPIC-008` focused on operator-interface rendering, shared menu metadata, function-key support, and mixed menu/command input. Updated the backlog roadmap and README so the project’s visible direction is consistent across top-level documents.

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Keep current parser and command-registry work as foundation, not discard it | Rewrite the project plan around a UI-first prototype only | The command engine remains necessary underneath the future operator interface, so the work already completed is still valid | N/A |
| Add a dedicated operator-interface epic instead of folding screens into `EPIC-002` | Keep everything in the command epic | Separating command semantics from presentation makes the roadmap clearer and keeps future stories better scoped | N/A |

## Problems, failed approaches, and recovery

The main mismatch was conceptual rather than technical: the implementation work was heading in a useful direction, but the product documents were under-describing the intended final interface. The recovery was to correct the docs before more stories were planned against the wrong end-state.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| Product direction correction | `docs/PRD.md` now states an AS/400-inspired emulator with a 5250-style interface | Pass |
| Backlog alignment | `docs/backlog/README.md`, `docs/backlog/epics/EPIC-002-command-experience.md`, `docs/backlog/epics/EPIC-008-operator-interface.md` | Pass |
| Top-level visibility | `README.md` now states that the current prompt is a foundation layer, not the final interface | Pass |

## Material changes

- `docs/PRD.md`: corrected product framing, MVP scope, user journeys, and requirements.
- `docs/backlog/README.md`: added the operator-interface epic to the roadmap.
- `docs/backlog/epics/EPIC-002-command-experience.md`: clarified command-engine scope.
- `docs/backlog/epics/EPIC-008-operator-interface.md`: added the new operator-interface epic.
- `README.md`: clarified the long-term UI target versus the current engine state.
- `build-logs/index.md`: added this entry.

## Deviations and remaining risks

This correction was made while `US-002-03` work was already in progress on its own story branch. For clean history, the documentation correction should ideally be committed separately from the command-metadata implementation rather than merged as part of the same story.

## Lessons learned

Projects like this need the target interaction model stated early and explicitly. “Shell,” “terminal app,” and “emulator” sound close, but they lead to very different backlog choices once UI work begins.

## Next action

Split this documentation correction into its own backlog item or branch before merge, then continue with `US-002-04` for command help and `EPIC-008` for the first full-screen menu work.

## Correction history

None.
