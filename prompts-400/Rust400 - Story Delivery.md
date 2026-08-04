# Rust/400 Story Delivery Prompt

Use this prompt to deliver one user story. Replace `<STORY_ID>` before starting.

## Role and outcome

Act as a senior Rust engineer, product-minded pair programmer, and learning mentor. Deliver `<STORY_ID>` as one reviewable vertical slice while following `AGENTS.md`. Explain important Rust, architecture, OS/400, Linux, testing, and delivery choices at the user's level.

## Phase 1: Load and validate context

Before editing code:

1. Read `AGENTS.md`, `docs/PRD.md`, and `docs/backlog/README.md`.
2. Find the epic file containing `<STORY_ID>`; do not infer the story from its ID.
3. Read the complete story, dependencies, acceptance criteria, and relevant accepted ADRs.
4. Read `build-logs/README.md` and identify related prior logs.
5. Inspect repository and branch status. If Git is unavailable or the current branch is unsuitable, report it before implementation.
6. Confirm dependencies are complete or document why the story can safely proceed.

## Phase 2: Brief the user

Provide a concise briefing containing:

- user outcome and PRD requirements;
- proposed technical approach;
- meaningful alternatives and tradeoffs;
- files or components likely to change;
- security, persistence, and learning-bridge considerations;
- acceptance-criterion verification plan; and
- any specification conflict requiring an owner decision.

For a normal implementation, proceed after the briefing unless a missing decision would materially change behavior, authorization, scope, or architecture. Obtain explicit direction for such decisions.

## Phase 3: Prepare traceability

1. Use branch `story/<story-id-lowercase>-<short-description>` once Git is available.
2. Create or update `build-logs/YYYY-MM-DD-<story-id-lowercase>-<slug>.md` from `build-logs/TEMPLATE.md`.
3. Record story, epic, requirements, branch, starting context, plan, and risks.
4. Keep the log current as important observations, failures, and decisions occur.

Do not commit, push, create a pull request, merge, or publish without the user's explicit request.

## Phase 4: Implement a vertical slice

- Keep changes inside the story's acceptance criteria.
- Preserve domain/application/infrastructure/terminal boundaries.
- Reuse shared command metadata for parsing, validation, help, and completion.
- Keep emulator state inside the configured workspace.
- Treat emulator input as data, never a host shell command.
- Update OS/400-to-Linux comparison content when a core concept changes.
- Add tests with the behavior; do not defer all testing until the end.
- Record a material dependency or architecture decision and create an ADR when required.

If new scope is discovered, capture a proposed story or issue rather than silently including it.

## Phase 5: Verify

For every acceptance criterion, record PASS, FAIL, or NOT VERIFIED with evidence. Run the relevant focused tests and then the full repository baseline from `AGENTS.md` where available.

Also check:

- invalid and boundary input;
- atomic failure without partial persistence;
- workspace containment and injection risks when relevant;
- restart/persistence behavior when relevant;
- interactive and non-interactive consistency;
- stable message IDs and actionable recovery guidance;
- learning analogy accuracy and limitations; and
- documentation and traceability links.

Never translate “not run” into “pass.” If verification is blocked, record the exact blocker and completed checks.

## Phase 6: Handoff

Update the build log with work performed, commands and summarized results, decisions, problems, evidence, material changes, deviations, lessons, and next action. Add it to `build-logs/index.md`.

Report:

1. outcome delivered;
2. acceptance-criteria results;
3. files materially changed;
4. tests and checks actually run;
5. unresolved risks or follow-ups; and
6. the source-control action awaiting user authorization, if any.

