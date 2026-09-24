---
id: PRP-0005
status: draft
date: 2026-09-25
area: domain
related-issues: []
superseded-by: []
---

# PRP-0005: A soul's kind follows its set

## Problem

ADR-0029 makes a soul `Ordinary` or `Boss(InnateAttribute)`, and each soul
states its kind. Nothing ties the kind to the set, so the domain can hold a boss
soul of 破势 or an ordinary 土蜘蛛. Neither exists in the game. A reading or a
wire message that carries one is bad data, and the domain accepts it.

The game fixes which sets are boss-style. The 灰机 阴阳师 wiki's
`Data:Soul.json` (revision 2026-08-26, local research) marks thirteen of its 70
sets `type` 2 (单件属性): the
seven 首领御魂 土蜘蛛, 胧车, 荒骷髅, 地震鲶, 蜃气楼, 鬼灵歌伎, 夜荒魂, and the
six 星痕御魂 八咫镜, 天羽羽斩, 预言星盘, 月之石, 纺缘锤, 稻荷穗箭. Its ids are
the scheme's soul bits, which give suit codes 50–54, 77, 91, and 94–99 through
the bit table of `scheme::mapping`.

What is not established is that a soul read from the game carries those same
suit codes. The bit table's suit codes are the prior tool's identifiers, to be
re-established by a reader recording (ADR-0014, `scheme-code.md`). Until one
does, a check keyed on suit codes could refuse real souls.

## Goals and non-goals

- Goal: a soul whose kind disagrees with its set cannot be built.
- Goal: the list of boss-style sets is sourced, and tied to the soul-bit table
  by a test.
- Non-goal: inferring an innate attribute. The set says whether a soul has one;
  only the reading says which.
- Non-goal: changing the core wire. `Soul.suit_code` and `Soul.kind` already
  carry what the check needs.

## Proposed design

- `SoulKind = Ordinary(OrdinarySet) | Boss(BossSet, InnateAttribute)`. The set
  moves into the kind, and `Soul::set()` reads it, so kind and set cannot
  disagree by construction.
- `BossSet::new(set)` and `OrdinarySet::new(set)` are the only constructors,
  each `Option`, from one table of the thirteen suit codes with its source in a
  comment. Every set is exactly one of the two.
- `SoulKind::of(set, innate: Option<InnateAttribute>)` returns the kind or a
  structured `KindMismatch`: `InnateOnOrdinarySet` or `NoInnateOnBossSet`.
- A test in `scheme::mapping` checks that the thirteen codes are those of soul
  bits 33–37, 42, 55, and 58–63, so the two tables cannot drift apart.
- The daemon's query conversion reports a mismatch as `query.malformed`. Decode
  from a reading does the same when it exists.

A prototype of this design, with its tests, is kept on a local branch.

## Compatibility and migration

Every place that builds a `Soul` passes the set through `SoulKind::of` instead
of a separate `set` field. The wire, the Dart bindings, and the recorded session
do not change.

## Alternatives

- **Keep the kind carried, not checked (ADR-0029 as accepted).** The current
  state; it admits a kind the set contradicts.
- **Check at runtime only**, with private fields and `Soul::new(...) -> Result`.
  Rejected in favour of the type-level form, which needs no accessor for every
  field and makes the mismatch unrepresentable rather than refused.
- **Only the seven 首领御魂.** Not chosen: the maintainer chose all thirteen
  (2026-09-25), since the wiki's `type` 2 includes the six 星痕御魂 alike.

## Implementation and evidence

Waits on one piece of evidence: a reader recording or export in which souls of
at least one boss-style set and one ordinary set carry the suit codes the bit
table gives them. Then this proposal becomes an ADR amending ADR-0029, rule 5,
and the prototype is rebased and merged.

## Open questions

- Whether imported souls carry the bit table's suit codes (above).
- How 固有属性 treats a 星痕御魂: the maintainer's 2026-09-25 observation
  used 首领御魂 only, so the rule would be extrapolated to 星痕御魂 until
  observed.

## Outcome

Open.
