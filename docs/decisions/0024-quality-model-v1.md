---
id: ADR-0024
status: accepted
date: 2026-09-24
area: scoring
supersedes: []
superseded-by: []
related: [ADR-0003, ADR-0023]
---

# ADR-0024: Quality Model v1 scores six-star souls against equal-weight archetypes, anchored at zero, the expected soul, and the attainable maximum

## Status

Accepted, 2026-09-24. The definitions are in
[spec/quality-model.md](../spec/quality-model.md); the derivation, the numbers,
and the rejected candidates are in `papers/quality-model-v1/paper.md`.

## Context

ADR-0003 asks for a target-independent quality score, pass 1, before any need
profile is applied. Four facts shape it:

- **Value depends on use.** Effect HIT means nothing to a damage dealer, and
  Crit nothing to a healer. A single weight vector for every soul is the legacy
  averaging failure (ADR-0003).
- **Rarity is not value.** A score that rewards improbable outcomes as such
  would rank a speed soul above a crit soul of equal worth whenever speed
  happens to be rarer.
- **The mechanics are sourced only for six stars,** and even there the law of
  one increment within its range is open (`spec/soul-mechanics.md`).
- **Players read tiers.** The UI shows N < R < SR < SSR < SP < UR, and a tier
  must mean what its definition says, including that no lower tier outranks UR.

## Decision

1. **Domain.** v1 scores six-star souls only. Lower stars are not scored.
2. **Archetypes, not one weight vector.** The quality profiles are a small
   published catalogue: v1 has three, 输出, 命中 and 抵抗. Each values four
   sub-attributes with weight 1 and accepts a stated set of main attributes per
   slot. A soul's quality is its best result over the archetypes that accept its
   main attribute. The main attribute gates; it adds no points.
3. **Utility in roll units.** A sub-attribute contributes its value divided by
   the largest single increment, `e = S / hi`. An archetype's utility is the sum
   of its four `e`.
4. **Normalization anchored at three points.** 0 is no useful value, 50 is the
   expected utility of a +15 soul under the reference measure, and 100 is the
   attainable maximum. The score is linear between them. The expected utility
   depends on the increment's law only through its mean.
5. **The reference measure is the official one**: the notice's class weights
   36/36/28, shared equally within a class, as `spec/soul-mechanics.md` states.
   Each archetype's anchors are computed under it, so each archetype's 50 is its
   own expected soul. Archetypes that differ only within a class have identical
   scales; the others differ by a reported residual.
6. **Tiers.** N to SSR are bands at utility milestones: the expected soul, and
   five and seven useful increments at mean value. SP is a specialized soul,
   with more than five roll units on one useful line, at or above the SSR floor.
   UR is utility within one roll unit of the maximum. Precedence is UR, then SP,
   then the band. The exposed order is tier first, then score.
7. **Probabilities are never scored.** The probability of an equal or better
   soul is an explanation, not part of the score; it depends on the whole
   increment law, so v1's `QualityScore` does not carry it.

## Alternatives

Each is set out with numbers in the paper.

- **Percentile of utility.** Exactly fair across profiles by construction, but
  it compresses the whole range from SSR to UR into about one and a half points,
  so neither the display nor UR can separate the best souls.
- **Utility over the maximum alone.** The expected soul scores under 30, and a
  one-attribute profile's scores are a third of a four-attribute profile's.
- **Utility over the expectation alone.** Unbounded; a quarter of speed-only
  souls saturate at 100.
- **Utility plus a rarity bonus.** Rejected outright: rarity is not value.
- **The main attribute's value in utility.** A main is worth up to 22 roll units
  against the sub-attributes' 9, so the main would decide quality, and by its
  magnitude rather than its use. It is a need profile's concern, pass 2.
- **UR as distance to the Pareto frontier.** With unequal weights, a frontier
  point can have less utility than a non-frontier soul, so an SP could outscore
  a UR.
- **SP as "one attribute took every roll"** (the earlier UI draft). One perfect
  line beside three useless ones would be SP while scoring below SSR.

## Consequences

**Easier.** Every threshold is a statement in roll units, and changes to the
anchors, the catalogue, or the thresholds are a new model version. The
structural claims are proved in Lean (ADR-0023). The expected soul scores 50
under every archetype, by construction.

**Harder.** The anchors are exact fractions with large denominators, computed
rather than written down; the implementation takes them from the calibration's
output. The catalogue is authored and must be reviewed. Profiles with fewer
useful attributes are not on the same scale in tail rates, which is why they
belong to pass 2, where a score is comparable only within one need. SP is rarer
than UR under the reference measure; the ladder orders quality, not rarity.

## Amendment 2026-09-25

The SSR edge is six roll units, not `7μ`, and SP keeps `7μ` as a quality floor
of its own ([ADR-0025](0025-ssr-at-one-perfect-line.md)). Under this record a
soul with one perfect useful line and nothing useful beside it was SR; it is now
SSR, and still not SP. The model is `yata-quality-v1.1`. Everything else in this
record stands.
