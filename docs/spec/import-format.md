---
kind: spec
status: current
area: import
---

# Import formats

The external files a user can import, and how each one maps into the snapshot IR
([snapshot-ir.md](snapshot-ir.md)). One section per format. Every format detail
on this page (key names, nulls, units, quirks) ends at its format module in
`yata-daemon::import`; nothing past the IR knows it (ADR-0031). ADR-0030 decides
that the project names no program that produces a community format.

## Evidence

No community format belongs to this project, so each section states where its
shape comes from. The first community format's shape comes from **one sample
file the maintainer supplied**, which stays on the maintainer's machine; none of
its account data is in this repository. A statement marked _sample_ is what that
file shows, not a rule the format promises.

## How a file is recognized

```text
Format = YataSnapshot | Community(FormatTag)
FormatTag = MumuSnapshotV1          -- one case per community section below; closed
```

A file is recognized from its top-level keys alone, by the rules each section
states. A file that matches no format, or more than one, is refused. The
importer does not guess.

## `yata-snapshot`

Yata's own format: the IR itself, in the proto3 JSON mapping of
`snapshot.proto`. Its recognition, versions, and compatibility are
[snapshot-ir.md](snapshot-ir.md), "The `yata-snapshot` file". It needs no
mapping.

## `mumu-snapshot-v1`

### Recognition

The top-level key `format` holds the string `"mumu-snapshot-v1"`.

### Source shape

A file is one JSON object. Its keys are camelCase, except `hero_equips`. Only
the keys below are read; any other key is ignored.

```text
File {
  format       : "mumu-snapshot-v1"
  completeness : string                  -- "complete" in the sample
  capturedAt   : string
  scope        : Scope                   -- which parts the file holds
  hero_equips  : [Soul]
  heroes       : { id → Hero }
  equipPresets : [[label: string, souls: [string; 6]]]
  currency     : { name → integer }
  realmCards   : [[id: string, kind: integer, _, _]]
  guild        : Guild
}

Scope { souls, heroes, items, realmCards, guild, taskRecords : bool }

Soul {
  id                  : string           -- 24 hexadecimal digits           sample
  setId               : string           -- the set's Chinese name, 涂佛
  slot                : integer          -- position, numbered from 1       sample
  quality             : integer          -- star
  level               : integer
  mainAttrType        : AttributeName
  mainAttrValue       : number
  subAttributes       : [SubAttribute]
  initialSubstatCount : null             -- null on every soul; not read    sample
  equippedState       : null             -- null on every soul; not read    sample
}

SubAttribute =
    Rolled { type: AttributeName, value: number, enhancementCount: integer | null }
  | Innate { type: AttributeName, value: number, fixedAttribute: true,
             enhancementCount: null }

Hero { heroId: string of digits, level: integer, star: integer,
       awake: 0 | 1, lock: bool, … }     -- other keys not read

Guild { level: integer, activeMemberCount: integer, … }  -- other keys, and members, not read

AttributeName =
    "attack_flat" | "attack_rate" | "defense_flat" | "defense_rate"
  | "hp_flat" | "hp_rate" | "speed"
  | "crit_rate" | "crit_damage" | "effect_hit" | "effect_resist"
```

What the sample shows beyond the types:

- Every soul in position 1 has main attribute `attack_flat`, so positions start
  at 1.
- The 62 set names each match a set of `SoulSet`'s name table.
- A value is the unrounded stored value. A rate is a fraction (`0.55` is 55%).
- An `Innate` entry appears exactly once on each soul of a boss set and on no
  other soul. Its value is `0.08` or `0.16`.
- Each preset lists six soul ids by position; every id names a soul of the file,
  and the soul in position `i` has slot `i + 1`.
- No soul links to a hero, and no hero to a soul.
- Read with the units below, every soul is legal by `mechanics::assess`.

### Sections

`completeness` must be `"complete"`; any other value refuses the file. It
applies to what `scope` says the file holds:

```text
souls      = Present(Complete)  if scope.souls       else Absent
shikigami  = Present(Complete)  if scope.heroes      else Absent
presets    = Present(Complete)  if scope.souls       else Absent   -- presets ride with the souls
assets     = Present(Complete)  if scope.items and scope.realmCards else Absent
guild      = Present(Complete)  if scope.guild       else Absent
```

A section that `scope` includes and whose keys are missing refuses the file.

### Mapping

```text
hero_equips[]            → souls.souls[]
  id                     → id
  setId                  → set
  slot, quality, level   → slot, star, level
  mainAttrType, _Value   → main
  subAttributes: Rolled  → rolled[], in order; enhancementCount → rolls (null → None)
  subAttributes: Innate  → innate (its value is not kept); more than one refuses the soul

heroes: id → Hero        → shikigami.instances[]
  id (the map key)       → id
  heroId                 → species
  level, star            → level, star
  awake                  → evolved (1 → true)
  lock                   → locked

equipPresets[]           → presets.presets[]: label, souls[i] → souls[i]

currency: name → n       → assets.currencies: snake_case name → the Currency of that name
realmCards[]             → assets.realm_cards[]: [0] → id, [1] → kind

guild                    → guild: level, activeMemberCount → member_count
```

```text
AttributeName → SoulAttribute    unit
  attack_flat   → AtkFlat        as written
  attack_rate   → AtkPercent     × 100 (a fraction to percentage points)
  defense_flat  → DefFlat        as written
  defense_rate  → DefPercent     × 100
  hp_flat       → HpFlat         as written
  hp_rate       → HpPercent      × 100
  speed         → Spd            as written
  crit_rate     → Crit           × 100
  crit_damage   → CritDmg        × 100
  effect_hit    → EffectHit      × 100
  effect_resist → EffectRes      × 100
```

A record whose field is missing or of the wrong kind, whose attribute or
currency name is not in the tables, or whose sub-attribute entry carries
`fixedAttribute` with any value but `true`, is left out and named. Rules of the
domain (a set name, a position, a roll count) are admission's
([snapshot-ir.md](snapshot-ir.md), "Admission").

**Identity.** Soul and hero ids are 24 hexadecimal digits. That the same soul
keeps its id across two files of one account is expected and not yet shown by a
second sample.

## CSV template

The route that involves no third-party program: a user writes the souls in a
spreadsheet. A CSV adapter is planned (the CSV compiler); until it lands, the
template is only described. The file is UTF-8 and comma-separated. It has
exactly this header row, and one row per soul:

```text
id,set,slot,star,level,main_type,main_value,sub1_type,sub1_value,sub1_rolls,sub2_type,sub2_value,sub2_rolls,sub3_type,sub3_value,sub3_rolls,sub4_type,sub4_value,sub4_rolls,innate_type
```

| Column                    | Holds                                                                     |
| ------------------------- | ------------------------------------------------------------------------- |
| `id`                      | any text unique within the file; it identifies the soul across imports    |
| `set`                     | the set's Chinese name, as the game shows it                              |
| `slot`, `star`, `level`   | 1–6, 1–6, 0–15                                                            |
| `main_type`, `main_value` | the main attribute: an `AttributeName` and its value                      |
| `subN_type`, `subN_value` | a sub-attribute, in the order the game shows them; both empty when absent |
| `subN_rolls`              | how many times it was strengthened, 0–5; required with the sub-attribute  |
| `innate_type`             | the innate attribute (固有属性); empty for an ordinary soul               |

Values are written in the units of `mumu-snapshot-v1`: a rate as a fraction
(`0.15` for 15%), and a flat value or speed as the number the game shows. A CSV
gives a souls section only, `Present(Complete)`; every other section is absent.

## Related

- [ADR-0030](../decisions/0030-no-game-reader.md): the inventory comes from a
  file
- [ADR-0031](../decisions/0031-snapshot-ir.md): formats end at the IR
- [snapshot-ir.md](snapshot-ir.md): the IR and Yata's own format
- [glossary.md](glossary.md): `SoulAttribute`, `SoulSet`, and the game terms
