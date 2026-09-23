---
id: ADR-0001
status: accepted
date: 2026-09-23
area: domain
supersedes: []
superseded-by: []
related: [ADR-0002, ADR-0003, ADR-0004, ADR-0005, ADR-0014]
---

# ADR-0001: The domain core is pure; every effect enters at the boundary

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published. It establishes the premise the other decisions are
built on.

## Context

This project is a clean-room rewrite of an earlier tool for the same game
(ADR-0014). That tool fused domain code with effects, and the fusion repeated at
every scale: derived-result caches were written from inside the computation that
produced them, so a result could not be computed without also being stored;
wall-clock time was read inside scoring paths instead of being passed in;
progress was reported through a global event, so the computation layer knew a UI
existed; and a recommendation stored its own explanation tree as an opaque
serialized string, so the reasoning behind it could not be inspected, queried,
or re-derived — only displayed.

The one exception in that tool is the shape this decision generalizes: a
simulation module took its randomness as a caller-supplied seed. That module was
the only part whose result was reproducible without a running application, and
therefore the only part comparable against a fixture.

Two forces make the general rule a requirement rather than a preference:

- **No read channel can be exercised in CI.** Reading the desktop client's
  memory and reading through the MuMu emulator both require a running game,
  which CI does not have. The only way a decode or scoring rule gets a test is
  by feeding recorded bytes to a function that needs nothing else: no clock, no
  filesystem, no event bus, no process handle. This makes golden fixtures
  mandatory, and a fixture is worthless against a function that reads the world.
- **The derived features all ask a hypothetical.** Matching and head/tail
  analysis each answer "what would a Shikigami's panel be if these souls were
  equipped?" That is one function shape — `Inventory -> Plan -> Inventory` —
  evaluated speculatively, before anything is committed.

## Decision

The domain core is a set of pure functions over immutable values. Code in
`yata-core`, the pure crate (ADR-0005), obeys all four rules:

1. **Pure where it computes.** `decode(bytes) -> Soul`,
   `score(selector, soul) -> Score`, `apply_plan(inventory, plan) -> Inventory`,
   `project(facts) -> Inventory` are total functions of their arguments. No I/O,
   no clock, no randomness, no logging, no globals, no event emission.
2. **State is a value, not a location.** `Inventory`, `Snapshot`, and every
   derived result are immutable. Producing a new state returns a new value;
   nothing is mutated in place, and nothing is written to a store as a side
   effect of computing it.
3. **Hypotheticals are evaluated in memory.** Because state is a value and
   computation is pure, any plan can be applied and scored speculatively. This
   is not an optimization deferred to later; it is the reason the derived
   analyses are expressible at all, and it is what allows a generated plan to be
   evaluated before it is offered to the user.
4. **Every effect is injected at the boundary.** Clock, randomness (as a seed),
   file and store access, process spawning, progress reporting, and logging
   enter the core as arguments or are performed by the caller around it. Errors
   are values: the core returns `Result`, and panics are reserved for violated
   invariants, not for expected failure.

The presentation layer and the persistence layer are both boundary code. They
may hold no second copy of a domain rule: no score arithmetic, no set-id
mapping, no slot decision, no import validation.

## Alternatives

- **A service layer whose use cases own both orchestration and effects.**
  Rejected: it is the shape that produces opaque explanation strings and scoring
  paths that cannot run outside the application, and it is why analysis features
  built that way cannot be tested without the game installed.
- **Pure core with a dependency-injection container or trait-object graph**
  (effects reached through injected `Clock`, `Store`, `EventBus` traits).
  Rejected as the default: it keeps the dependency inversions without removing
  the reason for them. Effect traits multiply — every new effect needs a trait,
  a production impl, a test impl, and a constructor parameter — and the
  resulting signatures still describe the world. Passing a value or returning a
  value is simpler and yields the same testability.
- **An algebraic-effects library.** Rejected for now: it buys composable effect
  handlers that this project has no use for, at the cost of a macro-heavy
  dialect that few maintainers will recognize. If the effect count grows past a
  handful, this is worth revisiting.

## Consequences

**Easier.** Every decode and scoring rule is testable from a recorded byte
string. Golden fixtures cover the parts of the system that CI cannot otherwise
reach. The derived analyses share one speculative-evaluation mechanism instead
of ad-hoc caches. Deterministic replay becomes possible: given the same fact log
and the same seed, the projection is reproducible, which is what makes storage
compaction (ADR-0002) safe.

**Harder.** Effectful work must be lifted to a caller, which means the
application layer carries explicit arguments for time, seed, and progress that a
global would have hidden. This is more typing at every call site and it is the
intended cost. Contributors used to a service layer will look for one and not
find it.

**Where the rest is decided.**
[ADR-0002](0002-fact-log-projection-and-fact-schema.md) records the persistence
shape this premise implies. How the Flutter layer reaches the core is
[ADR-0004](0004-core-process-boundary.md). Where the effectful application layer
lives in the crate graph is [ADR-0005](0005-crate-boundaries-by-rule.md).
