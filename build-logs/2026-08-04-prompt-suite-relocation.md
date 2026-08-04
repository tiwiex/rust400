# Build Log: Prompt suite relocation and guide

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Project owner and Codex |
| Story | N/A — process documentation |
| Epic | EPIC-001 |
| PRD requirements | Product goal 6; PRD section 15 |
| Branch | `main` — no baseline commit yet |
| Pull request | Not opened |
| Status | Verified |

## Intended outcome

Document the relocated Rust/400 prompt suite so contributors know which prompts are active, their order, authority, outputs, and safe operating rules.

## Starting context

The project owner moved the five active Rust/400 prompts from `prompts/` into `prompts-400/`. The original ASPIRE prompts remained in `prompts/`, and its README still linked to the old active-prompt locations.

## Work performed

- Added `prompts-400/README.md` with authority rules, prompt catalogue, BA sequence, story-delivery loop, examples, operating rules, and maintenance guidance.
- Updated `prompts/README.md` to identify itself as the ASPIRE reference archive and point to the active suite.
- Added this entry to the chronological build-log index.

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Treat `prompts-400/` as active and `prompts/` as archive | Mix both suites in one folder | Clear provenance reduces the chance of running incompatible ASPIRE instructions | N/A |
| Document an authority order | Let each prompt stand alone | Prompts must not silently override PRD, ADR, story, or repository guardrails | N/A |
| Recommend manual story delivery first | Adapt autonomous loops immediately | Manual execution provides evidence needed to simplify and safely automate the process | N/A |

## Problems, failed approaches, and recovery

The user referred to `promps-400`, but filesystem inspection found `prompts-400`. The existing directory spelling was preserved rather than creating a second near-duplicate path.

## Verification evidence

| Requirement | Evidence | Result |
|---|---|---|
| All active prompts are catalogued | `prompts-400/README.md` prompt table | Pass |
| Prompt order is documented | BA baseline and story-delivery sections | Pass |
| Archive no longer links to moved files | `prompts/README.md` archive pointer | Pass |
| Relocation is chronologically recorded | `build-logs/index.md` and this log | Pass |

## Material changes

- `prompts-400/README.md`: Added the active-suite guide.
- `prompts/README.md`: Replaced stale active-prompt links with the new suite location.
- `build-logs/index.md`: Registered this entry.

## Deviations and remaining risks

- Historical build logs correctly mention the former paths and were not rewritten.
- The new prompt suite has not yet been exercised end to end.

## Lessons learned

- Separating active prompts from their source archive makes provenance and authority visible.
- A prompt catalogue should describe outputs and prerequisites, not merely filenames.

## Next action

Use `prompts-400/Rust400 - Use Cases.md` to produce the formal use-case catalogue, then review its coverage gaps.

## Correction history

No corrections recorded.

