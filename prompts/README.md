# Prompt Library

## Status

The numbered prompts in this directory were imported as reference material from another project. They are not Rust/400 policy and must not be run unchanged. Active adaptations have moved to [`../prompts-400/`](../prompts-400/README.md).

Several contain project-specific assumptions, including:

- the product name ASPIRE;
- political-campaign and multi-tenant requirements;
- Python, `uv`, Ruff, Pyright, FastAPI, Celery, and Alembic;
- PostgreSQL, Redis, RabbitMQ, Docker, and frontend/backend paths;
- hard-coded repository owner and GitHub commands;
- `brain/...` document paths and different story identifiers; and
- autonomous commit, rebase, push, and pull-request behavior.

Those assumptions conflict with Rust/400's Rust stack, paths, backlog, safety model, and current lack of a functioning Git repository.

## What is reusable

| Source prompt | Reusable idea | Rust/400 treatment |
|---|---|---|
| Prompt 1 | Structured product discovery | Already reflected in `docs/PRD.md`; use as a completeness checklist only |
| Prompts 2 and 2b | Diagram briefs and visual generation | Defer until architecture and user flows are baselined |
| Prompt 3 | Use-case derivation | Adapted as `Rust400 - Use Cases.md` |
| Prompt 4 | Requirements traceability and outcome epics | Adapted as `Rust400 - Traceability and Epic Audit.md` |
| Prompt 5 | Issue, branch, commit, and PR conventions | Adapted as `Rust400 - GitHub Conventions.md` |
| Prompt 6 | Human-gated story delivery | Adapted as `Rust400 - Story Delivery.md` |
| Prompt 7 | Evidence-based independent review | Adapted as `Rust400 - Branch Review.md` |
| Prompts 6A, 7A, 8, and 9 | Autonomous loops and compliance gates | Experimental archive; do not use before normal single-story delivery works reliably |

## Active Rust/400 prompts

See the active [Rust/400 Prompt Suite](../prompts-400/README.md). It contains story delivery, branch review, use-case, traceability, and GitHub-convention prompts.

`AGENTS.md` is authoritative if an active prompt conflicts with it. The PRD and accepted ADRs remain authoritative for product and architecture decisions.

## Safe adoption sequence

1. Baseline the PRD and backlog.
2. Establish the initial Git baseline and choose the hosting workflow.
3. Complete one small story using the human-gated delivery prompt.
4. Review it with the branch-review prompt.
5. Simplify any ceremony that did not produce useful evidence.
6. Automate only stable, mechanical checks in CI or hooks.
7. Consider bounded autonomous workflows only after several successful manual story cycles.

This sequence keeps the useful discipline from the imported pack without inheriting its unrelated architecture or excessive process.
