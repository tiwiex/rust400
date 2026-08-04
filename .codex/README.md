# Project Codex Configuration

Rust/400's durable agent guidance lives in [`../AGENTS.md`](../AGENTS.md). This `.codex/` directory is reserved for Codex runtime configuration that truly needs to be shared by the repository.

## Why there is no `config.toml` yet

A project `.codex/config.toml` can carry trusted-repository settings such as sandbox policy, approval behavior, hooks, MCP servers, model defaults, or reasoning defaults. None is currently required to implement the documentation backlog, and personal or managed safety settings should not be weakened by a repository default.

Add `config.toml` only when the project has a concrete, reviewed need, for example:

- a repository-scoped documentation or issue-tracker MCP service;
- a non-destructive hook that checks story/build-log traceability;
- a shared setting required by the chosen development environment; or
- a verified sandbox requirement that remains least-privilege.

When adding configuration:

1. verify keys against current official Codex documentation;
2. explain the need in an ADR or build log;
3. avoid embedding credentials or machine-specific absolute paths;
4. preserve workspace-only write access and approval for consequential actions;
5. test behavior in a trusted checkout; and
6. document how contributors can opt out or override personal defaults.

Potential future enforcement belongs in a hook or CI check only when it can be checked mechanically. Product intent, architecture, and review expectations remain in `AGENTS.md`.

