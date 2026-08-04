# ADR-0004: Define commands from shared metadata

| Field | Value |
|---|---|
| Status | Accepted |
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

Adopt typed static Rust command definitions as the shared metadata source for the MVP. Each command definition records its name, summary, parameter metadata, validation rules, and handler identity in code. Validation, command-specific help, registry checks, and typed request-building all consume that same definition.

The prototype demonstrated the approach with `CRTLIB` end to end:

- parser output resolves the command name;
- shared metadata defines `LIB` and `TEXT`;
- validation consumes the same definition;
- help renders from the same definition; and
- a typed `CreateLibraryRequest` is built for the handler path.

This keeps the design explicit and easy to navigate for learners while satisfying the non-drift goal in `FR-005`.

## Consequences

### Positive

- The architecture cannot satisfy execution first and postpone help consistency.
- A bounded prototype will expose ergonomics before dozens of commands depend on the design.

### Negative

- Adding or changing commands requires a code change and recompilation.
- Help text and examples remain code-owned until a later design proves a declarative layer is worth the complexity.

### Neutral or follow-up

- `US-002-04` can now expand help and discovery on top of the accepted registry rather than inventing a second metadata source.
- Learning concept content may still remain separate from executable command metadata if linked and validated from one source later.

## Verification

- `CRTLIB` must remain covered by an end-to-end test from parsed syntax to typed request.
- Tests must prove validation and help consume the same command definition.
- Registry tests must fail when required command documentation metadata is missing.

## References

- [PRD](../PRD.md)
- [EPIC-002](../backlog/epics/EPIC-002-command-experience.md)

## Amendment history

- 2026-08-04: Initial proposal; representation intentionally deferred pending a vertical prototype.
- 2026-08-04: Accepted typed static Rust definitions after a working `CRTLIB` prototype proved shared validation, help, metadata checks, and typed request-building.
