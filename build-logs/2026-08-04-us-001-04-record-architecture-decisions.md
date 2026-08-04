# Build Log: Record architecture decisions

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Project owner and Codex |
| Story | `US-001-04` |
| Epic | `EPIC-001` |
| PRD requirements | NFR maintainability and observability |
| Branch | `story/us-001-04-record-architecture-decisions` |
| Pull request | Not opened |
| Status | Verified |

## Intended outcome

Give contributors a durable, reviewable process for proposing, accepting, rejecting, deprecating, and superseding significant architecture decisions before dependent implementation merges.

## Starting context

Three stories had been merged. Their build logs recorded choices, but no ADR lifecycle, index, or template existed. Workspace containment was already a significant merged decision. Persistence and command-definition representation remained explicitly open in the PRD.

## Plan

1. Define a lightweight ADR lifecycle and ownership policy.
2. Add a template and discoverable index.
3. Accept the ADR-process decision and transparently record the existing workspace decision.
4. Create proposed decision gates for persistence and shared command definitions without choosing prematurely.
5. Validate links, statuses, and story acceptance criteria.

## Work performed

- Added the ADR guide, lifecycle, workflow, numbering policy, and template.
- Added an accepted meta-ADR establishing when records are required.
- Added a retrospective accepted ADR for the workspace boundary merged before the process existed.
- Added proposed ADRs that block persistence and command-definition implementation until evidence and owner approval exist.
- Linked architecture decisions from the root README.

### Commands and observations

```text
$ ./scripts/check.sh
Formatting: PASS
Clippy: PASS
Tests: PASS (8 passed; 0 failed)

$ <ADR structural validation>
4 indexed decisions and 5 ADR-format files including the template: PASS
Accepted, Proposed, and supersession fields: PASS

$ <local Markdown link validation>
Local Markdown links: PASS

$ git diff --check
Completed successfully with no whitespace errors.
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Store ADRs as numbered Markdown | Build logs only; GitHub discussions only | Cloneable, linkable records keep current and historical reasoning discoverable | ADR-0001 |
| Keep accepted meaning immutable | Edit current decision in place | Supersession preserves the learning history and explains migrations | ADR-0001 |
| Record workspace decision retrospectively | Ignore pre-process choices; pretend ADR preceded code | Transparent timing is more trustworthy than manufactured history | ADR-0002 |
| Leave persistence and command representation Proposed | Select technologies during this documentation story | Both choices benefit from small evidence-producing spikes closer to implementation | ADR-0003, ADR-0004 |

## Problems, failed approaches, and recovery

- The first structural-validation shell command used `path` as a loop variable. In zsh, `path` is a special array tied to `PATH`, so subsequent `rg` lookup failed. No project file changed. The check was rerun with the task-specific variable `adr_file` and passed.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1: ADR template, index, and lifecycle exist | Structural and link validation of `docs/adr/` | Pass |
| AC 2: persistence and command-definition choices recorded before implementation | ADR-0003 and ADR-0004 explicitly block dependent merges pending acceptance | Pass |
| AC 3: superseded decisions link to replacements | Lifecycle, workflow, and template require reciprocal links; no decision is superseded yet | Pass |

## Material changes

- `docs/adr/README.md`: Defines purpose, lifecycle, workflow, and approval.
- `docs/adr/index.md`: Catalogues current ADR status.
- `docs/adr/0000-template.md`: Provides a reusable decision structure.
- `docs/adr/0001-use-architecture-decision-records.md`: Establishes the ADR practice.
- `docs/adr/0002-contain-state-in-explicit-workspaces.md`: Records the current workspace architecture.
- `docs/adr/0003-select-persistence-mechanism.md`: Gates persistence selection on evidence.
- `docs/adr/0004-define-commands-from-shared-metadata.md`: Gates command representation on a vertical prototype.
- `README.md`: Links the ADR index.

## Deviations and remaining risks

- Persistence and command representation remain intentionally undecided.
- Markdown rules are manually reviewed; automated ADR validation may be added after drift is observed.
- Proposed ADRs are decision gates, not accepted technical choices; dependent stories need refinement/spikes.

## Lessons learned

- ADRs are most useful when they make an undecided choice and its implementation gate visible, not only when they announce completed decisions.
- Retrospective ADRs should state their timing rather than implying the process existed earlier.
- Shell validation scripts must avoid zsh special variable names such as `path`.

## Next action

Perform the branch review, then commit, push, open the PR, and obtain hosted CI evidence through the established manual workflow.

## Correction history

No corrections recorded.
