# Rust/400

Rust/400 is an educational, OS/400-inspired shell for Linux. It aims to teach selected OS/400 concepts alongside their closest Linux analogies while explaining where those comparisons break down.

Rust/400 is not affiliated with IBM and is not an IBM i replacement or binary-compatible emulator. See the [Product Requirements Document](docs/PRD.md) for the product boundary and planned MVP.

## Current status

The project initializes an isolated permanent or disposable workspace and then prints a deliberate placeholder message. Command parsing and emulated system behavior belong to later user stories.

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
Rust/400 development shell: initialization complete; no commands are available yet.
Workspace: /absolute/path/to/rust400-system
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
- [Development process](docs/DEVELOPMENT_PROCESS.md)
- [Architecture decisions](docs/adr/index.md)
- [Active Rust/400 prompt suite](prompts-400/README.md)
- [Build-log process](build-logs/README.md)
