---
id: PRP-0003
status: draft
date: 2026-09-24
area: scoring
related-issues: []
superseded-by: []
---

# PRP-0003: Set affinity and Shikigami affinity as two kinds of pass-2 fit

## Problem

Quality Model v1 (ADR-0024) scores a soul without its set: a 心眼 soul and
a 招财猫 soul with the same sub-attributes get the same quality. That is
intended. Quality asks whether a soul is good as an item, and the set is not a
property of its rolls. But which set a soul belongs to is what makes its rolls
useful or wasted.

The maintainer's example (2026-09-24): a 心眼 soul with 效果命中 maxed is of
little use as a 心眼, so its fit to its own set is
low. 丑时之女 uses 效果命中 and uses the 心眼 effect, so the same soul
fits 丑时之女 very well. Pass 1 cannot say either thing. Pass 2 as specified has
only one kind of fit, a soul against one `NeedProfile` (`scoring.md`).

## Goals and non-goals

Goals:

- Record the direction: set fit and Shikigami fit are two kinds of affinity in
  pass 2, and quality stays set-independent.
- Name the data each needs and where it must come from.

Non-goals, until this proposal is taken up:

- the formulas, the profile formats, and the thresholds
- any set or Shikigami data
- any change to Quality Model v1

## Proposed design

A direction, not yet a design.

1. **Set affinity (套装适配度).** It answers how well a soul's sub-attributes
   suit the usual use of its set. Each set has a profile of the attributes its
   holders usually want. A soul is scored against the profile of its own set, so
   a 心眼 with 效果命中 maxed scores low.
2. **Shikigami affinity (式神适配度).** It answers how useful a soul is to one
   Shikigami. This is the existing `NeedProfile` and `fit`: the Shikigami's
   wanted attributes, the sets it uses, floors and caps. 丑时之女 with
   a 效果命中心眼 scores high.
3. **The inversion is the output.** One soul can have a high quality, a low set
   affinity, and a high Shikigami affinity. Reaching that row directly is what
   ADR-0003 exists for.
4. **Normalization, if it carries over.** Each profile could reuse v1's anchors:
   0 for no useful value, 50 for the profile's own expected soul, 100 for the
   attainable maximum. Scores would be compared only within one profile, as
   `scoring.md` already requires of affinity.

## Compatibility and migration

Nothing changes until an ADR accepts a design. Quality Model v1 is unaffected.

## Alternatives

- **Set as a gate on the quality archetypes.** For example, 破势 would be judged
  only as `output`. Offered on 2026-09-24 and set aside by the maintainer in
  favour of this direction: it would put set knowledge into pass 1, and it could
  not express a soul that is poor for its set yet excellent for one Shikigami.
- **Per-set useful attributes inside quality.** Rejected for the same reason,
  and because profiles of different sizes break the common scale of pass 1
  (`papers/quality-model-v1/paper.md`, § 12).

## Implementation and evidence

None yet.

## Open questions

- **Where the set profiles come from.** Seventy sets, each with its usual role
  and wanted attributes, must be sourced or authored and reviewed; none is
  invented. A draft table, each row with its source or marked TBD, is the first
  step.
- **Where the Shikigami needs come from.** Derived from the damage model or
  authored (`scoring.md`, provenance). The 丑时之女 example is the maintainer's
  judgement, recorded as such.
- Whether set affinity uses the soul's main attribute, as the quality archetypes
  do, or its sub-attributes only.
- Whether a boss soul's innate attribute (固有属性) enters set affinity.
- How a set whose holders are of several kinds is profiled: one profile, or
  several and the best of them.

## Outcome

Open. The maintainer will take up the design later.
