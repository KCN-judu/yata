---
kind: architecture
status: current
area: process
---

# Architecture overview

How this project is put together right now: the layers, the boundaries between
them, and which crate owns what. This page is the entry point for "where does
this belong"; the format of each boundary's payloads lives in the spec pages it
links.

Everything here is _designed_. Nothing is _implemented_ — see
`../project/status.md`.

## The shape

Four boundaries, three of them process or language transitions. Only two of the
four can fail in ways the other two cannot, and keeping them apart is the point
of the diagram.

```text
  the game                       separate repository                    this repository
  ────────                       ───────────────────                    ───────────────

  ┌────────────┐  ①  memory read   ┌───────────────┐
  │ game       │ ───────────────►  │ yata-reader   │        ┌──────────────────────────┐
  │ process    │  (read-only)      │ (worker exe)  │        │  Flutter app             │
  └────────────┘                   └───────┬───────┘        │  presentation only       │
                                           │                └───────────▲──────────────┘
                                           │ ② named pipe:              │
                                           │ frames (ADR-0006),         │ ④ stdio frames
                                           │ its own schema (ADR-0006)  │ typed protocol,
                                           │                            │ generated (ADR-0004)
                        ┌──────────────────▼────────────────────────────┴──────────────┐
                        │  yata-daemon — the core, as a separate process                │
                        │                                                               │
                        │     the protocol session, every effect, the store             │
                        │     (─────► ③ SQLite fact log, durable), QR, probe spawning   │
                        │                                                               │
                        │   yata-core       pure: domain, decode, scheme, fold,         │
                        │                   query, scoring                              │
                        │   yata-protocol   pure: frame codec, generated types          │
                        │   yata-store      pure: store instructions → SQL (ADR-0019)   │
                        └───────────────────────────────────────────────────────────────┘
```

Reading the numbered edges as _risk classes_ rather than as data flow is the
useful part:

| Edge             | Class                                                                                        | Why it is where it is                                                                                                |
| ---------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| ① game → probe   | **irreversible if wrong.** Reads another process's memory; cannot be tested without the game | Isolated in a separate repository so this repository can forbid `unsafe` outright (ADR-0005, ADR-0007)               |
| ② probe → core   | **unverifiable live, verifiable by replay.** No CI can run a game                            | Pipe, not temp file; the protocol is specified and decoded _here_, and recorded sessions become golden fixtures      |
| ③ core → store   | **the only durable state.** Mistakes outlive the process                                     | Append-only fact log; every write is an added fact, never an edited one (ADR-0002)                                   |
| ④ core → Flutter | **the only place UI latency is paid.** Everything above it is microseconds                   | A process boundary, not FFI: a core panic cannot take down the UI, and the same binary serves CI headless (ADR-0004) |

Edges ③ and ④ are inside this repository and fully testable. Edge ② is testable
only against recorded bytes. Edge ① is testable only by a human with the game
running. **No rule is allowed to depend on edge ① to be tested** — see "What CI
can and cannot see" below.

Edges ② and ④ have the same shape — a child process reached over pipes — which
is deliberate. The UI layer knows exactly one child process (the daemon); the
probe is the daemon's child, not the app's. That keeps the number of process
relationships the presentation layer must survive at one.

## Crates

Which crates exist is decided by rule, not by list:
[ADR-0005](../decisions/0005-crate-boundaries-by-rule.md) states the split
triggers (four, and a fifth added by ADR-0019), the merge rule, and the
invariants. This table is the current result. A crate is added or folded by
naming its trigger and editing this table.

| Crate           | Class     | Owns                                                                                                                                                                                         | Trigger                                |
| --------------- | --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------- |
| `yata-core`     | pure      | domain vocabulary; decode rules from probe records to domain values; scheme-code model, payload codec, and `matches`; the projection fold; query evaluation; scoring                         | root                                   |
| `yata-protocol` | pure      | the frame codec; Rust types generated from the core and probe schemas                                                                                                                        | 3 — the probe repository depends on it |
| `yata-store`    | pure      | the store's instruction set; the SQLite schema; translation of instructions into SQL and of result rows into typed values                                                                    | 5 — language boundary (ADR-0019)       |
| `yata-daemon`   | effectful | the binary; the protocol session; domain ↔ message conversion; the store (fact schema, fact codec and lifting, the SQLite executor); probe spawning (Windows only); QR reading and rendering | 1 — the effectful shell                |

`app/` holds the Flutter desktop application; its package layout is outside
ADR-0005.

**One binary, two front ends.** `yata-daemon serve` runs the protocol on stdio
and is what the Flutter app spawns; the one-shot subcommands (`replay`, `score`,
`check`) run the same session code with no UI and are how CI and a developer
answer "is this a core bug or a UI bug". There is deliberately no separate CLI
binary: two binaries sharing a session type drift, and the subcommand that
answers a question during an incident is exactly the one that would drift
without anyone noticing.

Dependency direction rules (ADR-0005 invariants):

- `yata-core` depends on no workspace crate and knows nothing about the wire, so
  the whole domain is exercisable from `cargo test` with no protocol, no Dart,
  no process.
- `yata-protocol` depends on no workspace crate either, so the probe repository
  can use it without compiling the domain.
- `yata-store` depends on no workspace crate and on no SQLite binding. It is the
  only crate that contains SQL; the daemon's executor runs its plans and cannot
  build one (ADR-0019).
- Nothing depends on `yata-daemon`. It is the only effectful crate and the only
  one that spawns anything, touches a file, or reads a clock. A pure crate that
  needs to persist something has the wrong responsibility; it returns a value
  and the daemon persists it.

## Layer responsibilities

The rule is ADR-0001's, applied to four layers instead of two: **semantic truth
flows up; presentation never flows down and redefines it.**

| Layer                      | Owns                                                                                                                                                                      | Never does                                                                                                                      |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| **Flutter app** (ADR-0012) | presentation, interaction, layout, ephemeral view state, optimistic rendering                                                                                             | decode a payload, map a set id, compute or interpolate a score, decide a slot, decide what fits a Shikigami, validate an import |
| **`yata-protocol`**        | the frame codec and the generated message types                                                                                                                           | any branch that changes a value; any domain rule; any dependency on the core                                                    |
| **`yata-daemon`**          | the protocol session, use cases, domain ↔ message conversion, the store, effect injection, process spawn, progress, error mapping, the request-id and revision discipline | domain arithmetic or any semantic judgment (it calls `yata-core`)                                                               |
| **pure crates**            | every decode and scoring judgment, the fold, the matching                                                                                                                 | I/O, clock, randomness, logging, event emission, panics for expected failure                                                    |

## The two passes

The functional core of this application is two scoring passes, not one. This is
the decision that shapes every layer above it (ADR-0003). Both passes live in
`yata-core` until a split trigger says otherwise (ADR-0005).

Pass 1 is **target-independent** and answers _is this soul any good?_ — a
`QualityScore` from the soul's own attributes, with no Shikigami in the input.
It is what the legacy application computed, and it is not enough, because it
cannot distinguish "good" from "good for something".

Pass 2 is **target-relative** and answers _what is this soul worth to whom?_ It
takes the inventory plus a set of Shikigami need profiles and produces, for each
soul, a fit against each need, plus — above that — candidate loadouts and what
each would change. A soul with a mediocre generic score that is concentrated in
one needed attribute scores low in pass 1 and high in pass 2 for the Shikigami
that need it. That inversion is the feature; a single pass cannot produce it at
any weight setting, because the weights would have to be the average over all
Shikigami, and the average is exactly what erases the signal.

Pass 2 works at three levels (ADR-0028): one soul against a need, one loadout
against a need, and one swap in a loadout. A need's floors and caps are sums
over six souls, so they are decided on a loadout, never on one soul. Pass 1 does
not bound pass 2, so it prunes nothing by itself; a quality threshold on
candidates is a user's policy.

The definitions are in `../spec/scoring.md`. The shapes in one line:

```text
quality  : (params, Soul)                              -> QualityScore
fit      : (params, NeedProfile, Soul)                 -> AffinityScore     // one soul, one need
evaluate : (params, NeedProfile, Loadout, Baseline?)   -> LoadoutFit        // one loadout
swap     : (params, NeedProfile, Loadout, slot, Soul, Baseline?) -> SwapDelta  // against what is worn
match    : (params, Inventory, [NeedProfile])          -> MatchingResult    // assignment + deltas
```

All of them are total, pure, and parameterized by a versioned parameter set, so
the same code can be re-run under a different set of weights and the result is
comparable rather than merely different.

## Derived results are not stored

Nothing derived is durable. `QualityScore`, `AffinityScore`, and
`MatchingResult` are computed on demand and held in a memo keyed by
`(projection revision, parameter set version)`. The legacy application persisted
four independent derived caches and maintained four invalidation schemes; all
four are replaced by that key.

The consequences are deliberate: a parameter change is a cache miss, not a
migration; changing the definition of a need profile re-scores the whole
inventory at the cost of one recomputation; and there is never a stored number
whose provenance is a version of the rules that no longer exists. The only
durable derived thing is the user's _decisions_ — locks, discards, equips, notes
— and those are facts like any other (ADR-0002).

## Data exchange paths

The user-facing answer to "how many paths are there" is: **eleven, of which two
are outside this repository's control, one exists for testing, and one carries a
probe reading by file** (ADR-0008).

| #   | Path                                     | Format                                                            | Lives in                              | Verifiable                           |
| --- | ---------------------------------------- | ----------------------------------------------------------------- | ------------------------------------- | ------------------------------------ |
| 1   | game process → probe                     | engine-specific binary read                                       | probe repo                            | manually, with the game              |
| 2   | probe → core (data)                      | protobuf in length-prefixed frames, one message per frame         | `../spec/probe-protocol.md`           | **fixture replay**                   |
| 3   | probe → core (progress, diagnostics)     | same pipe, same framing, different kind                           | same                                  | fixture replay                       |
| 4   | core → probe (control: cancel, shutdown) | same pipe, reverse direction                                      | same                                  | fixture replay                       |
| 5   | core → blob store (raw snapshot bytes)   | content-addressed by SHA-256                                      | `yata-daemon`, store module           | `cargo test`                         |
| 6   | core → fact log (durable state)          | versioned tagged records, append-only                             | `../spec/fact-format.md`              | `cargo test`                         |
| 7   | fact log → projection (in memory)        | pure fold, no wire format                                         | `yata-core`                           | `cargo test`                         |
| 8   | core → Flutter (queries and commands)    | same framing, the core schema                                     | `../spec/core-protocol.md`, ADR-0004  | `cargo test` + Dart tests            |
| 9   | core → Flutter (subscriptions)           | same wire; the message carries a revision, not a value            | same                                  | same                                 |
| 10  | recording file → core                    | the reader's frame stream captured by the daemon, frame for frame | `../spec/probe-protocol.md`           | `cargo test` (this is the mechanism) |
| 11  | probe export file → core                 | proto3 JSON of a `ProbeExport`, the probe schema's own message    | `../spec/probe-protocol.md`, ADR-0008 | `cargo test`                         |

Paths 5–10 are inside this repository or generated from it and are covered by
ordinary tests. Paths 1–4 are the probe boundary, and path 10 is what makes them
testable without the game: a recording is the probe's own byte stream, so the
decoder cannot tell it from a live pipe (ADR-0006).

Paths 2–4 and 10 speak the same wire as paths 8 and 9 — one framing codec, one
encoding, one versioning story (ADR-0006). The two channels are still separate
schemas with disjoint error-code namespaces, so a change to one cannot silently
change the other.

Two rules keep this list from growing, which is its own kind of design:

**A new channel is a new path 1–4, and needs an ADR.** Reading the game a second
way (a third memory route, a new emulator, a network capture) is a decision to
trust a new source, and the glossary's "two channels only" is the current answer
(`../spec/glossary.md`, and [ADR-0006](../decisions/0006-reader-channel.md) for
what a channel must speak once it exists).

**No path exists only to serve the UI.** A boundary whose purpose is "the UI
needs it more conveniently" is a violation of the layer table above; the fix is
a better shape on path 8, not a twelfth path.

### What each boundary carries, and in which direction

The counter-intuitive one is path 9. A subscription carries **no data** — only
the fact that something changed and the new revision. The UI then issues a path
8 query for the value it needs. Sending the changed value down the subscription
would put a second copy of the truth in the Flutter layer, where it can go stale
silently; a revision cannot.

Path 8 is where the one real performance question lives. A query returns a
_page_: the result rows plus a total, a cursor, and the revision they are valid
at. The cursor is an opaque domain value (a sort key continuation), not an
offset and not a SQL LIMIT — nothing in this project knows what a query plan is.
Full-inventory queries are bounded by a row budget; when a result exceeds it the
core says so and the UI paginates, which keeps the working set in Rust and stops
the Flutter heap from becoming the new inventory store (the legacy application's
actual failure mode).

## Where the dirty code lives

There is exactly one kind of dirty code in this project — reading another
process's memory — and it lives outside both this repository and any crate in
this workspace.

```text
   separate repository: yata-reader                    this repository
   ───────────────────────────────                 ───────────────
   probe.exe  ── reads ──► game                    yata-core::import (pure)
                                                   yata-daemon  (spawns, pumps)
                  ② pipes ▲   │                                   ▲
                          └───┘                                   │
                  spawn + stdin/stdout ───────────────────────────┘
```

| Concern                                                                | Where it lives                                                  | Why there                                                                                                              |
| ---------------------------------------------------------------------- | --------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| external read-only memory access, structure parsing, offsets, patterns | reader repo (open source, ADR-0007)                             | needs `unsafe`; must not be able to weaken this workspace's lints; held to `spec/reader-security.md`                   |
| engine detection and per-engine read strategy                          | probe repo                                                      | changes when the game patches, on its own release cadence                                                              |
| the message schema                                                     | **this repo**, `docs/spec/probe-protocol.md`                    | the consumer defines the contract; the producer conforms                                                               |
| the frame codec and generated probe types                              | **this repo**, `yata-protocol`, which the probe repo depends on | one codec, not two that agree today (ADR-0006); the reason the crate exists (ADR-0005 trigger 3)                       |
| decoding a probe record into a domain value                            | **this repo**, `yata-core`                                      | it is a pure function over bytes and must be unit-testable                                                             |
| spawning, pumping, timeout, cancel, restart-on-crash                   | **this repo**, `yata-daemon`                                    | effects belong at the boundary (ADR-0001), and the daemon is the only crate that may spawn anything (ADR-0004 rule 10) |

The property this buys, stated plainly: **`unsafe_code = "forbid"` in the
workspace manifest is a constraint that can be violated.** Inside one workspace
it is decorative as soon as a member re-declares its lints to escape it, and
code that reads another process is the code most tempted to. A separate
repository removes the escape hatch rather than trusting nobody will use it.

## What CI can and cannot see

Stated once, here, because it explains why the test strategy is shaped the way
it is.

| Layer                                                         | In CI          | Mechanism                                                                                                      |
| ------------------------------------------------------------- | -------------- | -------------------------------------------------------------------------------------------------------------- |
| byte decode, fact format, fold, quality, fit, matching, query | yes            | `cargo test`, pure functions over recorded bytes                                                               |
| the probe protocol                                            | yes, by replay | a fixture session file fed through the real decoder                                                            |
| the probe's own behaviour against a live game                 | never          | manual, on the developer's machine                                                                             |
| the protocol surface                                          | yes            | `cargo test` on the Rust session; Dart tests against a stubbed transport, plus a committed-bindings diff check |
| the UI                                                        | yes, weakly    | widget tests over a fake repository; no scoring assertions, because the UI has no scores to assert             |

The gap is path 1, and it is closed by discipline rather than by tooling: **when
the probe can read the game, it records.** A fixture recorded today is the only
thing that will still be testable after the game patches.

## Open, and deliberately not decided here

| Question                                                                          | Where it goes                                               |
| --------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| the exact weights and need profiles for each Shikigami                            | a parameter set, `spec/scoring.md` + a data file, versioned |
| where need profiles come from (derived from the game's damage model vs. authored) | `../spec/scoring.md`, provenance-tagged per profile         |
| the matching algorithm and its complexity bound                                   | `../spec/scoring.md`                                        |
| the tag numbers of either schema                                                  | the two schema files, neither of which exists yet           |

## Related

- The premise every layer obeys:
  [ADR-0001](../decisions/0001-pure-core-effects-at-the-boundary.md)
- Persistence shape:
  [ADR-0002](../decisions/0002-fact-log-projection-and-fact-schema.md)
- Two-pass scoring: [ADR-0003](../decisions/0003-two-pass-scoring.md)
- The Flutter boundary: [ADR-0004](../decisions/0004-core-process-boundary.md)
- The probe wire: [ADR-0006](../decisions/0006-reader-channel.md)
- What crosses each boundary:
  [../spec/core-protocol.md](../spec/core-protocol.md),
  [../spec/probe-protocol.md](../spec/probe-protocol.md),
  [../spec/protocol-versions.md](../spec/protocol-versions.md)
- What the system means: [../spec/scoring.md](../spec/scoring.md),
  [../spec/glossary.md](../spec/glossary.md)
- The requirements every change must meet:
  [../guides/engineering-requirements.md](../guides/engineering-requirements.md)
