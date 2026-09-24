# Quality Model v1.1: SSR begins at one perfect line

- Date: 2026-09-25
- Area: scoring
- Affected: developers, maintainers
- Related: ADR-0024, ADR-0025

## What changed

- The SSR edge is six roll units, the most useful value one line can hold,
  instead of `7μ = 6.3`. SP keeps `7μ` as a quality floor of its own.
- A soul with one perfect useful line and nothing useful beside it is SSR
  instead of SR, and still not SP (test vector V04). V07 moves from SR to SSR.
  No other test vector changes.
- Under the reference measure and a uniform increment law, SSR or better goes
  from about 2.1% to about 3.3% of +15 souls; SP and UR are unchanged.
- The model identifier is `yata-quality-v1.1`. Scores are the same as v1's.
- Lean separates the SSR and SP floors and proves that a perfect single line is
  exactly SSR; the calibration checks its milestones against the Lean
  definitions.

## Compatibility and migration

No scorer is implemented yet, so nothing stored changes. Anything that quoted a
v1 tier must name its version: a v1 tier and a v1.1 tier of the same soul differ
when `6 ≤ U < 6.3`.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Quality model".
