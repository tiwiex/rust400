# Build Log: Project inception and product definition

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Project owner and Codex |
| Story | N/A — discovery before backlog baseline |
| Epic | EPIC-001 |
| PRD requirements | Product goals and PRD sections 1–21 |
| Branch | Not available |
| Pull request | Not opened |
| Status | Verified |

## Intended outcome

Define an achievable product boundary for a Rust application that recreates selected OS/400 concepts in a Linux terminal, serves returning fans, teaches Linux comparisons, and demonstrates disciplined business-analysis and delivery practices.

## Starting context

The workspace contained no product source or project documentation. The product owner supplied the initial idea: emulate OS/400 as a Linux shell using Rust, follow standard design principles, write a PRD, decompose work into epics and stories, and use branches to learn a professional development workflow.

The product name “Rust/400” is provisional. The intended experience draws from OS/400 concepts, but the project has no goal of binary compatibility or IBM i replacement.

## Plan

1. Define the product, users, boundaries, requirements, and acceptance criteria in a PRD.
2. Add a learning bridge connecting OS/400 concepts to Linux analogies and their limitations.
3. Decompose the MVP into outcome-based epics and testable stories.
4. Establish a chronological build-log practice for future implementation.

## Work performed

1. Created `docs/PRD.md` version 0.1 with product vision, scope, user journeys, functional and non-functional requirements, conceptual architecture constraints, acceptance criteria, risks, and delivery practices.
2. Revised the PRD to version 0.2. Added an OS/400-to-Linux learning bridge covering libraries, objects, library lists, jobs, subsystems, queues, profiles, authorities, spooled output, messages, system values, data areas, data queues, devices, the IFS, and CL-like commands.
3. Made the limitations of each Linux analogy part of the requirement to prevent false equivalence. For example, a library list resembles `PATH` search order, but it resolves typed objects rather than only executable files.
4. Created seven outcome-based epics and 32 user stories under `docs/backlog/`, with PRD links, dependencies, priorities, and acceptance criteria.
5. Created the `build-logs/` folder, process guide, reusable template, index, and this initial record.

### Commands and observations

```text
$ rg --files
No ordinary project files were returned at project start.

$ git status --short --branch
Reported that the workspace was not a recognized Git repository.

$ wc -l -w docs/PRD.md
Used to confirm the PRD was written and non-empty before backlog decomposition.
```

The workspace contains a `.git` path, but Git does not currently recognize the workspace as a repository. No branch or commit could therefore be created for this discovery work.

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Build an educational OS/400-inspired emulator | Full IBM i compatibility; visual-only replica | A focused emulator can deliver useful learning workflows without an unbounded compatibility goal | Future ADR |
| Make Linux comparisons a product feature | Documentation-only appendix | Built-in, bidirectional comparisons directly support the learner persona | N/A — product decision in PRD |
| Organize epics by user outcome | Organize epics by Rust module or architecture layer | Outcome epics preserve user value and allow vertical implementation slices | N/A — backlog convention |
| Keep build logs in their own root folder | Store all notes in issues or commit messages | Repository logs remain cloneable, reviewable learning artifacts and can link across tools | Future ADR if tooling changes |
| Do not automatically execute Linux comparison commands | Offer a pass-through shell | It preserves workspace safety and avoids confusing education with host administration | N/A — FR-023 and FR-026 |

## Problems, failed approaches, and recovery

- Git repository detection failed. Documentation work continued because it did not depend on Git, but branch and pull-request traceability remains blocked until repository initialization or repair is explicitly handled.
- Exact historical compatibility rules, target OS/400 era, persistence technology, first object types, and terminal presentation remain open decisions. The PRD records these instead of silently guessing.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| Product definition exists | `docs/PRD.md` sections 1–8 | Pass |
| Linux learning bridge exists | `docs/PRD.md` section 9 and FR-024 through FR-028 | Pass |
| Requirements can map to delivery work | `docs/backlog/README.md` and seven epic files | Pass |
| Stories are testable | Each story contains acceptance criteria and dependencies | Pass by documentation review |
| Development process can be recorded | `build-logs/README.md`, `TEMPLATE.md`, and this entry | Pass |
| Git-based traceability works | Git repository status | Pending |

## Material changes

- `docs/PRD.md`: Established and refined the product requirements baseline.
- `docs/backlog/README.md`: Defined traceability, epic sequence, story states, estimation, and branch conventions.
- `docs/backlog/epics/`: Added the seven MVP epics and their user stories.
- `build-logs/README.md`: Defined the build-log process and governance.
- `build-logs/TEMPLATE.md`: Added a reusable evidence-oriented log format.
- `build-logs/index.md`: Added the chronological register.
- `build-logs/2026-08-04-project-inception.md`: Recorded the discovery and documentation work.

## Deviations and remaining risks

- No Git branch, commits, or pull request exist because the workspace is not currently recognized as a Git repository.
- The PRD remains a draft until the product owner reviews the open decisions and approves a baseline.
- Epic order is proposed rather than scheduled or estimated.
- Historical OS/400 behavior and terminology will require accuracy review against trusted references during story refinement.

## Lessons learned

- Linux analogies become more trustworthy when every comparison explicitly states where it fails.
- A compatibility boundary is necessary before backlog decomposition; otherwise stories can drift toward implementing an entire operating system.
- Stable requirement, epic, and story identifiers make documentation navigable even before an issue tracker is selected.
- Recording failed setup assumptions—such as the unavailable Git repository—is useful delivery evidence rather than noise.

## Next action

Review and baseline the PRD and backlog, resolve or initialize the Git repository, then refine and implement `US-001-01` on `story/us-001-01-initialize-rust-project`.

## Correction history

- 2026-08-04: A later verification recognized the workspace as a new Git repository on `main` with no commits. The earlier failed status check remains recorded because it accurately describes the state observed during inception; current work may proceed once the initial baseline is deliberately created.
