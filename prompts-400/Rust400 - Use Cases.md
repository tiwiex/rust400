# Rust/400 Use-Case Catalogue Prompt

## Purpose

Produce `docs/USE_CASES.md` as the behavioral bridge between the PRD and the product backlog. Do not change implementation code during this task.

## Role

Act as a senior business analyst familiar with interactive shells, operating-system concepts, and requirements traceability. Preserve Rust/400's educational, OS/400-inspired—not compatible—product boundary.

## Phase 1: Load context

Read completely:

1. `AGENTS.md`
2. `docs/PRD.md`
3. `docs/backlog/README.md`
4. every file under `docs/backlog/epics/`
5. accepted ADRs under `docs/adr/`, if present
6. `build-logs/README.md`

If requirements and backlog behavior conflict, report the conflict before authoring use cases. The PRD is authoritative unless it explicitly delegates a detail.

## Phase 2: Define actors

Define each actor with name, type, description, goals, and primary interaction surface. Validate, merge, or extend this starting set:

### Human actors

- Returning Operator
- Systems Learner
- Linux-first Learner
- Rust Contributor
- Business-analysis Learner
- Maintainer / Release Manager

### System actors

- Interactive Terminal Adapter
- Non-interactive CLI / Script Runner
- Command Registry and Parser
- Emulator Workspace and Persistence Adapter
- Session / Job Manager
- Message and History Services
- Spool Manager
- Learning Content Provider
- Host Operating System
- Clock and Identity Providers

System components are secondary actors only when treating them as actors makes an external interaction or boundary clearer. Do not turn every internal module into an actor.

## Phase 3: Derive the use cases

Derive use cases from observable user goals, not one use case per command or one per Rust module. Group them under these candidate domains, changing the groups only when evidence supports it:

- Shell and Command Discovery (`UC-SHL-NNN`)
- Libraries and Objects (`UC-CAT-NNN`)
- Learning Bridge (`UC-LRN-NNN`)
- Sessions, Jobs, Messages, and History (`UC-JOB-NNN`)
- Spooled Output (`UC-SPL-NNN`)
- Batch and Automation (`UC-BAT-NNN`)
- Workspace Safety and Recovery (`UC-SEC-NNN`)
- Contribution and Delivery (`UC-DEV-NNN`)

Use stable, three-digit sequence numbers within each domain. Never renumber a published use case merely to improve ordering.

For each use case use this structure:

```markdown
## UC-XXX-NNN: Short outcome name

| Field | Value |
|---|---|
| Status | Draft / Baselined / Superseded |
| Primary actor | Actor |
| Secondary actors | Actors or None |
| PRD requirements | FR-NNN, NFR-... |
| Related epic | EPIC-NNN |
| Related stories | US-NNN-NN |

### Goal

The outcome and user value.

### Preconditions

- State that must already be true.

### Trigger

The actor action or event that starts the use case.

### Main success flow

1. Actor action.
2. Observable system response.

### Alternative flows

- **A1 — Condition:** Behavior, rejoin point, and outcome.

### Exception flows

- **E1 — Failure:** Message ID category, unchanged/changed state, recovery path, and outcome.

### Postconditions

- Successful state and externally visible evidence.

### Business and domain rules

- Applicable rule with PRD reference.

### Linux learning connection

- Closest analogy, shared idea, where it breaks, and related example—or `Not applicable`.
```

Rules for flows:

- Describe externally observable behavior, not functions, database calls, or terminal libraries.
- Include invalid input, missing object, authorization where applicable, workspace boundary failure, and restart/recovery behavior when relevant.
- State whether persistent state is unchanged after each failure.
- Keep interactive and non-interactive variants in one use case when the user goal is identical; split only when flows materially differ.
- Do not claim historical OS/400 behavior that the PRD has not baselined.

## Phase 4: Coverage analysis

End `docs/USE_CASES.md` with:

1. a summary table: use-case ID, name, primary actor, requirements, epic, and stories;
2. every Must `FR-*` with no use-case coverage;
3. every use case with no requirement, epic, or story;
4. requirements that are constraints rather than behavioral use cases, with an explanation;
5. conflicts or open decisions discovered; and
6. a document history entry.

Do not hide gaps by inventing requirements or changing story scope. Recommend the smallest follow-up change and wait for owner review before modifying the PRD or backlog.

## Phase 5: Record the work

Create a build-log entry from `build-logs/TEMPLATE.md`, add it to `build-logs/index.md`, and record sources, derivation decisions, coverage counts, gaps, and verification performed.

Report the output path, number of actors and use cases, coverage gaps, and decisions requiring product-owner review.

