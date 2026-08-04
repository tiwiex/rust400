# Rust/400 Product Backlog

## Purpose

This backlog translates the [Product Requirements Document](../PRD.md) into deliverable epics and user stories. It is the planning layer between product requirements and implementation work.

## Traceability model

```text
PRD requirement -> Epic -> User story -> Branch -> Pull request -> Test evidence -> Build log -> Release
```

Every MVP story must:

- identify its user and intended outcome;
- reference one or more `FR-*` or `NFR-*` requirements;
- contain observable acceptance criteria;
- identify dependencies that affect sequencing;
- use a stable story ID in its branch and pull request; and
- link to test evidence and a build-log entry before it is closed.

## Epic roadmap

| ID | Epic | Outcome | Suggested sequence |
|---|---|---|---|
| [EPIC-001](epics/EPIC-001-delivery-foundation.md) | Delivery foundation | Contributors can develop through a repeatable, traceable workflow | 1 |
| [EPIC-002](epics/EPIC-002-command-experience.md) | Command experience | Users can run, discover, and diagnose structured commands | 2 |
| [EPIC-003](epics/EPIC-003-learning-bridge.md) | OS/400-to-Linux learning bridge | Learners can connect OS/400 concepts to accurate Linux analogies | 3, then continuous |
| [EPIC-004](epics/EPIC-004-library-object-catalog.md) | Library and object catalog | Users can persist and resolve libraries and typed objects | 4 |
| [EPIC-005](epics/EPIC-005-jobs-messages-history.md) | Jobs, messages, and history | Sessions behave like inspectable jobs with useful operational feedback | 5 |
| [EPIC-006](epics/EPIC-006-spooling-and-batch.md) | Spooled output and batch execution | Users can retain reports and automate repeatable command flows | 6 |
| [EPIC-007](epics/EPIC-007-mvp-hardening.md) | MVP hardening and release | The MVP is safe, documented, measurable, and releasable | 7 |

Sequence numbers express technical dependency, not fixed iterations. EPIC-003 begins after the help framework exists and continues alongside each domain epic.

## Story states

| State | Meaning |
|---|---|
| Proposed | Captured but not yet refined or committed |
| Ready | Meets the Definition of Ready in the PRD |
| In progress | A contributor is actively implementing it |
| In review | Implementation and evidence are under review |
| Done | Meets the Definition of Done in the PRD |
| Blocked | Progress requires a documented dependency or decision |

The backlog files describe the baseline scope. A project board or issue tracker should hold current state, assignee, estimate, and iteration because those values change frequently.

## Priority and estimation

- Priority follows MoSCoW from the PRD: Must, Should, Could, Won't for the MVP.
- Estimate stories only after refinement, using one consistent scale such as 1, 2, 3, 5, 8.
- Split any story estimated above 8 or unlikely to complete in one iteration.
- Do not compare points between projects or treat them as hours.

## Branch and pull-request convention

Use `story/<story-id-lowercase>-<short-description>`, for example:

```text
story/us-002-01-command-loop
```

The pull request title begins with the story ID. The description links the epic, PRD requirements, acceptance criteria, test evidence, ADRs, and relevant build log.

## Change control

Minor clarification may be committed directly to a story document. A change to product scope or a numbered requirement must first update the PRD, record the reason in a build log, and then update every affected epic and story.

