# Quality Model v1 is defined, proved, and calibrated

- Date: 2026-09-24
- Area: scoring
- Affected: developers, maintainers
- Related: ADR-0023, ADR-0024

## What changed

- `docs/spec/quality-model.md` defines pass 1 exactly: three archetypes, values
  in roll units, a score anchored at 0, the expected soul, and the attainable
  maximum, and the tiers N to UR with their precedence. `scoring.md` defers to
  it.
- `formal/lean/` proves the model's structural properties in Lean 4 with
  Mathlib; `formal/calibration/` recomputes every number of the spec and the
  paper and fails preflight when a committed number differs.
- `papers/quality-model-v1/paper.md` explains the model, the rejected
  candidates, and the limits.
- Preflight gains `lean-build` and `quality-calibration`, in the `full` and the
  new `formal-ci` profiles; CI gains the `formal` job.

## Compatibility and migration

Nothing in the product changes; `yata-core` does not score yet. Contributors who
run `just check` with Lean installed also build the proofs; without Lean the
check is skipped.

## Evidence

[evidence/testing.md](../../evidence/testing.md), "Quality model".
