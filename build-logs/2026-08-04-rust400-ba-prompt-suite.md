# Build Log: Rust/400 BA and GitHub prompt suite

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Project owner and Codex |
| Story | N/A — process tooling |
| Epic | EPIC-001 |
| PRD requirements | Product goal 6; PRD section 15 |
| Branch | `main` — no baseline commit yet |
| Pull request | Not opened |
| Status | Verified |

## Intended outcome

Adapt the useful business-analysis and source-control workflows from the imported ASPIRE prompt pack into native Rust/400 prompts.

## Starting context

Rust/400 already had active story-delivery and branch-review prompts. The imported Prompt 3, Prompt 4, and Prompt 5 contained useful structures but assumed ASPIRE actors, political domains, `brain/...` paths, a Python/Docker stack, and fixed GitHub conventions.

## Work performed

- Created a use-case prompt with Rust/400 actors, behavioral domains, flow rules, Linux learning connections, gap reporting, and build-log evidence.
- Created a traceability prompt that inventories requirements, use cases, epics, stories, acceptance criteria, dependencies, and evidence.
- Changed the original epic-generation behavior into an audit-first process so the seven existing Rust/400 epics and stable IDs are not silently replaced.
- Created a GitHub-conventions prompt that can generate documentation and local issue/PR templates while requiring explicit authorization for remote mutations.
- Updated the prompt catalogue to mark all three adaptations active.

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Audit existing epics before changing them | Regenerate epics from scratch | The backlog already has stable IDs and stories; migration must be deliberate and owner-approved | N/A |
| Keep infrastructure constraints in traceability | Force every NFR into a user-flow use case | Not every quality constraint is an actor interaction, but all still need planned evidence | N/A |
| Generate local GitHub templates only | Automatically configure a remote repository | Remote provider, ownership, protections, and merge policy are not approved yet | N/A |
| Prefer useful links over one label per FR | Create `req:FR-*` labels | Hundreds of maintenance-sensitive labels would add ceremony without clear learning value | N/A |

## Problems, failed approaches, and recovery

No implementation failures occurred. The source prompts required substantial semantic adaptation rather than name replacement because their actors, architecture, paths, and compliance rules were unrelated to Rust/400.

## Verification evidence

| Requirement | Evidence | Result |
|---|---|---|
| Use-case workflow is Rust/400-specific | `prompts/Rust400 - Use Cases.md` | Pass |
| Traceability preserves current IDs | `prompts/Rust400 - Traceability and Epic Audit.md` | Pass |
| GitHub actions remain user-authorized | `prompts/Rust400 - GitHub Conventions.md` | Pass |
| Prompt catalogue links active adaptations | `prompts/README.md` | Pass |

## Material changes

- `prompts/Rust400 - Use Cases.md`: Added use-case generation workflow.
- `prompts/Rust400 - Traceability and Epic Audit.md`: Added traceability generation and epic audit workflow.
- `prompts/Rust400 - GitHub Conventions.md`: Added source-control convention and template workflow.
- `prompts/README.md`: Registered the active prompt set.
- `build-logs/index.md`: Registered this build log.

## Deviations and remaining risks

- The prompts have been reviewed structurally but not yet exercised to produce their target artifacts.
- GitHub-specific choices remain proposed until a remote and owner preferences are established.
- Diagram and autonomous-loop prompts remain unadapted.

## Lessons learned

- Prompt adaptation must translate domain assumptions, mutation authority, and artifact paths—not just product names.
- Traceability generation should be non-destructive by default once stable work-item IDs exist.
- Separating artifact generation from remote configuration keeps BA documentation work safe and reviewable.

## Next action

Run `Rust400 - Use Cases.md`, review the generated catalogue, then run the traceability audit. Apply the GitHub-conventions prompt after the repository-hosting and merge-policy decisions are ready for review.

## Correction history

No corrections recorded.

