---
id: ADR-0025
status: accepted
date: 2026-09-25
area: scoring
supersedes: []
superseded-by: []
related: [ADR-0024, ADR-0023]
---

# ADR-0025: Quality Model v1.1 starts SSR at one perfect line and gives SP a floor of its own

## Status

Accepted, 2026-09-25. Amends ADR-0024's tier decision; everything else in
ADR-0024 stands. The model becomes `yata-quality-v1.1`.

## Context

ADR-0024 set the SSR edge at seven useful increments at mean value,
`u_SSR = 7μ = 6.3` roll units, and used the same threshold as SP's quality
floor. That is consistent, but it gives the ladder a meaning players would not
expect. A soul with six maximum increments on one useful line and nothing useful
beside it has `U = 6`. It is the best one line can be, and under v1 it was SR,
because `6 < 6.3`. Test vector V04 is exactly this soul.

One threshold was also doing two jobs. The SSR edge asks whether a soul reaches
an exceptional overall quality. The SP floor asks whether an already exceptional
soul is also strong enough overall to carry the SP mark. Nothing requires those
two answers to change at the same point.

Six roll units have a structural meaning. One line holds at most six increments,
each at most one roll unit, so six is the most useful value one line can carry.
A soul above six has useful value on at least two lines (Lean:
`beyond_six_needs_another_line`).

The calibration compared four SSR edges. SP's floor stayed at 7μ and UR's
boundary at 8 throughout. Rates are for `output` under the uniform increment
law; `papers/quality-model-v1/paper.md`, § 7, has every archetype and law.

| Candidate                                      | `u_SSR` | SSR or better |
| ---------------------------------------------- | ------- | ------------- |
| A. v1: `7μ`                                    | 6.3     | 2.13%         |
| B. one perfect line                            | 6       | 3.33%         |
| C. `6μ`                                        | 5.4     | 6.36%         |
| D. the edge where SSR or better is 5% of souls | 5.488   | 5.01%         |

## Decision

1. **SSR begins at six roll units**, `u_SSR = 6`: the most useful value one
   perfect line can hold. The SSR score edge of archetype `p` is
   `c_SSR,p = g_p(6)`.
2. **SP keeps a floor of its own**, `u_SP_floor = 7μ = 6.3`, above the SSR edge.
   SP is a specialized soul whose score is at least `c_SP,p = g_p(7μ)`. The
   specialization predicate is unchanged: some useful line holds more than five
   roll units.
3. **The milestones are ordered** `u_SR < u_SSR < u_SP_floor < u_UR`: 4.5, 6,
   6.3, 8.
4. **Consequences for single lines.** A soul whose only useful value is one
   perfect line is exactly SSR: at least SSR by the line, not SP because
   `6 < 6.3`, and not UR (Lean: `perfect_single_line_is_SSR`).
5. **The model version is `yata-quality-v1.1`.** ADR-0024 makes any threshold
   change a new version. v1.1 is the smallest step, because scores are
   unchanged; only tiers differ, and only for souls with `6 ≤ U < 6.3`.
6. **Unchanged:** the domain, the catalogue, utility in roll units, the anchored
   normalization and its anchors, the SR, SP and UR milestones, the
   specialization predicate, the precedence UR, then SP, then the band, and the
   exposed order, tier first, then score.

## Alternatives

- **Keep `7μ` (A).** Rejected: the best possible single line stays SR.
- **`6μ = 5.4` (C).** Rejected: its edge falls in the middle of what six
  increments at mean value produce. So its rate depends most on the unknown
  increment law (6.4–7.3% across the three laws, against 3.30–3.33% for B), and
  it admits souls whose best line is well below perfect.
- **A percentile edge (D).** Rejected: the 5% target is a choice with no
  structural meaning, and the edge moves with the assumed increment law. The
  tiers stay utility milestones (ADR-0024).
- **Lower the SP floor with SSR.** Rejected: SP would then include a perfect
  line with nothing useful beside it, the case ADR-0024 excluded.

## Consequences

**Easier.** SSR has a one-sentence meaning: at least the best one line can be.
Its rate barely depends on the increment law. SSR admits only souls with a
seventh useful increment, or exactly six useful increments all at maximum, and
so the rate follows the hit law, which is exact.

**Harder.** The ladder has two thresholds between SR and UR, and the docs must
say which question each answers. An SSR soul can outscore an SP soul by up to
`t_UR − c_SP`, as before, because the exposed order is tier first.
