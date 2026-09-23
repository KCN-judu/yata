---
kind: spec
status: current
area: domain
---

# Soul mechanics

The game's rules for a Soul's level, rolls, and sub-attribute values, written as
inference rules. The core decides legality, roll counts, and the community
predicates (双速, 拉满, 真 17 速) by these rules and by nothing else. A case the
rules do not cover is a gap in this page, to be filled here; it is never settled
in code.

These rules describe the game. They are _implemented_ in `yata-core`
(`mechanics`), and every rule is _tested_; the tests are listed in
[../evidence/testing.md](../evidence/testing.md). The roll and acquisition
probabilities are implemented only as far as § Roll distribution states them.

Each rule's source and evidence level is kept in
`research/soul-mechanics-sources.md` (local research, not published). Where a
rule rests on no source, or on a reading of an ambiguous one, this page says so
at the rule.

Slot and main-attribute options come from the
[glossary](glossary.md#soul-slot-main-attribute-rules); this page names that
table `Main(k)` and does not repeat it.

## Notation

```text
Soul        s ::= ⟨k, σ, ℓ, m, S, c⟩
  k  : SoulSlot                 Slot1 … Slot6
  σ  : star                     an integer
  ℓ  : level                    an integer
  m  : SoulAttribute            the main attribute
  S  : SoulAttribute ⇀ ℚ≥0      sub-attributes and their stored values (a partial map)
  c  : SoulAttribute ⇀ ℕ        enhancement_count per sub-attribute, same domain as S

dom S                           the sub-attributes the soul has
hits(a)   = 1 + c(a)            value increments a received: its initial value plus its rolls
nodes(ℓ)  = ⌊ℓ / 3⌋             rolls the soul has had: one at each of +3, +6, +9, +12, +15
[lo(a), hi(a)]                  the range of one increment of a, at the soul's star (§ Values)
Max(a)    = 6 · hi(a)           the theoretical maximum of a at 6★
```

Values are in the unit the game displays: points for `Spd` and the flat
attributes, percentage points for the rest (`Crit` 2.4 means 2.4%). `S(a)` is
always the stored value, never the displayed one (§ Display).

**Comparing values.** The tables are exact decimals. A stored value arrives as a
binary floating-point number, which cannot hold most decimals exactly. Every
comparison between a stored value and a bound from the tables (`hits · lo`,
`hits · hi`, `n` in `true`, `Max − 1`) therefore allows a tolerance of `10⁻⁶` in
display units, in the direction that admits the value. The smallest step between
table values is `0.1`, so no two distinct bounds are closer than the tolerance.

A soul records sub-attributes and their values. Whether the game also records
`c`, or only lets it be inferred, is open (§ Open); the rules below hold either
way, and § Inferring roll counts says what can be known without it.

## Well-formed souls

`⊢ s ok` means a soul with these fields can exist in the game. The decoder
admits a soul only if it is well-formed; a soul that is not is a decode error,
never a value to score.

```text
 1 ≤ σ ≤ 6     0 ≤ ℓ ≤ 15     m ∈ Main(k)     |dom S| ≤ 4
 ∀a ∈ dom S.  c(a) ≥ 0  ∧  hits(a) · lo(a) ≤ S(a) ≤ hits(a) · hi(a)
 Σ_{a ∈ dom S} c(a)  ≤  nodes(ℓ)
─────────────────────────────────────────────────────────────── (W-Soul)
 ⊢ ⟨k, σ, ℓ, m, S, c⟩ ok
```

**No rule relates `m` to `dom S`.** A sub-attribute may be the same attribute as
the main attribute. A Slot 2 soul with main `Spd` and `Spd ∈ dom S` is
well-formed (双速).

## Rolls

Reaching a node (+3, +6, +9, +12, +15) applies exactly one roll. A roll adds a
new sub-attribute or strengthens one the soul has.

```text
 ℓ' = ℓ + 1     3 ∣ ℓ'     |dom S| < 4     a ∉ dom S     lo(a) ≤ δ ≤ hi(a)
─────────────────────────────────────────────────────────────── (R-Add)
 ⟨k, σ, ℓ, m, S, c⟩  →  ⟨k, σ, ℓ', m, S[a ↦ δ], c[a ↦ 0]⟩

 ℓ' = ℓ + 1     3 ∣ ℓ'     a ∈ dom S     lo(a) ≤ δ ≤ hi(a)
─────────────────────────────────────────────────────────────── (R-Up)
 ⟨k, σ, ℓ, m, S, c⟩  →  ⟨k, σ, ℓ', m, S[a ↦ S(a) + δ], c[a ↦ c(a) + 1]⟩

 ℓ' = ℓ + 1     ¬(3 ∣ ℓ')
─────────────────────────────────────────────────────────────── (R-Level)
 ⟨k, σ, ℓ, m, S, c⟩  →  ⟨k, σ, ℓ', m, S, c⟩
```

- `a` may equal `m` in R-Add (双速).
- **The rules are non-deterministic.** While `|dom S| < 4`, both R-Add and R-Up
  apply: a roll may add a new sub-attribute or strengthen an existing one, by
  chance. At `|dom S| = 4` only R-Up applies. A soul that starts with three
  sub-attributes can therefore reach +15 still holding three.
- Legality, inference, and the predicates need only which outcomes are possible.
  How likely each is, is § Roll distribution.
- The star never changes. Every level also raises the main attribute's value (§
  Main-attribute values).

From the rules: a +15 soul has had five rolls, `c(a) ≤ 5`, and `hits(a) ≤ 6`.
`hits(a) = 6` only if `a` was present at +0 and all five rolls strengthened it.

## Roll distribution

Which rule fires, and on which attribute, is a weighted draw. NetEase's official
probability notice names three classes and their weights; each class's weight is
shared equally by its members:

| Class  | Members                                     | Class weight | Weight per member `w(a)` |
| ------ | ------------------------------------------- | ------------ | ------------------------ |
| 攻击类 | `AtkFlat` `AtkPercent` `Crit` `CritDmg`     | 36%          | 9%                       |
| 防御类 | `DefFlat` `DefPercent` `HpFlat` `HpPercent` | 36%          | 9%                       |
| 功能类 | `Spd` `EffectHit` `EffectRes`               | 28%          | 28/3% ≈ 9.33%            |

```text
 |dom S| = 4     a ∈ dom S
─────────────────────────────────────── (P-Four)
 P(R-Up on a) = 1/4

 |dom S| < 4     a drawn with probability w(a) from all eleven attributes
─────────────────────────────────────── (P-Draw)
 a ∉ dom S ⟹ R-Add a          a ∈ dom S ⟹ R-Up a
```

- The draw ranges over all eleven attributes, the main attribute included. That
  is how a sub-attribute can equal the main attribute (双速).
- Below four sub-attributes, a roll adds a leg with probability `1 − q(S)`,
  where `q(S) = Σ_{a ∈ dom S} w(a)`, and otherwise strengthens the attribute it
  drew.
- The distribution of `δ` within `[lo(a), hi(a)]` is TBD.
- The official text for P-Draw reads "随机增加一条副属性"; read literally, a
  roll below four sub-attributes always adds one. P-Draw follows the maintainer
  and a published analysis instead. If a primary source shows the literal
  reading, P-Draw draws only among the attributes not in `dom S`, and a
  three-leg soul always gains its fourth leg at +3.
- **A contested reading of the weights.** A published analysis reads the class
  weights as an equal share of all eleven attributes, `w(a) = 1/11` (classes
  4/11, 4/11, 3/11), with the notice's figures as their rounding. The notice's
  28% is not the rounding of 3/11 (27.27%), so this page keeps the notice's
  weights. The two readings differ by under one percentage point per class, and
  no legality rule depends on either.

**A three-leg soul that stays three-leg.** A soul with three sub-attributes at
+0 keeps three only if all five draws land on its own three attributes:

```text
 P(stays three-leg at +15) = q(S)⁵,   q(S) ∈ [0.27, 0.28]
```

| Utility attributes among the three | `q(S)` | `q(S)⁵` | About one in |
| ---------------------------------- | ------ | ------- | ------------ |
| 0                                  | 0.2700 | 0.143%  | 697          |
| 1                                  | 0.2733 | 0.153%  | 655          |
| 2                                  | 0.2767 | 0.162%  | 617          |
| 3                                  | 0.2800 | 0.172%  | 581          |

The first roll at +3 adds the fourth leg with probability 72%–73%.

## Acquisition

How a soul's star, main attribute, and initial sub-attributes are drawn when it
is bought. The notice discloses this for four sources only: 秘魂屋 (refresh and
purchase), 勋章商店 (six-star), 秘卷屋 soul boxes, and 逢魔之时 chests. Souls
dropped by 御魂副本 and other content follow no disclosed distribution, and
nothing here is assumed for them.

**Star, 秘魂屋 refresh**, by the player's level:

| Player level | 3★  | 4★  | 5★  | 6★  |
| ------------ | --- | --- | --- | --- |
| ≤ 15         | 80% | 20% | —   | —   |
| 16–30        | 60% | 30% | 10% | —   |
| 31–40        | 30% | 50% | 20% | —   |
| 41–60        | —   | 30% | 50% | 20% |

**Main attribute**, all four sources. Slots 1, 3 and 5 have a fixed main
attribute and draw nothing. Slots 2, 4 and 6 draw from two groups:

- 特殊属性 (`Special`): `Spd` `EffectRes` `EffectHit` `Crit` `CritDmg`
- 其他属性 (`Ordinary`): `AtkFlat` `HpFlat` `DefFlat` `AtkPercent` `HpPercent`
  `DefPercent`

```text
 k ∈ {Slot2, Slot4, Slot6}
─────────────────────────────────────────────────────────────── (A-Main)
 P(m ∈ Special ∩ Main(k)) = 10%     P(m ∈ Ordinary ∩ Main(k)) = 90%
```

Within a group, the notice gives no split. Assuming an equal share among the
slot's legal options, which is unsourced:

| Slot  | `Special` options       | Each | `Ordinary` options                    | Each |
| ----- | ----------------------- | ---- | ------------------------------------- | ---- |
| Slot2 | `Spd`                   | 10%  | `AtkPercent` `DefPercent` `HpPercent` | 30%  |
| Slot4 | `EffectHit` `EffectRes` | 5%   | `AtkPercent` `DefPercent` `HpPercent` | 30%  |
| Slot6 | `Crit` `CritDmg`        | 5%   | `AtkPercent` `DefPercent` `HpPercent` | 30%  |

**Initial sub-attributes**, all four sources: each is drawn by
class, 攻击类 36%, 防御类 36%, 功能类 28%, the same weights as P-Draw. The
notice does not disclose how many initial sub-attributes a soul gets, or whether
two initial draws can land on the same attribute.

A published analysis of data-mined souls gives the count: two, three, or four
initial sub-attributes, each with probability about one third. It is community
evidence, not a notice, so W-Soul does not require `|dom S| ≥ 2`; the count is
used for probabilities only.

**奉纳 (offering a soul).** The chance of the 神赐 reward, per soul offered:

| Star offered | 神赐 |
| ------------ | ---- |
| 4★           | 2.1% |
| 5★           | 2.7% |
| 6★           | 5.3% |

## Values

One increment of a sub-attribute, whether its initial value or a roll, lies in
`[lo(a), hi(a)]`. At 6★:

| `SoulAttribute` | `lo` | `hi`  | `Max = 6 · hi` |
| --------------- | ---- | ----- | -------------- |
| `Spd`           | 2.4  | 3.0   | 18.0           |
| `Crit`          | 2.4  | 3.0   | 18.0           |
| `AtkPercent`    | 2.4  | 3.0   | 18.0           |
| `HpPercent`     | 2.4  | 3.0   | 18.0           |
| `DefPercent`    | 2.4  | 3.0   | 18.0           |
| `CritDmg`       | 3.2  | 4.0   | 24.0           |
| `EffectHit`     | 3.2  | 4.0   | 24.0           |
| `EffectRes`     | 3.2  | 4.0   | 24.0           |
| `AtkFlat`       | 21.6 | 27.0  | 162.0          |
| `HpFlat`        | 91.2 | 114.0 | 684.0          |
| `DefFlat`       | 4.0  | 5.0   | 30.0           |

At 6★ every range has `lo(a) = 0.8 · hi(a)`. That is an observation over the
table, not a rule the core may use to derive a missing range.

Ranges for 1★–5★ are TBD. Until they are sourced, a rule that needs `lo` or `hi`
for a soul below 6★, or for a flat attribute, has no answer, and the core
reports the soul as undecidable for that rule rather than guessing.

## Main-attribute values

A 6★ soul's main attribute grows linearly with its level: a base at +0 and a
fixed step per level.

```text
 σ = 6
─────────────────────────────────────── (M-Main)
 main(m, ℓ) = base(m) + ℓ · step(m)
```

| Main attribute `m`                                                   | `base` (+0) | `step` per level | +15  |
| -------------------------------------------------------------------- | ----------- | ---------------- | ---- |
| `AtkFlat`                                                            | 81          | 27               | 486  |
| `DefFlat`                                                            | 14          | 6                | 104  |
| `HpFlat`                                                             | 342         | 114              | 2052 |
| `Spd`                                                                | 12          | 3                | 57   |
| `CritDmg`                                                            | 14          | 5                | 89   |
| `AtkPercent` `DefPercent` `HpPercent` `Crit` `EffectHit` `EffectRes` | 10          | 3                | 55   |

The main attribute takes no roll, so its value at a level is fixed. The table is
in displayed units; until the reader's first recordings show how the game stores
it, a 6★ soul whose decoded main value differs from `main(m, ℓ)` is reported as
a warning, not rejected. Values below 6★ are TBD, and for those souls the check
is not made.

## Inferring roll counts

When `c` is not recorded, W-Soul bounds it. For `a ∈ dom S`:

```text
 ⌈S(a) / hi(a)⌉  ≤  hits(a)  ≤  ⌊S(a) / lo(a)⌋                       (I-Hits)
```

The interval can hold more than one integer. For `Spd` at 6★, `S = 14.8` admits
`hits ∈ {5, 6}`. Then `c(a)` is **ambiguous**, and a rule that depends on it is
evaluated on every admitted value:

- it is **certain** if it holds for all of them
- it is **impossible** if it holds for none
- it is **undetermined** otherwise, and the UI shows the estimate marker (`~`)

One consequence the tiers rely on: `S(a) > 5 · hi(a)` forces `hits(a) = 6`. For
`Spd`, any stored value above 15.0 proves all five rolls went to speed.

## Display

The game displays a sub-attribute rounded, and stores it with a fraction:

```text
 shown(x) = round(x)                                                  (D-Round)
```

- For `Spd`, `round` is to the nearest integer: 1.8 shows as 2, 1.3 as 1. How a
  tie (x.5) rounds is TBD.
- For the percentage attributes, the displayed precision is TBD.
- Rules use `S(a)`, never `shown(S(a))`. The reader reads stored values, so the
  core never has to invert D-Round.

## Community predicates

Each community term is a predicate on a well-formed soul. Wherever the docs or
the UI use the term, it means exactly this.

```text
 k = Slot2     m = Spd     Spd ∈ dom S
─────────────────────────────────────── (双速, double speed)
 double_speed(s)

 ℓ = 15     a ∈ dom S     c(a) = 5
─────────────────────────────────────── (拉满, maxed)
 maxed(s, a)

 a ∈ dom S     S(a) ≥ n
─────────────────────────────────────── (真 n, true n)
 true(s, a, n)

 maxed(s, a)     S(a) ≥ Max(a) − 1
─────────────────────────────────────── (顶段, top band)
 top_band(s, a)
```

- `maxed` is `hits(a) = 6`: the attribute was present at +0 and took every roll.
- `true(s, Spd, 17)` is 真 17 速. A stored 16.5 that displays as 17 does not
  satisfy it.
- `top_band(s, a)` places `S(a)` in `[Max(a) − 1, Max(a)]`: 17–18 for `Spd` and
  `Crit`, 23–24 for `CritDmg`. It is the band the maintainer names as the UR
  tier's anchor (PRP-0002).
- For `Spd`, `top_band` needs no recorded `c`: `S ≥ 17 > 15 = 5 · hi` forces
  `hits = 6` by I-Hits, and so `maxed`. For every 6★ attribute in the table,
  `Max − 1 > 5 · hi` holds, so `top_band` is always decidable from values alone.
  `maxed` without `top_band` may be undetermined.

## Open

- Whether the game records `c` per sub-attribute, or whether the core must
  always infer it by I-Hits.
- The distribution of `δ` within its range.
- Whether initial draws can repeat, and an official statement of the initial
  count.
- The split within a main-attribute group, and whether the equal share within a
  class holds.
- Any distribution for souls from 御魂副本 and other undisclosed sources.
- Whether the draw below four sub-attributes can land on an existing attribute
  (P-Draw) or only on a missing one; the official text reads either way.
- Increment ranges for 1★–5★.
- Main-attribute values below 6★.
- Whether the class weights are the notice's 36/36/28 or an equal 1/11 per
  attribute.
- D-Round's tie rule, and the displayed precision of the percentage attributes.
