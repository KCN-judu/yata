---
id: ADR-0005
status: accepted
date: 2026-09-23
area: process
supersedes: []
superseded-by: []
related:
  [
    ADR-0001,
    ADR-0002,
    ADR-0004,
    ADR-0006,
    ADR-0008,
    ADR-0009,
    ADR-0012,
    ADR-0019,
  ]
---

# ADR-0005: Crate boundaries are drawn by rule, not by list

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published. It answers the question ADR-0001 leaves open: where
the effectful application layer lives.

## Context

A workspace can be laid out in advance as a list of crates, one per concept.
Several facts bound what this workspace needs, and none of them is a concept:

- **The reader repository consumes our code.** ADR-0006 gives the reader channel
  the same framing and encoding as the core channel. The reader, in a separate
  repository, must encode exactly what the daemon decodes, so it depends on the
  framing codec and the probe schema from this workspace.
- **Scheme codes arrive as QR codes** (ADR-0009, `spec/scheme-code.md`). Reading
  one pulls in image decoding, a dependency class nothing else in the core
  needs.
- **The platforms are Windows and macOS** (ADR-0008). Reader spawning exists on
  Windows only, so part of the daemon is platform-specific.
- **Scoring is not designed yet** (`spec/scoring.md`). A crate created for it
  now would hold code whose shape nobody knows.
- **Persistence has its own protobuf schema** (ADR-0002, `spec/fact-format.md`),
  owned by the store.

A fixed list goes stale each time facts change, because it records _where the
lines fell_, not _why_. Its failure has two forms. Crates created early, before
their code, end up with one dependent and no reason to exist, and every change
pays for the crate boundary. Crates that should split don't, because the list
reads as settled and nobody knows what would justify changing it.

## Decision

**A crate boundary exists when, and only when, one of the triggers below
holds.** The number of crates is whatever the triggers produce at the time. The
architecture page records the current result and, for each crate, the trigger
that justifies it.

### Split triggers

A module becomes its own crate when any of these is true:

1. **Effect class.** A pure crate never gains an effect. Code that performs I/O,
   reads a clock, draws randomness, reads the environment, or spawns a process
   lives only in an effectful crate (ADR-0001). The reverse is allowed: an
   effectful crate may hold pure glue that belongs to its effect.
2. **Dependency weight.** A module needs a dependency that the crate's other
   dependents should not compile or ship. The trigger covers heavy dependencies,
   platform-specific ones, and ones that widen what a pure crate can do. The
   module moves out together with its dependency.
3. **External consumer.** Code that a repository outside this workspace depends
   on lives in a crate whose dependencies are exactly what that consumer needs,
   and no more.
4. **Measured cost.** A crate's build time or test time measurably slows
   everyday iteration. The measurement goes in the change that splits it, and
   the split follows an existing module boundary.

### What is not a reason to split

- a concept feels distinct, or has its own spec page
- a file or module is large
- a split is expected to be needed later
- symmetry with another crate

Each of these is a reason for a **module**. Modules are free; crate boundaries
are not.

### Merge rule

When a crate's trigger no longer holds — its heavy dependency is gone, its
external consumer left, its build cost fell — it is folded back into its
dependent. A crate with one dependent and no live trigger is a defect.

### Invariants, whatever the crate set is

- **Semantic judgment lives in the pure core.** Every decode rule, the
  projection fold, query evaluation, scheme-code meaning and `matches`, and
  scoring are pure (the layer table in `architecture/overview.md`). Only a split
  trigger moves any of it out, and then only into another pure crate.
- **Dependencies are one-way and acyclic.** The pure core depends on no
  workspace crate. Nothing depends on the daemon. A pure crate never depends on
  an effectful one.
- **A crate is created when its first code lands.** No empty crates, and no
  placeholders.
- **Workspace lints are inherited, never overridden.** Every crate declares
  `[lints] workspace = true` and nothing else. `unsafe_code` is `forbid`. A
  crate that seems to need `unsafe` belongs behind a process boundary, like the
  reader. It does not get a lint exception.

### Enforcement

- **Pure crates carry a `clippy.toml`** listing as `disallowed-methods` and
  `disallowed-types` the standard library's filesystem, network, process,
  environment, clock, and thread-spawn entry points. The pure-core rule is then
  a lint failure, not a review comment.
- **A preflight check tests the graph.** It holds the class of every crate (pure
  or effectful), rejects a pure crate that depends on an effectful one, and
  rejects any `[lints]` table other than `workspace = true`.
- **A split names its trigger.** The change that creates or folds a crate states
  which trigger holds and updates the crate table in `architecture/overview.md`.
  A split that follows these rules needs no new ADR, which is the point of this
  one. Only a new trigger, or an exception to one, needs an ADR.

### The initial crate set

Applying the triggers to the facts above gives three crates to start with:

| Crate           | Class     | Owns                                                                                                                                                                                                                                          | Trigger                                                                                             |
| --------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `yata-core`     | pure      | domain vocabulary; decode rules from probe records to domain values; scheme-code model, payload codec (Base64 and zlib), and `matches`; the projection fold; query evaluation; scoring when it is designed                                    | the root; everything semantic starts here                                                           |
| `yata-protocol` | pure      | the frame codec; the Rust types generated from the core and probe schemas                                                                                                                                                                     | 3: the reader repository depends on the codec and the probe schema, and must not compile the domain |
| `yata-daemon`   | effectful | the binary: the protocol session; conversion between domain values and messages; the store (redb, the fact schema, encoding and lifting of facts); reader spawning, Windows only; QR image reading and rendering; every clock, seed, and file | 1: the effectful shell                                                                              |

The three crates depend on each other as follows:

```text
yata-daemon ──► yata-core
      └───────► yata-protocol          (yata-protocol depends on no workspace crate)
```

The conversion between domain values and messages lives in `yata-daemon` because
`yata-protocol` is consumed by the reader repository and so must not depend on
the core (trigger 3). It is one module, directional, and the only place a domain
value becomes a message (ADR-0004).

These splits are expected and not yet made. Each waits for its trigger:

| Likely split                            | Trigger that would make it                                                                     |
| --------------------------------------- | ---------------------------------------------------------------------------------------------- |
| QR reading out of the daemon            | 4: image decoding dominates the daemon's build, or 2: something other than the daemon needs it |
| the store out of the daemon             | 3 or 2: a second binary or tool reads the fact log                                             |
| matching out of the core                | 2: the matching algorithm needs a solver dependency                                            |
| the probe schema out of `yata-protocol` | 3: the reader should not compile the core schema, and the cost is measured                     |

The Flutter side (`app/`) is outside this decision. Its package layout is
decided with the Flutter architecture (ADR-0012), and the same four triggers are
the basis for it.

## Alternatives

- **A fixed list of crates, one per concept** — `domain`, `import`, `score`,
  `affinity`, `store`, `protocol`, `daemon`. Rejected for the reasons in
  Context. Four of the seven (`import`, `score`, `affinity`, `store`) have no
  trigger, and the list has no place for the QR layer or the reader's
  dependency.
- **One crate until it hurts.** This is trigger 4 alone. Rejected because
  triggers 1 and 3 are correctness properties, not performance ones. Mixing pure
  and effectful code in one crate makes the pure-core rule unenforceable, and an
  external consumer of a large crate inherits all of it.
- **One crate per spec page.** Rejected: spec pages divide by meaning, and
  crates divide by compile-time and dependency facts. A page like
  `scheme-code.md` spans the core (model, codec) and the daemon (QR).

## Consequences

**Easier.** The crate set starts small and each crate's reason can be checked.
Splitting is routine: name the trigger, move the module, update one table.
Scoring, which has no design yet, starts as a module in the core and costs
nothing until it earns a crate.

**Harder.** The daemon starts large. Its size is accepted until trigger 4 is
measured, and it means `cargo test -p yata-daemon` covers the store and the QR
layer as well as the session. Every contributor has to know the triggers, and a
split proposed without one will be refused.

**Where the result is recorded.** The current crate table, with each crate's
trigger, lives in `docs/architecture/overview.md`.

## Amendment 2026-09-24

[ADR-0019](0019-sqlite-store-behind-an-instruction-crate.md) adds a fifth split
trigger:

5. **Language boundary.** Code that emits another language's text, such as SQL,
   lives in its own pure crate, so that the translation is testable alone and no
   other crate can emit that language.

It creates `yata-store` (pure): the store's instruction set, the schema, and the
translation of instructions into SQL and of result rows into typed values. The
daemon keeps the fact codec and lifting, and a small executor that runs
`yata-store`'s plans. `yata-store` depends on no workspace crate; the daemon
depends on it.
