---
kind: spec
status: current
area: import
---

# Import formats

The files a user imports to give Yata an inventory, and the CSV template a user
can fill in by hand to produce one. ADR-0030 decides that the inventory comes
only from such a file, in a community snapshot format, and that the project does
not name or recommend any program that produces one. This page describes each
format by its shape, one section per format.

The daemon does not parse these formats yet. PRP-0008 designs the importer, and
[project/status.md](../project/status.md) tracks it.

## Evidence

No format here belongs to this project, so every section states where its shape
comes from. The first format's shape comes from **one sample file the maintainer
supplied**, which stays on the maintainer's machine; none of its account data is
in this repository. A statement marked _sample_ is what that file shows, not a
rule the format promises. A second sample may contradict it.

Where a value meets the domain, for example a set name meeting `SoulSet`, the
mapping is a hypothesis until this project's own evidence establishes it
(ADR-0014).

## How a file is recognized

```text
type FormatTag = MumuSnapshotV1        -- one case per section below; closed
```

A file is recognized from its header alone: the top-level keys and values each
section lists under "Recognition". A file that matches no format, or more than
one, is refused. The importer does not guess. Each format is parsed by its own
module into one format-independent snapshot, and the fact that records the
import keeps the `FormatTag` (PRP-0008).

A format's tag names a file format. It does not refer to any program that writes
it.

## `mumu-snapshot-v1`

### Recognition

The top-level key `format` holds the string `"mumu-snapshot-v1"`.

### Shape

A file is one JSON object. Its keys are camelCase, except `hero_equips`.

```text
File {
  format       : "mumu-snapshot-v1"
  completeness : string            -- "complete" in the sample
  capturedAt   : string
  hero_equips  : [Soul]
  -- other keys (currency, heroes, guild, equipPresets, …) are not read
}

Soul {
  id                  : string           -- 24 hexadecimal digits           sample
  setId               : string           -- the set's Chinese name, 涂佛
  slot                : 1..6             -- position, numbered from 1       sample
  quality             : integer          -- star
  level               : 0..15
  mainAttrType        : AttributeName
  mainAttrValue       : number
  subAttributes       : [SubAttribute]
  initialSubstatCount : null             -- null on every soul             sample
  equippedState       : null             -- null on every soul             sample
}

SubAttribute =
    Rolled { type : AttributeName, value : number,
             enhancementCount : 0..5 }                    -- a sub-attribute
  | Innate { type : AttributeName, value : number,
             fixedAttribute : true,
             enhancementCount : null }                    -- 固有属性

AttributeName =
    "attack_flat" | "attack_rate" | "defense_flat" | "defense_rate"
  | "hp_flat" | "hp_rate" | "speed"
  | "crit_rate" | "crit_damage" | "effect_hit" | "effect_resist"
```

What the sample shows beyond the types:

- Every soul in position 1 has main attribute `attack_flat`, which is how
  positions are known to start at 1.
- The 62 set names in the sample each match a set of the suit-code table in the
  local research records.
- A value is the unrounded stored value. A rate is a fraction (`0.55` is 55%). A
  flat attack, defense, or HP value and speed are the numbers the game shows.
- An `Innate` entry appears exactly once on each soul of a boss set and on no
  other soul. Its value is `0.08` or `0.16`.
- No soul has a lock or discard field.

Each `AttributeName` names one `SoulAttribute`. The importer states that table,
decides which fields it requires, and decides what the fields that are null
throughout the sample may hold (PRP-0008).

### CSV template

A user who writes the inventory by hand fills in this CSV and converts it to the
shape above. The project ships no converter (ADR-0030, rule 5). The file is
UTF-8 and comma-separated. It has exactly this header row, and one row per soul:

```text
id,set,slot,star,level,main_type,main_value,sub1_type,sub1_value,sub1_rolls,sub2_type,sub2_value,sub2_rolls,sub3_type,sub3_value,sub3_rolls,sub4_type,sub4_value,sub4_rolls,innate_type,innate_value
```

| Column                        | Holds                                                                     |
| ----------------------------- | ------------------------------------------------------------------------- |
| `id`                          | any text unique within the file; it identifies the soul across imports    |
| `set`                         | the set's Chinese name, as the game shows it                              |
| `slot`                        | 1–6                                                                       |
| `star`                        | 1–6                                                                       |
| `level`                       | 0–15                                                                      |
| `main_type`, `main_value`     | the main attribute: an `AttributeName` and its value                      |
| `subN_type`, `subN_value`     | a sub-attribute, in the order the game shows them; both empty when absent |
| `subN_rolls`                  | how many times it was strengthened, 0–5; optional                         |
| `innate_type`, `innate_value` | the innate attribute (固有属性); both empty for an ordinary soul          |

Values are written in the format's units: a rate as a fraction (`0.15` for 15%),
and a flat value or speed as the number the game shows.

### From a row to a `Soul`

```text
id                         → id
set                        → setId
slot                       → slot
star                       → quality
level                      → level
main_type, main_value      → mainAttrType, mainAttrValue
subN_type, _value, _rolls  → subAttributes: a Rolled each, in order, skipping
                             empty pairs; an empty subN_rolls is written as null
innate_type, innate_value  → subAttributes: one Innate after the Rolled ones,
                             or none
(no column)                → initialSubstatCount: null, equippedState: null
```

The rows become `hero_equips`. The file's `format` is `"mumu-snapshot-v1"`, its
`completeness` is `"complete"` when the CSV lists every soul, and its
`capturedAt` is written by hand. Whether the importer accepts a sub-attribute
whose `enhancementCount` is null is decided with the importer (PRP-0008).

## Related

- [ADR-0030](../decisions/0030-no-game-reader.md): the decision this page serves
- [PRP-0008](../proposals/0008-community-snapshot-import.md): the importer
- [scheme-code.md](scheme-code.md): the suit codes and `SoulSet` identity
- [glossary.md](glossary.md): `SoulAttribute`, `SoulSet`, and the game terms
