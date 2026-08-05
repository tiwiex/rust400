# Libraries and objects in Rust/400

Rust/400 is trying to teach the AS/400 way of thinking while still running safely on Linux. The most important bridge is the idea of a library containing typed objects.

## Plain-language definition

In classic OS/400 terms, a library is a named container for system-managed objects. An object is not just "a file"; it is a typed thing the system understands and operates on consistently.

In Rust/400:

- a library is an emulated named container in the workspace catalog
- an object is a future typed catalog entry that belongs to a library
- object type will control which commands can act on it

## Closest Linux analogy

The closest Linux comparison is:

- library -> namespace or directory-like container
- object -> file or resource plus metadata
- library list -> ordered search path, a bit like `PATH`

This analogy helps, but it is not exact.

## Where the analogy breaks

On Linux, a directory mainly maps names to filesystem entries. On AS/400-style systems, a library is part of a typed object catalog. The operating model is more uniform:

- names are managed more centrally
- object type matters to command behavior
- qualified names such as `LIB/OBJ` matter more than raw filesystem paths
- the library list participates in object resolution

So in Rust/400 we should think "catalog and resolution model", not just "folder tree".

## Learning comparison table

| AS/400-style concept | Rust/400 meaning | Linux comparison | Important limit |
|---|---|---|---|
| Library | Named catalog container | Directory or namespace | Not a host path the user navigates directly |
| Object | Typed managed entry | File plus metadata | Type is first-class, not just a naming convention |
| Library list | Ordered resolution scope | `PATH`-style search order | Resolves more than executables |
| Qualified name `LIB/OBJ` | Explicit library plus object name | Explicit scope or absolute path | Does not expose arbitrary host traversal |

## First commands in this area

The first command in this slice is:

- `CRTLIB LIB(MYLIB) TEXT('Learning library')`

Its role is to create an emulated library record inside the Rust/400 workspace catalog.

Planned follow-on commands:

- `DSPLIB`
- `WRKLIB`
- `ADDLIBLE`
- `RMVLIBLE`
- `DSPOBJD`
- `WRKOBJ`

## Example comparison

Rust/400 example:

```text
CRTLIB LIB(ACCOUNTS) TEXT('Accounting training library')
```

Comparable Linux idea:

```text
mkdir accounts
```

But the Linux example is only a teaching aid. `mkdir` creates a filesystem directory. `CRTLIB` creates an emulator-managed catalog entry that will later hold typed objects and participate in library-list resolution.

## Current implementation status

As of August 4, 2026:

- `CRTLIB` persists library definitions in the workspace catalog
- duplicate library creation is rejected cleanly
- display and work-with commands are not implemented yet
- object records are planned but not yet delivered

## Design direction

The project direction for this area is:

1. persist libraries in a versioned catalog
2. display and browse libraries
3. introduce typed objects
4. resolve unqualified names through a library list
5. teach each step with Linux comparisons

That sequence keeps the nostalgic AS/400 model understandable while staying safe inside a Linux-hosted workspace.
