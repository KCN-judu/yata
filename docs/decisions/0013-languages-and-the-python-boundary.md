---
id: ADR-0013
status: accepted
date: 2026-09-23
area: tooling
supersedes: []
superseded-by: []
related: [ADR-0004, ADR-0005, ADR-0007]
---

# ADR-0013: Shipped code is Rust and Dart; Python is typed glue with a hard boundary

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published.

## Context

Three facts bound this choice:

- A Python program shipped to users brings an interpreter with it. Bundled into
  a single executable, it becomes an archive that users cannot readily inspect.
  That is at odds with a reader users are asked to audit (ADR-0007).
- The reader interprets data structures inside the game's process. Parsing a
  memory layout is byte work in any language, so the reader does not have to be
  written in the language of the program it reads.
- The engineering conventions use Python for tooling (`scripts/preflight.py`,
  the docs validator). The maintainer accepts Python there, on the condition
  that it is fully typed, strictly bounded, and never enters the project's main
  line.

## Decision

### Shipped code

| Component                                                                                 | Language       |
| ----------------------------------------------------------------------------------------- | -------------- |
| `yata-core`, `yata-protocol`, `yata-daemon`                                               | Rust           |
| `yata-reader`, including the helper pushed into the emulator (cross-compiled for Android) | Rust           |
| the application                                                                           | Dart (Flutter) |

Nothing that ships runs Python. No shipped binary embeds, bundles, spawns, or
requires a Python interpreter, and no installer or release artifact contains
one.

### Python is glue

Python may be used only for **tooling**: running checks, formatting, validating
documents, packaging releases, and managing fixtures. It is held to four rules.

1. **Location.** Python files live only in `scripts/` of either repository. A
   `.py` file anywhere else fails preflight.
2. **No project logic.** A script orchestrates tools and files. It never
   re-implements a domain rule, a decode, a scoring step, a codec, or a wire
   format. When a script needs the project's knowledge, it calls `yata-daemon`'s
   subcommands (the one-binary rule of ADR-0004) or a Rust `xtask`. A script
   that needs tests of its own logic, beyond orchestration, is a sign that the
   logic belongs in Rust.
3. **Fully typed, strictly checked.** Every function, parameter, and return
   value is annotated. Scripts pass `mypy --strict` and Ruff (lint and format)
   in preflight. `Any` is allowed only where a standard-library interface
   returns it, and it is narrowed immediately.
4. **Standard library by default.** A third-party dependency needs a stated
   reason and is pinned in a lock file. The Python version is pinned like every
   other toolchain.

The main line — the Rust workspace, the Flutter application, the reader — never
depends on a script. Deleting `scripts/` breaks the developer workflow and CI,
never the product.

## Alternatives

- **No Python at all, with tooling as a Rust `xtask`.** Offered and not chosen.
  Python is kept where it is convenient, under the rules above.
- **Python in the reader.** Rejected: it ships an interpreter and an archive
  that is harder to audit than source, and it contradicts ADR-0007's
  reproducible, inspectable binaries.
- **Python tooling without type checking.** Rejected by the maintainer: untyped
  glue is where silent breakage hides, and it is the part of a repository nobody
  re-reads.

## Consequences

**Easier.** An auditor of any shipped binary deals with Rust and Dart only.
Tooling stays quick to write.

**Harder.** Every script carries full annotations and must satisfy a strict
checker, so a quick script is slower to write. The pinned Python toolchain,
mypy, and Ruff join the list of pinned tools, and preflight gains the location,
typing, and lint checks.
