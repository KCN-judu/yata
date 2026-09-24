---
id: ADR-0023
status: accepted
date: 2026-09-24
area: process
supersedes: []
superseded-by: []
related: [ADR-0001, ADR-0003, ADR-0013, ADR-0016]
---

# ADR-0023: A public scoring standard is proved in Lean, reproduced by a calibration program, and explained in a paper, all outside the product

## Status

Accepted, 2026-09-24.

## Context

The quality score (ADR-0003) is shown to players as a number and a tier. A
public standard needs three kinds of support, and none of them is product code:

- **Structural properties.** Monotonicity, dominance, the bounds, and tier
  precedence must hold for every soul the rules allow, not only for the ones a
  test happens to pick. That calls for proofs.
- **Numbers.** The anchors, the thresholds, the tier rates, and the test vectors
  are derived from the game's rules. A reader must be able to recompute each
  published number, and a changed number must not slip into a document by hand.
- **Explanation.** Players and programmers need the derivation, the rejected
  alternatives, and the limits in one readable place. A spec page is normative
  and edited in place, so it cannot also be a versioned argument.

Two existing rules bound the choice. ADR-0013 keeps shipped code in Rust and
Dart, and keeps project logic out of Python. ADR-0016 keeps research material
local, while a public standard must be public.

## Decision

1. **`formal/lean/`: Lean 4 proofs.** The Lean project checks the structural
   theorems of each public scoring standard. Its toolchain and Mathlib version
   are pinned in `lean-toolchain` and `lakefile.toml`. It proves consequences of
   stated definitions; it says nothing about whether the game follows them.
2. **`formal/calibration/`: the numbers.** A standalone Rust program, outside
   the product workspace, recomputes every published number of a standard:
   exactly with rationals where the model allows it, and by seeded Monte Carlo
   with reported standard errors where it does not. It writes `out/` and fills
   the `<!-- generated:NAME -->` blocks of the documents that quote its numbers.
   Its `check` mode fails when any committed number differs.
3. **`papers/<name>/`: public manuscripts.** A paper explains one versioned
   standard: its derivation, alternatives, and limits. It is explanatory. The
   normative definitions live in `docs/spec/`, and a paper cites them and their
   version rather than restating them differently.
4. **Nothing in the product depends on these.** No crate, the application, or
   the reader imports, builds, or runs `formal/` or `papers/`. Lean is not a
   shipped language, so ADR-0013's table of shipped languages is unchanged. The
   calibration program is Rust, so ADR-0013's Python boundary is untouched.
5. **Checked like everything else.** `scripts/preflight.py` has two checks:
   `lean-build` builds every theorem, and `quality-calibration` lints the
   program and runs its `check`. Both are in the `full` profile and in the
   `formal-ci` profile, which the CI job `formal` runs. A host without Lean
   skips `lean-build`, as it skips the Flutter checks without Flutter.
6. **Evidence stays where it is.** A paper cites local research by path in a
   code span, as the docs do (ADR-0016). Anything a paper publishes from
   `research/` is published by that copy and must be publishable.

## Alternatives

- **Property tests in the product crate instead of proofs.** Rejected as the
  only support: property tests sample, while the tier precedence and dominance
  claims are universal. The production scorer will still carry the test vectors
  as golden tests.
- **Coq or Isabelle.** Not chosen: Lean 4 with Mathlib has the rational
  arithmetic, finite sums, and decision procedures these proofs need, and a
  prebuilt library cache that CI can download.
- **Calibration in Python or a notebook.** Rejected by ADR-0013: it is project
  logic, not glue.
- **The paper under `docs/spec/`.** Rejected: a spec is edited in place and is
  normative, while a paper argues for one version of a standard and is not
  rewritten when the next version comes.
- **The calibration program as a workspace crate.** Rejected: it would join the
  product's crate graph and lint profile for code the product never runs.

## Consequences

**Easier.** Every published number can be recomputed with one command, and a
changed number fails preflight until the documents are regenerated. The
standard's structural claims hold for all souls, not a sample.

**Harder.** Contributors who touch the standard need Lean and Mathlib, about
five gigabytes of cached build. The `formal` CI job downloads that cache. A
definition changed in the spec must be changed in the Lean files and in the
calibration program in the same change, or preflight fails.
