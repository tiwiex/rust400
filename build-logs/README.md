# Rust/400 Build Logs

## Purpose

Build logs are the chronological engineering journal for Rust/400. They explain what was attempted, what actually happened, how the result was verified, and what was learned. Together with the PRD, backlog, ADRs, commits, pull requests, and tests, they make the development process inspectable for learners and maintainers.

A build log is not:

- a substitute for version control;
- the authoritative product requirement;
- an architecture decision record for a long-lived decision; or
- a dump of every terminal line or sensitive environment value.

Use an ADR for significant decisions and link it from the relevant build log. Use an issue for unfinished work and link that too.

## Directory contents

| Path | Purpose |
|---|---|
| [`TEMPLATE.md`](TEMPLATE.md) | Copy for each new log entry |
| [`index.md`](index.md) | Chronological register of entries |
| `YYYY-MM-DD-<story-id>-<slug>.md` | One implementation or investigation entry |

For work not yet associated with a story, use a concise subject in place of the story ID. Once a story exists, prefer its stable ID.

Examples:

```text
2026-08-04-project-inception.md
2026-08-12-us-002-01-command-loop.md
2026-09-03-adr-0002-persistence-spike.md
```

## When to create an entry

Create a build log for:

- each completed or materially advanced user story;
- an architecture or technology spike;
- a difficult defect investigation;
- a schema migration or release;
- a significant failed experiment worth preserving; or
- a change that produces useful learning evidence.

Routine typo fixes and dependency-only updates may share a short maintenance entry.

## Writing rules

1. Start from `TEMPLATE.md` and complete it during the work, not from memory weeks later.
2. Record facts separately from interpretation. Include relevant commands and summarized output, but redact usernames, tokens, private paths, and unrelated environment details.
3. Explain failures and abandoned approaches; they are part of the learning value.
4. Link PRD requirements, epic, story, branch, pull request, commits, ADRs, and test evidence when available.
5. Use repository-relative links so logs remain useful after cloning.
6. Record dates and times in ISO 8601. Use UTC for machine-generated times; include an offset for local manual entries.
7. Never rewrite a merged historical log to make the process appear cleaner. Add a dated correction note or a new entry.
8. Add every entry to `index.md` in reverse chronological order.

## Minimum completion standard

A story-related build log is complete when it states:

- the intended outcome and scope;
- requirements and story addressed;
- important implementation choices and alternatives;
- verification commands and their results;
- deviations, failures, or remaining risks;
- files or components materially changed; and
- the next recommended action.

## Suggested pull-request check

For story work, the pull-request reviewer should confirm:

- the log exists and appears in the index;
- it links to the correct story and requirements;
- evidence supports the acceptance criteria;
- sensitive information is absent; and
- durable decisions are also captured in ADRs.

