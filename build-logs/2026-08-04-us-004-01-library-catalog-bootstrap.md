# Build Log: Bootstrap the library catalog and first `CRTLIB` behavior

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Codex |
| Story | `US-004-01` bootstrap slice |
| Epic | `EPIC-004` |
| PRD requirements | `FR-006`, `FR-007`, PRD sections 7.1, 8.2, and 9 |
| Branch | `story/us-004-01-library-catalog-bootstrap` |
| Pull request | Not opened |
| Status | Verified |

## Intended outcome

Create the first persistent library catalog behavior and add concept documentation that explains how Rust/400 libraries and objects relate to Linux concepts.

## Starting context

Rust/400 already parsed and validated `CRTLIB`, but the command handler only printed a placeholder message. The project PRD and Epic 004 already identified libraries and objects as a core learning bridge, but there was no dedicated concept page and no persisted catalog behavior yet.

## Plan

1. Add a concept document for libraries, objects, and Linux comparisons.
2. Implement a minimal versioned library catalog stored inside the workspace.
3. Replace the placeholder `CRTLIB` behavior with real persistence and verify it with tests.

## Story context

- Story title: Persist a versioned object catalog
- Acceptance criteria:
  - catalog state is reopened after restart without data loss for delivered library records;
  - stored records carry versioned catalog context and stable fields for the delivered slice;
  - invalid catalog content fails explicitly rather than being discarded.
- Definition of Done checks:
  - local quality checks pass;
  - user-facing documentation updated;
  - build log indexed;
  - persistence decision documented.

## Work performed

Added a dedicated learning document for libraries and objects, focused on the AS/400-to-Linux comparison that motivated the feature. Implemented a new library catalog module that stores library records under the workspace in a small versioned text file, using an atomic write-then-rename flow for catalog replacement. Updated the interactive command path so `CRTLIB` now creates a real library record and reports duplicates cleanly instead of only echoing parsed intent.

### Commands and observations

```text
$ cargo fmt --all && ./scripts/check.sh
All Rust/400 quality checks passed.

$ printf 'CRTLIB LIB(MYLIB) TEXT('\''Learning library'\'')\nEXIT\n' | cargo run -- --temporary-workspace
CRTLIB created library MYLIB with text 'Learning library'.
Session ended.
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Use a small versioned text catalog for the first library slice | SQLite; ad hoc per-library files | Keeps the first learning slice dependency-free and easy to inspect while preserving schema/version discipline | `ADR-0003` |

## Problems, failed approaches, and recovery

The main risk was over-designing the whole object system before a first vertical slice existed. To avoid that, the work focused on one real command and one concept document. This leaves room to evolve the catalog format later if the accepted ADR changes.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| `FR-006` persistence survives restart | `libraries::tests::list_libraries_reopens_persisted_records_after_restart` | Pass |
| `FR-007` create library behavior exists | `libraries::tests::create_library_persists_a_record_in_the_workspace_catalog` | Pass |
| duplicate libraries fail distinctly | `libraries::tests::create_library_rejects_duplicates` | Pass |
| catalog schema/version is checked | `libraries::tests::parse_catalog_rejects_unsupported_schema_headers` | Pass |
| local quality gate | `cargo fmt --all && ./scripts/check.sh` | Pass |

## Material changes

- `src/libraries.rs`: added persistent library catalog and tests.
- `src/main.rs`: replaced placeholder `CRTLIB` output with real library creation behavior.
- `src/lib.rs`: exported the new libraries module.
- `docs/concepts/libraries-and-objects.md`: added the learning bridge for libraries and objects.

## Deviations and remaining risks

- This is a bootstrap slice of `US-004-01`, not the complete object catalog described in Epic 004.
- The catalog currently stores library records only; typed objects and authority metadata remain future work.
- Display commands such as `DSPLIB` and `WRKLIB` are not implemented yet.

## Lessons learned

The best way to make Rust/400 feel authentic is to turn one AS/400 concept into a working emulator behavior and document the Linux comparison right beside it. That keeps the project educational and product-oriented at the same time.

## Next action

Implement `DSPLIB` and `WRKLIB` on top of the new library catalog, then formalize the broader object record format for the next Epic 004 slice.

## Correction history

- 2026-08-04: Initial entry.
