# Rust/400 GitHub Conventions Prompt

## Purpose

Produce `docs/GITHUB_CONVENTIONS.md` and repository templates that connect Rust/400 requirements, epics, stories, branches, pull requests, tests, build logs, ADRs, and releases.

This prompt creates local repository files only. Do not create remote labels, milestones, issues, branches, commits, pull requests, or releases without explicit user authorization.

## Phase 1: Load context

Read completely:

1. `AGENTS.md`
2. `docs/PRD.md`
3. `docs/backlog/README.md`
4. `docs/TRACEABILITY_MATRIX.md`, if present
5. all current epic files
6. `build-logs/README.md` and `build-logs/TEMPLATE.md`
7. accepted ADRs related to source control or delivery
8. existing `.github/` files, if any

Inspect Git status, current branch, remotes, and existing conventions without mutating them. If a Git hosting provider has not been selected, write provider-neutral conventions and clearly mark GitHub-specific automation as proposed.

## Phase 2: Define the conventions

Create `docs/GITHUB_CONVENTIONS.md` covering:

### Work-item hierarchy

- one milestone per epic, when milestones are used;
- one issue per user story;
- tasks as checklists or linked issues only when independently assignable;
- bugs linked to violated requirements or acceptance criteria; and
- spikes with time bounds, question, evidence, and decision outcome.

### Naming

- Milestone: `EPIC-NNN: Outcome name`
- Story branch: `story/us-nnn-nn-short-description`
- Bug branch: `bug/issue-nnn-short-description`
- Chore branch: `chore/issue-nnn-short-description`
- PR title: `[US-NNN-NN] Short outcome`
- Release tag: propose a SemVer-compatible scheme without declaring a release cadence not approved in the PRD.

Define the main development branch only after inspecting the repository. Do not assume `main` if evidence differs.

### Commit messages

Use Conventional Commit subjects where practical:

```text
<type>(<domain>): <description> [US-NNN-NN] [FR-NNN]
```

Define a small Rust/400 domain list from current epics, such as `delivery`, `shell`, `learning`, `catalog`, `jobs`, `spool`, and `release`. Permit `docs`, `test`, `build`, and `ci` scopes only when their meaning is clear. Do not require a requirement tag when a documentation or maintenance commit has no honest requirement mapping.

### Labels

Define a minimal, non-overlapping label taxonomy:

- type: story, bug, task, spike, documentation;
- priority: must, should, could;
- status only if the issue tracker—not a project board—needs it;
- epic label only if milestones alone are insufficient;
- domain labels matching the stable domain list;
- risk: security, data-integrity, compatibility, or learning-accuracy when applicable.

Avoid hundreds of `req:FR-NNN` labels unless the owner explicitly prefers label-based traceability; links and issue fields are usually easier to maintain.

### Merge and history policy

Present squash, merge-commit, and rebase strategies with tradeoffs for educational traceability. Recommend one policy, but mark it proposed until owner approval. Preserve build-log truth if history is rewritten.

### Required evidence

Specify what a story issue and PR must link: story, epic, requirements, use cases, acceptance criteria, build log, ADRs, tests, manual evidence, risks, and documentation.

### Permissions and safety

- no direct story commits to the main branch;
- no force push to shared protected branches;
- no secrets in issues, Actions, logs, examples, or fixtures;
- least-privilege workflow permissions;
- pinned or reviewed third-party Actions;
- explicit approval before remote or release mutations by an agent; and
- branch protection/check requirements once a remote exists.

## Phase 3: Create repository templates

Create or merge carefully with existing files:

- `.github/ISSUE_TEMPLATE/user-story.yml`
- `.github/ISSUE_TEMPLATE/bug-report.yml`
- `.github/ISSUE_TEMPLATE/config.yml`
- `.github/PULL_REQUEST_TEMPLATE.md`
- `.github/CODEOWNERS` only if real ownership information is known; otherwise document it as pending and do not invent handles.

The user-story form should capture:

- story ID and epic;
- persona, capability, and benefit;
- requirements and use cases;
- acceptance criteria as checkboxes;
- dependencies and risks;
- Definition of Ready; and
- expected build-log path.

The pull-request template should capture:

- summary and scope;
- story/epic/requirement/use-case links;
- acceptance-criteria evidence;
- tests actually run with results;
- security, persistence, and learning-bridge impact;
- build log and ADRs;
- documentation and traceability updates;
- known limitations; and
- reviewer checklist.

Do not add GitHub Actions during this task. Propose them separately after the Rust toolchain and authoritative local commands exist.

## Phase 4: Verify and record

- Validate YAML syntax using an available safe local tool.
- Check template links and story/requirement formats.
- Ensure new conventions do not conflict with `AGENTS.md` or the backlog.
- Record unresolved decisions rather than selecting repository owners, remotes, protection rules, or secrets.
- Create and index a build-log entry with files, decisions, verification, and remaining setup.

Report created files, proposed decisions awaiting approval, checks run, and any action that must be completed through GitHub by the owner.

