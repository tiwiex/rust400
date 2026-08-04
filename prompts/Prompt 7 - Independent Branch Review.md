# Prompt 7 — Independent Branch Review

**Part of a sequence** (all in this folder): reviews the output of `Prompt 6 - Interactive Build
Protocol.md` or `Prompt 6A - Autonomous Build Loop.md` for a single story/branch. See
`Prompt 7A - Autonomous Multi-Story Branch Review.md` instead when reviewing a stacked multi-story
autonomous run.

You are performing an independent review of a story branch before merge.
Load all context before forming any opinion — do not skip any item.

**Story:** `<STORY_ID>`
**Branch:** `<BRANCH_NAME>`
**Implementation model:** `<IMPLEMENTATION_LLM>`

---

## Mandatory context load

Read all of the following before reviewing anything:

1. `brain/build-and-dev/reference-docs/canonical_decisions.md`
2. `brain/build-and-dev/specifications/deliverables/GITHUB_CONVENTIONS.md`
3. The Epic file for this story — glob `brain/build-and-dev/specifications/deliverables/epics/` for the file containing `<STORY_ID>`
4. `brain/build-and-dev/build-logs/{EPIC-XX}/{US-CODE-NN}/STORY_LOG.md`
5. All micro-commit logs in `brain/build-and-dev/build-logs/{EPIC-XX}/{US-CODE-NN}/`
6. The full branch diff — `git diff main..HEAD`
7. The proposed PR message

Also note: if this branch diverged from main before recent commits landed on main, a
`git diff main..HEAD` will show those main-only changes as deletions. Check `git log main..HEAD`
to confirm what commits are actually on the branch before treating diff noise as findings.
For PR readiness, also check `git fetch origin` and `git diff origin/main...HEAD` so the review uses
the current remote base.

---

## Section 1 — Acceptance Criteria

For each AC in the epic spec: **PASS / FAIL / NOT VERIFIED**

Provide evidence for each — test name, code path, or verify.sh output.

---

## Section 2 — Scope

List every file changed on the branch. Flag anything the story had no reason to touch.
Be specific: what changed and why it is unexpected.

---

## Section 3 — Code review

Review the actual implementation changes:

- Does the code do what it claims to do?
- Correctness issues, edge cases missed, error handling gaps?
- ASPIRE patterns followed: adapter pattern, `tenant_id`, fail-fast, no silent fallbacks?
- Overbuilt or underbuilt relative to the ACs?

Focus on substantive issues, not style preferences.

Mandatory focused audits:

- **D8 tenant query audit:** for every tenant-scoped model touched, inspect ORM queries and confirm
  they explicitly filter by `tenant_id`; RLS is defence in depth, not a substitute for the ORM
  predicate.
- **Domain invariant audit:** expand key story words such as "validated", "current", "source of
  truth", "admin", and "operator" into concrete invariants from the epic prose, ACs, and technical
  notes. Verify every relevant invariant, not only the easiest visible one.
- **Auth/RBAC exposure audit:** for every new or changed API route, script, task, or admin action,
  answer who can read/write it. Sensitive financial/admin data must not default to "any
  authenticated tenant user" without explicit story evidence.
- **Cache trust audit:** for every tenant-scoped cached payload, confirm the cache key, payload
  tenant identity, validation-on-read, and invalidation/staleness behavior.
- **Replay/idempotency audit:** for auth, OTP, refresh tokens, payments, rewards, and provider
  callbacks, confirm consume/confirm/credit operations cannot be replayed or double-applied. For
  refresh-token reuse invalidation, check concurrent sibling refresh/session creation as well as
  replay of the same token row.
- **Database constraint audit:** when behavior depends on partial unique indexes, current-row swaps,
  or migration constraints, require either a live DB regression test or a clearly documented reason
  why service-side ordering is sufficient.
- **Generated verification audit:** if `verify.sh` exists, confirm it uses the repo runner or writable
  cache env vars (`UV_CACHE_DIR`, `COREPACK_HOME`) inside the script, uses the repository
  integration lock (`scripts/with-integration-lock` or equivalent `flock`) for Docker-backed gates,
  and fails rather than accepting skipped live integration tests during pre-PR verification.
- **Lesson capture audit:** if review finds a reusable miss, name the prompt/checklist/doc where it
  should be encoded so future agents do not rediscover it.

---

## Section 4 — Commit log review

Read each micro-commit log and compare it against the actual diff for that commit:

- Does the log accurately describe what the commit did?
- Is the commit subject well-formed — correct type, approved domain, story tag?
- Is the domain appropriate for the epic this story belongs to?
- Did any commit bundle unrelated changes?

Flag gaps between what the log claims and what the diff shows.

---

## Section 5 — PR message review

Review the proposed PR message against the diff:

- Does the summary accurately represent the work?
- Is each ticked checklist item genuinely true?
- Are the test types accurately described?
- Is the traceability section correct?

---

## Section 6 — Process artifacts

| Artifact | Present and complete? |
|---|---|
| STORY_LOG.md exists and is filled in (incl. Implementation LLM field) | Y / N |
| Micro-commit logs present — one per commit | Y / N |
| verify.sh present and covers all ACs | Y / N |
| Commit subjects use an approved domain from GITHUB_CONVENTIONS.md §8 | Y / N |
| No changes to files outside story scope | Y / N |

---

## Section 7 — Concerns

**Blocking (must fix before merge):** List each. If none: "None."

**Non-blocking (worth noting):** List each. If none: "None."

---

## Section 8 — Verdict

Choose one: **APPROVE / APPROVE WITH MINOR FIXES / REQUEST CHANGES / BLOCK MERGE**

One sentence reason.

---

## Section 9 — Model observation

One or two factual sentences for the experiment log — what was notable about how this
model followed ASPIRE instructions. Not a rating. Examples: followed micro-commit
discipline well; made out-of-scope edits to shared docs; commit messages were
well-formed throughout; optimistic PR checklist ticks without evidence.
