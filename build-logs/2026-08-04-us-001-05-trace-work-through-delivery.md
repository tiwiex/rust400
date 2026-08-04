# Build Log: Trace work through delivery artifacts

| Field | Value |
|---|---|
| Date | 2026-08-04 |
| Author | Codex |
| Story | `US-001-05` |
| Epic | `EPIC-001` |
| PRD requirements | Product goal 6; PRD section 15 |
| Branch | `story/us-001-05-trace-work-through-delivery` |
| Pull request | Not opened |
| Status | Verified |

Use the same stable story ID in the branch name, pull-request title, and build-log filename whenever the work belongs to a numbered story.

## Intended outcome

Make Rust/400's delivery workflow teachable by documenting how a requirement becomes a story, a branch, a pull request, verification evidence, and a build-log record.

## Starting context

The repository already had a PRD, backlog, build-log process, ADR process, and story branches in practice. What was missing was one explicit guide connecting those parts into a single learner-friendly workflow, plus templates that enforced the same stable story ID across story, branch, pull request, and build log.

## Plan

1. Confirm the story acceptance criteria and current documentation baseline.
2. Add a central development-process guide and reusable templates.
3. Verify documentation links and quality checks, then record traceability evidence.

## Story context

- Story title: Trace work through branches and build logs
- Acceptance criteria:
  - Story, branch, pull-request, and build-log templates request the same stable story ID.
  - A worked example demonstrates navigation from a PRD requirement to implementation evidence.
  - The Definition of Done prevents closing a story without tests, documentation, and build-log evidence where applicable.
- Definition of Done checks:
  - Documentation updated
  - Templates updated
  - Verification recorded
  - Build-log entry added to index

## Work performed

Confirmed that `EPIC-001` already defined `US-001-05` and that the backlog and build-log guides already described parts of the traceability model. Added a new development-process guide that explains the full delivery chain, stable identifiers, artifact rules, Definition of Ready, Definition of Done, a reviewer checklist, and a worked example based on `US-001-03`.

Added a story template and a pull-request template so the same story ID is now requested at planning and review time. Extended the build-log template with explicit story-context fields so acceptance criteria and Definition of Done checks can be captured consistently during implementation.

Updated the root README and backlog README so contributors can discover the new process guidance from the main documentation paths.

### Commands and observations

```text
$ git status --short --branch
## main...origin/main
?? prompts.zip

$ rg -n "US-001-05|trace work|Definition of Done|Definition of Ready" docs/backlog/epics/EPIC-001-delivery-foundation.md docs/backlog/README.md docs/adr/README.md README.md build-logs/TEMPLATE.md
Confirmed the story definition and acceptance criteria before editing.

$ ./scripts/check.sh
Formatting, clippy, and tests passed.
```

## Decisions and tradeoffs

| Decision | Alternatives considered | Reason | ADR |
|---|---|---|---|
| Put the learner-facing workflow in one `docs/DEVELOPMENT_PROCESS.md` guide | Spread the guidance across README, backlog README, and build-log README only | One central guide makes the delivery chain easier to study while keeping specialized docs focused | N/A |
| Use `US-001-03` as the worked example | Invent a fictional example; use `US-001-01` | `US-001-03` has clear requirement mapping, code evidence, and a completed build log, so it demonstrates the full chain better | N/A |

## Problems, failed approaches, and recovery

Creating the story branch initially failed because this environment can read `.git` but cannot write branch references without approval. After escalating that one Git action, the branch was created successfully and the rest of the work continued normally.

## Verification evidence

| Acceptance criterion or requirement | Evidence | Result |
|---|---|---|
| AC 1 | `docs/backlog/STORY_TEMPLATE.md`, `.github/PULL_REQUEST_TEMPLATE.md`, `build-logs/TEMPLATE.md` all request the stable story ID | Pass |
| AC 2 | `docs/DEVELOPMENT_PROCESS.md` worked example traces `FR-006` -> `EPIC-001` -> `US-001-03` -> code -> build log | Pass |
| AC 3 | `docs/DEVELOPMENT_PROCESS.md` Definition of Done requires tests, documentation, and build-log evidence where applicable | Pass |

## Material changes

- `docs/DEVELOPMENT_PROCESS.md`: added the central delivery-workflow guide and worked example.
- `docs/backlog/STORY_TEMPLATE.md`: added a reusable user-story template with traceability fields.
- `.github/PULL_REQUEST_TEMPLATE.md`: added a review template aligned to stories, requirements, and evidence.
- `build-logs/TEMPLATE.md`: added story-context prompts and stable-ID guidance.
- `docs/backlog/README.md`: linked the story template from backlog guidance.
- `README.md`: linked the development-process guide from the main documentation index.
- `build-logs/index.md`: registered this build-log entry.

## Deviations and remaining risks

No GitHub issue template was added in this story because the acceptance criteria only required story, branch, pull-request, and build-log templates. If we later want GitHub-native story intake, that can be a separate improvement.

## Lessons learned

Traceability becomes much easier to teach when every artifact uses the same stable identifier. The branch name alone is not enough; the story definition, pull request, build log, and verification notes all need the same anchor to make navigation obvious.

## Next action

Move to the next foundation or command-experience story and keep using the new templates so the process stays consistent in live work, not just in documentation.

## Correction history

None.
