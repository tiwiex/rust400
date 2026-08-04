# EPIC-008: Operator interface and menu presentation

## Outcome

Users can interact with Rust/400 through a recognizably AS/400-style full-screen terminal experience with menus, a command line, contextual status areas, and function-key guidance.

## Scope and success

This epic adds the visible operator interface that sits on top of the command engine. It covers screen rendering, menu definitions, navigation flow, footer legends, and the coexistence of menu selection with direct command entry. It succeeds when a learner can start Rust/400 and recognize the main menu workflow as clearly AS/400-inspired even though the implementation remains an original educational emulator.

## Stories

### US-008-01: Render a full-screen main menu

**As a** returning operator, **I want** a full-screen main menu **so that** the emulator feels like an AS/400-style environment instead of a plain shell.

- Requirements: `FR-001`, `FR-017A`
- Priority: Must
- Dependencies: `US-002-03`

Acceptance criteria:

1. Startup renders a full-screen text UI with a title area, system context area, main content area, input line, and footer legend.
2. The screen includes a "selection or command" style input affordance.
3. Automated snapshot or rendering tests verify the stable screen layout.

### US-008-02: Define menu screens from shared metadata

**As a** maintainer, **I want** menu options and navigation defined from shared metadata **so that** rendered screens and navigation logic cannot silently disagree.

- Requirements: `FR-005A`, `FR-017A`
- Priority: Must
- Dependencies: `US-008-01`

Acceptance criteria:

1. A shared menu definition describes title, options, footer hints, and target actions.
2. Rendering and navigation consume the same menu definition.
3. A test fails if a registered menu lacks required display metadata.

### US-008-03: Support function-key actions

**As a** returning operator, **I want** common function-key actions **so that** navigation feels closer to the AS/400 interaction model.

- Requirements: `FR-019A`
- Priority: Should
- Dependencies: `US-008-01`

Acceptance criteria:

1. A documented subset of function-key actions such as Exit, Cancel, Prompt, Refresh, and Help is supported.
2. Function-key hints render in the footer legend for the active screen.
3. Unsupported keys fail gracefully with a documented message.

### US-008-04: Mix menu selection with direct commands

**As a** user, **I want** to choose a menu option or type a command from the same screen **so that** I can use either workflow naturally.

- Requirements: `FR-001`, domain rule 10
- Priority: Must
- Dependencies: `US-008-01`, `US-002-04`

Acceptance criteria:

1. The main menu accepts either a numbered selection or a command in the documented input area.
2. The input-handling rules are documented and tested for ambiguous or invalid entries.
3. Navigation back to the main menu remains consistent after menu-driven and command-driven flows.
