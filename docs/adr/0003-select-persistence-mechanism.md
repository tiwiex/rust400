# ADR-0003: Select the persistence mechanism

| Field | Value |
|---|---|
| Status | Proposed — decision pending |
| Date | 2026-08-04 |
| Owners | Project owner |
| Related requirements | `FR-006`, `FR-010`, `FR-014`; PRD sections 12.2 and 13 |
| Related stories | `US-004-01` and later catalog stories |
| Supersedes | None |
| Superseded by | None |

## Context

The library and object catalog must survive restarts, make command mutations atomic, carry a schema version, detect corruption or incompatibility, and remain inside the approved workspace. The PRD intentionally leaves the persistence technology open.

## Decision drivers

- Atomic state changes at a command boundary.
- Reliable recovery and schema migration.
- Straightforward backup and inspection for learners.
- Performance at the MVP reference scale of 10,000 objects.
- Minimal operational requirements and dependency risk.
- Isolation from arbitrary host paths.

## Options considered

### Option A: Embedded SQLite database

Provides transactions, indexes, constraints, and mature recovery behavior in one workspace file, at the cost of a runtime dependency and relational schema design.

### Option B: Versioned JSON or another text-file format

Easy to inspect and initially simple, but atomic multi-record mutation, indexing, concurrency, and migration require additional application logic.

### Option C: Hybrid metadata database and content files

Separates catalog queries from larger object content, but adds consistency and backup coordination concerns before the MVP demonstrates a need.

## Decision

No option is accepted yet. Before `US-004-01` implementation begins, conduct a bounded spike comparing SQLite and versioned files against atomicity, recovery, migration, inspectability, reference-scale performance, crate maintenance, and license criteria. Record measured evidence here and obtain owner approval.

## Consequences

### Positive

- Persistence implementation cannot silently select a technology.
- The comparison will use Rust/400-specific scale and learning requirements.

### Negative

- Catalog implementation remains blocked until the decision is accepted.

### Neutral or follow-up

- Create a spike story before `US-004-01` or refine that story to include the decision gate.
- Update dependency and backup guidance after acceptance.

## Verification

- The accepted revision must cite spike results and selected crate/version if applicable.
- `US-004-01` cannot merge while this ADR remains Proposed.

## References

- [PRD](../PRD.md)
- [EPIC-004](../backlog/epics/EPIC-004-library-object-catalog.md)

## Amendment history

- 2026-08-04: Initial proposal; decision intentionally deferred pending evidence.

