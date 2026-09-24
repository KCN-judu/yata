---
kind: spec
status: current
area: scoring
---

# Scoring

What the two scoring passes mean, what they take, what they return, and what
makes two results comparable. The architectural shape is ADR-0003; this page is
the definitions.

Nothing on this page is _implemented_. The functions are _designed_: the
signatures and the comparability rules below are what the implementation must
satisfy. Pass 1 is defined exactly by [quality-model.md](quality-model.md); pass
2's need profiles are not yet authored.

## The three functions

```text
quality : (params: ParamSet, soul: Soul)                    -> QualityScore
fit     : (params: ParamSet, need: NeedProfile, soul: Soul) -> AffinityScore
match   : (params: ParamSet, inv: Inventory, needs: [NeedProfile]) -> MatchingResult
```

All three are total, pure functions of their arguments (ADR-0001). None reads a
clock, a file, a random source, or a global. None has a side effect. Given the
same arguments they return the same value, which is what makes every result on
this page reproducible from a fixture.

`params` is not optional and not implicit. There is no "current weights" global;
a score without its parameter set is not a value this system produces.

## Parameter set

A `ParamSet` carries an identity, a version, and everything the three functions
read:

| Field           | Meaning                                                                                                                                   |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `id`            | a stable identifier for this set (`legacy-compatible`, `speed-farming`, …)                                                                |
| `version`       | bumped whenever any weight or floor in the set changes                                                                                    |
| `quality_model` | the pass-1 model and its parameters: `yata-quality-v1`, whose catalogue, anchors, and thresholds are [quality-model.md](quality-model.md) |
| `need_profiles` | the profiles `fit` and `match` score against                                                                                              |
| `quality_floor` | the pass-1 threshold below which `match` will not consider a soul, per need                                                               |

Two results are **comparable** only if produced by an equal `(id, version)`. The
system never merges results across parameter versions, and never displays a
stored score without its parameter set identity. A user switching parameter sets
sees a re-computation, not a re-sort of previously computed numbers, because the
previously computed numbers are not the same numbers.

`quality_floor` existing per need rather than globally is deliberate: "too weak
to bother" is a property of a need, not of a soul.

## Pass 1 — `QualityScore`

Answers: _is this Soul good in itself, with no holder in mind?_ The input
contains no Shikigami. The exact definition is
[quality-model.md](quality-model.md) (ADR-0024); this section states only what
pass 2 relies on.

| Field       | Meaning                                                                                         |
| ----------- | ----------------------------------------------------------------------------------------------- |
| `total`     | 0–100, the single comparable number, for sorting and thresholds                                 |
| `tier`      | N, R, SR, SSR, SP or UR                                                                         |
| `archetype` | the best-fit archetype of the published catalogue, and the score under every accepted archetype |
| `depth`     | the deepest useful line against its attainable maximum; explanatory                             |
| `breadth`   | how many useful lines carry the utility; explanatory                                            |
| `slot_fit`  | which archetypes accept the soul's main attribute                                               |
| `growth`    | below +15, the score of the expected +15 soul                                                   |

What pass 2 may rely on:

- **Values are relative to a roll, not absolute.** A sub-attribute counts as its
  value over its largest single increment, so HP flat, which rolls in the
  hundreds, does not outweigh Crit. The legacy model had this defect.
- **`depth` and `breadth` explain `total`; they are never added into it.** Which
  shape is better depends on the holder, and that is pass 2's question.
- **The main attribute gates, it does not score.** `slot_fit` states which
  archetypes can use the main attribute. A main attribute illegal for its slot
  is a decode error, never a low score.
- **`total` is monotone.** A soul no worse on every useful sub-attribute never
  scores lower, and never takes a lower tier.

## Pass 2 — `NeedProfile` and `AffinityScore`

A `NeedProfile` is one Shikigami's, or one role's, stated requirement.

| Field             | Meaning                                                                                                                                      |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`, `name`      | identifier and display name                                                                                                                  |
| `provenance`      | `derived` or `authored` — **required**, and shown in the UI                                                                                  |
| `source`          | for `derived`, what it was derived from; for `authored`, who wrote it and when                                                               |
| `sets`            | required or preferred `SoulSet`s, and how much they matter (a 4-piece is a stronger constraint than a 2-piece)                               |
| `main_attributes` | per `SoulSlot`, the acceptable main attributes, ranked                                                                                       |
| `desired`         | per `SoulAttribute`, how much this need wants it — the target's weight vector, distinct from the generic one                                 |
| `floors`          | per `SoulAttribute`, a minimum below which the soul is disqualified for this need                                                            |
| `caps`            | per `SoulAttribute`, a value above which extra is wasted (a need that only cares about surviving does not reward the 40th point of Crit DMG) |

`floors` and `caps` are what make this pass different in kind from pass 1 rather
than in degree. A floor is disqualifying and a cap means marginal value goes to
zero; a weighted sum has neither, and their absence is why a single-pass model
cannot express "this soul is exactly what this Shikigami needs and nothing
else".

`AffinityScore` reports:

| Field       | Meaning                                                                                        |
| ----------- | ---------------------------------------------------------------------------------------------- |
| `value`     | 0–100, comparable only within one `(need, params)` pair                                        |
| `qualifies` | whether every floor is met; a non-qualifying score is reported but never ranked as a candidate |
| `blocking`  | which floors are unmet, and by how much — the actionable part                                  |
| `wasted`    | value above caps, or main attributes this need cannot use                                      |
| `missing`   | requirements not satisfiable from this soul, with the best available shortfall                 |

`blocking` and `missing` are the fields that answer "why is this not good
enough", and they are structured values, not a serialized explanation string.
Reasoning stored as text is the counterexample: it can be displayed and nothing
else.

## Matching — `match`

`match` takes the whole inventory and every need, and produces candidate
assignments.

```text
MatchingResult {
  per_need: [
    { need_id
    , candidates: [ { soul_id, affinity, delta } ]   // ranked, qualifying only
    , current:    Option<LoadoutRef>                 // what the account has equipped now
    , best:       Option<Loadout>                    // the top-six assignment under the constraint
    , delta_vs_current: Option<DeltaSet>
    }
  ]
  inversions: [ { soul_id, quality, need_id, affinity } ]   // low quality, high affinity
}
```

Four rules:

**The assignment constraint is real.** A Soul is equipped once. `best` is the
best qualifying assignment of distinct souls to the six slots, not the six
highest individual affinities — a soul cannot be its own substitute, and the
distinction is exactly where a naive implementation produces a loadout the user
cannot build.

**Pruning is by `quality_floor`, and it is correctness-relevant, not an
optimization.** No soul whose `quality.total` is below a need's floor can appear
in that need's candidates. A pruning bug is a scoring bug, so it gets a property
test: pruning must not change the result, only the time taken.

**`inversions` is a first-class output, not a report.** It is the list of souls
whose `quality.total` is low and whose best `affinity` is high, sorted by the
size of the gap. This is the feature ADR-0003 exists for — it is the answer to
"which of my bad souls are actually good". It is computed, never stored, and it
is the one view in the product that cannot be produced by any single-pass
scoring model.

**`delta_vs_current` says what would change, not what is good.** The
application's job is to tell the user what to do differently, which requires
knowing what they have now. This is what makes `Loadout` (a domain value)
distinct from `Inventory` (a fact).

## Worked example

The shape of the inversion, with arbitrary numbers rather than `yata-quality-v1`
scores, for the test suite to instantiate:

```text
soul A   slot 6, Crit DMG main, sub: Crit +18, Atk% +5, Def +30, HP +100
         quality.total 62    — concentrated and strong
         affinity("single-target burst") 71   qualifies
         affinity("speed control")        9    does not qualify (needs Spd main on slot 2)

soul B   slot 2, Spd main, sub: Effect HIT +22, Spd +11, Atk% +3, Def% +2
         quality.total 41    — low: raw attribute values are small, breadth is poor
         affinity("speed control")        88   qualifies
         affinity("single-target burst")  17   qualifies, ranks last

inversions: [ { B, quality 41, need "speed control", affinity 88 } ]
```

Soul B is the row the legacy application could not produce. Under a single
generic score it ranks below average and a threshold discards it; under two
passes it is the best answer to a specific question, and it appears in
`inversions` because that is precisely what it is.

## Provenance and honesty rules

These are presentation constraints with the same weight as the arithmetic,
because a score the user cannot interpret is worse than no score:

- A score is always shown with its `ParamSet` identity and version.
- An `authored` need profile is labeled as authored. A recommendation resting on
  a judgment call must not look like one resting on the game's arithmetic.
- `qualifies: false` results are never displayed as recommendations, only as
  explanations (this is what `blocking` is for).
- No score is ever displayed for a soul that failed to decode. A decode failure
  is reported as a decode failure; inventing a low score for unreadable data is
  how the legacy application produced confidently wrong discard advice.

## Not decided here

- the minimum evidence for changing the quality model's parameters
- how `derived` need profiles are computed from the game's damage model
- how `authored` profiles are created, reviewed, and versioned
- the matching algorithm and its complexity bound

Each is a future entry on this page or a decision record, not an architectural
gap.

## Related

- The premise:
  [ADR-0001](../decisions/0001-pure-core-effects-at-the-boundary.md)
- Two passes, and why one cannot work:
  [ADR-0003](../decisions/0003-two-pass-scoring.md)
- Where these crates sit:
  [../architecture/overview.md](../architecture/overview.md)
- Slot and attribute facts: [glossary.md](glossary.md)
