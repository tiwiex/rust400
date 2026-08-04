# Manual Prompt 6 Run Prompt

**Status:** Copy/paste run prompt v0.1
**Date:** 2026-07-01
**Purpose:** Start a normal human-gated Prompt 6 story run in a separate worktree or IDE window.

---

## 1. When To Use This

Use this for normal Core Track story implementation when a human will approve:

- the approach
- the micro-commit plan
- branch creation or validation
- each commit
- final PR readiness

Do not use this for autonomous experiment runs. Autonomous runs use
`brain/build-and-dev/prompts/Prompt 6A - Autonomous Build Loop.md`.

---

## 2. Create The Worktree

From the main checkout:

```bash
cd /home/oshodi/projects/Aspire

git fetch origin
git checkout main
git pull --ff-only origin main

mkdir -p ../Aspire-worktrees

git worktree add \
  -b feature/{STORY-ID}-{short-slug} \
  ../Aspire-worktrees/{epic-id-lower}-{story-id-lower}-manual \
  main
```

Example:

```bash
git worktree add \
  -b feature/US-GEO-DATA-01-postgis-inec-seed \
  ../Aspire-worktrees/epic-03-us-geo-data-01-manual \
  main
```

If the branch already exists but is not attached to a worktree, omit `-b`:

```bash
git worktree add \
  ../Aspire-worktrees/epic-03-us-geo-data-01-manual \
  feature/US-GEO-DATA-01-postgis-inec-seed
```

Open the worktree folder in a separate IDE/agent window.

---

## 3. Paste This Into The Agent

Replace `{STORY-ID}` and `{EPIC-ID}` before pasting.

```text
Use `brain/build-and-dev/prompts/Prompt 6 - Interactive Build Protocol.md`.

We are starting the manual Prompt 6 process for:

Story: {STORY-ID}
Epic: {EPIC-ID}

Follow Prompt 6 exactly:
- Load all required context before forming an opinion.
- Create/update the story log.
- Give me the Step 1 briefing.
- Stop at every approval gate.
- Do not use the autonomous loop prompt.
- Do not write code until I approve the approach, micro-commit plan, and branch step.
- Preserve vendor-neutral/local-first behavior; do not call live external services unless the story explicitly requires it and I approve.

If the current branch already matches the story branch, validate it and record that the branch gate is satisfied; otherwise propose the correct branch name and wait.
```

---

## 4. EPIC-03 Starter

For `US-GEO-DATA-01`, paste:

```text
Use `brain/build-and-dev/prompts/Prompt 6 - Interactive Build Protocol.md`.

We are starting the manual Prompt 6 process for:

Story: US-GEO-DATA-01
Epic: EPIC-03

Follow Prompt 6 exactly:
- Load all required context before forming an opinion.
- Create/update the story log.
- Give me the Step 1 briefing.
- Stop at every approval gate.
- Do not use the autonomous loop prompt.
- Do not write code until I approve the approach, micro-commit plan, and branch step.
- Preserve vendor-neutral/local-first behavior; do not call live external services unless the story explicitly requires it and I approve.

If the current branch already matches the story branch, validate it and record that the branch gate is satisfied; otherwise propose the correct branch name and wait.
```

---

## 5. Cleanup

After the PR is merged and `main` is synced, remove the worktree if no further work will continue there:

```bash
git worktree remove ../Aspire-worktrees/{epic-id-lower}-{story-id-lower}-manual
git worktree prune
```

Keep the branch only when you expect to continue work from it.
