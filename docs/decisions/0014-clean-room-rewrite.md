---
id: ADR-0014
status: accepted
date: 2026-09-23
area: process
supersedes: []
superseded-by: []
related: [ADR-0007, ADR-0013, ADR-0016]
---

# ADR-0014: Nothing is ported from the prior tool; it is a source of facts, not of code

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published. It applies to this repository and to `yata-reader`.

## Context

This project is a rewrite of an earlier tool for the same game. The maintainer
has studied that tool closely. The question is what may be carried over from it.

Two facts decide it:

- **No license grant covers the prior tool's code.** Nothing in it grants this
  project the right to reuse its code, and `yata-reader` is published under
  `MIT OR Apache-2.0` (ADR-0007), so code of unclear provenance cannot go into
  it.
- **Its implementation is not a template.** This project is designed from its
  own specifications, not from the prior tool's structure.

## Decision

**No code, test, script, binary, or asset of the prior tool enters either
repository,** whether copied, translated line by line into another language, or
closely paraphrased. Both repositories are written from this project's own
specifications.

**What may be taken is facts about the game and its platforms,** because facts
are what the rewrite needs and they are not the prior tool's expression:

| May be taken, as a hypothesis to test                             | May not be taken                                       |
| ----------------------------------------------------------------- | ------------------------------------------------------ |
| memory layout of the game's objects                               | the prior tool's source files, or translations of them |
| field and key names the game uses                                 | its tests, fixtures, or data files                     |
| the game's identifiers                                            | its binaries, libraries, scripts, or emulator helpers  |
| scheme-code layouts                                               | its UI text, icons, or other assets                    |
| behaviour of adb, the MuMu emulator, and Windows APIs as observed | its architecture as a template                         |
| the prior tool's mistakes, as things to avoid                     |                                                        |

**Facts enter through documents, not through the prior tool's source.**

- A fact is recorded, with its evidence and confidence, in a spec page (such as
  [spec/scheme-code.md](../spec/scheme-code.md),
  [spec/probe-protocol.md](../spec/probe-protocol.md), or
  [spec/reader-security.md](../spec/reader-security.md)) or in the project's
  local research records (ADR-0016).
- Implementation is written from those pages. Someone implementing a feature
  reads the spec, not the prior tool.
- A fact taken from the prior tool is a hypothesis until this project's own
  evidence establishes it: a recording, a fixture, a single-variable test in the
  game. The evidence marks kept with the scheme-code layout are the model for
  this.

## Alternatives

- **Port selected components.** Rejected: no license grant covers them, and the
  maintainer does not want the components themselves.
- **Ask the prior tool's author for a license.** Not needed. Nothing is wanted
  beyond facts, and facts need no license. If a future need for its code arises,
  this is the path, and it would be a new ADR.
- **Formal clean-room separation,** with the people who read the prior tool's
  code never writing implementation. Rejected as impractical for a project of
  this size, and unnecessary when nothing is copied and every fact passes
  through a written spec.

## Consequences

**Easier.** Both repositories have clear provenance, and nothing in them needs
attribution to the prior tool. `yata-reader`'s license is clean.

**Harder.** Everything is written from scratch, including parts where the prior
tool's code would have been a shortcut. Every fact taken from it must be
re-established against this project's own recordings before code relies on it,
which puts the recording work early in milestone 1.
