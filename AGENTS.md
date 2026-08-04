# Rust/400 Repository Guidance

## Mission

Rust/400 is an educational, OS/400-inspired shell for Linux. It teaches selected OS/400 concepts alongside Linux analogies while remaining safe, original, and explicit about differences.

It is not an IBM i replacement, binary-compatible emulator, host-administration shell, or reproduction of proprietary IBM code, documentation, screens, or assets.

## Authoritative project documents

Read the smallest relevant set before changing the repository:

1. `docs/PRD.md` for product scope, numbered requirements, domain rules, and acceptance criteria.
2. `docs/backlog/README.md` for delivery conventions.
3. The story's file under `docs/backlog/epics/` for its outcome, dependencies, and acceptance criteria.
4. Relevant accepted ADRs under `docs/adr/` once that directory exists.
5. `build-logs/README.md` and `build-logs/TEMPLATE.md` for evidence requirements.

If these sources conflict, stop implementation and surface the conflict. Do not silently redefine a requirement in code.

## Story workflow

For implementation tied to a user story:

1. Identify the exact `US-NNN-NN`, epic, requirements, dependencies, and acceptance criteria.
2. Confirm dependencies are satisfied and the story is Ready according to the PRD.
3. Briefly state the intended outcome, proposed approach, alternatives, risks, and verification plan before editing code.
4. Once Git is available, work on `story/us-nnn-nn-short-description`; do not implement story work directly on the main branch.
5. Keep the change within one story. Record discovered scope separately rather than expanding the story invisibly.
6. Add or update a build-log entry during the work. Record failed approaches and deviations honestly.
7. Verify each acceptance criterion with automated evidence where practical and explicit manual evidence otherwise.
8. Update documentation, learning content, and traceability affected by the behavior.
9. Do not commit, push, open a pull request, merge, or publish a release unless the user explicitly requests that external or source-control action.

If the workspace is not recognized as a Git repository, documentation-only work may continue, but record the limitation in its build log and do not pretend branch traceability exists.

## Product guardrails

- Preserve the “inspired, not compatible” product boundary.
- Keep mutable emulator state beneath one explicit workspace.
- Never turn emulator input into arbitrary host shell execution.
- Treat host paths and emulated object names as different types and trust domains.
- Validate path traversal, absolute-path injection, symlink escape, unsafe overwrite, and command injection boundaries.
- Do not require root privileges.
- Do not expose secrets in logs, history, tests, fixtures, or examples.
- Do not add Rust `unsafe` code without an accepted ADR and targeted safety evidence.
- Do not use IBM proprietary source, copied screen layouts, documentation text, or visual assets.

## OS/400-to-Linux learning rules

Every new core OS/400-inspired concept or command must update the learning bridge when applicable. Include:

- the OS/400-inspired definition;
- the closest Linux analogy;
- the shared mental model;
- where the analogy breaks;
- distinct Rust/400 and Linux examples; and
- a label for emulated, historically inspired, or intentionally different behavior.

Linux examples are educational text. Do not execute them automatically. Never describe a library as simply a directory, an object as simply a file, or a job as simply a process without explaining the lost semantics.

## Architecture and Rust rules

- Keep dependencies directed from terminal/CLI adapters to application use cases, domain policies, and infrastructure interfaces as described in the PRD.
- Domain logic must not depend on terminal rendering or a concrete persistence library.
- Interactive and non-interactive modes must share parser, command definitions, validation, and domain behavior.
- Prefer one clear binary for the MVP until evidence justifies multiple crates.
- Use stable Rust and standard Cargo conventions.
- Prefer small, explicit types and functions over premature frameworks or abstraction.
- Model identifiers, qualified names, object types, host paths, and messages with distinct types where this prevents invalid states.
- Command mutation must be atomic: complete the state transition or leave persistent state unchanged.
- Return structured errors with stable message IDs for expected user or domain failures; do not panic for ordinary invalid input.
- Add dependencies only when their benefit exceeds their maintenance, security, and learning cost. Record material dependency choices.

## Verification baseline

Use the repository's documented commands once the Rust project exists. Until replaced by a checked-in task runner, the expected baseline is:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Run the narrowest relevant checks during iteration and the full baseline before calling a story verified. Add security-boundary and disposable-workspace integration tests when the story affects host interaction or persistence.

Never claim a check passed unless it was run successfully in the current workspace. Distinguish “not run,” “blocked,” and “failed.”

## Documentation and evidence

- Preserve stable `FR-*`, `NFR-*`, `EPIC-*`, and `US-*` identifiers.
- Link a changed requirement to affected epics and stories.
- Use ADRs for durable architectural decisions; do not use a build log as their replacement.
- Use build logs for chronological work, commands, observations, evidence, problems, and lessons.
- Do not rewrite historical logs to make a process look cleaner. Append a correction.
- Prefer repository-relative links inside repository Markdown.

## Review priorities

Review in this order:

1. User-visible correctness and acceptance criteria.
2. Workspace containment, command injection, data integrity, and secret handling.
3. Product-scope and learning-analogy accuracy.
4. Architectural boundaries and maintainability.
5. Test quality and traceability evidence.
6. Performance against the PRD's stated scale assumptions.

Report substantive findings before stylistic suggestions. Include file and line evidence where possible.
