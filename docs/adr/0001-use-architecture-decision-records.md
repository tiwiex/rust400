# ADR-0001: Use Architecture Decision Records

| Field | Value |
|---|---|
| Status | Accepted |
| Date | 2026-08-04 |
| Owners | Project owner |
| Related requirements | PRD section 12.6; product goal 6 |
| Related stories | `US-001-04` |
| Supersedes | None |
| Superseded by | None |

## Context

Rust/400 is both an emulator project and a learning project. Significant choices will affect security boundaries, historical analogies, persistence, command behavior, testing, and future extensibility. Code and commit history show what changed but do not reliably preserve the options and reasoning that led to a choice.

## Decision drivers

- Preserve architectural reasoning for learners and future contributors.
- Distinguish current policy from experiments and rejected options.
- Link product requirements, story work, implementation, and verification.
- Prevent important choices from being made invisibly inside a code review.
- Keep the process light enough for a small open-source project.

## Options considered

### Option A: Use short repository ADRs

Store numbered Markdown records in `docs/adr/`, maintain an index, and use an explicit lifecycle.

### Option B: Record decisions only in build logs

This reduces artifact count but mixes chronological work evidence with durable current architecture and makes supersession difficult to follow.

### Option C: Rely on issues, pull requests, and commit messages

Discussion stays close to implementation, but it becomes provider-dependent, harder to discover after cloning, and unclear which conclusion currently governs the code.

## Decision

Rust/400 will use short, numbered Markdown ADRs for significant architectural decisions. Proposed decisions do not authorize dependent implementation. Accepted records remain immutable in meaning and are replaced through explicit supersession.

## Consequences

### Positive

- Contributors can discover current decisions and their rationale in a clone.
- Rejected and superseded approaches remain visible.
- Stories and reviews gain a concrete conformance reference.

### Negative

- Significant changes require maintaining an additional artifact and index entry.
- Poorly scoped ADRs could create unnecessary ceremony.

### Neutral or follow-up

- Reviewers must decide whether a change is significant enough to require an ADR.
- The process may later gain mechanical link or status validation if manual drift becomes a problem.

## Verification

- The repository contains an index, lifecycle guide, and reusable template.
- Material decisions are proposed and accepted before their dependent implementation merges.
- Superseded records link to their replacements in both directions.

## References

- [PRD](../PRD.md)
- [EPIC-001](../backlog/epics/EPIC-001-delivery-foundation.md)
- [Build-log process](../../build-logs/README.md)

## Amendment history

- 2026-08-04: Accepted as part of `US-001-04` with owner authorization to proceed with the story.

