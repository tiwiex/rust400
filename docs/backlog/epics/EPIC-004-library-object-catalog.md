# EPIC-004: Library and object catalog

## Outcome

Users can create, inspect, resolve, and safely manage persistent libraries and typed objects using OS/400-inspired naming rules.

## Stories

### US-004-01: Persist a versioned object catalog

**As a** user, **I want** emulator state to survive restarts **so that** my system behaves as a persistent environment.

- Requirements: FR-006, FR-010; NFR reliability
- Priority: Must
- Dependencies: US-001-03, ADR for persistence

Acceptance criteria:

1. Catalog state is reopened after a normal restart without data loss.
2. Stored records include schema version, stable identity, normalized name, library, type, owner, timestamps, and description.
3. Incompatible or corrupt state fails with a stable message and recovery guidance rather than being discarded.

### US-004-02: Manage libraries

**As a** returning operator, **I want** to create, list, display, and delete libraries **so that** I can organize emulated objects.

- Requirements: FR-007, FR-013, FR-014
- Priority: Must
- Dependencies: US-004-01, US-002-05

Acceptance criteria:

1. `CRTLIB`, `WRKLIB`, and `DSPLIB` perform their documented operations and render stable results.
2. `DLTLIB` rejects protected or non-empty libraries according to documented policy.
3. Duplicate and missing-library cases produce distinct message IDs without partial changes.

### US-004-03: Maintain a session library list

**As a** returning operator, **I want** an ordered library list **so that** unqualified object names resolve predictably.

- Requirements: FR-008, FR-009
- Priority: Must
- Dependencies: US-004-02, US-002-01

Acceptance criteria:

1. `ADDLIBLE`, `RMVLIBLE`, and `DSPLIBL` update or show the current session's ordered list.
2. Adding a missing or duplicate library follows documented behavior and provides an appropriate message.
3. A new session receives a documented default library list.

### US-004-04: Resolve qualified and unqualified object names

**As a** command user, **I want** names resolved through explicit qualification or the library list **so that** commands behave consistently.

- Requirements: FR-009; domain rules 4 and 5
- Priority: Must
- Dependencies: US-004-03

Acceptance criteria:

1. A qualified name resolves only in the named library.
2. An unqualified name resolves to the first matching object in library-list order.
3. Reordering the library list changes unqualified resolution but not qualified resolution.
4. Not-found and invalid-name results use stable message IDs.

### US-004-05: Inspect and delete typed objects

**As a** returning operator, **I want** to list, describe, and delete supported objects **so that** I can manage the emulated catalog.

- Requirements: FR-010, FR-011, FR-014
- Priority: Must
- Dependencies: US-004-04

Acceptance criteria:

1. `WRKOBJ` filters objects by documented name, library, and type selectors.
2. `DSPOBJD` displays the documented metadata for a resolved object.
3. `DLTOBJ` validates type, authority, protection, and dependencies before one atomic deletion.
4. Large listings are bounded or paginated.

