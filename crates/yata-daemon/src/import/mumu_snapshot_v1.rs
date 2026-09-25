//! `mumu-snapshot-v1` (`spec/import-format.md`): its header rule, its fields, its attribute
//! names, and its units. Nothing outside this module knows them.

use serde_json::{Map, Value};
use yata_core::import::snapshot::{SnapshotSoul, SnapshotSub, Valued};
use yata_core::soul::SoulAttribute;

use super::{ImportError, Kind, Parsed, Problem, Record, RecordReason};

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

pub(super) fn parse(file: &Map<String, Value>) -> Result<Parsed, ImportError> {
    let completeness = top(file, "completeness", Kind::String, Value::as_str)?;
    if completeness != COMPLETE {
        return Err(ImportError::NotComplete {
            stated: completeness.to_owned(),
        });
    }
    let captured_at = top(file, "capturedAt", Kind::String, Value::as_str)?.to_owned();
    let souls = top(file, "hero_equips", Kind::Array, Value::as_array)?;
    Ok(Parsed {
        captured_at,
        records: souls
            .iter()
            .map(|v| Record {
                id: v.get("id").and_then(Value::as_str).map(str::to_owned),
                soul: soul(v),
            })
            .collect(),
    })
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
) -> Result<T, RecordReason> {
    let v = o.get(key).ok_or_else(|| RecordReason::Shape {
        field: path(at, key),
        problem: Problem::Missing,
    })?;
    read(v).ok_or_else(|| RecordReason::Shape {
        field: path(at, key),
        problem: Problem::WrongKind { expected: kind },
    })
}

/// An attribute name and its value, in display units.
fn valued(o: &Value, at: &str, name_key: &str, value_key: &str) -> Result<Valued, RecordReason> {
    let name = get(o, at, name_key, Kind::String, Value::as_str)?;
    let (attribute, scale) = attribute(name).ok_or_else(|| RecordReason::UnknownAttribute {
        field: path(at, name_key),
        name: name.to_owned(),
    })?;
    Ok(Valued {
        attribute,
        value: get(o, at, value_key, Kind::Number, Value::as_f64)? * scale,
    })
}

/// One entry of `subAttributes`: a rolled sub-attribute, or the innate attribute.
enum Entry {
    Rolled(SnapshotSub),
    Innate(SoulAttribute),
}

fn entry(o: &Value, index: usize) -> Result<Entry, RecordReason> {
    let at = format!("subAttributes[{index}]");
    if !o.is_object() {
        return Err(RecordReason::Shape {
            field: at,
            problem: Problem::WrongKind {
                expected: Kind::Object,
            },
        });
    }
    let valued = valued(o, &at, "type", "value")?;
    let count = o
        .get("enhancementCount")
        .ok_or_else(|| RecordReason::Shape {
            field: path(&at, "enhancementCount"),
            problem: Problem::Missing,
        })?;
    let wrong = |expected| RecordReason::Shape {
        field: path(&at, "enhancementCount"),
        problem: Problem::WrongKind { expected },
    };
    match o.get("fixedAttribute") {
        // A rolled sub-attribute: a count, or null when the file leaves it out.
        None => {
            let rolls = match count {
                Value::Null => None,
                n => Some(n.as_u64().ok_or_else(|| wrong(Kind::NonNegativeInteger))?),
            };
            Ok(Entry::Rolled(SnapshotSub { valued, rolls }))
        }
        // The innate attribute: never rolled.
        Some(Value::Bool(true)) => {
            if !count.is_null() {
                return Err(wrong(Kind::Null));
            }
            Ok(Entry::Innate(valued.attribute))
        }
        Some(_) => Err(RecordReason::UnknownSubAttribute { index }),
    }
}

fn soul(v: &Value) -> Result<SnapshotSoul, RecordReason> {
    if !v.is_object() {
        return Err(RecordReason::Shape {
            field: String::new(),
            problem: Problem::WrongKind {
                expected: Kind::Object,
            },
        });
    }
    let mut subs = Vec::new();
    let mut innate = Vec::new();
    for (i, e) in get(v, "", "subAttributes", Kind::Array, Value::as_array)?
        .iter()
        .enumerate()
    {
        match entry(e, i)? {
            Entry::Rolled(s) => subs.push(s),
            Entry::Innate(a) => innate.push(a),
        }
    }
    Ok(SnapshotSoul {
        id: get(v, "", "id", Kind::String, Value::as_str)?.to_owned(),
        set: get(v, "", "setId", Kind::String, Value::as_str)?.to_owned(),
        slot: get(v, "", "slot", Kind::Integer, Value::as_i64)?,
        star: get(v, "", "quality", Kind::Integer, Value::as_i64)?,
        level: get(v, "", "level", Kind::Integer, Value::as_i64)?,
        main: valued(v, "", "mainAttrType", "mainAttrValue")?,
        subs,
        innate,
    })
}
