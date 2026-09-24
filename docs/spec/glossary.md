---
kind: spec
status: current
area: domain
---

# Glossary

Canonical English terms for all game and project concepts used in code,
documentation, and test names. Each entry gives the English identifier used in
code, the Chinese game term, and the authoritative source (EN client string,
Fandom wiki, or project-defined).

When a concept is a sub-type of Soul (御魂), its identifier uses `Soul` as a
qualifier prefix — `SoulSet`, `SoulSlot`, `SoulAttribute` — to avoid clashing
with Dart and Rust standard-library names (`Set<T>`, slot APIs). Concepts
outside the Soul sub-system use bare terms: `Shikigami`, `Guild`, `Realm`.

## CN ↔ EN reference table

| 中文                   | Code identifier                | EN client string   | Notes                                                                                                                                                                                                        |
| ---------------------- | ------------------------------ | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 御魂                   | `Soul`                         | Soul               | Fandom wiki; community also uses _Mitama_                                                                                                                                                                    |
| 套装                   | `SoulSet`                      | —                  | Qualified to avoid `Set<T>` clash; identified by the game's suit code                                                                                                                                        |
| 号位                   | `SoulSlot`                     | —                  | Qualified; values `Slot1`–`Slot6`                                                                                                                                                                            |
| 主属性                 | `MainAttribute`                | Main Attribute     | Red text in UI                                                                                                                                                                                               |
| 副属性                 | `SubAttribute`                 | Sub-attribute      | Up to 4 per soul                                                                                                                                                                                             |
| 属性类型               | `SoulAttribute`                | —                  | Enum of all attribute kinds                                                                                                                                                                                  |
| 特殊属性               | —                              | Special Attribute  | SPD, Effect HIT/RES, Crit, Crit DMG                                                                                                                                                                          |
| 普通属性               | —                              | Ordinary Attribute | ATK/DEF/HP flat and % variants                                                                                                                                                                               |
| 星级                   | `star` (u8, 1–6)               | Star               | Never "quality" or "rarity"                                                                                                                                                                                  |
| 强化                   | `strengthening`                | Strengthening      | Level 0 → +15                                                                                                                                                                                                |
| 弃置                   | `discard`                      | Discard            | Marking a soul to be exchanged at the Altar                                                                                                                                                                  |
| 强化方案               | `StrengtheningSchemeSet`       | —                  | A container of named strengthening plans; one code carries the whole set                                                                                                                                     |
| 强化子方案             | `StrengtheningPlan`            | —                  | One named set of selection conditions inside a strengthening scheme set                                                                                                                                      |
| 弃置方案               | `DiscardScheme`                | —                  | The same, for discard selection                                                                                                                                                                              |
| 方案码                 | `SchemeCode`                   | —                  | The share form of a scheme: a QR code carrying a Base64 string, which the game emits and accepts back                                                                                                        |
| 筛选条件               | `SoulSelection`                | —                  | The filter groups of a scheme, a plan, or the soul filter panel (`scheme-code.md`)                                                                                                                           |
| 固有属性               | `InnateAttribute`              | —                  | A boss soul's innate attribute; one of six `SoulAttribute`s. The import preview labels it 固定属性                                                                                                           |
| 数量                   | `SubCount`                     | —                  | The filter's sub-attribute count group: 不足2条, 2条, 3条, 4条. The community's 几条腿 names the same group; what it counts is open                                                                          |
| 等级 (筛选)            | `LevelBand`                    | —                  | The filter's level bands: 0–2, 3–5, 6–8, 9–11, 12–14, 15                                                                                                                                                     |
| 强化节点               | `roll`                         | —                  | Triggered at +3/6/9/12/15                                                                                                                                                                                    |
| 强化次数（单条副属性） | `enhancement_count`            | —                  | Per sub-attribute roll count                                                                                                                                                                                 |
| 胚子                   | `PristineSoul`                 | —                  | Project-defined; a +0 soul with 4 initial sub-attributes                                                                                                                                                     |
| 套装效果               | `SetEffect`                    | Set Effect         | 2-piece and 4-piece variants                                                                                                                                                                                 |
| 奉纳                   | —                              | —                  | Offering souls for a chance of the 神赐 reward; rates in [soul-mechanics](soul-mechanics.md#acquisition). No EN client string sourced yet                                                                    |
| 腿（三腿、四腿）       | `initial_sub_count`            | —                  | Community term: one sub-attribute; 三腿 / 四腿 is a +0 soul with three / four. Below four, a roll may add a leg or strengthen an existing one, by chance; see [soul-mechanics](soul-mechanics.md#rolls)      |
| 双速                   | —                              | —                  | Community term: a Slot 2 soul whose main attribute and a sub-attribute are both SPD. Predicate `double_speed`; see [soul-mechanics](soul-mechanics.md#community-predicates)                                  |
| 拉满                   | —                              | —                  | Community term: all five rolls of a +15 soul landed on one sub-attribute (暴击拉满, 速度拉满). Predicate `maxed`; see [soul-mechanics](soul-mechanics.md#community-predicates)                               |
| 显示值 / 实际值        | displayed value / stored value | —                  | The game shows sub-attribute values rounded; the stored value has a fraction. Rules use stored values only; see [soul-mechanics](soul-mechanics.md#display)                                                  |
| 顶段                   | —                              | —                  | Project term: a maxed sub-attribute within 1 of its theoretical maximum. Predicate `top_band`; see [soul-mechanics](soul-mechanics.md#community-predicates)                                                  |
| 真 17 速               | —                              | —                  | Community term: a stored SPD sub-attribute of at least 17.0, as opposed to a 16.5 that displays as 17. Predicate `true(s, Spd, 17)`; see [soul-mechanics](soul-mechanics.md#community-predicates)            |
| 欧皇降临               | —                              | —                  | The game's celebration of an exceptionally lucky result. Its trigger is not sourced (searched 2026-09-23); the project uses it only as the anchor for the UR score tier (PRP-0002), never as a rule          |
| 式神                   | `Shikigami`                    | Shikigami          | EN client string                                                                                                                                                                                             |
| 觉醒                   | `evolution` / `Evolution`      | Evolution          | EN client string; community uses _Awakening_ — we follow the client                                                                                                                                          |
| 觉醒素材               | `EvoMaterial`                  | Evo Materials      | EN client string                                                                                                                                                                                             |
| 碎片                   | `Shard`                        | Shard              | Community term; JP: 欠片                                                                                                                                                                                     |
| 寮                     | `Guild`                        | Guild              | EN client string (陰陽寮)                                                                                                                                                                                    |
| 结界                   | `Realm`                        | Realm              | EN client string                                                                                                                                                                                             |
| 结界卡                 | `RealmCard`                    | Realm Card         | EN client string                                                                                                                                                                                             |
| 结界突破               | `RealmRaid`                    | Realm Raid         | EN client string                                                                                                                                                                                             |
| 百鬼夜行               | `DemonParade`                  | Demon Parade       | EN client string                                                                                                                                                                                             |
| 逢魔之时               | `DemonEncounter`               | Demon Encounter    | EN client string                                                                                                                                                                                             |
| 角色档案               | `GameProfile`                  | —                  | Project-defined; represents one game account                                                                                                                                                                 |
| 快照                   | `Snapshot`                     | —                  | Project-defined; immutable import record                                                                                                                                                                     |
| 库存投影               | `InventoryItem`                | —                  | Project-defined; current-state view over snapshots                                                                                                                                                           |
| 采集事件               | `AcquisitionEvent`             | —                  | Project-defined; one import session                                                                                                                                                                          |
| 神龛                   | `Altar`                        | Altar              | Where discarded souls are exchanged for gold                                                                                                                                                                 |
| 藏宝阁                 | `Cbg`                          | —                  | NetEase's account and item trading platform (cbg.163.com); no EN client string, and the legacy codebase used `cbg` throughout                                                                                |
| 道具                   | `Item`                         | —                  | Project-defined; a consumable resource tracked in the game inventory. The legacy import covered 21 categories; the authoritative list must be sourced from the game or EN wiki before this row is finalized. |
| 游戏资产               | `GameAsset`                    | —                  | Project feature: the combined inventory of `Item`s and `RealmCard`s readable from the probe                                                                                                                  |

## Soul slot main-attribute rules

Every Soul occupies one of six slots on a Shikigami. Slots 1, 3, and 5 have a
fixed main attribute; slots 2, 4, and 6 have a variable main attribute chosen at
acquisition. Sub- attributes are position-independent — any type may appear in
any slot.

| Slot    | Main attribute options                      |
| ------- | ------------------------------------------- |
| `Slot1` | ATK flat (fixed)                            |
| `Slot2` | SPD · ATK% · DEF% · HP%                     |
| `Slot3` | DEF flat (fixed)                            |
| `Slot4` | Effect HIT · Effect RES · ATK% · DEF% · HP% |
| `Slot5` | HP flat (fixed)                             |
| `Slot6` | Crit · Crit DMG · ATK% · DEF% · HP%         |

Speed as a main attribute appears only on Slot2. This is the domain basis for
the "speed-head" concept in the head/tail analysis feature.

## SoulAttribute enum values

Used wherever an attribute type is referenced in code. The special attributes
(non-percentage stats other than the flat trio) are the primary targets of
scoring weight configuration.

| Identifier   | Category | CN         |
| ------------ | -------- | ---------- |
| `AtkFlat`    | Ordinary | 攻击力     |
| `AtkPercent` | Ordinary | 攻击力加成 |
| `DefFlat`    | Ordinary | 防御力     |
| `DefPercent` | Ordinary | 防御力加成 |
| `HpFlat`     | Ordinary | 生命值     |
| `HpPercent`  | Ordinary | 生命值加成 |
| `Spd`        | Special  | 速度       |
| `EffectHit`  | Special  | 效果命中   |
| `EffectRes`  | Special  | 效果抵抗   |
| `Crit`       | Special  | 暴击       |
| `CritDmg`    | Special  | 暴击伤害   |

## PristineSoul definition

A `PristineSoul` is a Soul at level +0 that started with four sub-attributes
(initial sub-attribute count = 4). The term is project-defined; it has no
official EN game equivalent (community Chinese term: 胚子). The decision
question for a PristineSoul is whether the four sub-attribute types and their
initial values make it worth strengthening to +15.

## Notes on contested terms

**Evolution vs. Awakening**: The EN client button reads "Evolution"; community
English guides say "Awakening". Code and documentation in this project use
`Evolution` / `evolution` throughout. The CN term is 觉醒. Any occurrence of
"Awakening" in code is a defect.

**Star vs. Quality**: The wiki and client use "star" (1★–6★). A prior tool used
`quality` for this field. This project uses `star` exclusively. `quality` is not
a synonym here.

**Mitama vs. Soul**: Both appear in the English-speaking community. Code uses
`Soul` (the Fandom wiki's primary term). The word Mitama may appear in
user-facing copy if it aids recognition, but identifiers are `Soul`-prefixed.
