---
id: ADR-0028
status: accepted
date: 2026-09-25
area: scoring
supersedes: []
superseded-by: []
related: [ADR-0003, ADR-0001, ADR-0023, ADR-0024, ADR-0027]
---

# ADR-0028: Pass 2 evaluates one soul, one loadout, and one swap; hard requirements gate and never score

## Status

Accepted, 2026-09-25, from PRP-0004, which takes up PRP-0003's direction. It
amends ADR-0003 property 3 (an appended amendment there), and it answers the
question ADR-0027 left to pass 2: how an unrated soul is treated. The
definitions are in [spec/scoring.md](../spec/scoring.md); the rationale, the
worked examples and the handoff are in PRP-0004.

## Context

ADR-0003 fixed the two passes and the signatures of `fit` and `match`, but the
pass-2 fields of `scoring.md` could not be implemented as written:

- Floors and caps were stated per soul. What a player puts a floor or a cap on —
  a Shikigami's Effect HIT, Crit, or Speed — is a sum over six souls, their set
  effects, and the Shikigami's own base. "Every floor is met" has no meaning for
  one soul.
- `match` was pruned by `quality.total`. Pass 1 does not bound pass 2: a soul
  below its expected quality can be the best soul for one need. Pruning by
  quality removed the inversions ADR-0003 property 4 calls the most valuable
  output.
- A soul's set carried a weight, as if one soul could complete a four-piece set.
- Nothing separated a disqualifying mismatch from a lower preference.
- The comparison with what a Shikigami wears, which the feature scope makes the
  usual way into matching, had no named inputs.

Two facts arrived with this decision. Quality Model v2 (ADR-0027) leaves some
souls unrated. And the maintainer decided that set use, a set's usual use read
from its effect text, only ever adds keep suggestions.

## Decision

1. **Three levels.** A constraint is evaluated at the smallest level that can
   decide it, and at no smaller one. `fit(params, need, soul)` decides what one
   soul decides: its slot's main attribute, whether its set has a place in the
   need's set plans, per-soul floors, and its affinity.
   `evaluate(params, need, loadout, baseline?)` decides what six souls decide:
   set plans, stat floors, ceilings and windows, saturation, and a loadout's
   value. `swap` is two evaluations and a difference: the comparison with what a
   Shikigami wears now. No statement about a sum over six souls is made about
   one soul.
2. **A closed need vocabulary.** A `NeedProfile` holds attribute weights,
   accepted and preferred mains per slot, set plans, per-soul floors, loadout
   targets (`AtLeast`, `AtMost`, `Within` on a soul contribution or on a panel
   value), and saturations (a knee and a residual weight). There is no
   expression language. A profile is checked on load and refused when
   ill-formed.
3. **Hard requirements gate; soft preferences rank.** Each requirement is `Met`,
   `Unmet` with its shortfall, or `Undetermined` with the missing input. Hard
   requirements combine into an admission by three-valued conjunction. Only
   admissible results are ranked; an inadmissible result keeps its value and is
   shown as an explanation. Soft preferences break ties and are reported. No
   hard requirement is ever a numeric penalty.
4. **Affinity is anchored like quality.** A soul's affinity for a need is its
   weighted utility in roll units, normalized at 0 (nothing useful), 50 (the
   need's expected +15 soul under the reference measure), and 100 (the most one
   soul can give the need). The main attribute gates and adds nothing. Only
   weight ratios matter.
5. **A set never adds to a soul's affinity.** Whether a soul's set has a place
   is a gate, its role in a plan is returned separately, and whether a plan is
   complete is decided on a loadout only.
6. **Set use only adds a keep suggestion.** A set's usual use is an authored
   need profile whose subject is the set. Its one output is a keep suggestion
   when the soul's utility reaches pass 1's SR milestone in units of the
   profile's heaviest weight. It never changes quality, a tier, a sort position,
   or a Shikigami's affinity, and a low set-use fit never produces a discard
   suggestion.
7. **No guessed inputs.** A panel target without the Shikigami's base values is
   `Undetermined`, never evaluated with a guessed or zero base.
8. **Pruning is by the need, and quality is a policy.** Only a need-derived
   bound may prune without changing a result. A per-need `candidate_floor` on
   `quality.total` remains, absent by default, as a user's policy that says it
   applied. It never excludes an unrated soul.
9. **Inversions are computed over every soul.** A soul is an inversion for a
   need when it is admissible for the need, its quality is unrated or below 50,
   and its affinity is above 50: below its expected soul in itself, above it for
   this need.
10. **Every number is explained.** Results carry typed reasons, never text, and
    every number traces to a profile item and its basis: the profile's
    provenance, or a named game rule. A game rule cited by a profile must be a
    rule of a spec page.

## Alternatives

- **One scalar with penalties for hard mismatches.** Rejected: every penalty
  size is arbitrary, and the result can no longer say that a soul is unusable.
- **Floors and caps on single souls.** Rejected as undefined for sums. Per-soul
  floors remain for a need that constrains one soul.
- **A set bonus or multiplier on a soul's affinity.** Rejected: it scores a
  four-piece outcome one soul cannot produce.
- **Set affinity as its own function over its own profile type.** Rejected: an
  ordinary profile with a set as its subject keeps the two kinds of affinity
  from drifting apart.
- **Set use as a score shown beside quality.** Rejected by the maintainer: it
  would raise souls for their set, which is the averaging ADR-0003 exists to
  avoid, in a new place.
- **An expression language for needs.** Rejected: it cannot be checked on load
  or explained reason by reason.
- **Pruning by quality as a correctness step.** Rejected; kept as a policy.

## Consequences

**Easier.** Each result says what disqualified a soul, what it carries, and what
a swap would change, in fields a test can assert. The inversion list includes
the souls pass 1 cannot rate. Set use and Shikigami affinity share one function.

**Harder.** The loadout level needs data that does not exist yet: sourced rules
for set activation and for panel values, a set catalogue, and Shikigami records
in the probe schema. Need profiles, including the set-use profiles, must be
authored and reviewed. The structural claims — the attainable maximum, scale
invariance, monotonicity, the concavity of saturation — are proved and the
constants reproduced under ADR-0023 before pass 2 is implemented. `match`'s
search and its complexity bound need their own proposal.

**Records.** `scoring.md` states the contract. `query.md` renames the admission
field. The glossary gains the new terms. The architecture overview no longer
says that pass 1 prunes pass 2. The status row names this record.
