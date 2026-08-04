# Rust/400 Prompt Suite

## Purpose

This directory contains the active, repository-specific prompts for planning, delivering, and reviewing Rust/400. They were adapted from the ASPIRE prompt pack in `../prompts/`, but use Rust/400's product boundary, Rust toolchain, document paths, identifiers, safety rules, and build-log process.

The ASPIRE prompts are reference material only. Do not run them unchanged against Rust/400.

## Authority

Prompts guide a particular workflow; they do not override project requirements or repository guardrails. If sources conflict, use this order:

1. explicit instruction from the project owner for the current task;
2. [`AGENTS.md`](../AGENTS.md) for durable repository guardrails;
3. [`docs/PRD.md`](../docs/PRD.md) for approved product scope and requirements;
4. accepted ADRs for durable architecture decisions;
5. the relevant epic and user story;
6. the selected prompt in this directory; and
7. historical ASPIRE prompts as non-authoritative reference material.

Do not silently resolve a material conflict. Record it and request an owner decision when it would change behavior, scope, security, data integrity, or architecture.

## Prompt catalogue

| Prompt | Use it when | Primary output |
|---|---|---|
| [Use Cases](<Rust400 - Use Cases.md>) | Turning the PRD and backlog into actor-centered behavioral flows | `docs/USE_CASES.md` |
| [Traceability and Epic Audit](<Rust400 - Traceability and Epic Audit.md>) | Mapping goals through stories/evidence and checking epic coverage | `docs/TRACEABILITY_MATRIX.md` and an epic-audit report |
| [GitHub Conventions](<Rust400 - GitHub Conventions.md>) | Defining issue, branch, commit, PR, label, milestone, and merge practices | `docs/GITHUB_CONVENTIONS.md` and local `.github/` templates |
| [Story Delivery](<Rust400 - Story Delivery.md>) | Implementing one approved, Ready user story | Code, tests, documentation, and an indexed build log |
| [Branch Review](<Rust400 - Branch Review.md>) | Reviewing one story branch before merge | Evidence-based findings, acceptance results, and merge verdict |

## Recommended order

### Product and BA baseline

1. Review and baseline `docs/PRD.md`.
2. Run **Use Cases**.
3. Review the catalogue and resolve requirement conflicts or coverage gaps.
4. Run **Traceability and Epic Audit**.
5. Approve any proposed backlog changes before editing stable IDs.
6. Run **GitHub Conventions** after source-control and hosting preferences are ready for decision.

### Story delivery loop

For each Ready story:

```text
Story Delivery
    -> focused implementation and tests
    -> build-log evidence
    -> Branch Review
    -> owner-authorized PR/merge
    -> traceability update
```

Start with `US-001-01`. Exercise the manual single-story loop successfully before introducing autonomous or multi-story automation.

## How to run a prompt

1. Start a fresh task or thread in the repository root.
2. Name the prompt and provide its required placeholder values, such as story ID and branch.
3. Ask the agent to read the prompt completely and follow it together with `AGENTS.md`.
4. Review generated documentation or the pre-implementation briefing before consequential actions.
5. Keep source-control and remote actions explicitly authorized.

Example:

```text
Follow prompts-400/Rust400 - Use Cases.md completely.
Generate the Rust/400 use-case catalogue, record the work in a build log,
and stop after reporting coverage gaps. Do not modify the PRD or backlog.
```

Story example:

```text
Follow prompts-400/Rust400 - Story Delivery.md for US-001-01.
Do not commit, push, or open a pull request unless I explicitly approve it.
```

## Operating rules

- Read the selected prompt completely before acting.
- Use the repository's actual state; do not assume files, tools, remotes, branches, or completed dependencies.
- Preserve stable `FR-*`, `NFR-*`, `UC-*`, `EPIC-*`, and `US-*` identifiers.
- Never label planned work as implemented or unrun checks as passing.
- Keep emulator data inside its configured workspace and never execute educational Linux examples automatically.
- Update and index a build log when the prompt requires one.
- Require an ADR for durable architectural decisions.
- Do not commit, push, create a PR, merge, publish, or configure remote services without explicit authorization.

## Maintaining the suite

When editing a prompt:

1. keep its purpose and expected outputs explicit;
2. remove machine-, user-, repository-, or provider-specific assumptions unless intentionally required;
3. keep paths aligned with the current repository structure;
4. record material process changes in a build log;
5. update this catalogue if a prompt is added, renamed, superseded, or retired; and
6. preserve the source ASPIRE prompts for provenance rather than overwriting them.

Future candidates include Rust/400 diagram generation, release review, and bounded autonomous delivery. Add them only when their inputs and manual workflow are stable.

