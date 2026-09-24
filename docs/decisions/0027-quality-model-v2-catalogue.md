---
id: ADR-0027
status: accepted
date: 2026-09-25
area: scoring
supersedes: []
superseded-by: []
related: [ADR-0024, ADR-0025, ADR-0023, ADR-0003]
---

# ADR-0027: Quality Model v2 judges souls against output, hit, healing, and a gated speed archetype

## Status

Accepted, 2026-09-25. Amends ADR-0024's catalogue and the parts of it that
assumed every archetype holds four attributes. ADR-0025's tiers stand for the
four-attribute archetypes. The model becomes `yata-quality-v2`. The definitions
are in [spec/quality-model.md](../spec/quality-model.md); the derivation and the
numbers are in `papers/quality-model-v2/paper.md`.

## Context

The v1 catalogue was `output`, `hit` and `resist`, each of four attributes with
weight 1. It followed the shape of the catalogue rather than the souls players
keep, in three ways.

- **`resist` was there for symmetry.** `{Spd, EffectRes, HpPercent, DefPercent}`
  mirrors `hit` one class member apart, which made its scale provably equal to
  `hit`'s. That is convenient. But a resist-oriented build is not a
  general-purpose quality pattern of the weight of the others. EffectRes matters
  for particular Shikigami, sets and needs, which is pass 2's question
  (ADR-0003, PRP-0003).
- **No archetype described healing.** HP-based healers that can crit value
  `HpPercent`, `Crit`, `CritDmg` and `Spd`. `output` values `AtkPercent` in
  `HpPercent`'s place, so a healing soul scored as a mediocre output soul.
- **A pure Speed soul had no reading of its own.** A soul whose value is one
  long Speed line was judged only as a four-attribute soul, where one line
  reaches at most six of nine roll units. Players keep such souls for their
  Speed alone.

The earlier requirement that every archetype hold four attributes made
archetypes comparable only by construction. The comparability actually comes
from each archetype's own anchors: 0 at no useful value, 50 at the archetype's
own expected soul, 100 at its own attainable maximum.

## Decision

1. **The catalogue is `output`, `hit`, `healing` and `speed`.** All weights stay
   1; no weight is calibrated in this version.
   - `output`: `{AtkPercent, Crit, CritDmg, Spd}`, unchanged.
   - `hit`: `{EffectHit, Spd, HpPercent, DefPercent}`, unchanged.
   - `healing`: `{HpPercent, Crit, CritDmg, Spd}`. It is `output` with
     `AtkPercent` and `HpPercent` exchanged.
   - `speed`: `{Spd}`, one attribute. Its utility is `U_speed = e_Spd`.
   - `resist` is removed. EffectRes is useful to no archetype of pass 1.
2. **Archetypes may differ in size.** An archetype's utility is the sum of `e_a`
   over its own useful set, and its maximum is its own: `M = 9` for the
   four-attribute archetypes and `M_speed = 6` for `speed`, the most one line
   can hold. Its expected utility is `E = μ · E[K]` for its own set under the
   unchanged reference measure. Cross-archetype comparability is provided by
   archetype-specific normalization, not by forcing every archetype to contain
   the same number of equally useful attributes.
3. **An archetype may have an eligibility predicate.** A soul is judged by the
   archetypes that accept its main attribute and whose predicate holds, its
   _candidates_. `speed` is eligible exactly when `e_Spd ≥ 4`, four roll units,
   which is 12 displayed Speed at six stars. The gate reads the line's value,
   never the number of increments inferred from it. The other archetypes are
   always eligible. Below the gate `speed` is not a candidate; it is not scored
   low. The four-roll-unit gate is Yata's design choice, not a game rule.
4. **The gate is not a condition on the reference.** `E_speed` is computed over
   all +15 souls. The gate decides when the speed reading is shown; the
   normalization describes the Speed dimension. Conditioning the reference on
   the gate would put 60% of eligible souls below 50, calling a soul in the top
   quarter percent of Speed below the expected soul.
5. **`speed` has its own tier rule:** an eligible soul is SSR, and UR when
   `e_Spd > 5 = M_speed − 1`, the same frontier principle as the four-attribute
   UR. It has no N, R or SR, because the gate lies above them. It has no SP: the
   generic specialization rule, some useful line above five roll units, is
   exactly the UR condition for a one-line archetype, so an SP rule would never
   fire. The four-attribute ladder of ADR-0024 and ADR-0025 is unchanged for
   `output`, `hit` and `healing`.
6. **Main attributes.** `healing` accepts `Spd` and `HpPercent` on slot 2,
   `HpPercent` on slot 4, and `Crit`, `CritDmg` and `HpPercent` on slot 6.
   `speed` accepts the mains of roles that want Speed with survival or control:
   `Spd`, `HpPercent` and `DefPercent` on slot 2; `EffectHit`, `EffectRes`,
   `HpPercent` and `DefPercent` on slot 4; `HpPercent` and `DefPercent` on
   slot 6. Offensive mains belong to `output`, whose useful set holds Speed
   already. Slots 1, 3 and 5 are accepted by every archetype. The main attribute
   still adds nothing to utility; a Speed main does not count toward `U_speed`.
7. **Unrated souls.** Every legal main is accepted by some archetype, but a slot
   4 `EffectRes` soul whose Speed is below the gate has no candidate. Such a
   soul is _unrated_ in pass 1: no score and no tier, and its value is pass 2's
   question. Every other legal soul has a candidate. The v1 requirement that
   every soul receive a quality score was a consequence of `resist`; it is not
   kept by re-adding it.
8. **Explanations.** `depth` is `100 · max e_a / 6`, which for `speed` is
   `100 · e_Spd / 6`. `breadth` is not applicable to a one-attribute archetype
   and is omitted, not shown as 0 or 100. `growth` is the existing exact
   recursion over the archetype's own useful set.
9. **The model version is `yata-quality-v2`.** The catalogue changed, and
   ADR-0024 makes that a new version.

## Alternatives

- **Keep `resist`.** Rejected: kept only for symmetry, and it answers a pass-2
  question in pass 1.
- **`healing` with `DefPercent`.** Rejected: added only to keep the old shape.
- **`speed` with filler attributes to reach four.** Rejected: the filler would
  put unrelated value into a Speed reading.
- **A gate by increments, "Speed took four rolls".** Rejected: four low
  increments would outrank fewer high ones on history rather than value. Under
  the published range the two nearly coincide, since five increments always
  reach four roll units and four reach it only at maximum, but the rule is
  stated on values.
- **Condition `speed`'s reference on the gate.** Rejected; see decision 4.
- **The four-attribute ladder for `speed`.** Its SSR milestone, six, is
  `speed`'s maximum, and its UR boundary, eight, is unreachable; eligible souls
  would be R or SR and then jump to nothing. The generic SP rule coincides with
  UR.
- **A second, invented condition to give `speed` an SP.** Rejected: an SP with
  no meaning of its own.
- **Weights now: EffectHit and Speed heavier in `hit`.** Deferred. The
  maintainer expects them to weigh more in a future weighted model. That is a
  separate calibration problem, and this version isolates the catalogue from it.

## Consequences

**Easier.** Each archetype is the quality pattern it names. `healing` and
`output` share one scale exactly, because `AtkPercent` and `HpPercent` have the
same official weight (Lean: `healing_is_output_relabelled`; calibration:
asserted). A pure Speed soul has a reading of its own, shown only when it means
something.

**Harder.** `hit` no longer has a partner of identical scale. Its anchors differ
from `output`'s in the second decimal, as before. The catalogue has two tier
rules. A `speed` UR can score as low as `g_speed(5) ≈ 90.6`, below the
four-attribute UR boundary of about 92.2. The exposed order is tier first, so it
still ranks above every SSR and SP. Some souls are unrated, and the application
must say so rather than show a number.
