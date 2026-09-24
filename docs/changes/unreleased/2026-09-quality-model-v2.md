# Quality Model v2: output, hit, healing, and a gated speed archetype

- Date: 2026-09-25
- Area: scoring
- Affected: developers, maintainers
- Related: ADR-0024, ADR-0025, ADR-0027

## What changed

- The catalogue is `output`, `hit`, `healing` and `speed`. `resist` is removed:
  EffectRes is useful to no pass-1 archetype.
- `healing` values `HpPercent`, `Crit`, `CritDmg` and `Spd`. It is `output` with
  `AtkPercent` and `HpPercent` exchanged, and shares `output`'s anchors and tier
  rates exactly.
- `speed` values `Spd` alone and is a candidate only when the Speed line holds
  at least four roll units, 12 displayed Speed; below that it is not scored at
  all. It is SSR from the gate and UR above five roll units, with no N, R, SR or
  SP. Its maximum is six roll units, and its expected utility is taken over all
  +15 souls. About one +15 soul in four hundred is eligible.
- A slot 4 EffectRes soul whose Speed is below the gate has no candidate
  archetype and is unrated: no score and no tier.
- `breadth` is omitted for `speed`, not shown as 0 or 100.
- `output`'s and `hit`'s scores and the four-attribute ladder are those of v1.1.
  A soul's quality can still change: V01 lost `resist` and scores 0; V11 is now
  UR under `speed`. Vectors V18 to V30 are new.
- The model identifier is `yata-quality-v2`. Weights stay equal; weighting is a
  separate, later problem.
- Lean has archetypes of any size, an eligibility predicate, and `speed`'s own
  bounds and tier rule; the calibration computes `speed`'s eligibility rate and
  its distribution.

## Compatibility and migration

No scorer is implemented yet, so nothing stored changes. Anything that quoted a
v1.1 result must name its version. `papers/quality-model-v2/paper.md` explains
v2; `papers/archive/quality-model-v1/paper.md` is kept, frozen, as the record of
v1.1.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Quality model".
