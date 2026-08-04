# Build Log: Codex guardrails and prompt curation

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Project owner and Codex |
| Story | N/A — development-process refinement |
| Epic | EPIC-001 |
| PRD requirements | Product goal 6; PRD section 15 |
| Branch | Not available |
| Pull request | Not opened |
| Status | Verified |

## Intended outcome

Add durable, repository-scoped guidance for Codex and turn an imported prompt pack into useful Rust/400 workflows without inheriting unrelated project assumptions.

## Starting context

The `.codex/` directory was empty. The `prompts/` directory contained fourteen substantial prompts originally written for an ASPIRE political platform using Python, Docker, multi-tenant services, different document paths, and hard-coded GitHub commands. Rust/400 had a PRD, backlog, and build-log process but no repository-level agent guidance.

During final verification, Git recognized the workspace as a new repository on `main` with no commits. This supersedes the earlier project-inception observation that Git did not recognize the workspace, without changing the fact that no branch or commit history exists yet.

## Plan

1. Determine the correct Codex surface for durable guidance versus runtime configuration.
2. Audit imported prompts for reusable lifecycle ideas and unsafe or irrelevant assumptions.
3. Add minimal repository guardrails and adapted Rust/400 prompts.
4. Record and verify the changes.

## Work performed

- Added root `AGENTS.md` as the authoritative Codex guidance for product scope, story flow, safety, architecture, learning content, verification, evidence, and review.
- Reserved `.codex/` for future trusted-repository runtime settings and documented why no `config.toml` is justified yet.
- Classified the imported prompts as reference material rather than active instructions.
- Added Rust/400-specific prompts for one-story delivery and independent branch review.
- Explicitly deferred autonomous multi-story loops until normal human-visible story delivery has proven reliable.

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Put durable guardrails in `AGENTS.md` | Put behavioral instructions in `.codex/config.toml` | Repository conventions and verification expectations belong in agent guidance; config is for runtime settings | N/A |
| Do not add `config.toml` yet | Pin model, sandbox, or approval defaults immediately | No shared runtime setting is currently necessary, and repository defaults should not weaken managed or personal safety policy | N/A |
| Adapt two prompts | Copy all fourteen; delete originals | Story delivery and review have immediate value; originals remain useful reference evidence | N/A |
| Keep source-control mutations user-authorized | Adopt imported autonomous commit/PR loops | The repository has no baseline commit or remote workflow yet, and the owner is learning the process interactively | N/A |

## Problems, failed approaches, and recovery

- The first shell loop split prompt filenames on spaces and failed to read their contents. The follow-up used null-delimited file discovery and explicitly quoted relevant paths.
- The official Codex manual helper could not resolve the documentation host under sandboxed network restrictions. The recommendation therefore follows the available official skill surface map; project config keys were not guessed or added.
- The official web fallback produced no usable page content in this environment. This is why `.codex/config.toml` remains intentionally absent rather than being created from memory.

## Verification evidence

| Requirement | Evidence | Result |
|---|---|---|
| Durable repository guardrails | Root `AGENTS.md` | Pass |
| `.codex` purpose documented | `.codex/README.md` | Pass |
| Imported prompts classified | `prompts/README.md` | Pass |
| Human-gated delivery workflow available | `prompts/Rust400 - Story Delivery.md` | Pass |
| Independent review workflow available | `prompts/Rust400 - Branch Review.md` | Pass |
| Project-specific ASPIRE rules excluded from active prompts | Review of active Rust/400 prompt contents | Pass |

## Material changes

- `AGENTS.md`: Added durable repository guidance.
- `.codex/README.md`: Reserved project configuration and explained the no-config decision.
- `prompts/README.md`: Added prompt provenance, classification, and adoption sequence.
- `prompts/Rust400 - Story Delivery.md`: Added the active single-story workflow.
- `prompts/Rust400 - Branch Review.md`: Added the active review workflow.
- `build-logs/index.md`: Registered this entry.

## Deviations and remaining risks

- No Codex runtime configuration or enforcement hook was added.
- The imported reference prompts still contain unrelated hard-coded instructions; the README warning must remain prominent.
- Branch review cannot be tested until an initial baseline and story branch exist.

## Lessons learned

- Prompt packs should be treated like code dependencies: inspect provenance, assumptions, paths, tools, and mutation permissions before adoption.
- A concise repository instruction file is more reliable than requiring an agent to load thousands of lines of prompts for every task.
- Behavioral expectations and mechanical enforcement should be separated; automate checks only after the intended process stabilizes.

## Next action

Review the new guardrails, establish the initial Git baseline, and then use the story-delivery prompt for `US-001-01`.

## Correction history

No corrections recorded.
