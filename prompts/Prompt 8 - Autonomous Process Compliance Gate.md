# Prompt 8 — Autonomous Process Compliance Gate

**Status:** Active v1.0
**Date:** 2026-07-08
**Purpose:** Pre-commit blocking gate that enforces Prompt 6A's lifecycle requirements and Prompt 7's
audit checklist. Prevents "tests pass = done" drift by making process artifacts non-skippable.

**Relationship to other prompts:**
- **Prompt 6A** (§10–12) defines WHAT must exist before a commit (story log, micro-commit log,
  verify.sh, gate records, multi-line body). Prompt 8 is the enforcement mechanism.
- **Prompt 7** (§3 audits) defines the independent review checklist. Prompt 8 requires a simulated
  Prompt 7 review pass before story completion.
- **Prompt 7A** defines the multi-story stack review. Prompt 8 does not replace 7A — it ensures
  each individual commit/story already passed 7-level scrutiny before 7A runs at the end.

---

## When This Gate Fires

**Before every `git commit` on an autonomous experiment branch.** No exceptions for "I'll batch the
artifacts later" — that path leads to zero artifacts.

This gate also fires at **story completion** (before marking a story done and proceeding to the next).

---

## Pre-Commit Checklist (ALL must be YES)

Before executing `git commit`, halt and verify each item. If ANY item is NO, produce it first.

### 1. Story Log Exists and Is Current

```text
[ ] STORY_LOG.md exists at brain/build-and-dev/build-logs/{EPIC-XX}/{US-XXX-NN}/STORY_LOG.md
[ ] Story header fields are filled (Epic, Status, Started, Implementation LLM)
[ ] Acceptance Criteria section lists all ACs with PASS/FAIL/NOT VERIFIED status
[ ] Experiment Metadata section is populated (branch, worktree, model, prompt version)
```

If the story log doesn't exist yet (first commit for this story), create it now using the template
from Prompt 6A §8 before committing.

### 2. Micro-Commit Log Exists for THIS Commit

```text
[ ] A .md file exists at brain/build-and-dev/build-logs/{EPIC-XX}/{US-XXX-NN}/{slug}.md
[ ] Contains: What changed, Why, Gotchas/constraints, Checks run, Commit message
[ ] The commit message in the log matches what you're about to commit
[ ] Files listed match the actual staged files
```

The slug should be the commit's short description in kebab-case (e.g., `add-d32-hard-root-index.md`).

### 3. Commit Message Has Full Multi-Line Body

```text
[ ] Subject: <type>(<domain>): <description> [US-XXX-NN][FR-XXX]
[ ] Subject ≤ 150 characters
[ ] Domain is from the approved list (GITHUB_CONVENTIONS §8)
[ ] Story tag present for feat/fix/test/refactor/infra/perf commits
[ ] Body present with:
    - Files changed: <list>
    - Why: <architectural/business reason>
    - What: <mechanics in 1-2 bullets>
```

### 4. Relevant Checks Have Been Run

```text
[ ] ruff check passes
[ ] ruff format passes
[ ] pyright passes (0 errors)
[ ] pytest passes for the affected test files (at minimum)
[ ] Full pytest suite passes (for story completion commits)
```

### 5. Autonomous Gate Record Written

```text
[ ] A commit-readiness gate record exists in STORY_LOG.md for this commit
[ ] Contains: Timestamp, Files changed, Checks run, AC affected, Risk tier, Decision
```

---

## Story Completion Checklist (fires at end of story, before proceeding)

In addition to the per-commit checklist above, verify:

### 6. verify.sh Exists and Passes

```text
[ ] verify.sh exists at brain/build-and-dev/build-logs/{EPIC-XX}/{US-XXX-NN}/verify.sh
[ ] Is self-contained (sets UV_CACHE_DIR, uses scripts/with-integration-lock if Docker-backed)
[ ] Covers all acceptance criteria for this story
[ ] Passes when run with --ci flag
[ ] Evidence pasted into STORY_LOG.md
```

### 7. Independent Review Has Been Performed (Simulated Prompt 7)

```text
[ ] D8 Tenant Query Audit: every tenant-scoped model query has explicit tenant_id predicate
[ ] Domain Invariant Audit: key story terms expanded to concrete invariants and verified
[ ] Auth/RBAC Audit: new routes have correct access control
[ ] Cache Trust Audit: cached payloads have correct tenant identity and invalidation
[ ] Idempotency/Replay Audit: consume/credit/callback operations cannot be double-applied
[ ] Adapter Audit (D7): external calls behind Protocol, no SDK imports in business logic
[ ] Database Constraint Audit: partial indexes, unique constraints proven by test
[ ] Migration Audit: single Alembic head, reversible, no data loss
```

Record the audit results in STORY_LOG.md under `## Independent Review`.

### 8. Run Note Updated

```text
[ ] Gate log table has an entry for this story
[ ] Branch tip updated
[ ] Test count updated
[ ] Any findings or lessons recorded
```

---

## Enforcement Mechanism

This prompt works by being loaded into agent context alongside Prompt 6A. The agent must:

1. **Check this gate before every `git commit` command.** Not after. Before.
2. **Produce missing artifacts inline** — do not defer to "batch later."
3. **If the gate finds a NO**, stop the commit attempt, produce the missing artifact, then re-check.
4. **Log gate failures** — if you catch yourself about to commit without artifacts, record that in the
   story log as a near-miss. This builds evidence of whether the gate is working.

### Recovery from skipped gates

If commits were made without this gate (e.g., prior sessions, context loss), remediation is:

1. Write micro-commit logs retroactively for each un-logged commit
2. Update STORY_LOG.md with the missing sections
3. Write verify.sh and run it
4. Perform the Prompt 7 review against the existing diffs
5. Amend commit messages with full bodies (interactive rebase)
6. Update run note with remediation record
7. Note in the story log that artifacts were produced retroactively (honesty over appearance)

---

## What This Gate Does NOT Do

- It does not replace Prompt 6A — it enforces a subset of 6A's requirements at the commit boundary.
- It does not replace Prompt 7/7A — those are the full review protocols. This gate ensures a
  simulated review happens per-story; 7A is still the final stack-wide review.
- It does not prevent all process failures — an agent can still write low-quality artifacts. But
  the existence of artifacts creates an auditable trail that 7A can evaluate.
- It does not apply to `docs` or `chore` commits that are not tied to a story (per GITHUB_CONVENTIONS).
  Those still need proper commit messages but not story logs or verify.sh.

---

## Origin

This prompt was created after a session where an agent acknowledged instructions to "follow Prompt 6A
and 7/7A rigorously" but then produced 6 commits with zero process artifacts — no story logs, no
micro-commit logs, no verify.sh, no reviews, no multi-line bodies. The code was correct (299 tests
passed) but the process that catches design errors, D26 violations, and spec drift was entirely
absent. The failure mode was: agent prioritized shipping code over lifecycle, conflated "tests pass"
with "done," and never re-read the prompts during implementation.

The fix is structural: a blocking gate that fires at the commit boundary, not a behavioral
instruction that says "please also do the process stuff."
