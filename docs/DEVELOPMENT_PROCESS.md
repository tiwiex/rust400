# Rust/400 Development Process

## Purpose

This guide explains how Rust/400 work moves from an idea in the PRD to verified evidence in a merged change. It exists for two audiences:

- contributors who need a repeatable workflow; and
- business-analysis learners who want to study traceability across delivery artifacts.

Use this guide together with the [PRD](PRD.md), the [backlog](backlog/README.md), the [ADR index](adr/index.md), and the [build-log guide](../build-logs/README.md).

## Delivery chain

```text
Product goal -> PRD requirement -> Epic -> User story -> Story branch -> Pull request -> Checks/tests -> Build log -> Merge
```

Each step answers a different question:

| Artifact | Main question answered |
|---|---|
| PRD | What problem are we solving and what must the product do? |
| Epic | What large outcome groups the work? |
| User story | What small user-facing or contributor-facing slice should we deliver next? |
| ADR | Why was an important technical decision made this way? |
| Story branch | Where is the implementation work isolated? |
| Pull request | What changed, why, and what evidence supports it? |
| Checks/tests | Does the change still meet the agreed technical bar? |
| Build log | What actually happened during delivery and what did we learn? |

## Standard workflow

1. Select a Ready story from the backlog.
2. Confirm the story links back to one or more PRD requirements.
3. Create a branch named `story/<story-id-lowercase>-<short-description>`.
4. Create or update the build log for that story as work begins.
5. Implement the smallest useful slice that satisfies the acceptance criteria.
6. Add or update tests, documentation, and ADRs where needed.
7. Run the local quality gate with `./scripts/check.sh`.
8. Open a pull request whose title starts with the same story ID.
9. Link the PR to the story, requirements, build log, ADRs, and verification evidence.
10. Merge only after the evidence supports the story's Definition of Done.

## Stable IDs

Rust/400 uses stable identifiers so a learner can follow one piece of work through every layer.

| Artifact | Example |
|---|---|
| Epic | `EPIC-001` |
| Story | `US-001-05` |
| Requirement | `FR-006`, `NFR-SEC-001` |
| ADR | `ADR-0002` |
| Branch | `story/us-001-05-trace-work-through-delivery` |
| Build log file | `2026-08-04-us-001-05-trace-work-through-delivery.md` |
| Pull request title | `US-001-05: Trace work through branches and build logs` |

The same story ID should appear in the story text, branch name, pull-request title, build log, and commit messages where practical.

## Artifact rules

### PRD

The PRD defines product goals, scope, functional requirements, non-functional requirements, and success measures. Do not change numbered requirements casually. If scope changes, update the PRD first and then update affected epics and stories.

### Epic and story backlog

The backlog translates requirements into deliverable increments. Every story should state:

- who benefits;
- what outcome is desired;
- which requirements it supports;
- observable acceptance criteria; and
- any dependency that affects sequencing.

Use [STORY_TEMPLATE.md](backlog/STORY_TEMPLATE.md) when adding or rewriting a story.

### ADRs

Use an ADR when the team makes a durable architectural choice with meaningful tradeoffs. Build logs can mention the choice, but the lasting rationale belongs in [`docs/adr/`](adr/README.md).

### Branches

One story belongs on one branch. Branch names follow:

```text
story/us-001-05-trace-work-through-delivery
```

Short-lived exploration that does not map to a story may use another branch type later, but MVP implementation work should stay on `story/...` branches for clarity.

### Pull requests

A pull request is the review package, not just a diff. It should show:

- the story and epic being delivered;
- the linked PRD requirements;
- acceptance criteria and how each was verified;
- ADRs touched or required; and
- risks, follow-ups, or deliberate deferrals.

Use [`.github/PULL_REQUEST_TEMPLATE.md`](../.github/PULL_REQUEST_TEMPLATE.md) for consistency.

### Build logs

Build logs capture chronology, verification, failed approaches, and lessons learned. They are especially important in Rust/400 because the project is also a learning artifact about disciplined delivery.

Use [`build-logs/TEMPLATE.md`](../build-logs/TEMPLATE.md) and add every new entry to [`build-logs/index.md`](../build-logs/index.md).

## Worked traceability example

This example shows how one implemented story maps through the delivery chain.

| Layer | Rust/400 example |
|---|---|
| Product goal | Goal 3: keep emulated state isolated from the host system by default |
| PRD requirement | `FR-006` persistent emulated system beneath a configurable Linux workspace |
| Epic | [EPIC-001](backlog/epics/EPIC-001-delivery-foundation.md) |
| Story | `US-001-03` Create isolated emulator workspaces |
| Branch | `story/us-001-03-isolated-emulator-workspaces` |
| Implementation evidence | [`src/workspace.rs`](../src/workspace.rs), [`src/lib.rs`](../src/lib.rs), tests in [`src/workspace.rs`](../src/workspace.rs) and [`src/main.rs`](../src/main.rs) |
| Build log | [2026-08-04-us-001-03-isolated-emulator-workspaces.md](../build-logs/2026-08-04-us-001-03-isolated-emulator-workspaces.md) |
| Merge evidence | repository history after merge to `main` |

How to read the example:

1. Start at `FR-006` in the PRD to understand the product need.
2. Open `EPIC-001` to see why workspace isolation was foundational.
3. Read `US-001-03` for the scoped outcome and acceptance criteria.
4. Inspect the build log to see the implementation sequence, evidence, and tradeoffs.
5. Review the tests and current code to confirm the delivered behavior.

This same pattern should work for every MVP story.

## Definition Of Ready

A story is Ready when:

- it links to at least one PRD requirement or goal;
- the user value or contributor outcome is clear;
- acceptance criteria are observable and testable;
- dependencies and assumptions are named; and
- the change is small enough to complete on one story branch.

If a story needs an unresolved architectural choice before work can start, record that as an ADR first or mark the story Blocked.

## Definition Of Done

A story is Done only when all applicable items below are true:

- implementation satisfies the acceptance criteria;
- automated checks pass locally with `./scripts/check.sh`;
- tests are added or updated where behavior changed;
- user-facing or contributor-facing documentation is updated where needed;
- durable technical decisions are captured in ADRs where applicable;
- a build-log entry exists, is indexed, and includes verification evidence;
- the branch, pull request, and build log all use the correct stable story ID; and
- remaining risks or follow-up work are documented rather than hidden.

For documentation-only stories, "tests" may mean link checks, formatting checks, review evidence, or another appropriate verification method. The build log should say what was used.

## Reviewer checklist

Use this quick check before merging story work:

- Does the branch name match the story ID?
- Does the pull-request title start with the same story ID?
- Are the linked PRD requirements the right ones?
- Is there evidence for each acceptance criterion?
- Is the build log present and added to the index?
- Are new architectural decisions captured in ADRs rather than only in comments or PR text?
- Are any deferrals or risks stated clearly?

## Practical notes for learners

- The branch is where work happens.
- The pull request is where work is explained and reviewed.
- The build log is where the real journey is preserved.
- The ADR is where long-lived technical reasoning is preserved.

If you can move comfortably from requirement to story to code to evidence, you are already practicing the core of traceable delivery work.
