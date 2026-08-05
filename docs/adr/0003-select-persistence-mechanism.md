# ADR-0003: Select the persistence mechanism

| Field | Value |
|---|---|
| Status | Accepted |
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

Adopt a versioned text-file catalog inside the workspace for the first MVP library/object slices. The catalog must include an explicit schema header, fail closed on unsupported versions or malformed rows, and use write-then-rename replacement for atomic command-boundary updates.

SQLite remains a viable later migration path if object volume, query complexity, concurrency, or integrity constraints outgrow the text catalog.

## Consequences

### Positive

- Keeps the first persistent catalog dependency-free and easy for learners to inspect.
- Fits the current project size and current single-user workspace model.
- Preserves schema/version discipline before the object model expands.

### Negative

- Querying and migrations will require application logic rather than database features.
- Richer object relations may become awkward if the catalog grows substantially.

### Neutral or follow-up

- Re-evaluate the decision when typed objects, library-list resolution, and delete/update workflows are implemented.
- Document any future migration path so learners can see why persistence strategies change over time.

## Verification

- `libraries::tests::parse_catalog_rejects_unsupported_schema_headers`
- `libraries::tests::list_libraries_reopens_persisted_records_after_restart`
- local quality gate via `cargo fmt --all && ./scripts/check.sh`

## References

- [PRD](../PRD.md)
- [EPIC-004](../backlog/epics/EPIC-004-library-object-catalog.md)

## Amendment history

- 2026-08-04: Accepted a versioned text catalog for the first library/object persistence slices.
- 2026-08-04: Initial proposal created earlier the same day before the first implementation slice existed.
