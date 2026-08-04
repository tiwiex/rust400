# Architecture Decision Records

## Purpose

Architecture Decision Records (ADRs) preserve significant Rust/400 technical decisions, their context, considered alternatives, and consequences. They help contributors understand why the system took a particular direction without treating commit messages or build logs as permanent architecture documentation.

Use the [ADR index](index.md) to find current and historical decisions. Start new records from [ADR-0000](0000-template.md).

## When an ADR is required

Create or update an ADR before merging a change that materially affects:

- dependency direction or domain boundaries;
- persistent format, storage engine, migrations, or recovery;
- command-definition, parsing, validation, help, or extension design;
- emulator workspace or host-security boundaries;
- concurrency, job execution, or transaction semantics;
- public compatibility or supported-platform policy;
- a substantial runtime dependency or use of Rust `unsafe`; or
- a previously accepted architecture decision.

An ADR is usually unnecessary for local refactoring, test arrangement, formatting, or an implementation detail already covered by an accepted decision.

## Status lifecycle

| Status | Meaning | Permitted transition |
|---|---|---|
| Proposed | Under discussion; implementation depending on it must not merge | Accepted, Rejected, or Superseded |
| Accepted | Approved and currently governs affected work | Deprecated or Superseded |
| Rejected | Considered but intentionally not adopted | Final; use a new ADR if circumstances change |
| Deprecated | Once accepted but no longer recommended; still relevant to existing behavior | Superseded |
| Superseded | Replaced by a named later ADR | Final; must link to its replacement |

Do not rewrite the decision or consequences of an accepted ADR to match later thinking. Create a new ADR, mark the old record `Superseded`, and add reciprocal links. Typographical corrections and clarifications that do not change meaning may be appended to the amendment history.

## Workflow

1. Copy `0000-template.md` to the next available four-digit number and a concise kebab-case name.
2. Set status to `Proposed` and complete the context, drivers, options, proposed decision, and consequences.
3. Add the record to `index.md` in numeric order.
4. Link the relevant PRD requirements, epics, stories, build logs, and pull request.
5. Obtain owner review before changing the status to `Accepted` or `Rejected`.
6. Implement accepted decisions and cite the ADR in affected story logs and pull requests.
7. Supersede decisions with a new record rather than deleting history.

## Numbering and filenames

Use monotonically increasing four-digit numbers:

```text
0001-use-architecture-decision-records.md
0002-contain-state-in-explicit-workspaces.md
```

Numbers are identities, not priority. Never reuse the number of a deleted or rejected proposal.

## Ownership and approval

The project owner approves product-shaping and cross-cutting architecture decisions. Story reviewers confirm that implementation follows all relevant accepted ADRs. An agent may draft a proposed ADR but must not mark a material choice accepted without owner authorization.

