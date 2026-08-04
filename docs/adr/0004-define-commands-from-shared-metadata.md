# ADR-0004: Define commands from shared metadata

| Field | Value |
|---|---|
| Status | Proposed — decision pending |
| Date | 2026-08-04 |
| Owners | Project owner |
| Related requirements | `FR-003`, `FR-004`, `FR-005`, `FR-017`, `FR-019` |
| Related stories | `US-002-02`, `US-002-03`, `US-002-04` |
| Supersedes | None |
| Superseded by | None |

## Context

Parsing, validation, execution, built-in help, and completion need a shared description of each CL-like command. Independent definitions would drift; an overly dynamic plug-in model would add complexity before the MVP command set is understood.

## Decision drivers

- Keep documented syntax and runtime validation consistent.
- Provide type-safe handler inputs and structured errors.
- Support interactive and non-interactive adapters from the same definitions.
- Make command examples and learning content testable.
- Avoid procedural macros or plug-in complexity without demonstrated value.

## Options considered

### Option A: Typed static Rust definitions

Commands register typed metadata and handlers in code. This favors compile-time checks and straightforward navigation but requires recompilation to add commands.

### Option B: Declarative data files loaded at runtime

Commands or syntax metadata live in files. This improves data-driven extension but needs schema validation and a safe way to bind behavior to handlers.

### Option C: Procedural macros or generated code

One declaration could generate parser/help wiring, but macro complexity may obscure the design for learners and make diagnostics harder.

## Decision

The shared-source principle from `FR-005` is already required, but its concrete representation is not accepted yet. Before `US-002-03` implementation begins, build a small vertical prototype covering one command's definition, validation, help, and typed handler request. Compare static typed definitions with a minimal declarative representation and obtain owner approval.

## Consequences

### Positive

- The architecture cannot satisfy execution first and postpone help consistency.
- A bounded prototype will expose ergonomics before dozens of commands depend on the design.

### Negative

- `US-002-03` cannot merge until this ADR is accepted.

### Neutral or follow-up

- `US-002-02` parser work may explore syntax, but must avoid locking command metadata into an unapproved representation.
- Learning concept content may remain separate from executable command metadata if linked and validated from one source later.

## Verification

- The accepted revision must show one end-to-end command example.
- Tests must prove validation and help consume the same command definition.
- `US-002-03` cannot merge while this ADR remains Proposed.

## References

- [PRD](../PRD.md)
- [EPIC-002](../backlog/epics/EPIC-002-command-experience.md)

## Amendment history

- 2026-08-04: Initial proposal; representation intentionally deferred pending a vertical prototype.

