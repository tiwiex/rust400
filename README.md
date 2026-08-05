# Rust/400

Rust/400 is an educational, AS/400-inspired emulator for Linux. It aims to teach selected AS/400 and OS/400 concepts alongside their closest Linux analogies while building toward a recognizably green-screen-style operator experience.

Rust/400 is not affiliated with IBM and is not an IBM i replacement or binary-compatible emulator. See the [Product Requirements Document](docs/PRD.md) for the product boundary and planned MVP.

## Current status

The long-term target is a 5250-style, AS/400-like full-screen experience with menus, a command line, and function-key guidance. The project is not there yet. The current implementation is the underlying command engine: it initializes an isolated permanent or disposable workspace, starts an interactive command loop, and uses shared command metadata to drive validation, command-specific help, and typed handler requests from one registry.

The latest UI story now renders a text-mode main menu layout with:

- a title area
- system context
- numbered main-menu options
- a `Selection or command` input line
- a footer legend with function-key hints

The menu content is now driven by shared menu metadata rather than being hard-coded directly inside the renderer. That means labels, options, and footer hints can be validated and reused by both rendering and future navigation logic.
The main screen input field now accepts both:

- numbered menu selections such as `1` or `90`
- direct commands such as `HELP` or `CRTLIB LIB(MYLIB)`

The current function-key MVP treats typed entries such as `F3` or `F12` as function-key actions from the same input line.

The first persistent library-management slice is now live:

- `CRTLIB LIB(MYLIB) TEXT('Learning library')` creates an emulated library in the workspace catalog
- `DSPLIB LIB(MYLIB)` displays the saved library details
- repeating the same library creation reports a duplicate-library message
- the catalog is stored inside the workspace and reopens on the next session

See [docs/concepts/libraries-and-objects.md](docs/concepts/libraries-and-objects.md) for the AS/400-to-Linux learning bridge behind this model.

Command names follow the OS/400-style single-token form. For example, use `SNDMSG`, not `SND MSG`.

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
MAIN                            IBM i Main Menu              System: RUST400

  Select one of the following:

     1. User tasks
     2. Office tasks
     3. General system tasks
     4. Files, libraries, and folders
     5. Programming
     6. Communications
     7. Define or change the system
     8. Problem handling
     9. Display a menu
     10. Information Assistant options
     11. IBM i Access tasks

     90. Sign off

  Selection or command
  ===>
  ----------------------------------------------------------------------------
  F3=Exit   F4=Prompt   F9=Retrieve   F12=Cancel   F13=Information Assistant
  F23=Set initial menu

  User: MW           Job: QPADEV0001

  Workspace: /absolute/path/to/rust400-system
  Enter EXIT in the command line to end the session.
  ===>
```

Example mixed input behavior today:

```text
  ===> 1
Menu selection 1 -> User tasks opens menu 'USR'.
... User Tasks menu renders ...
  ===> 2
Menu selection 2 -> Send a message. Type SNDMSG MSG('Hello') TO(QSYSOPR). Rust/400 uses single-token command names, not 'SND MSG'.
  ===> crtlib lib(mylib) text('Learning library')
CRTLIB created library MYLIB with text 'Learning library'.
  ===> dsplib lib(mylib)
Library: MYLIB
Text: Learning library
Created: 1722816000
  ===> help cmd(crtlib)
Command: CRTLIB
Summary: Create an emulated library definition.
...
  ===> 77
Selection '77' is not valid on menu MAIN.
  ===> 90
Menu selection 90 -> Sign off
Session ended.
```

That current screen is the first presentation slice, not the final interface target. The product direction now explicitly aims toward richer full-screen menu navigation above this command engine.

Current function-key behavior:

- `F3`: exit the session
- `F4`: prompt with input guidance
- `F9`: retrieve the most recent direct input
- `F12`: cancel and remain on the main menu
- `F13`: show Information Assistant guidance
- `F23`: report that setting the initial menu is not implemented yet

Unsupported function keys fail gracefully with a menu-specific message.

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
