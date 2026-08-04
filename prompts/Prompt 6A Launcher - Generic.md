# Generic Autonomous Run Prompt

**Status:** Copy/paste run prompt v0.1
**Date:** 2026-07-04
**Base loop:** `brain/build-and-dev/prompts/Prompt 6A - Autonomous Build Loop.md`
**Purpose:** Start an autonomous build-loop run over **any** story range, in **one reused worktree**,
without hand-writing branch names. You provide the story range; the agent discovers epic, titles,
and slugs, plans per-story branches, and runs the 09 loop.

This generalizes the epic-specific Phase B/C run prompts (`11_…`, `13_…`) and reuses the
parameterization pattern from the manual run prompt (`12_…`). Unlike those, it hardcodes nothing and
creates per-story **branches** inside a single worktree rather than a worktree per story.

---

## 1. How This Works (read once)

- **You create the worktree once, for the whole range**, and open it in VSCode.
- **The agent never creates another worktree.** Between stories it runs `git switch -c` inside the
  same folder — same window, same context. `git worktree add` would spawn a new folder and throw the
  agent out of context; `git switch` does not.
- **One story = one branch = one draft PR** (09 §13). The agent computes each branch name from the
  story title, not from a hand-typed slug.
- The only thing you must supply is the **story range**. Everything else has a sensible default below.

---

## 2. Options And Defaults

| Option | Default | Override when |
|---|---|---|
| **Story range** | *(required — the one input you must give)* | — |
| **Epic** | Auto-derived from the story-ID prefix via the GITHUB_CONVENTIONS epic-code table | — |
| **Tool / model / effort** | Auto-discovered from branch name + running agent; if unknown, record `UNKNOWN` and ask | — |
| **Cadence** | `per-story` (stop for human approval after each story's completion gate) | `end-of-loop` only for well-specified, low-risk ranges where an early mistake is cheap to unwind |
| **Branch topology** | *Derived from cadence — not a separate knob.* `per-story` → **independent** (branch off freshly-pulled `main`); `end-of-loop` → **stacked** (branch off predecessor's tip, PR base = predecessor) | — |
| **Publication** | Enabled — push each story's branch and open a **draft** PR after `verify.sh --ci` passes | Disable to keep the run purely local (no push, no PR) |
| **Merge** | **Never.** The loop only ever opens draft PRs; humans merge | — |
| **Live external providers** | None — use stub adapters; local unit/static checks first | Only if a story explicitly requires it **and** a human approves |
| **Docker** | Verification gates only, with the integration lock | — |
| **Control doc** | The epic file + this prompt. Use a per-epic experiment control doc **only if one exists** | — |

Default cadence is `per-story` because it flows dependencies through `main` (each PR merged before the
next story starts) and keeps every PR independent and small. Choose `end-of-loop` only when you accept
stacked PRs and won't merge mid-loop.

---

## 3. Create The Worktree (once, for the range)

Fill in the range and a short run tag, then run from the main checkout. The worktree is created in a
**detached HEAD** state — the loop switches off it to real `feature/…` branches per story, leaving
no holding branches behind.

```bash
cd /home/oshodi/projects/Aspire

git fetch origin
git checkout main
git pull --ff-only origin main

mkdir -p ../Aspire-worktrees

# Example range US-CFG-05..US-CFG-06 with tool/model tag; adjust to your run.
RANGE_TAG="epic-23-us-cfg-05-06-claude-opus48-autonomous"

git worktree add \
  -d \
  "../Aspire-worktrees/${RANGE_TAG}" \
  main

cd "../Aspire-worktrees/${RANGE_TAG}"
```

Open `../Aspire-worktrees/${RANGE_TAG}` in a separate VSCode/agent window, then paste §4.

---

## 4. Paste This Into The Agent

Replace `{STORY-RANGE}` (e.g. `US-CFG-05 through US-CFG-06`). Leave the rest unless you are overriding
a default from §2.

```text
Use the autonomous build loop in:
brain/build-and-dev/prompts/Prompt 6A - Autonomous Build Loop.md

Story range: {STORY-RANGE}.

Before running the loop, do a one-time DISCOVERY + BRANCH PLAN preflight, then wait for my
confirmation of the plan:

1. Resolve the epic code from the story-ID prefix using the epic-code table in
   brain/build-and-dev/specifications/deliverables/GITHUB_CONVENTIONS.md (e.g. CFG -> EPIC-23).
2. Locate the epic file under
   brain/build-and-dev/specifications/deliverables/epics/EPIC-XX_*.md and extract each in-range
   story's title from its `**US-XXX-NN: Title**` heading.
3. For each story, compute a branch name per GITHUB_CONVENTIONS Branch Naming:
   feature/US-XXX-NN-<slug>, where <slug> is derived from the title: lowercase, hyphen-separated,
   no underscores, no spaces, <= 5 words, filler words dropped, story-ID prefix matching exactly.
4. Print a branch-plan table (Story ID | Title | Branch name) and STOP for my confirmation.
   Branch/PR names are outward-facing artifacts (09 §13) — do not proceed on an unconfirmed plan.

Cadence: per-story. After each story's completion gate, stop and wait for my explicit approval
before starting the next story. Record whether it is safe to continue, but do not proceed until
approved.
  (If I set cadence to end-of-loop instead: run the whole range without stopping at completion
   gates, opening a draft PR per completed story as you go, and gather one review at the end.
   Hard-stops — Level C decision, missing/contradictory acceptance criteria, tests still red after
   one repair pass, or any control-doc stop — still halt the entire loop immediately.)

Per-story branch handling (inside the ONE reused worktree — never create another worktree; use
git switch, not git worktree add):
- per-story cadence -> before implementing each story: git fetch origin; git switch main;
  git pull --ff-only; git switch -c feature/US-XXX-NN-<slug> main. Independent PRs; I merge each
  before the next story starts, so dependencies flow through main.
- end-of-loop cadence -> branch the first story off freshly-pulled main; branch each later story off
  its predecessor's branch tip (stacked). Set each stacked PR's base to its predecessor and note the
  required merge order in the PR body.

Autonomy:
- may commit to the current story's feature branch
- may push the branch after each completed story gate and drafted PR message
- may create a draft PR only after final verify.sh --ci passes
- may not merge
- may not alter canonical decisions
- may not alter active specs to fit implementation convenience
- no live external providers; use stub adapters; local unit/static checks first
- Docker allowed only for verification gates, and must use the integration lock
- generated verify.sh scripts must be self-contained: use scripts/agent-run or set writable cache
  env vars inside the script, and use scripts/with-integration-lock for Docker-backed gates
- must use scripts/agent-run for ordinary project tooling

Branch/worktree discovery:
- infer worktree, base commit, tool, model, and effort from git/pwd where possible
- if the current branch is main, stop
- if the worktree is dirty before starting, stop
- if tool/model/effort is not encoded in the branch, record UNKNOWN and ask me

Before every PR create/refresh (09 §13):
- run git fetch origin
- inspect git diff origin/main...HEAD
- merge origin/main into the story branch if main moved; re-chain migrations to a single alembic head
- rerun verify.sh --ci after the sync
- confirm the PR diff does not show main-only work as deletions
- PR title must follow the house convention [US-XXX-NN] Short description; verify against gh pr list

Known lessons to apply:
- D8 query audit: every tenant-scoped ORM query must explicitly filter by tenant_id
- replay/idempotency audit for any consume/confirm/credit/rotate/callback path touched
- generated verify.sh must use scripts/agent-run or writable cache env vars inside the script
- generated verify.sh must use scripts/with-integration-lock for Docker-backed gates
- live integration tests must not count as evidence if they skipped
- record compact timestamps (date + time + timezone when known), AI usage, known lessons applied,
  and new lesson candidates
- preserve historical gate records, but refresh current-state fields before phase completion

Before declaring the range complete (09 §15):
- refresh current status/completed/duration fields for every story in the range
- refresh the experiment run note and each story's draft PR to match the final branch state
- search for stale placeholders/current-state drift: `Completed: —`, `Duration: —`,
  `pending commit`, stale "awaiting approval" status lines, `NOT VERIFIED`, and risks code now fixes
- leave historical gate records intact unless factually wrong

Start with the DISCOVERY + BRANCH PLAN preflight, wait for my confirmation, then run the loop.
```

---

## 5. Expected Output

Per in-range story (`US-XXX-NN`):

```text
brain/build-and-dev/experiments/{EPIC}_LOOP/{run-id}.md      (run note — required at range end)
brain/build-and-dev/build-logs/{EPIC}/US-XXX-NN/STORY_LOG.md
brain/build-and-dev/build-logs/{EPIC}/US-XXX-NN/*.md
brain/build-and-dev/build-logs/{EPIC}/US-XXX-NN/verify.sh
```

Plus one draft PR per completed story (publication enabled). A story that stops before implementation
gets a `STORY_LOG.md` entry only, no PR. The run note is required — if it is missing at range end, the
range is not complete.

The agent must not edit canonical decisions or active specs unless it stops and asks for human
approval.

---

## 6. Human Review After The Range

1. Run Prompt 7 / 7A independent review against the branch(es).
2. Check the D8 tenant query audit and replay/idempotency audit.
3. Check story logs and micro-commit logs match actual commits.
4. Confirm `verify.sh --ci` passed after any `origin/main` sync.
5. For end-of-loop/stacked runs, confirm PR bases and merge order are correct.
6. Decide whether to merge, request changes, or stop.

---

## 7. Cleanup

After the range's PRs are merged and `main` is synced:

```bash
git worktree remove "../Aspire-worktrees/${RANGE_TAG}"
# Git auto-prunes when removed this way; no holding branch to delete
```

Keep the worktree only if you expect to continue work from it.
