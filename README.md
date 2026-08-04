# Rust/400

Rust/400 is an educational, AS/400-inspired emulator for Linux. It aims to teach selected AS/400 and OS/400 concepts alongside their closest Linux analogies while building toward a recognizably green-screen-style operator experience.

Rust/400 is not affiliated with IBM and is not an IBM i replacement or binary-compatible emulator. See the [Product Requirements Document](docs/PRD.md) for the product boundary and planned MVP.

## Current status

The long-term target is a 5250-style, AS/400-like full-screen experience with menus, a command line, and function-key guidance. The project is not there yet. The current implementation is the underlying command engine: it initializes an isolated permanent or disposable workspace, starts an interactive command loop, and uses shared command metadata to drive validation, command-specific help, and typed handler requests from one registry.

## Prerequisites

- A current 64-bit Linux environment
- [Rustup](https://rustup.rs/) or an equivalent installation containing stable Rust 1.93.1 or newer

The checked-in `rust-toolchain.toml` selects the stable channel with the `rustfmt` and `clippy` components. `Cargo.toml` records Rust 1.93.1 as the minimum supported Rust version. Cargo will use the stable toolchain automatically when Rustup manages the installation.

## Build and run

```sh
cargo build
cargo run -- --workspace /absolute/path/to/rust400-system
```

Expected startup output:

```text
Rust/400 interactive shell: initialization complete.
Workspace: /absolute/path/to/rust400-system
Type EXIT to end the session. Additional commands will arrive in later stories.
R400>
```

That current prompt is a foundation layer, not the final interface target. The product direction now explicitly aims toward a full-screen main-menu presentation above this command engine.

Example parser behavior today:

```text
R400> help cmd(crtlib)
Command: CRTLIB
Summary: Create an emulated library definition.
Parameters:
- LIB (required): Names the library to create.
- TEXT (optional): Supplies a descriptive text for the library.
Handler: CreateLibrary
R400> crtlib lib(mylib) text('Learning library')
Command 'CRTLIB' is mapped to handler CreateLibrary for library 'MYLIB' with text 'Learning library'.
R400> crtlib text('Missing required LIB')
Validation error: missing required parameter LIB
R400> crtlib lib(
Syntax error: parameter LIB is missing a value
```

Rust/400 requires an absolute path and rejects parent traversal and symbolic links in workspace paths. Initialization creates `system.meta` beneath the workspace. It does not treat the workspace path as an emulated object name.

For an automatically removed demonstration or test workspace:

```sh
cargo run -- --temporary-workspace
```

## Test

```sh
cargo test --all-targets --all-features
```

## Format

Check formatting without changing files:

```sh
cargo fmt --all -- --check
```

Apply standard Rust formatting:

```sh
cargo fmt --all
```

## Lint

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

## Complete quality gate

Run the same formatting, linting, and test sequence used by continuous integration:

```sh
./scripts/check.sh
```

The script stops at the first failed stage, names that stage, and returns its nonzero exit status. GitHub Actions runs it for changes pushed to `main` or a `story/**` branch and for pull requests.

## Development workflow

- Read [AGENTS.md](AGENTS.md) for repository guardrails.
- Select work from the [product backlog](docs/backlog/README.md).
- Implement one Ready user story on its own `story/...` branch.
- Record meaningful work and verification in the [build-log index](build-logs/index.md).
- Do not claim compatibility with historical OS/400 behavior unless the relevant behavior has been researched, decided, and documented.

## Project documentation

- [Product requirements](docs/PRD.md)
- [Epics and user stories](docs/backlog/README.md)
- [Operator interface epic](docs/backlog/epics/EPIC-008-operator-interface.md)
- [Development process](docs/DEVELOPMENT_PROCESS.md)
- [Architecture decisions](docs/adr/index.md)
- [Active Rust/400 prompt suite](prompts-400/README.md)
- [Build-log process](build-logs/README.md)
