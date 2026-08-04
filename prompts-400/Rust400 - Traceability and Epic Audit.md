# Rust/400 Traceability Matrix and Epic Audit Prompt

## Purpose

Produce `docs/TRACEABILITY_MATRIX.md` and audit the existing epic/story decomposition against the approved PRD and use cases. Preserve stable IDs; do not regenerate the backlog merely because a different grouping is aesthetically preferable.

## Role

Act as a senior technical product manager and business analyst. Treat gaps, conflicts, duplicate coverage, and unsupported stories as findings to expose—not defects to conceal with invented mappings.

## Phase 1: Load context

Read completely:

1. `AGENTS.md`
2. `docs/PRD.md`
3. `docs/USE_CASES.md`
4. `docs/backlog/README.md`
5. every file under `docs/backlog/epics/`
6. accepted ADRs under `docs/adr/`, if present
7. relevant build logs

Stop and report a blocker if `docs/USE_CASES.md` does not exist or has unresolved structural errors that make mappings unreliable.

## Phase 2: Inventory without interpretation

Extract and count:

- product goals;
- all `FR-*` requirements and priorities;
- all named `NFR-*` requirements plus unnumbered NFR constraints;
- all `UC-*` use cases;
- all `EPIC-*` epics;
- all `US-*` stories;
- acceptance criteria at PRD and story levels; and
- dependencies between epics and stories.

Check for duplicate IDs before analyzing coverage.

## Phase 3: Audit the epic model

Evaluate the current seven epics as outcome clusters:

1. Delivery foundation
2. Command experience
3. OS/400-to-Linux learning bridge
4. Library and object catalog
5. Jobs, messages, and history
6. Spooled output and batch execution
7. MVP hardening and release

For each epic answer:

- Does it produce a coherent user or delivery outcome?
- Are its requirements and use cases closely related?
- Can its stories be delivered incrementally?
- Are dependencies explicit and acyclic?
- Is any story misplaced, duplicated, unsupported, or too broad?
- Does the epic contain between roughly four and eight MVP stories for understandable review scope? Treat this as guidance, not a hard rule.

If a merge, split, rename, or move is materially justified, present a change proposal containing current IDs, affected traceability, benefits, migration cost, and risks. Wait for owner approval before modifying epic files or stable IDs.

## Phase 4: Create the matrix

Write `docs/TRACEABILITY_MATRIX.md` with document status, version, generation date, source documents, and these tables.

### Table 1 — Goals to requirements

```text
Goal | Requirement IDs | Coverage status | Notes
```

### Table 2 — Functional requirements forward traceability

```text
FR | Requirement summary | Priority | Use cases | Epics | Stories | Verification status
```

### Table 3 — Non-functional requirements

```text
NFR or PRD section | Constraint summary | Affected epics/stories | Planned evidence | Status
```

Do not assign new NFR identifiers without owner approval. Where the PRD uses unnumbered constraints, cite the exact section.

### Table 4 — Use-case implementation traceability

```text
Use case | Primary actor | Requirements | Epic | Stories | Coverage status
```

### Table 5 — Story verification traceability

```text
Story | Epic | Requirements | Use cases | Acceptance-criteria count | Build log | PR/test evidence | Status
```

Before implementation, evidence fields may be `Planned`. Do not label planned work as implemented or verified.

### Table 6 — Epic coverage and dependencies

```text
Epic | Outcome | FR count | UC count | Story count | Depends on | Blocks
```

Add a Mermaid dependency graph only if it makes the epic sequence clearer and contains no invented dependency.

### Table 7 — Coverage gaps and conflicts

Include:

- Must FR with no use case, epic, or story;
- use case with no requirement, epic, or story;
- story with no requirement or use case;
- epic with no supported product/delivery outcome;
- duplicate or contradictory mappings;
- acceptance criterion with no planned evidence;
- NFR with no affected work or evidence strategy; and
- open PRD decision that blocks reliable traceability.

For each give severity, likely cause, affected IDs, recommended owner action, and whether delivery is blocked.

## Phase 5: Consistency rules

- Preserve requirement, use-case, epic, and story wording by concise quotation or faithful summary; do not subtly broaden it.
- A requirement may map to multiple use cases and stories.
- A story may satisfy multiple requirements, but every mapping must be defensible from its acceptance criteria.
- Infrastructure stories may map directly to NFRs or delivery goals when no user-flow use case is appropriate; label this explicitly.
- Out-of-scope and Won't requirements must not map to implementation stories.
- Historical OS/400 research tasks do not prove emulator behavior.
- Documentation presence does not prove runtime acceptance criteria.

## Phase 6: Review and recording

Validate counts and link targets mechanically where possible. Create and index a build log describing source versions, counts, gaps, proposed epic changes, and checks performed.

Report:

1. coverage percentages for Must FRs at use-case, epic, and story levels;
2. all blocking gaps;
3. proposed epic changes awaiting approval; and
4. the path to the traceability matrix.

Do not edit the PRD, use cases, epics, or stories during this audit unless the owner separately approves the exact proposed changes.

