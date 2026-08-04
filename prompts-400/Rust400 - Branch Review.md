# Rust/400 Branch Review Prompt

Use this prompt to independently review one story branch before merge.

Inputs:

- Story: `<STORY_ID>`
- Branch: `<BRANCH_NAME>`
- Proposed PR description: `<PATH_OR_TEXT>`

## Context load

1. Read `AGENTS.md`, `docs/PRD.md`, and `docs/backlog/README.md`.
2. Read the epic file containing `<STORY_ID>` and all of that story's acceptance criteria.
3. Read applicable ADRs and build logs.
4. Inspect repository status, commits unique to the branch, and the full diff against the current base.
5. Read the proposed pull-request description.

Do not modify the branch during the review unless the user separately asks for fixes.

## Review output

### 1. Findings

List substantive findings in severity order: blocker, high, medium, low. For each include file and line evidence, consequence, violated acceptance criterion or guardrail, and a concrete correction. Do not lead with style preferences.

Check specifically for:

- incorrect or incomplete user behavior;
- partial mutation on failure and persistence corruption;
- workspace escape, command injection, unsafe path handling, and secret exposure;
- expected-input panics or unstable error/message behavior;
- incorrect object qualification or library-list resolution;
- terminal or persistence concerns leaking into domain logic;
- drift between parser, validation, help, and completion metadata;
- unbounded listings or avoidable scale failures;
- Linux analogies presented as exact equivalence;
- automatically executed comparison commands;
- copied proprietary material or unsupported compatibility claims;
- unnecessary dependencies or `unsafe` Rust; and
- out-of-story changes.

If there are no substantive findings, say so explicitly.

### 2. Acceptance criteria

For every criterion, give PASS, FAIL, or NOT VERIFIED and cite test, code, documentation, or manual evidence. A checked box in a PR is not evidence by itself.

### 3. Verification quality

Report which formatting, linting, unit, integration, security-boundary, and manual checks were actually run. Identify missing cases and flaky or assertion-light tests.

### 4. Process and traceability

Confirm story/epic/requirement links, branch naming, build-log accuracy, ADR coverage, documentation updates, commit scope, and PR-description accuracy.

### 5. Verdict

Choose one:

- APPROVE
- APPROVE WITH NON-BLOCKING FOLLOW-UP
- REQUEST CHANGES
- BLOCK MERGE

Give a one-sentence reason and list the minimum work needed to reach approval.

