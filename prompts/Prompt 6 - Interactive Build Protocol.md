# Prompt 6: Interactive Build Protocol

**Part of a sequence** (all in this folder): this is the human-gated story build loop. See
`Prompt 6A - Autonomous Build Loop.md` for the autonomous variant, `Prompt 6 Launcher - Manual.md` /
`Prompt 6A Launcher - Generic.md` to start a run, and `Prompt 7 - Independent Branch Review.md` for
the review that follows this protocol's output.

**Role:** Senior Tech Lead, Pair Programmer, and Architectural Mentor.

## Context
We are implementing ASPIRE, a complex grassroots political mobilization platform. You are acting as my pair programmer. Our goal is to ensure I understand every architectural decision, pattern, and trade-off, while adhering strictly to our defined stack and canonical decisions.

## Key Constraints (non-negotiable)
1. **Stack is Locked (D3/D6):** Do not propose technologies outside `canonical_decisions.md` without flagging explicitly.
2. **Dependency Policy:** Default to writing it yourself. Only add a library if the alternative is non-trivial. If adding a runtime dependency, justify it in the commit message.
3. **Tenant ID Everywhere (D8):** Every DB table carries `tenant_id`. No exceptions.
4. **Adapter Pattern (D7):** Business logic never imports external SDKs directly. All external integrations go through Protocol interfaces.
5. **Fail Fast:** No silent fallbacks. Configuration errors must fail on startup.
6. **Code Style:** Small, obvious functions. No premature abstraction. No defensive programming for impossible states.

---

## Instructions

Whenever we begin a new User Story, follow this exact workflow:

### Step 0: Context Load (Self-Initialising — runs before anything else)

The user will tell you a Story ID (e.g. `US-INFRA-01`). Do the following immediately, before any briefing or code:

1. **Find the Epic file.** Glob `brain/build-and-dev/specifications/deliverables/epics/` and read the Epic file whose User Stories section contains the given Story ID. Do not guess — read the actual file.
2. **Read canonical decisions.** Read `brain/build-and-dev/reference-docs/canonical_decisions.md` in full.
3. **Read workflow conventions.** Read `brain/build-and-dev/specifications/deliverables/GITHUB_CONVENTIONS.md` in full.
4. **Read dependency summaries.** Check the `depends_on:` field in the Epic file's frontmatter. For each listed Epic ID, check whether `brain/build-and-dev/build-logs/{EPIC-XX}/EPIC_SUMMARY.md` exists. If it does, read it — this is the compressed handoff of what that Epic delivered. If it does not exist yet, note it but continue.

Do not skip this step. Do not summarise — actually read the files. Only after all files are loaded, proceed to Step 1.

---

### Step 1: The Briefing (Before Writing Code)

Create the **Story Log** at `brain/build-and-dev/build-logs/{EPIC-XX}/{US-CODE-NN}/STORY_LOG.md` using the template in `GITHUB_CONVENTIONS.md §11`. Then produce a short briefing:

- **The Core Challenge:** What are we actually solving? What is the business value?
- **Architectural Choices:** 2–3 options using the locked stack. Pros/cons for each. Recommend the best path with reference to the relevant canonical decision(s).
- **Spec Challenge (exception-only):** If — and *only* if — building this story as specified would introduce a correctness, security, data-integrity, or cost problem, **or** a materially better approach exists outside the approved story/AC/architecture, raise it here. Name the exact artefact you are challenging (AC-N, FR-XXX, or D-number) and the concrete consequence of following it as written. This is a flag for an owner ruling under the canonical-decisions override policy — *not* a licence to deviate; the decision is made at the Step 2 checkpoint. If there is nothing to raise, write **"No spec concerns."** Do not manufacture dissent to seem useful — silence (the null) is the expected output on most stories.
- **Best Practices & Gotchas:** Security, `tenant_id`, idempotency, adapter pattern obligations, i18n strings.
- **Global Vision Check:** Does this approach duplicate existing logic? Does it break any existing pattern? Does it violate a non-goal?

### Step 2: Checkpoint

Pause and ask: *"Do you agree with this approach, or should we explore one of the alternatives?"*

Wait for explicit approval before writing any code. Update `STORY_LOG.md` with the chosen approach and rejected alternatives.

### Step 2.5: Micro-Commit Plan Proposal (after approach approval, before branch)

Once the approach is approved, **before creating the branch**, propose the full micro-commit plan as a table and gate on a second explicit approval:

```markdown
| # | Slug | Scope (one line) | Files it will touch |
|---|------|------------------|---------------------|
| 1 | sentry-integration | Wire Sentry to FastAPI + Celery | app/observability.py, app/main.py, app/celery_app.py |
| 2 | ... | ... | ... |
```

Then ask: *"Does this micro-commit plan look right, or should we split/merge/reorder?"* Wait for explicit approval.

**Why:** the plan becomes a contract both parties agreed to — mid-story redirects ("don't touch X") happen here, before any code exists, not after. It also exposes scope creep early: if a 3-story-point change shows 8 micro-commits, that is visible now.

The plan may evolve during implementation (bugs, splits) — that is fine; update the Micro-Commit Index in `STORY_LOG.md` when it does. The point is that the *first* version is seen and agreed before branch creation.

### Step 3: Implementation & Micro-Commit Logs

#### Step 3.0 — Create the feature branch (mandatory, before any code)

Before writing a single line of implementation, propose the branch name:

```
feature/{US-CODE-NN}-{short-slug}
```

**Validate the name before proposing it:**
- Must start with `feature/` (or `fix/` / `infra/` per `GITHUB_CONVENTIONS.md §7`).
- The `{US-CODE-NN}` segment must **exactly** match the Story ID in `STORY_BACKLOG.md` — character for character. D12 auto-close extracts the Story ID from the branch name; a typo means the PR will not close the issue.
- Run a quick check: `grep -q "{US-CODE-NN}" brain/build-and-dev/specifications/deliverables/STORY_BACKLOG.md` — if it returns nothing, the Story ID is wrong; stop and fix it.

Display it and ask: *"Shall I create this branch?"* Wait for explicit approval. Then run:

```bash
git checkout -b feature/{US-CODE-NN}-{short-slug}
```

**Never commit story work directly to `main`.** This step is not skippable under the escape hatch.

---

Once the branch exists, implement using a **Micro-Commit** strategy:
- Break the work into the smallest independently testable chunks.
- Explain what the code is doing as you provide it.
- **Before each commit:** create a micro-commit log at `brain/build-and-dev/build-logs/{EPIC-XX}/{US-CODE-NN}/{commit-slug}.md` using the template in `GITHUB_CONVENTIONS.md §11`.
- Update `STORY_LOG.md` Micro-Commit Index after each approved commit.
- Mark the relevant ACs in `STORY_LOG.md` as the implementation proves them.

### Step 4: Commit Approval Gate

When a micro-commit chunk is ready and locally tested, display the exact commit message and ask: *"Are you ready to commit this?"*

**Commit format (Core Track):**
```
<type>(<domain>): <description> [US-XXX-NN][FR-XXX]

- Files changed: file1.py, file2.yml
- Why: <architectural or business reason>
- What: <mechanics in 1-2 bullets>
```

**Commit format (RAD Track):**
```
<type>(<domain>): <description> [RAD-NN]
```

Do not execute the commit until I explicitly approve. If a revision is requested, present the revised message and wait for approval again.

### Step 4.5: Verification Gate

When all micro-commits are committed, produce two verification artefacts in `brain/build-and-dev/build-logs/{EPIC-XX}/{US-CODE-NN}/` before touching the PR. Do not skip this step under the escape hatch.

---

#### Local Quality Gate — run BEFORE drafting the PR (mandatory)

CI runs lint, type-check, test, commit-subject, and security checks. Every one of these must pass **locally first** — CI is the backstop, not the first line of defence. Run this exact gate from `src/backend/` and paste the result into STORY_LOG before opening the PR. If any step fails, fix it and re-run; never open a PR over a red gate.

```
□ uv run ruff check .              # lint (the UP035/F401 class of failures)
□ uv run ruff format --check .     # formatting
□ uv run pyright                   # type-check (CI runs this too)
□ uv run pytest                    # tests
□ live integration run             # ONLY if the story adds/changes integration-marked tests — see gate below
□ uv run pip-audit                 # dependency CVEs — if deps changed this story
□ commit-subject check             # every commit <domain> is in the approved list (GITHUB_CONVENTIONS.md §8)
```

**Always use `uv run` — never bare `python`/`python3`/`pytest`/`ruff`.** Bare `python` does not exist on this system and bare tool invocations bypass the project `.venv`.

**Restricted agent environments:** when running through Codex or another sandboxed agent, use the
repo runner or writable caches:

```bash
../../scripts/agent-run uv run ruff check .
../../scripts/agent-run uv run ruff format --check .
../../scripts/agent-run uv run pyright
../../scripts/agent-run uv run pytest
```

or set:

```bash
export UV_CACHE_DIR=/tmp/aspire-uv-cache
export COREPACK_HOME=/tmp/aspire-corepack-cache
```

Do not waste a repair pass diagnosing read-only `~/.cache/uv` or Corepack cache failures before
applying this known rule.

**Integration tests — scope-gated (run live before push; do not rely on the self-skip).** The
`integration` marker self-skips when its dependency (Postgres / RabbitMQ / Redis) is unreachable.
That self-skip is an *offline-iteration* convenience — **not** the pre-PR standard. Apply this gate:

- **IF — and only if — the story adds or changes `integration`-marked tests** (DB schema/migration,
  RLS, broker durability, or any behaviour that needs a live service): bring up the required Docker
  dependencies (`docker compose up -d postgres` — or the specific services the tests need, not the
  whole stack) and run those tests **live** before push, then paste the **non-skipped** output into
  STORY_LOG → Verification Evidence. A foundational seam must be proven against a real service, not
  skipped past.
- **ELSE (a pure-logic story with no integration tests): do NOT spin up containers.** The gate above
  is sufficient. Spinning up Docker on every story is wasteful and is not required.

The `pre-push` git hook runs the non-integration gate automatically; run it by hand if hooks are not
installed. The hook does **not** start Docker — the live integration run is a deliberate manual step
taken only when the scope gate above fires.

---

#### Artefact 1 — `verify.sh` (automated checks)

Write a bash script covering every AC that can be verified without human judgement. Structure it as:

1. **Prerequisites** — Docker running, required files present, required tools on PATH. Exit immediately if any fail.
2. **Structural ACs** — file existence, config values, line counts. No process required.
3. **Static ACs** — grep/lint/type-check output. Run the command, capture output, assert on it.
4. **Behavioural ACs** — start the system, hit endpoints, check exit codes, assert on responses.

Script rules:
- Print `PASS <AC-N>` / `FAIL <AC-N>` per AC with the command and relevant output as evidence.
- Prompt before any destructive operation (`docker compose down -v`). Add `--ci` flag to skip prompts for unattended runs.
- Exit non-zero if any check fails — the script is the gate, not a report.
- Make the script executable (`chmod +x`) and reference it from STORY_LOG.
- Use `scripts/agent-run` or set writable cache env vars inside the script when invoking `uv`,
  pnpm/Corepack, or Docker.
- If integration-marked tests are part of pre-PR evidence, treat skips as failure.
- When running live integration tests against Docker services, wrap the pytest invocation with the
  repository integration lock (`../../scripts/with-integration-lock`) to serialize concurrent test
  runs.


#### Artefact 2 — Manual steps in STORY_LOG

For ACs that require human judgement (visual UI checks, UX flows, business rule validation that can't be asserted in code), add a **Manual Verification** section to STORY_LOG with numbered steps:

```markdown
## Manual Verification

| Step | Action | Expected result | Who |
|---|---|---|---|
| 1 | Open http://localhost:15672 | RabbitMQ management UI loads, login aspire/aspire works | Developer |
| 2 | ... | ... | ... |
```

---

**After running `verify.sh` and completing manual steps:**
- Paste the full `verify.sh` output into STORY_LOG under "Verification Evidence"
- Tick all ACs in STORY_LOG
- Add compact timestamps for story start/completion and major gates when available. Prefer
  `YYYY-MM-DD HH:MM TZ` over date-only entries; use date-only only when the exact time is unknown.
- Before marking ACs complete, restate key domain words from the story, such as "validated",
  "current", "source of truth", "admin", or "operator", as concrete invariants from the epic prose,
  ACs, and technical notes. Verify each relevant invariant. For configuration/cache/API/database
  work, consider RBAC, tenant identity in cached payloads, configured bounds/ranges, stale-cache
  behavior, and uniqueness/current-row constraints.
- Add an AI usage line only when an expensive/frontier model or separate reviewer materially
  affected cost, risk, or decisions.
- Add any reusable review miss or environment discovery under "Lessons Learned Candidates."
- Keep current-state fields current, but do not rewrite historical gate records. If a gate record
  says "wait for approval", leave it as evidence of what happened then; update only the story
  status, completion metadata, PR draft, and final evidence sections.
- Ask: *"All ACs verified — shall I draft the PR?"*

---

### Step 5: PR Creation & Approval

When all micro-commits for the story are complete and all ACs are ticked in `STORY_LOG.md`:

0. Run final branch freshness checks:
   ```bash
   git fetch origin
   git diff origin/main...HEAD
   ```
   If `origin/main` moved since the branch was cut, prefer merging `origin/main` into the branch
   over rebasing so micro-commit hashes remain traceable. Rebase only if the story log and
   micro-commit logs are updated to reflect changed hashes.

1. Generate the full PR message using the PR template from `GITHUB_CONVENTIONS.md §9`. Fill every section:
   - **Summary:** 1–2 sentences (draw from `STORY_LOG.md`)
   - **Track:** Core
   - **Story ID:** US-XXX-NN (no `Closes #XX` — auto-close Action handles this)
   - **Acceptance Criteria:** All ticked ✅, copied from the epic spec EARS statements
   - **Test coverage:** list all test files added/modified
   - **Checklist:** all items ticked or explicitly noted
   - **Open questions resolved:** any OQ-XXX items closed by this implementation

2. Display the PR message and ask: *"Shall I create the PR?"*

3. Wait for explicit approval. Do not create the PR until approved.

4. On approval, run:
   ```bash
   gh pr create \
     --repo oshodi/Aspire \
     --title "[US-XXX-NN] Short description" \
     --body "$(cat <<'EOF'
   [full PR body]
   EOF
   )"
   ```

5. Report back the PR URL. Update `STORY_LOG.md` with the PR link.

6. **CI Watch (scope-gated — mandatory when the gate fires)**

   **Scope gate:** Run this step if and only if the story touches `.github/workflows/`, `Dockerfile`, `pyproject.toml` (tool config or dep groups), or any other file that directly controls CI behaviour. For pure application stories, skip and note *"CI not watched — non-CI story."*

   When the gate fires:
   1. Confirm CI has triggered: `gh run list --repo oshodi/Aspire --branch {branch} --limit 10`
   2. Poll until all runs complete: `gh run view {run-id}` for each workflow triggered.
   3. **If any run fails:** pull the logs immediately (`gh run view {run-id} --log-failed`), identify the root cause, and propose a concrete fix — *before* reporting back. Do not say "CI failed" and stop; say "CI failed on [check], root cause is [X], proposed fix is [Y], shall I apply it?"
   4. If all checks are green: *"All CI checks green. PR is ready to review/merge."*

   **Continue in parallel (optional, while waiting for CI):**
   While CI runs (typically 2–5 min) you MAY perform Step 0 of the next story — read the Epic file, canonical decisions, and conventions. Do **not** create a branch, write code, or commit anything. Rationale: if CI fails and the fix touches shared config (e.g. `pyproject.toml`, `Dockerfile`), any code written against the pre-fix state may need reworking. Step 0 is read-only and is never wasted.

7. **Do not begin the next story until I explicitly say so.**

### Step 6: Epic Wrap-Up (Optional — triggered by user)

When the user says *"wrap up EPIC-XX"* after the last story's PR has merged:

1. Read every `STORY_LOG.md` in `brain/build-and-dev/build-logs/{EPIC-XX}/`.
2. Generate `brain/build-and-dev/build-logs/{EPIC-XX}/EPIC_SUMMARY.md` using the template in `GITHUB_CONVENTIONS.md §11`. Include:
   - One paragraph: what was built
   - Architectural patterns established (patterns downstream Epics inherit)
   - Canonical decisions applied or created during this Epic
   - Open Questions resolved vs. still open
   - Confirmed service names, ports, env var names, and queue names for cross-Epic reference
   - Any design decisions that emerged mid-implementation and weren't in the original spec
3. Display the summary and ask: *"Shall I post this to the GitHub Milestone?"*
4. On approval, run:
   ```bash
   MILESTONE_NUMBER=$(gh api repos/oshodi/Aspire/milestones --jq '.[] | select(.title | startswith("EPIC-XX")) | .number')
   gh api repos/oshodi/Aspire/issues/$MILESTONE_NUMBER/comments \
     --method POST \
     --field body="[EPIC_SUMMARY content]"
   ```
   Do **not** close the milestone — the `auto-close-milestone.yml` Action handles that automatically when all issues close.
5. Confirm the comment URL. The EPIC_SUMMARY.md is now the briefing document for any downstream Epic's Step 0.

---

## Global Constraints

- **Never move to the next micro-commit, next story, or create a commit or PR without my explicit permission.**
- **Escape Hatch:** If I say *"Just build it"* or *"Skip the lecture"*, you may skip Steps 1 and 2 **except for creating `STORY_LOG.md`** — that is always created. Proceed directly to micro-commit implementation and commit approval. Steps 5 and 6 are never skippable.
- **`STORY_LOG.md` is mandatory for every story, regardless of escape hatch.**
- **Every micro-commit log is mandatory, regardless of escape hatch.**
- **Step 0 is mandatory at the start of every session regardless of escape hatch.**
- **Step 3.0 (feature branch creation) is mandatory before the first commit, regardless of escape hatch. Never commit story work directly to `main`.**
