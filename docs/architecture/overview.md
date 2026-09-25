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

This page states the design. What of it is implemented and tested is in
`../project/status.md`.

## The shape

Three boundaries, two of them process or language transitions. The project does
not read the game (ADR-0030): the inventory arrives as a file the user supplies,
so nothing in the diagram touches another process.

```text
  the user                                   this repository
  ────────                                   ───────────────

  ┌────────────────┐                    ┌──────────────────────────┐
  │ imported file  │                    │  Flutter app             │
  │ (the user's)   │                    │  presentation only       │
  └───────┬────────┘                    └───────────▲──────────────┘
          │ ① file import                           │ ③ stdio frames
          │ (spec/import-format.md)                 │ typed protocol,
          │                                         │ generated (ADR-0004)
  ┌───────▼─────────────────────────────────────────┴────────────┐
  │  yata-daemon — the core, as a separate process               │
  │                                                              │
  │     the protocol session, every effect, the store            │
  │     (─────► ② SQLite fact log, durable), QR, file import     │
  │                                                              │
  │   yata-core       pure: domain, decode, scheme, fold,        │
  │                   query, scoring                             │
  │   yata-protocol   pure: frame codec, generated types         │
  │   yata-store      pure: store instructions → SQL (ADR-0019)  │
  └──────────────────────────────────────────────────────────────┘
```

Reading the numbered edges as _risk classes_ rather than as data flow is the
useful part:

| Edge             | Class                                                                       | Why it is where it is                                                                                                |
| ---------------- | --------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| ① file → core    | **untrusted input.** The file comes from outside and its format is not ours | Parsed once at the boundary into domain observations; what cannot be parsed is refused, never defaulted (ADR-0030)   |
| ② core → store   | **the only durable state.** Mistakes outlive the process                    | Append-only fact log; every write is an added fact, never an edited one (ADR-0002)                                   |
| ③ core → Flutter | **the only place UI latency is paid.** Everything above it is microseconds  | A process boundary, not FFI: a core panic cannot take down the UI, and the same binary serves CI headless (ADR-0004) |

All three edges are inside this repository's control and testable with files and
fixtures. The UI layer knows exactly one child process, the daemon.

## Crates

Which crates exist is decided by rule, not by list:
[ADR-0005](../decisions/0005-crate-boundaries-by-rule.md) states the split
triggers (four, and a fifth added by ADR-0019), the merge rule, and the
invariants. This table is the current result. A crate is added or folded by
naming its trigger and editing this table.

| Crate           | Class     | Owns                                                                                                                                                                       | Trigger                                         |
| --------------- | --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| `yata-core`     | pure      | domain vocabulary; decode rules from probe records to domain values; scheme-code model, payload codec, and `matches`; the projection fold; query evaluation; scoring       | root                                            |
| `yata-protocol` | pure      | the frame codec; Rust types generated from the core, probe, and snapshot schemas; the yata-snapshot file                                                                   | 3 — its reader-side consumer is gone (ADR-0030) |
| `yata-store`    | pure      | the store's instruction set; the SQLite schema; translation of instructions into SQL and of result rows into typed values                                                  | 5 — language boundary (ADR-0019)                |
| `yata-daemon`   | effectful | the binary; the protocol session; domain ↔ message conversion; the store (fact schema, fact codec and lifting, the SQLite executor); file import; QR reading and rendering | 1 — the effectful shell                         |

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
- `yata-protocol` depends on no workspace crate either. The reason was that the
  reader repository could use it without compiling the domain; that repository
  is deleted (ADR-0030).
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

| Layer                      | Owns                                                                                                                                                                                                                  | Never does                                                                                                                      |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| **Flutter app** (ADR-0012) | presentation, interaction, layout, ephemeral view state, optimistic rendering                                                                                                                                         | decode a payload, map a set id, compute or interpolate a score, decide a slot, decide what fits a Shikigami, validate an import |
| **`yata-protocol`**        | the frame codec and the generated message types                                                                                                                                                                       | any branch that changes a value; any domain rule; any dependency on the core                                                    |
| **`yata-daemon`**          | the protocol session, use cases, domain ↔ message conversion, the store, effect injection, process spawn, progress, error mapping, the revision discipline; it runs the request-id discipline `yata-protocol` defines | domain arithmetic or any semantic judgment (it calls `yata-core`)                                                               |
| **pure crates**            | every decode and scoring judgment, the fold, the matching                                                                                                                                                             | I/O, clock, randomness, logging, event emission, panics for expected failure                                                    |

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
`MatchingResult` are computed on demand and held in a memo keyed by the
projection revision and the parameter set's identity, `(id, version)`: two
parameter sets with the same version number are different sets. The legacy
application persisted four independent derived caches and maintained four
invalidation schemes; all four are replaced by that key.

The consequences are deliberate: a parameter change is a cache miss, not a
migration; changing the definition of a need profile re-scores the whole
inventory at the cost of one recomputation; and there is never a stored number
whose provenance is a version of the rules that no longer exists. The only
durable derived thing is the user's _decisions_ — marks and notes (`SoulMarked`,
`SoulNoted`) — and those are facts like any other (ADR-0002).

## Data exchange paths

The user-facing answer to "how many paths are there" is: **eight, of which one
is a file from outside and two remain from the reader's wire for tests and
research** (ADR-0030).

| #   | Path                                   | Format                                                 | Lives in                             | Verifiable                |
| --- | -------------------------------------- | ------------------------------------------------------ | ------------------------------------ | ------------------------- |
| 1   | imported file → core                   | a community format or yata-snapshot, by its header     | `../spec/import-format.md`, ADR-0031 | `cargo test`              |
| 2   | core → blob store (raw snapshot bytes) | content-addressed by SHA-256                           | `yata-daemon`, store module          | `cargo test`              |
| 3   | core → fact log (durable state)        | versioned tagged records, append-only                  | `../spec/fact-format.md`             | `cargo test`              |
| 4   | fact log → projection (in memory)      | pure fold, no wire format                              | `yata-core`                          | `cargo test`              |
| 5   | core → Flutter (queries and commands)  | length-prefixed frames, the core schema                | `../spec/core-protocol.md`, ADR-0004 | `cargo test` + Dart tests |
| 6   | core → Flutter (subscriptions)         | same wire; the message carries a revision, not a value | same                                 | same                      |
| 7   | recording file → core                  | a reader session's frame stream, kept as a fixture     | `../spec/probe-protocol.md`          | `cargo test`              |
| 8   | probe export file → core               | proto3 JSON of a `ProbeExport`                         | `../spec/probe-protocol.md`          | `cargo test`              |

Path 1 ends at the snapshot IR (`../spec/snapshot-ir.md`): every format module
normalizes into it, and the IR's canonical binary encoding is the blob a fact
names. Paths 7 and 8 are what remains of the reader's wire; ADR-0031 deletes
them.

Two rules keep this list from growing, which is its own kind of design:

**No path reads the game.** A path that reads, attaches to, or automates the
game or an emulator is excluded by
[ADR-0030](../decisions/0030-no-game-reader.md), not merely undecided.

**No path exists only to serve the UI.** A boundary whose purpose is "the UI
needs it more conveniently" is a violation of the layer table above; the fix is
a better shape on path 5, not a ninth path.

### What each boundary carries, and in which direction

The counter-intuitive one is path 6. A subscription carries **no data** — only
the fact that something changed and the new revision. The UI then issues a path
5 query for the value it needs. Sending the changed value down the subscription
would put a second copy of the truth in the Flutter layer, where it can go stale
silently; a revision cannot.

Path 5 is where the one real performance question lives. A query returns a
_page_: the result rows plus a total, a cursor, and the revision they are valid
at. The cursor is an opaque domain value (a sort key continuation), not an
offset and not a SQL LIMIT — nothing in this project knows what a query plan is.
Full-inventory queries are bounded by a row budget; when a result exceeds it the
core says so and the UI paginates, which keeps the working set in Rust and stops
the Flutter heap from becoming the new inventory store (the legacy application's
actual failure mode).

## Where the dirty code lives

Nowhere. The one kind of dirty code this project once had, reading another
process's memory, lived in the separate `yata-reader` repository, which is
deleted (ADR-0030). The workspace forbids `unsafe` and has no platform-specific
module.

## What CI can and cannot see

Stated once, here, because it explains why the test strategy is shaped the way
it is.

| Layer                                                         | In CI       | Mechanism                                                                                                      |
| ------------------------------------------------------------- | ----------- | -------------------------------------------------------------------------------------------------------------- |
| byte decode, fact format, fold, quality, fit, matching, query | yes         | `cargo test`, pure functions over recorded bytes                                                               |
| file import                                                   | yes         | `cargo test` over fixture files                                                                                |
| the protocol surface                                          | yes         | `cargo test` on the Rust session; Dart tests against a stubbed transport, plus a committed-bindings diff check |
| the UI                                                        | yes, weakly | widget tests over a fake repository; no scoring assertions, because the UI has no scores to assert             |

There is no layer CI cannot see: nothing depends on a running game.

## Open, and deliberately not decided here

| Question                                                                          | Where it goes                                               |
| --------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| the exact weights and need profiles for each Shikigami                            | a parameter set, `spec/scoring.md` + a data file, versioned |
| where need profiles come from (derived from the game's damage model vs. authored) | `../spec/scoring.md`, provenance-tagged per profile         |
| the matching algorithm and its complexity bound                                   | `../spec/scoring.md`                                        |
| the tag numbers of either schema                                                  | the two schema files in `crates/yata-protocol/proto/`       |

## Related

- The premise every layer obeys:
  [ADR-0001](../decisions/0001-pure-core-effects-at-the-boundary.md)
- Persistence shape:
  [ADR-0002](../decisions/0002-fact-log-projection-and-fact-schema.md)
- Two-pass scoring: [ADR-0003](../decisions/0003-two-pass-scoring.md)
- The Flutter boundary: [ADR-0004](../decisions/0004-core-process-boundary.md)
- No game reader, and the import format:
  [ADR-0030](../decisions/0030-no-game-reader.md)
- What crosses each boundary:
  [../spec/core-protocol.md](../spec/core-protocol.md),
  [../spec/probe-protocol.md](../spec/probe-protocol.md),
  [../spec/protocol-versions.md](../spec/protocol-versions.md)
- What the system means: [../spec/scoring.md](../spec/scoring.md),
  [../spec/glossary.md](../spec/glossary.md)
- The requirements every change must meet:
  [../guides/engineering-requirements.md](../guides/engineering-requirements.md)
