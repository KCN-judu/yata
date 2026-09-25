---
kind: spec
status: current
area: import
---

# Snapshot IR

What Yata received from one imported file, in Yata's own vocabulary: the
intermediate representation between the format modules and the domain
(ADR-0031). Every format module normalizes into it. Admission turns it into
domain values. Nothing downstream of admission reads it, and nothing downstream
of normalization knows which format it came from.

The IR is not the domain. It may hold a value the domain refuses: a position of
7, a set name no set has, a sub-attribute without its roll count. Admission
decides, and names the reason.

The external formats, and how each maps into this page, are
[import-format.md](import-format.md).

## Snapshot

```text
YataSnapshot {
  schema      : SchemaVersion             -- this page's version, not a scoring version
  provenance  : Provenance
  captured_at : Option<Text>              -- as the source states it; informational
  account     : Option<AccountRef>        -- the account the file says it belongs to
  souls       : Section<Souls>
  shikigami   : Section<ShikigamiRoster>
  presets     : Section<GamePresets>
  assets      : Section<Assets>
  guild       : Section<Guild>
}

Section<T>   = Absent
             | Present { completeness: Completeness, value: T }
Completeness = Complete        -- the source says the section holds everything it covers
             | Partial         -- the source says it holds a subset
             | Unstated        -- the source says neither
```

`Absent` means the file says nothing about the section. `Present` with an empty
list means the file says the section is empty. The two are never interchanged,
and neither is ever produced to fill in for the other.

A snapshot's identity is the SHA-256 digest of its canonical encoding (below).
It is not a field of the snapshot.

## Provenance

```text
Provenance {
  format   : SourceFormat
  original : Digest              -- SHA-256 of the file's bytes, as imported
}

SourceFormat = YataSnapshot { major: u32, minor: u32 }
             | Community(FormatTag)
FormatTag    = MumuSnapshotV1    -- one case per section of import-format.md
```

Provenance is shown and kept. No rule outside the format modules reads it.

## Sections

### Souls

```text
Souls = { souls: [SoulRecord] }

SoulRecord {
  id      : SourceId
  set     : SetName                  -- the set's name as the game shows it
  slot    : i64                      -- position, meant 1..6
  star    : i64                      -- meant 1..6
  level   : i64                      -- meant 0..15
  main    : Valued
  rolled  : [RolledSub]              -- sub-attributes, in the game's order
  innate  : Option<SoulAttribute>    -- 固有属性; its value is not kept (ADR-0029)
}

Valued    = { attribute: SoulAttribute, value: f64 }   -- display units: a rate in percentage points
RolledSub = { valued: Valued, rolls: Option<i64> }     -- rolls: the strengthening count, if stated
SetName   = non-empty Text
```

The set is a name, not a `SoulSet`: that a name names a set is admission's
question. The attribute is a `SoulAttribute` because the eleven attributes are
Yata's vocabulary (`glossary.md`). A value is in display units whatever units
the file used; converting is the format module's work.

### Shikigami

```text
ShikigamiRoster = { instances: [ShikigamiRecord] }

ShikigamiRecord {
  id      : SourceId          -- this owned copy
  species : SpeciesNumber     -- the game's number for the Shikigami, unresolved
  level   : i64               -- meant 1..40
  star    : i64               -- meant 1..6
  evolved : bool              -- 觉醒
  locked  : bool              -- 锁定 in the game
}

SpeciesNumber = u32
```

No catalogue maps a species number to a Shikigami yet, so the number stays a
number. Which souls a Shikigami wears is not part of any section (ADR-0031, rule
9).

### Game presets

```text
GamePresets = { presets: [GamePreset] }

GamePreset {
  label : Text                        -- as the player named it in the game
  souls : [Option<SourceId>; 6]       -- by position: index i is slot i + 1
}
```

A game preset is a loadout the player saved in the game (御魂方案). It is not
current equipment and not a plan authored in Yata.

### Assets

```text
Assets = {
  currencies  : [(Currency, u64)]     -- each currency at most once
  realm_cards : [RealmCardRecord]
}

Currency = ActionPoint | ArAmulet | AutoPoint | BrokenAmulet | Coin | Contrib
         | DemonSoul | FooleryPass | GoldOfuda | Honor | Jade | Medal
         | MysteryAmulet | Ofuda | RealmRaidPass | ReverseScale | SJade | Scale
         | SkinToken | SpSkinToken | TotemPass

RealmCardRecord { id: SourceId, kind: RealmCardKind }
RealmCardKind = u32                   -- the game's number for the card, unresolved
```

The currencies are the 21 the first format carries, named after its keys. Which
game item each one is (勾玉, 金币, …) is not established for every name, so
`glossary.md` does not list them yet, and no feature reads them. A realm card's
other values are not imported until their meaning is established.

### Guild

```text
Guild { level: u32, member_count: u32 }
```

The guild itself, and nothing about its members. A member record carries another
player's data, and its fields' meanings are not established. Members are added
by a minor version once both are settled.

## Identity

```text
SourceId = non-empty Text, ≤ 64 characters
```

A `SourceId` is what the file calls the record. Repeated imports into one
profile fold by it: a soul with the same id in a later snapshot is the same
soul. So a format module states, in `import-format.md`, whether its ids are
stable across files, and emits only ids that are. No id is a memory address or a
position.

## Validation

Each step refuses before the next runs:

```text
1. detect         the header: one format, or UnknownFormat / AmbiguousFormat / UnsupportedVersion
2. parse          the syntax: MalformedSource
3. source shape   fields and kinds: a record left out (Malformed), or the file refused
4. normalize      names and units into the IR: UnsupportedSourceValue, NormalizationFailure
5. IR-local       limits; one innate at most; each currency once; ids unique per section
6. references     a preset's soul ids name souls of the same snapshot; broken: InconsistentReference
7. admission      domain values, record by record (below)
```

A preset section present without a souls section is refused at step 6: its
references could name nothing. A reference into an absent section of another
snapshot is never checked here; the projection resolves it.

Nothing fills a missing value to let a later step pass.

## Admission

```text
admit : YataSnapshot -> AdmittedSnapshot
AdmittedSnapshot = the snapshot, each present section's records split into
                   admitted domain values and RecordDefect { index, id, reason }

admit_soul      : SoulRecord      -> Result<(GameSoulId, Soul), SoulAdmissionError>
admit_shikigami : ShikigamiRecord -> Result<ShikigamiInstance, ShikigamiAdmissionError>
admit_preset    : GamePreset      -> Result<AdmittedPreset, PresetAdmissionError>
```

Admission is pure. A soul is admitted when its set name names a set, its
position, star, and level are in range, its values are stored values, every
rolled sub-attribute states its roll count (the maintainer, 2026-09-25), and its
innate attribute, if any, is one of the six (ADR-0029). A Shikigami instance is
admitted when its level and star are in range. A preset is admitted when each
position's soul, where the snapshot's souls hold it, has that position. Legality
against the game's tables is not admission: that is `mechanics::assess`.

## Capabilities

```text
Capability   = Inventory              -- souls: the list, the filters, quality, recommendations
             | ShikigamiCollection    -- shikigami
             | GamePresets            -- presets: the player's saved loadouts
             | Assets                 -- assets
             | GuildView              -- guild

requires : Capability -> NonEmpty<SectionKind>
Availability = Unavailable { missing: NonEmpty<SectionKind> }
             | Available { completeness: Completeness }   -- the weakest of the required sections

availability : ProfileState -> Capability -> Availability
```

`requires` is one table, total over `Capability`. Availability is computed from
the profile's folded sections, never from one file and never from its format. An
unavailable capability is reported with the missing sections; it is not answered
with empty data. Completeness orders `Unstated < Partial < Complete`.

## The `yata-snapshot` file

The schema is `crates/yata-protocol/proto/snapshot.proto`, package
`yata.snapshot.v1`. The file form is the proto3 JSON mapping, UTF-8. The stored
form, whose digest is the snapshot's identity, is the protobuf binary encoding:
fields in tag order and no map fields, so one snapshot has one encoding.

- **Recognition.** The top-level key `yataSnapshot` holds `{major, minor}`.
- **Versions.** The version is read before the body. A higher major is
  `UnsupportedVersion`. The same major with any minor is read: an unknown field
  or section is skipped, so an unknown section reads as absent, which is what
  this build can use. An unknown enum value is never reinterpreted: in a record,
  the record is left out and its section becomes partial; in the header, the
  provenance, or a completeness, the file is refused.
- **Provenance on import.** Importing a yata-snapshot file records that file as
  the provenance: its format and its digest. The provenance written inside it,
  from an earlier import, is replaced. A stored snapshot keeps its own.
- **Equivalence.** Two snapshots are equivalent when their Rust values are
  equal. `YataSnapshot → encode → decode` gives an equivalent snapshot, and the
  binary encoding of equivalent snapshots is byte-identical.

A `yata-snapshot` file is imported state. It is not a backup: marks, notes,
profiles, authored need profiles, and Yata's plans are not in it.

## Limits

Checked before allocation, and a file over any of them is refused whole:

| Limit                    | Value     |
| ------------------------ | --------- |
| file size                | 64 MiB    |
| souls                    | 20 000    |
| sub-attributes per soul  | 8         |
| Shikigami instances      | 20 000    |
| game presets             | 1 000     |
| currencies               | 64        |
| realm cards              | 10 000    |
| a text (id, name, label) | 256 chars |

The game showed a soul capacity of 6 000 (the maintainer's screenshot,
2026-09-25); the limits leave room above every count seen. The file limit is the
fact log's blob limit, since the file is kept as a blob.

## Related

- [ADR-0031](../decisions/0031-snapshot-ir.md): the decision this page serves
- [import-format.md](import-format.md): the external formats
- [fact-format.md](fact-format.md): how an import is recorded and folded
- [soul-mechanics.md](soul-mechanics.md): the rules admission does not apply
