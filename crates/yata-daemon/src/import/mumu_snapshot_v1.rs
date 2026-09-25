//! `mumu-snapshot-v1` (`spec/import-format.md`): its header rule, its fields, names, nulls, and
//! units, and its normalization into the IR. Nothing outside this module knows any of them.

use serde_json::{Map, Value};
use yata_core::fact::Digest;
use yata_core::import::ir::{
    Assets, Completeness, Currency, FormatTag, GamePreset, GamePresets, Guild, IrError, Label,
    Provenance, RealmCardKind, RealmCardRecord, RolledSub, SchemaVersion, Section, SectionKind,
    SetName, ShikigamiRecord, ShikigamiRoster, SoulRecord, Souls, SourceFormat, SourceId,
    SpeciesNumber, Valued, YataSnapshot, limits,
};
use yata_core::soul::SoulAttribute;

use super::{ImportError, Kind, Normalized, Problem, SourceDefect, SourceReason, partial_if};

const TAG: &str = "mumu-snapshot-v1";
const COMPLETE: &str = "complete";

/// Recognised when the top-level `format` is this format's tag.
pub(super) fn recognises(header: &Map<String, Value>) -> bool {
    header.get("format").and_then(Value::as_str) == Some(TAG)
}

/// The format's attribute names. A rate is written as a fraction and becomes percentage points;
/// a flat value and speed are written as the game shows them.
fn attribute(name: &str) -> Option<(SoulAttribute, f64)> {
    use SoulAttribute::*;
    Some(match name {
        "attack_flat" => (AtkFlat, 1.0),
        "attack_rate" => (AtkPercent, 100.0),
        "defense_flat" => (DefFlat, 1.0),
        "defense_rate" => (DefPercent, 100.0),
        "hp_flat" => (HpFlat, 1.0),
        "hp_rate" => (HpPercent, 100.0),
        "speed" => (Spd, 1.0),
        "crit_rate" => (Crit, 100.0),
        "crit_damage" => (CritDmg, 100.0),
        "effect_hit" => (EffectHit, 100.0),
        "effect_resist" => (EffectRes, 100.0),
        _ => return None,
    })
}

/// The format's currency keys.
fn currency(name: &str) -> Option<Currency> {
    use Currency::*;
    Some(match name {
        "action_point" => ActionPoint,
        "ar_amulet" => ArAmulet,
        "auto_point" => AutoPoint,
        "broken_amulet" => BrokenAmulet,
        "coin" => Coin,
        "contrib" => Contrib,
        "demon_soul" => DemonSoul,
        "foolery_pass" => FooleryPass,
        "gold_ofuda" => GoldOfuda,
        "honor" => Honor,
        "jade" => Jade,
        "medal" => Medal,
        "mystery_amulet" => MysteryAmulet,
        "ofuda" => Ofuda,
        "realm_raid_pass" => RealmRaidPass,
        "reverse_scale" => ReverseScale,
        "s_jade" => SJade,
        "scale" => Scale,
        "skin_token" => SkinToken,
        "sp_skin_token" => SpSkinToken,
        "totem_pass" => TotemPass,
        _ => return None,
    })
}

// ---- file level ---------------------------------------------------------------------------

fn top<'v, T>(
    file: &'v Map<String, Value>,
    field: &'static str,
    kind: Kind,
    read: fn(&'v Value) -> Option<T>,
) -> Result<T, ImportError> {
    let v = file.get(field).ok_or(ImportError::Shape {
        field,
        problem: Problem::Missing,
    })?;
    read(v).ok_or(ImportError::Shape {
        field,
        problem: Problem::WrongKind { expected: kind },
    })
}

/// Which sections the file says it holds.
struct Scope {
    souls: bool,
    heroes: bool,
    assets: bool,
    guild: bool,
}

fn scope(file: &Map<String, Value>) -> Result<Scope, ImportError> {
    let s = top(file, "scope", Kind::Object, Value::as_object)?;
    let flag = |key: &str| -> Result<bool, ImportError> {
        let shape = |problem| ImportError::Shape {
            field: "scope",
            problem,
        };
        s.get(key)
            .ok_or(shape(Problem::Missing))?
            .as_bool()
            .ok_or(shape(Problem::WrongKind {
                expected: Kind::Bool,
            }))
    };
    Ok(Scope {
        souls: flag("souls")?,
        heroes: flag("heroes")?,
        assets: flag("items")? && flag("realmCards")?,
        guild: flag("guild")?,
    })
}

/// A section the scope includes, built from its records; `Absent` otherwise. It is complete, as
/// the file says, unless normalization left a record out.
fn section<T>(
    held: bool,
    defects: &mut Vec<SourceDefect>,
    build: impl FnOnce(&mut Vec<SourceDefect>) -> Result<T, ImportError>,
) -> Result<Section<T>, ImportError> {
    if !held {
        return Ok(Section::Absent);
    }
    let before = defects.len();
    let value = build(defects)?;
    let lost = defects.len() > before;
    Ok(partial_if(
        Section::Present {
            completeness: Completeness::Complete,
            value,
        },
        lost,
    ))
}

fn bounded(section: SectionKind, count: usize, limit: usize) -> Result<(), ImportError> {
    if count > limit {
        return Err(ImportError::Ir(IrError::TooMany {
            section,
            count,
            limit,
        }));
    }
    Ok(())
}

/// Normalize each record, keeping the ones that normalize and reporting the rest.
fn records<I: Iterator, T>(
    section: SectionKind,
    items: I,
    defects: &mut Vec<SourceDefect>,
    f: impl Fn(I::Item) -> Result<T, SourceReason>,
) -> Vec<T> {
    let mut out = Vec::new();
    for (index, item) in items.enumerate() {
        match f(item) {
            Ok(v) => out.push(v),
            Err(reason) => defects.push(SourceDefect {
                section,
                index,
                reason,
            }),
        }
    }
    out
}

pub(super) fn normalize(
    file: &Map<String, Value>,
    original: Digest,
) -> Result<Normalized, ImportError> {
    let completeness = top(file, "completeness", Kind::String, Value::as_str)?;
    if completeness != COMPLETE {
        return Err(ImportError::UnsupportedSourceValue {
            field: "completeness",
            value: completeness.to_owned(),
        });
    }
    let captured_at = top(file, "capturedAt", Kind::String, Value::as_str)?.to_owned();
    let scope = scope(file)?;
    let mut left_out = Vec::new();

    let souls = section(scope.souls, &mut left_out, |d| {
        let list = top(file, "hero_equips", Kind::Array, Value::as_array)?;
        bounded(SectionKind::Souls, list.len(), limits::SOULS)?;
        Ok(Souls {
            souls: records(SectionKind::Souls, list.iter(), d, soul),
        })
    })?;
    // The presets ride with the souls: the file keeps them beside the souls they name.
    let presets = section(scope.souls, &mut left_out, |d| {
        let list = top(file, "equipPresets", Kind::Array, Value::as_array)?;
        bounded(SectionKind::Presets, list.len(), limits::PRESETS)?;
        Ok(GamePresets {
            presets: records(SectionKind::Presets, list.iter(), d, preset),
        })
    })?;
    let shikigami = section(scope.heroes, &mut left_out, |d| {
        let map = top(file, "heroes", Kind::Object, Value::as_object)?;
        bounded(SectionKind::Shikigami, map.len(), limits::SHIKIGAMI)?;
        Ok(ShikigamiRoster {
            instances: records(SectionKind::Shikigami, map.iter(), d, |(k, v)| hero(k, v)),
        })
    })?;
    let assets = section(scope.assets, &mut left_out, |d| {
        let money = top(file, "currency", Kind::Object, Value::as_object)?;
        let cards = top(file, "realmCards", Kind::Array, Value::as_array)?;
        bounded(SectionKind::Assets, money.len(), limits::CURRENCIES)?;
        bounded(SectionKind::Assets, cards.len(), limits::REALM_CARDS)?;
        Ok(Assets {
            currencies: records(SectionKind::Assets, money.iter(), d, |(k, v)| {
                let c = currency(k).ok_or_else(|| SourceReason::UnsupportedValue {
                    field: "currency".to_owned(),
                    value: k.clone(),
                })?;
                let n = v
                    .as_u64()
                    .ok_or_else(|| malformed(k, Kind::NonNegativeInteger))?;
                Ok((c, n))
            }),
            realm_cards: records(SectionKind::Assets, cards.iter(), d, realm_card),
        })
    })?;
    let guild = section(scope.guild, &mut left_out, |_| {
        let g = top(file, "guild", Kind::Object, Value::as_object)?;
        let count = |key: &str| -> Result<u32, ImportError> {
            let shape = |problem| ImportError::Shape {
                field: "guild",
                problem,
            };
            g.get(key)
                .ok_or(shape(Problem::Missing))?
                .as_u64()
                .and_then(|n| u32::try_from(n).ok())
                .ok_or(shape(Problem::WrongKind {
                    expected: Kind::NonNegativeInteger,
                }))
        };
        Ok(Guild {
            level: count("level")?,
            member_count: count("activeMemberCount")?,
        })
    })?;

    Ok(Normalized {
        snapshot: YataSnapshot {
            schema: SchemaVersion::CURRENT,
            provenance: Provenance {
                format: SourceFormat::Community(FormatTag::MumuSnapshotV1),
                original,
            },
            captured_at: Some(captured_at),
            // What in the file identifies the account is not established (import-format.md).
            account: None,
            souls,
            shikigami,
            presets,
            assets,
            guild,
        },
        left_out,
    })
}

// ---- record level -------------------------------------------------------------------------

fn malformed(field: &str, expected: Kind) -> SourceReason {
    SourceReason::Malformed {
        field: field.to_owned(),
        problem: Problem::WrongKind { expected },
    }
}

/// A path inside a record: `key` under `at`.
fn path(at: &str, key: &str) -> String {
    if at.is_empty() {
        key.to_owned()
    } else {
        format!("{at}.{key}")
    }
}

fn get<'v, T>(
    o: &'v Value,
    at: &str,
    key: &str,
    kind: Kind,
    read: fn(&'v Value) -> Option<T>,
) -> Result<T, SourceReason> {
    let v = o.get(key).ok_or_else(|| SourceReason::Malformed {
        field: path(at, key),
        problem: Problem::Missing,
    })?;
    read(v).ok_or_else(|| malformed(&path(at, key), kind))
}

fn text<T, E>(field: &str, t: &str, make: fn(&str) -> Result<T, E>) -> Result<T, SourceReason> {
    make(t).map_err(|_| SourceReason::Text {
        field: field.to_owned(),
    })
}

/// An attribute name and its value, in display units.
fn valued(o: &Value, at: &str, name_key: &str, value_key: &str) -> Result<Valued, SourceReason> {
    let name = get(o, at, name_key, Kind::String, Value::as_str)?;
    let (attribute, scale) = attribute(name).ok_or_else(|| SourceReason::UnsupportedValue {
        field: path(at, name_key),
        value: name.to_owned(),
    })?;
    Ok(Valued {
        attribute,
        value: get(o, at, value_key, Kind::Number, Value::as_f64)? * scale,
    })
}

/// One entry of `subAttributes`: a rolled sub-attribute, or the innate attribute.
enum Entry {
    Rolled(RolledSub),
    Innate(SoulAttribute),
}

fn entry(o: &Value, index: usize) -> Result<Entry, SourceReason> {
    let at = format!("subAttributes[{index}]");
    if !o.is_object() {
        return Err(malformed(&at, Kind::Object));
    }
    let valued = valued(o, &at, "type", "value")?;
    let field = path(&at, "enhancementCount");
    let count = o
        .get("enhancementCount")
        .ok_or_else(|| SourceReason::Malformed {
            field: field.clone(),
            problem: Problem::Missing,
        })?;
    match o.get("fixedAttribute") {
        // A rolled sub-attribute: a count, or null when the file leaves it out.
        None => {
            let rolls = match count {
                Value::Null => None,
                n => Some(n.as_i64().ok_or_else(|| malformed(&field, Kind::Integer))?),
            };
            Ok(Entry::Rolled(RolledSub { valued, rolls }))
        }
        // The innate attribute: never rolled, and its value not kept.
        Some(Value::Bool(true)) => {
            if !count.is_null() {
                return Err(malformed(&field, Kind::Null));
            }
            Ok(Entry::Innate(valued.attribute))
        }
        Some(_) => Err(SourceReason::UnknownSubAttribute { index }),
    }
}

fn soul(v: &Value) -> Result<SoulRecord, SourceReason> {
    if !v.is_object() {
        return Err(malformed("", Kind::Object));
    }
    let entries = get(v, "", "subAttributes", Kind::Array, Value::as_array)?;
    let mut rolled = Vec::new();
    let mut innate = Vec::new();
    for (i, e) in entries.iter().enumerate() {
        match entry(e, i)? {
            Entry::Rolled(s) => rolled.push(s),
            Entry::Innate(a) => innate.push(a),
        }
    }
    let innate = match innate.as_slice() {
        [] => None,
        [one] => Some(*one),
        several => {
            return Err(SourceReason::SeveralInnate {
                count: several.len(),
            });
        }
    };
    Ok(SoulRecord {
        id: text(
            "id",
            get(v, "", "id", Kind::String, Value::as_str)?,
            SourceId::new,
        )?,
        set: text(
            "setId",
            get(v, "", "setId", Kind::String, Value::as_str)?,
            SetName::new,
        )?,
        slot: get(v, "", "slot", Kind::Integer, Value::as_i64)?,
        star: get(v, "", "quality", Kind::Integer, Value::as_i64)?,
        level: get(v, "", "level", Kind::Integer, Value::as_i64)?,
        main: valued(v, "", "mainAttrType", "mainAttrValue")?,
        rolled,
        innate,
    })
}

fn preset(v: &Value) -> Result<GamePreset, SourceReason> {
    let pair = v.as_array().ok_or_else(|| malformed("", Kind::Array))?;
    let [label, souls] = pair.as_slice() else {
        return Err(malformed("", Kind::Array));
    };
    let label = label
        .as_str()
        .ok_or_else(|| malformed("[0]", Kind::String))?;
    let souls = souls
        .as_array()
        .ok_or_else(|| malformed("[1]", Kind::Array))?;
    let ids: Vec<Option<SourceId>> = souls
        .iter()
        .enumerate()
        .map(|(i, s)| match s {
            Value::Null => Ok(None),
            Value::String(t) => text(&format!("[1][{i}]"), t, SourceId::new).map(Some),
            _ => Err(malformed(&format!("[1][{i}]"), Kind::String)),
        })
        .collect::<Result<_, _>>()?;
    let souls: [Option<SourceId>; 6] = ids.try_into().map_err(|_| malformed("[1]", Kind::Array))?;
    Ok(GamePreset {
        label: text("[0]", label, Label::new)?,
        souls,
    })
}

fn hero(key: &str, v: &Value) -> Result<ShikigamiRecord, SourceReason> {
    let number = get(v, "", "heroId", Kind::String, Value::as_str)?;
    let species = Some(number)
        .filter(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
        .and_then(|n| n.parse::<u32>().ok())
        .ok_or_else(|| SourceReason::UnsupportedValue {
            field: "heroId".to_owned(),
            value: number.to_owned(),
        })?;
    let evolved = match get(v, "", "awake", Kind::Integer, Value::as_i64)? {
        0 => false,
        1 => true,
        n => {
            return Err(SourceReason::UnsupportedValue {
                field: "awake".to_owned(),
                value: n.to_string(),
            });
        }
    };
    Ok(ShikigamiRecord {
        id: text("(key)", key, SourceId::new)?,
        species: SpeciesNumber(species),
        level: get(v, "", "level", Kind::Integer, Value::as_i64)?,
        star: get(v, "", "star", Kind::Integer, Value::as_i64)?,
        evolved,
        locked: get(v, "", "lock", Kind::Bool, Value::as_bool)?,
    })
}

fn realm_card(v: &Value) -> Result<RealmCardRecord, SourceReason> {
    let tuple = v.as_array().ok_or_else(|| malformed("", Kind::Array))?;
    let id = tuple
        .first()
        .and_then(Value::as_str)
        .ok_or_else(|| malformed("[0]", Kind::String))?;
    let kind = tuple
        .get(1)
        .and_then(Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| malformed("[1]", Kind::NonNegativeInteger))?;
    Ok(RealmCardRecord {
        id: text("[0]", id, SourceId::new)?,
        kind: RealmCardKind(kind),
    })
}
