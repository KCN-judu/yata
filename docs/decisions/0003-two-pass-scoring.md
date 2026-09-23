---
id: ADR-0003
status: accepted
date: 2026-09-23
area: scoring
supersedes: []
superseded-by: []
related: [ADR-0001, ADR-0002, ADR-0005]
---

# ADR-0003: Scoring is two passes — a target-independent quality score, then target-relative affinity and matching

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published.

## Context

An earlier tool for the same game computes exactly one number per Soul. It is
the only scoring concept in that tool, and the whole product sits downstream of
it: the soul list is sorted by it, the keep/discard advice is a threshold on it,
and the derived analyses consume it as if it were a fact about the soul.

It is not a fact about the soul. It is an average.

A Soul's value depends on who holds it. Effect HIT is close to worthless on a
damage-dealing Shikigami and is the entire reason a control Shikigami exists.
Crit DMG is the point of one build and a rounding error on another. Slot 6's
main attribute is chosen at acquisition and its worth is entirely a function of
the intended holder. A single-number model collapses all of this into a weighted
sum, which means its weights are, by construction, the average over all
Shikigami — and averaging is precisely the operation that destroys the
information a user needs.

Three concrete failures follow, all of them reported by users of that tool:

- **"Which of my bad souls are actually good?"** The list is sorted by the
  average, so a soul that is excellent for one narrow role and useless elsewhere
  ranks below a soul that is mildly useful everywhere. The user's best find is
  buried under their most forgettable one.
- **"What should I do with this soul?"** A discard threshold on a single number
  cannot answer it. The correct answer is often "keep it, it is one of the few
  things in your inventory that fits X", which requires knowing X.
- **"Is this soul good?"** It cannot say. It can only say "relative to the
  average Shikigami", which is a question nobody asked.

There is a second, independent defect: scoring weights that live in code are not
inspectable or reproducible. A change to them changes every historical
recommendation with no record of what changed or why.

## Decision

Scoring is two passes. They have separate modules, separate shapes, and separate
audiences. Where the modules sit in the crate graph follows
[ADR-0005](0005-crate-boundaries-by-rule.md).

**Pass 1 — quality, target-independent.**
`quality(params, soul) -> QualityScore`. The input is one Soul and a parameter
set; no Shikigami appears. The output is an assessment of the Soul in isolation:
its raw attribute strength, whether its four sub-attributes are concentrated or
scattered, how far they are from what a roll can reach. This pass exists to
answer _is this thing any good in itself_, and to be the pruning predicate for
pass 2.

**Pass 2 — affinity, target-relative.**
`fit(params, need, soul) -> AffinityScore` scores one Soul against one
`NeedProfile` — a Shikigami's or role's stated requirement, carrying required
sets, required main attributes per slot, weighted desired attributes, and floors
below which a mismatch is disqualifying.
`match(params, inventory, needs) -> MatchingResult` lifts this to the whole
inventory: for each need, the candidate loadouts, the best assignment under the
constraint that a Soul can be equipped once, and the delta each assignment would
produce.

Four properties are part of the decision, not implementation details:

1. **The parameter set is a value with a version.** Every weight, floor, and
   need profile carries an identity and a version. `quality` and `fit` are pure
   functions of `(params, …)`, so the same inventory can be scored under two
   parameter sets and the two results compared. No score is ever reported
   without the parameter set that produced it.
2. **A need profile declares its provenance.** A profile is either _derived_
   (computed from the game's damage model or a documented role) or _authored_ (a
   human's judgment). The distinction is carried in the value and shown in the
   UI. A recommendation whose basis is someone's opinion must not look like one
   whose basis is arithmetic.
3. **Pass 1 prunes pass 2.** `match` does not consider a Soul against a need
   when that Soul cannot reach the need's floors. The failure of a single-number
   model here is not that it scores everything — it is that it has nothing to
   prune _with_.
4. **Pass 2 may invert pass 1, and that is the output.** A Soul with a low
   `QualityScore` and a high `AffinityScore` for some need is the single most
   valuable row this system produces. The UI must be able to reach it directly,
   and it is the reason ADR-0001's "hypotheticals are evaluated in memory"
   matters: the inversion is computed, not stored (see ADR-0002 and
   [overview.md](../architecture/overview.md)).

## Alternatives

- **One pass with a single configurable weight vector.** Rejected: it is the
  single-number design with a settings screen. A user can retune it for one
  Shikigami, but they cannot see affinity for all of them at once, and every
  retune invalidates their previous reading of the list. It also cannot produce
  the inversion in property 4 at all — a scalar ordering cannot contain it.
- **One pass, but scored per Shikigami from the start** (the inventory is stored
  once per Shikigami role and everything is target-relative). Rejected: it
  destroys the target-independent question, which is real and asked constantly
  ("is this Soul worth keeping at all, with no particular Shikigami in mind"),
  and it multiplies storage by the number of needs. The two passes are cheap;
  the second one is defined by the first.
- **Affinity as a scoring parameter set rather than a subsystem** (the same
  function, a different `params` passed in). Rejected on the module boundary:
  the parameterized-quality version is the alternative above wearing a uniform.
  `fit` has a genuinely different signature and a genuinely different input
  (`NeedProfile`), and pretending otherwise puts the matching algorithm inside
  the module that also owns the single-number score.
- **Solve it in the UI** (compute affinity in Dart from inventory data the core
  sends up). Rejected: it is exactly the "truth in the UI layer" anti-pattern
  the architecture forbids, and it would make the most interesting logic in the
  product the only logic with no Rust test.

## Consequences

**Easier.** The interesting question becomes expressible, and answerable in a
way that is testable without a UI: `match` is a pure function, so a fixture of
ten Souls and three needs has an expected result that a test can assert. Weight
changes become parameter versions rather than edits, so a user's historical
recommendations remain interpretable. The "low quality, high affinity" inversion
is a query over computed values rather than a feature that has to be built
specially.

**Harder.** Need profiles must be authored or derived, and that is real work
with real judgment in it — the scoring system is only as good as the needs fed
to it, and the project has no needs yet. The matching pass has a combinatorial
core (an assignment under a one-Soul-one-slot constraint), so it needs a
complexity bound and a property test for pruning correctness, on the same
evidence standard as any other pure function. And the UI has two scores per Soul
to present without confusing the user about which question each answers — that
is a presentation problem this record creates and does not solve.

**Where the details live.** The weights, the need-profile authoring workflow,
the matching algorithm, and the complexity bound are specification, not
architecture: [scoring.md](../spec/scoring.md). ADR-0002's "derived results are
not stored" is the persistence half of this decision.
