# Rust/400 Product Requirements Document

| Field | Value |
|---|---|
| Product | Rust/400 |
| Document status | Draft for review |
| Version | 0.3 |
| Date | 2026-08-04 |
| Product type | Educational, AS/400-inspired emulator for Linux |
| Initial interface | 5250-style terminal user interface over a command engine |

## 1. Executive summary

Rust/400 is an open-source Rust application that recreates selected AS/400 and OS/400 concepts and workflows in a Linux terminal. It is intended for enthusiasts, learners, and developers who want to explore a menu-driven operator experience, command entry, libraries, objects, jobs, messages, and spooled output without needing IBM hardware or a licensed IBM i environment.

The product will provide a safe, deterministic emulation layer over a private workspace on Linux. It will not attempt to run IBM i binaries, reproduce every OS/400 subsystem, or act as a drop-in replacement for IBM i. The first release will emphasize a recognizable green-screen-style interactive experience and a maintainable architecture over breadth.

The project also serves as a practical example of disciplined product delivery. Requirements will be traceable from this PRD to epics, user stories, acceptance criteria, branches, commits, tests, and releases.

## 2. Problem statement

Access to legacy AS/400 and OS/400 environments is limited, and learning materials often assume access to an existing system. General Linux shells do not expose the object-oriented organization, command conventions, job model, menu flow, or operator experience familiar to AS/400 users.

An enthusiast needs an accessible environment in which to recall or learn those concepts. A software-development learner also needs a realistic project through which to practise business analysis, decomposition, source control, testing, and release management.

## 3. Product vision

Provide the most approachable AS/400-inspired learning environment on Linux: nostalgic enough to feel familiar, constrained enough to understand, and engineered well enough to demonstrate professional Rust and product-development practices.

## 4. Goals and success measures

### 4.1 Product goals

1. Deliver a 5250-style interactive experience with a consistent AS/400-inspired command and menu model.
2. Model a useful subset of libraries, objects, jobs, messages, users, and spooled output.
3. Keep emulated state isolated from the host system by default.
4. Make behavior discoverable through menus, built-in help, prompting, and actionable error messages.
5. Build the code as modular Rust components with automated tests and documented architectural decisions.
6. Maintain requirement-to-release traceability as an educational example.
7. Teach each emulated OS/400 concept alongside its closest Linux counterpart, including the limits of the comparison.

### 4.2 Initial success measures

| Measure | Target for v0.1 MVP |
|---|---|
| Supported commands | At least 12 commands across the core workflows |
| Menu coverage | One main menu plus at least 3 linked task menus or task screens |
| Critical end-to-end workflows | 100% passing in automated tests |
| Command parser tests | At least 90% branch coverage for documented syntax |
| Unhandled crash rate | Zero in the documented happy paths and common invalid-input cases |
| Startup time | Under 1 second on a typical supported development machine |
| Interactive response time | Under 100 ms for local metadata operations at the 95th percentile |
| Documentation traceability | Every delivered story references a PRD requirement and test evidence |

These targets may be revised after a baseline implementation provides reliable measurements.

## 5. Users and personas

### 5.1 Returning operator

A former AS/400 or OS/400 user who wants a familiar menu-driven and command-driven experience for nostalgia, demonstrations, or experimentation.

### 5.2 Systems learner

A developer or student who wants to understand OS/400 concepts without access to an IBM i installation.

### 5.3 Rust contributor

A developer who wants to learn Rust and professional delivery practices by implementing well-scoped stories in a modular codebase.

### 5.4 Business-analysis learner

A learner practising how product goals become epics, user stories, acceptance criteria, tests, and release evidence.

## 6. Product principles

- **Inspired, not compatible:** Preserve recognizable concepts and interaction patterns while documenting intentional differences.
- **Safe by default:** Emulated actions remain inside an explicitly configured workspace unless a future capability is separately authorized.
- **Deterministic:** The same state and command should yield the same result.
- **Discoverable:** Users should be able to learn commands and menus without leaving the emulator.
- **Layered interaction:** The command engine, screen model, and terminal rendering should stay separable so the project can support both full-screen menus and command-driven workflows.
- **Modular:** Domain logic must not depend directly on terminal rendering or storage details.
- **Incremental:** Each story should add a small, testable vertical slice.
- **Traceable:** Requirements, decisions, implementation, and verification should remain linked.

## 7. Scope

### 7.1 MVP scope (v0.1)

The MVP will include:

- A 5250-style full-screen terminal experience with a main menu, command line, and function-key legend.
- A command engine that supports interactive command entry from the screen and a non-interactive single-command mode.
- An OS/400-inspired command parser supporting command names, positional input where documented, and keyword parameters such as `LIB(MYLIB)`.
- Case-insensitive command and unquoted identifier handling, with normalized stored names.
- A persistent emulated system stored beneath one configurable Linux workspace directory.
- A screen model that supports menu titles, selectable numbered options, status areas, footer legends, and a "selection or command" input field.
- A minimal function-key model for commonly expected actions such as exit, cancel, prompt, refresh, and help.
- A library list and a basic object catalog.
- Core operations to create, inspect, list, and remove emulated libraries and selected object types.
- A simple job identity for each shell session and the ability to display current job information.
- Informational, warning, and error messages with stable message identifiers.
- A minimal output queue containing viewable spooled text output.
- Built-in command help and command completion where practical.
- A batch/script mode that reads commands from a UTF-8 text file.
- Configuration, structured diagnostics, and an auditable command history.

The proposed starter command set is:

| Area | Candidate commands | Purpose |
|---|---|---|
| Discovery | `HELP`, `WRKCMD`, menu navigation | Learn and list available commands and screens |
| Session | `SIGNON`, `SIGNOFF`, `DSPJOB`, main menu | Establish and inspect a session/job |
| Libraries | `CRTLIB`, `DLTLIB`, `DSPLIB`, `WRKLIB`, `ADDLIBLE`, `RMVLIBLE` | Manage libraries and the library list |
| Objects | `DSPOBJD`, `WRKOBJ`, `DLTOBJ` | Inspect and manage catalogued objects |
| Messages | `SNDMSG`, `DSPMSG` | Send and inspect local messages |
| Spooling | `WRKSPLF`, `DSPSPLF` | List and view generated text output |
| Utility | `DSPSYSVAL`, `HISTORY`, `EXIT` | Inspect the emulator and control the shell |

The exact MVP command list will be finalized during epic decomposition. A command or menu path is considered supported only when its syntax or screen behavior, errors, help, and tests are documented.

### 7.2 Later-release candidates

- Additional CL-like commands and richer prompting.
- Richer screen-oriented forms inspired by 5250 workflows.
- More complete subsystem, job queue, and output queue simulation.
- Data areas, data queues, user spaces, and additional object types.
- A record-oriented local database abstraction.
- A constrained CL-like scripting language.
- Remote terminal access and multiple concurrent sessions.
- Import/export tools and plug-in commands.

### 7.3 Out of scope for the MVP

- Binary, source, or protocol compatibility with IBM i.
- Execution or translation of IBM i programs, RPG, COBOL, or licensed CL source.
- A complete DB2 for i implementation.
- Full 5250 network protocol emulation or binary compatibility with IBM terminal sessions.
- Production workload hosting, high availability, or security certification.
- Direct mutation of arbitrary host files, users, processes, or system configuration.
- Pixel-perfect reproduction of proprietary interfaces or distribution of IBM assets.

## 8. Key user journeys

### 8.1 Start and explore

1. The user starts Rust/400.
2. The application opens or initializes the configured emulated system.
3. The user signs on with a local emulator profile or enters a development session.
4. The emulator displays a main menu with a command line, system context, and function-key legend.
5. The user selects a menu option or enters `HELP` or `WRKCMD` and receives usable guidance.

### 8.2 Manage libraries and objects

1. The user creates a library.
2. The user adds it to the session library list.
3. The user creates or encounters an object in that library.
4. The user lists and displays its description using a qualified or library-list-resolved name.
5. State remains available after the application restarts.

### 8.3 Diagnose an invalid command

1. The user enters an unknown command or invalid parameter from the command line or a promptable screen.
2. The emulator rejects it without changing state.
3. The emulator displays a stable message ID, a plain-language explanation, and corrective guidance.
4. The user can request relevant help and retry.

### 8.4 Produce and inspect output

1. A supported command produces report output.
2. The output is stored as an emulated spooled file associated with the current job.
3. The user lists spooled files and views the selected text content.

### 8.5 Run a repeatable batch

1. The user supplies a command script to non-interactive mode.
2. Commands execute in order against an isolated test or user workspace.
3. The process emits human-readable output and a meaningful exit status.
4. On failure, the report identifies the command and stable message ID.

### 8.6 Navigate by menu

1. The user reaches the main menu after startup or sign-on.
2. The screen shows numbered options, a system label, a command line, and function-key hints.
3. The user chooses a numbered option or enters a command directly.
4. The emulator opens the corresponding menu or task screen and preserves a clear way back.
5. The user can return, cancel, or request help without losing the session.

### 8.7 Learn through comparison

1. The user encounters an OS/400-inspired command or concept.
2. The user requests its learning explanation from help or concept documentation.
3. Rust/400 explains the concept in OS/400 terms, identifies the closest Linux analogue, and provides a small example of each.
4. The explanation explicitly identifies important differences so the analogy does not become a false equivalence.
5. Where safe and useful, the user can view—not automatically execute—a comparable Linux command.

## 9. OS/400-to-Linux learning bridge

Linux and OS/400 organize and operate systems differently. The mappings below are teaching analogies, not claims of one-to-one compatibility. Rust/400 should use them to connect new concepts to knowledge a Linux user may already have.

| OS/400-inspired concept | Closest Linux concept | Useful comparison | Important difference |
|---|---|---|---|
| Library | Namespace, package/catalog, and sometimes a directory | Both organize named resources and help resolve names | An OS/400 library is a catalog for typed system objects; a Linux directory primarily maps names to filesystem entries |
| Object | Typed, managed resource; sometimes a file or device node | Both have a name, identity, ownership, metadata, and authorities | OS/400 treats many system resources uniformly as typed objects; Linux exposes several different models such as files, processes, sockets, and services |
| Library list | Shell `PATH` and search order | Both resolve an unqualified name by searching an ordered list | `PATH` mainly locates executable files, while a library list resolves many object types in libraries |
| Qualified name such as `LIB/OBJ` | Absolute or explicitly scoped path | Qualification avoids relying on a search order | An OS/400 qualification names a library and object, not a host filesystem path |
| Current library | Current working context or default namespace | Both provide a default location/context for some unqualified operations | It is not equivalent to Linux's current working directory and does not expose host-directory traversal |
| Object type | File type, resource kind, or service type | Type controls valid operations and interpretation | Linux file extensions are often conventional, whereas an OS/400 object type is managed system metadata |
| User profile | Linux user account | Both identify a user and participate in authorization | Profile attributes, special authorities, ownership rules, and authentication models differ |
| Group profile / authority list | Linux groups, mode bits, POSIX ACLs, and capabilities | Both grant shared access without assigning permissions separately to every user | Permission evaluation and privileged-authority semantics are not identical |
| Job | Process plus session and execution metadata | Both represent running work with identity, status, owner, and resources | An OS/400 job contains richer platform-specific routing, queue, accounting, and message behavior than a single Linux process |
| Subsystem | `systemd` service/slice, process supervisor, or workload partition | Both provide an environment in which work is admitted and managed | A subsystem is not merely a daemon and its job-routing behavior has no single Linux equivalent |
| Job queue | Batch queue, `at`, cron staging, or a scheduler queue | Work can wait and be selected for execution | Linux tools vary, and a cron schedule is not itself a persistent OS/400-style job queue |
| Output queue and spooled file | Print spool such as CUPS, queued report, or captured output file | Output is retained and selected for later viewing or printing | Rust/400 spooled files are typed emulator records, not ordinary redirected stdout files |
| Message queue | Mailbox, journal/event stream, notification queue, or IPC queue | Producers send information that a recipient can inspect later | OS/400 operational messages combine user, job, program, and inquiry workflows that no single Linux facility reproduces |
| System value | `sysctl`, environment-independent configuration, or `/etc` setting | Both expose system-wide behavioral settings | Linux settings are distributed across mechanisms; OS/400 system values form a more uniform named interface |
| Data area | Small state/configuration file or key-value entry | Both persist small values shared by programs | A data area is a typed named object with platform-defined operations and authorities |
| Data queue | FIFO, POSIX message queue, or broker queue | Programs exchange queued messages | Delivery, persistence, keying, and waiting semantics vary substantially |
| Device description | `/dev` entry plus `udev` and system configuration | Both describe devices used by applications or operators | A Linux device node alone does not contain the configuration and operational model of an OS/400 device description |
| Integrated File System (IFS) | Linux filesystem hierarchy | Both provide hierarchical path-based files and directories | The IFS coexists with the library/object model; in Rust/400 it must remain isolated from arbitrary host paths |
| Command Language command | Shell command plus structured CLI options | Both invoke operations from a textual command line | CL keyword parameters and prompting are schema-driven and are not equivalent to conventional `--option` parsing |
| Command prompting/help | `man`, `--help`, and shell completion | Both teach syntax and available parameters | OS/400 prompting can be interactive and parameter-aware in a more structured way |

### 9.1 Teaching presentation requirements

Each core concept page should contain:

- a plain-language OS/400-inspired definition;
- the closest Linux analogy and an explicit “where the analogy breaks” note;
- one Rust/400 example and one comparable Linux example;
- links to related Rust/400 commands and concepts;
- a label distinguishing emulated behavior from historical behavior; and
- terminology suitable for both returning operators and Linux-first learners.

Example learning card:

```text
Concept: Library list
Rust/400: DSPLIBL
Linux analogy: printf '%s' "$PATH" | tr ':' '\n'
Shared idea: Search locations are checked in order.
Key difference: PATH searches for executable files; a library list resolves
typed objects across libraries.
Try next: CRTLIB, ADDLIBLE, DSPOBJD
```

Comparable Linux commands must be displayed as educational examples. Rust/400 shall not silently execute them or use them to mutate the host system.

## 10. Functional requirements

Priority uses MoSCoW: Must, Should, Could, Won't for the MVP.

| ID | Requirement | Priority |
|---|---|---|
| FR-001 | The system shall provide an interactive terminal experience with a command line, menu navigation, and a "selection or command" interaction model. | Must |
| FR-002 | The system shall execute one command or a UTF-8 command script non-interactively. | Must |
| FR-003 | The parser shall recognize case-insensitive command names and keyword parameters in the documented CL-like syntax. | Must |
| FR-004 | The parser shall validate required, optional, repeated, mutually exclusive, and invalid parameters according to a command definition. | Must |
| FR-005 | The system shall expose command definitions to both execution and built-in help so syntax does not drift between them. | Must |
| FR-005A | The system shall expose menu and screen definitions to rendering and navigation logic so options, labels, and navigation behavior do not drift. | Must |
| FR-006 | The system shall persist emulator state beneath a configurable workspace and reopen it on the next run. | Must |
| FR-007 | The system shall create, list, display, and delete emulated libraries, subject to validation and dependency rules. | Must |
| FR-008 | The system shall maintain an ordered library list for each active session. | Must |
| FR-009 | The system shall resolve unqualified object names through the current library list and accept qualified names where supported. | Must |
| FR-010 | The object catalog shall store a stable identity, normalized name, library, type, owner, creation time, modification time, and text description. | Must |
| FR-011 | The system shall list, display, and delete supported catalogued objects. | Must |
| FR-012 | Each shell session shall have a job identity and expose its basic status and timestamps. | Must |
| FR-013 | The system shall emit messages with stable IDs, severity, text, cause, and recovery guidance where applicable. | Must |
| FR-014 | Failed validation shall not partially mutate persistent state. | Must |
| FR-015 | The system shall create, list, and display text-based spooled files for designated commands. | Must |
| FR-016 | The system shall record command history with timestamp, session/job identity, normalized command, and outcome; secrets shall be redacted. | Must |
| FR-017 | The shell shall provide built-in summary and command-specific help. | Must |
| FR-017A | The emulator shall provide a main menu and at least one subordinate task menu or task screen in the MVP. | Must |
| FR-018 | The process shall return exit code `0` for successful non-interactive execution and a documented nonzero code for failure. | Must |
| FR-019 | The system should offer interactive completion for commands and known keyword names. | Should |
| FR-019A | The emulator should support a documented subset of function-key actions such as Exit, Cancel, Prompt, Refresh, and Help. | Should |
| FR-020 | The system should support local emulator user profiles and a sign-on flow without claiming host authentication. | Should |
| FR-021 | The system should provide a dry-run or isolated temporary workspace suitable for demonstrations and tests. | Should |
| FR-022 | The system could export selected catalog, message, or spooled-output data in a documented interchange format. | Could |
| FR-023 | The MVP will not invoke arbitrary host shell commands from the emulated prompt. | Won't |
| FR-024 | Built-in help for every core concept shall provide its closest Linux analogy and explain at least one material difference. | Must |
| FR-025 | The system shall provide a browsable concept glossary linking OS/400-inspired terms, Linux analogues, related commands, and examples. | Must |
| FR-026 | Linux examples shown by the learning bridge shall be clearly separated from Rust/400 commands and shall not execute automatically. | Must |
| FR-027 | The learning bridge should allow a user to navigate in both directions: from an OS/400-inspired concept to Linux and from a Linux concept to relevant Rust/400 concepts. | Should |
| FR-028 | Help output should label behavior as emulated, historically inspired, or intentionally different where confusion is likely. | Should |

## 11. Domain rules

1. Emulated identifiers use a documented canonical form. The MVP target is uppercase ASCII and a maximum length selected during architecture design; deviations from historical OS/400 rules must be documented.
2. Every object belongs to exactly one library.
3. Object identity is distinct from its display name so storage internals do not leak into commands.
4. A qualified object name takes precedence over library-list resolution.
5. When an unqualified name matches multiple libraries, the first match in the ordered library list wins.
6. Destructive operations must validate protected objects, dependencies, and authority before committing a change.
7. A command either completes its intended state transition or leaves persistent state unchanged.
8. Times are stored in UTC and rendered using configured local-time preferences.
9. Host paths are never treated as emulated object names and must not be accepted where an object identifier is required.
10. Menu selections and command entry must coexist without ambiguity; when both are present, the documented screen behavior decides which input takes precedence.

## 12. Non-functional requirements

### 12.1 Safety and security

- **NFR-SEC-001:** By default, all mutable emulator data shall remain inside one canonicalized workspace directory.
- **NFR-SEC-002:** User-controlled paths shall be validated against traversal, symlink escape, and unsafe overwrite.
- **NFR-SEC-003:** The application shall not require root privileges.
- **NFR-SEC-004:** Secrets, if introduced, shall not be written to history or ordinary diagnostic logs.
- **NFR-SEC-005:** Rust `unsafe` code shall be prohibited unless justified by an approved architecture decision record and targeted tests.

### 12.2 Reliability and data integrity

- Persistent changes shall be atomic at the command boundary.
- The application shall detect incompatible or corrupt state and fail with recovery guidance rather than silently discarding data.
- Persisted formats shall carry a schema version and have a documented migration policy before the first stable release.
- Critical workflows shall be covered by integration tests using disposable workspaces.

### 12.3 Performance and scale assumptions

- The MVP is a single-machine application optimized for one interactive user.
- The reference test dataset is 100 libraries, 10,000 catalogued objects, and 10,000 history entries.
- Listing commands shall support bounded or paginated output so large results do not exhaust memory or overwhelm the terminal.

### 12.4 Portability

- The initial supported platform is current 64-bit Linux on a terminal with UTF-8 support.
- Platform-dependent behavior shall be isolated behind interfaces to permit future macOS or Windows support.

### 12.5 Accessibility and usability

- Core workflows shall remain usable without color.
- Output shall respect non-interactive terminals and provide a `NO_COLOR`-compatible mode.
- Error messages shall include a stable identifier and corrective action when known.
- Tables shall have a plain-text rendering suitable for copying and automated tests.
- The screen model shall have a text-first representation that can be snapshot-tested without requiring a GUI.

### 12.6 Maintainability and observability

- Domain, application, storage, and terminal concerns shall have explicit boundaries.
- Commands shall use structured request/result types rather than embedding business logic in rendering code.
- Logs shall be structured, configurable by level, and separate from normal command output.
- Public behavior shall be tested; implementation details should not become contractual accidentally.
- Significant architectural choices shall be recorded as Architecture Decision Records (ADRs).

## 13. Conceptual architecture constraints

This PRD does not prescribe a final module layout, but implementation should preserve the following dependency direction:

```text
Terminal / CLI adapters
        |
Application commands and use cases
        |
Domain model and policies
        |
Storage, clock, identity, and host-service interfaces
```

Infrastructure implementations may depend on domain-defined interfaces; domain logic must not depend on the terminal framework or concrete persistence library. The parser and renderer should be reusable by interactive and non-interactive entry points. A single binary is preferred for the MVP unless evidence justifies a workspace of multiple crates.

## 14. MVP acceptance criteria

The MVP is acceptable when all of the following are demonstrably true:

1. On a clean Linux account, a user can build or install Rust/400 using documented steps and initialize an isolated emulated system.
2. The user can discover supported commands and view accurate command-specific help.
3. The user can create two libraries, alter the ordered library list, and verify unqualified object resolution follows that order.
4. The user can inspect object metadata and restart the program without losing committed state.
5. An invalid command, invalid parameter, and unresolved object each return an appropriate stable message ID without an unhandled panic or partial state change.
6. A report-producing command creates a spooled file that can be listed and displayed.
7. A command script can execute the same core workflow in a disposable workspace and returns documented exit codes.
8. Automated unit and integration tests pass in continuous integration on the supported Rust toolchain.
9. The repository contains user documentation, contributor guidance, ADRs for major decisions, and traceability from each MVP story to one or more requirements in this document.
10. A security test demonstrates that crafted object names or script input cannot write outside the configured workspace.
11. From built-in help, a user can compare every MVP domain concept with its closest Linux analogue and see where the analogy is incomplete.
12. A Linux-first learner can start from at least `PATH`, process, user, permissions, and print-spool concepts and navigate to the corresponding Rust/400 learning material.

## 15. Delivery and requirements traceability

The recommended artifact chain is:

```text
Product goal -> PRD requirement -> Epic -> User story -> Acceptance criteria
             -> Branch/commits -> Pull request -> Automated/manual evidence -> Release
```

### 15.1 Work-item conventions

- Epics group outcomes, not code layers.
- Each user story uses the form: “As a [persona], I want [capability], so that [benefit].”
- Every story references at least one `FR-*` or `NFR-*` identifier.
- Acceptance criteria use observable Given/When/Then examples where useful.
- Technical tasks may support a story but do not replace its user outcome.
- Bugs reference the violated requirement or acceptance criterion when one exists.

### 15.2 Source-control conventions

- Use short-lived branches from the main development line.
- Suggested branch names: `story/<id>-<short-description>`, `bug/<id>-<short-description>`, and `chore/<id>-<short-description>`.
- Pull requests should be small enough to review, link the work item, identify requirements addressed, include test evidence, and update relevant documentation.
- Merge only after automated checks and review requirements pass.
- Prefer a consistent merge policy, selected and documented before implementation begins.

### 15.3 Definition of Ready

A story is ready when its persona and outcome are clear, linked requirements are identified, acceptance criteria and important error cases are testable, dependencies are known, and it is small enough to complete in one iteration.

### 15.4 Definition of Done

A story is done when acceptance criteria pass, relevant automated tests and documentation are updated, diagnostics and security implications are addressed, the pull request is reviewed and merged, and traceability links and evidence are complete.

## 16. Proposed release stages

| Stage | Outcome |
|---|---|
| Foundation | Repository standards, CI, ADR process, shell skeleton, configuration, and disposable workspace |
| Vertical slice | Parse and execute one read-only command end to end with help, messages, and tests |
| Core catalog | Persistent libraries, objects, library-list resolution, and transactional mutations |
| Learning bridge | Bidirectional glossary, comparison cards, examples, and accuracy review |
| Operator experience | Jobs, messages, history, spooled output, completion, and consistent rendering |
| MVP hardening | Batch mode, migration/version handling, security tests, performance baseline, and documentation |

Release planning and dates will be created after epics are estimated. This avoids presenting speculative dates as commitments.

## 17. Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Scope expands toward full IBM i compatibility | MVP never converges | Publish a compatibility statement and require explicit prioritization for every added subsystem |
| UI scope expands toward pixel-perfect emulation too early | Rendering work overwhelms functional learning value | Keep the MVP target at "recognizably AS/400-like" text rendering and defer protocol-accurate or pixel-perfect reproduction |
| Historical behavior is remembered differently by contributors | Inconsistent user experience | Maintain command specifications with examples and record intentional deviations |
| Trademark or proprietary-material concerns | Distribution or community risk | Use original branding/assets, state non-affiliation, and avoid copying proprietary code, screens, or documentation |
| Host integration escapes the emulator workspace | Data loss or security issue | Deny arbitrary host execution, canonicalize paths, use least privilege, and add adversarial integration tests |
| Persistence schema changes corrupt user state | Loss of trust | Version schemas, make atomic writes, test recovery, and document backups/migrations |
| Architecture work displaces user value | Slow feedback | Begin with a thin end-to-end slice and introduce abstractions only at demonstrated boundaries |
| Educational process becomes excessive ceremony | Contributor friction | Keep templates brief and measure artifacts by decision and traceability value |
| Linux analogies are taught as exact equivalents | Learners form incorrect mental models | Require every mapping to include where the analogy breaks and review it against trusted platform references |

## 18. Assumptions and dependencies

- Rust stable and Cargo are available to contributors.
- The project can use well-maintained open-source crates subject to dependency and license review.
- Initial operation is local and single-user; multi-process concurrency is not an MVP requirement.
- The project will use a hosted Git service capable of issues, pull requests, and CI, but the provider has not yet been selected.
- Historical references will be used for conceptual research only; implementation and documentation will be original.

## 19. Open decisions

The following decisions should be resolved through discovery or ADRs before their affected epics begin:

1. Project and executable name, including a trademark review of “Rust/400.”
2. Which historical OS/400 release or general era should guide terminology and default behavior.
3. Exact MVP identifier rules and the extent of intentional compatibility.
4. Persistence mechanism: embedded database, files, or a hybrid.
5. Whether `SIGNON` is required in the default local experience or available as an optional simulation mode.
6. The first supported object types beyond libraries and generic catalog entries.
7. Desired terminal style: 5250-style green-screen presentation with menu navigation and a command line.
8. Repository host, work-item identifier format, merge strategy, release cadence, and license.
9. Whether learning comparisons live only in built-in help or are generated from one source into both the shell and a documentation site.
10. Which Linux baseline should examples target when distributions use different tools, such as `systemd` versus other init systems.

## 20. Product terminology and legal note

“AS/400,” “OS/400,” and “IBM i” are IBM product names or trademarks. Rust/400 is currently a descriptive working title for an independent educational project and is not affiliated with or endorsed by IBM. The project must use original code, text, and visual assets and should include an appropriate NOTICE or attribution document before public distribution.

## 21. Approval criteria for this PRD

This PRD is ready to baseline when the product owner agrees on:

- the product boundary of an inspired educational emulator rather than a compatible replacement;
- the personas, MVP user journeys, and exclusions;
- the prioritized functional and non-functional requirements;
- the acceptance criteria and traceability approach; and
- owners or planned ADRs for the open decisions.

Once baselined, changes to goals, scope, or numbered requirements should be recorded in document history and reflected in affected epics and stories.
