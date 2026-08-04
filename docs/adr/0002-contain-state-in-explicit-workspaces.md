# ADR-0002: Contain emulator state in explicit workspaces

| Field | Value |
|---|---|
| Status | Accepted — retrospective |
| Date | 2026-08-04 |
| Owners | Project owner |
| Related requirements | `FR-006`, `FR-021`, `NFR-SEC-001`, `NFR-SEC-002`, `NFR-SEC-003` |
| Related stories | `US-001-03` |
| Supersedes | None |
| Superseded by | None |

## Context

Rust/400 needs persistent and disposable state while remaining safe to run under an ordinary Linux account. Emulated object names must not become arbitrary host paths. Workspace containment was implemented in `US-001-03` before the ADR process existed, so this record transparently documents the already reviewed and merged decision.

## Decision drivers

- Prevent traversal and symbolic-link escape from the emulator boundary.
- Keep all mutable state discoverable and removable by the user.
- Support repeatable tests and demonstrations without permanent residue.
- Avoid root privileges and arbitrary host-system mutation.
- Keep the initial policy understandable to learners.

## Options considered

### Option A: Explicit absolute workspace with strict symlink rejection

Permanent operation requires an absolute root. Relative children reject parent traversal, and symbolic links are rejected in workspace paths. Temporary operation allocates and cleans a unique directory.

### Option B: Use the current working directory implicitly

This is convenient but makes state placement dependent on how the program was launched and risks contaminating unrelated directories.

### Option C: Permit symlinks after canonicalization

This is more flexible but makes the teaching and security model more complex and introduces additional race and policy questions.

## Decision

Rust/400 will place mutable emulator state beneath one explicit, canonical absolute workspace. The initial implementation rejects parent traversal and all symbolic-link components. Disposable mode uses a unique system temporary directory and removes it on clean release.

This decision covers the MVP's single-process threat model. It does not claim race-proof containment against a concurrent hostile process replacing filesystem components between validation and mutation.

## Consequences

### Positive

- Users can identify the emulator's complete mutable footprint.
- Adversarial traversal and symlink cases have a simple deny policy.
- Tests can use isolated disposable state.

### Negative

- Legitimate symlink-based workspace arrangements are rejected.
- Cleanup after abnormal termination is not guaranteed.
- Future multi-process or hostile-host requirements may require descriptor-relative filesystem APIs or a stronger sandbox.

### Neutral or follow-up

- Persistence implementations must accept the workspace abstraction rather than constructing arbitrary host paths.
- Relaxing symlink policy requires a superseding ADR and security tests.

## Verification

- Integration tests assert both rejection and absence of outside artifacts for traversal and symlink attempts.
- Permanent CLI initialization creates state only beneath the selected root.
- Temporary workspace tests confirm clean-release removal.

## References

- [US-001-03 build log](../../build-logs/2026-08-04-us-001-03-isolated-emulator-workspaces.md)
- [Workspace implementation](../../src/workspace.rs)
- [Workspace security tests](../../tests/workspace_isolation.rs)

## Amendment history

- 2026-08-04: Recorded retrospectively after PR #2 because the ADR process was introduced in the following story.

