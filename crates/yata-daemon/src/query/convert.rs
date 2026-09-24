//! Core-protocol messages to the core's values: the souls of a supplied inventory and a query.
//!
//! Conversion only. What a wire value can hold and a domain value cannot — an unknown enum number,
//! a missing node, a suit code past a byte — is refused here; whether a tree means something is
//! [`yata_core::query::compile`]'s. Every repeated field is bounded before it is walked.

use std::collections::{BTreeMap, BTreeSet};

use yata_core::query::{
    Direction, EnumValue, Expr, Field, Limit, MAX_EXPR_DEPTH, MAX_EXPR_NODES, MAX_SORT_KEYS,
    MAX_TEST_VALUES, ParamSetRef, QualityComponent, QueryError, SchemeRef, SortKey, SoulQuery,
    Test,
};
use yata_core::scheme::selection::{
    InnateAttribute, LevelBand, SetChoice, SoulSelection, SubAttributeMode, SubCount,
};
use yata_core::soul::{Innate, Soul, SoulAttribute, SoulSet, SoulSlot, SubAttribute};
use yata_protocol::core as wire;

use super::{MAX_INVENTORY_SOULS, RequestError, WireProblem};

/// The longest soul id accepted, in bytes (`core.proto`, `Soul.soul_id`).
pub const MAX_SOUL_ID_BYTES: usize = 64;

/// The sub-attributes one soul may list: one per attribute at most.
const MAX_SUBS: usize = SoulAttribute::ALL.len();

fn malformed(p: WireProblem) -> RequestError {
    RequestError::Wire(p)
}

/// The inventory keyed by soul id; a repeated id is refused, never resolved by position.
pub fn inventory(souls: Vec<wire::Soul>) -> Result<BTreeMap<String, Soul>, RequestError> {
    if souls.len() > MAX_INVENTORY_SOULS {
        return Err(RequestError::InventoryTooLarge { souls: souls.len() });
    }
    let mut out = BTreeMap::new();
    for (index, s) in souls.into_iter().enumerate() {
        let id = s.soul_id.clone();
        if id.is_empty() || id.len() > MAX_SOUL_ID_BYTES {
            return Err(RequestError::BadSoul {
                index,
                problem: WireProblem::SoulId,
            });
        }
        let soul = soul(s).map_err(|problem| RequestError::BadSoul { index, problem })?;
        if out.insert(id, soul).is_some() {
            return Err(RequestError::RepeatedSoulId { index });
        }
    }
    Ok(out)
}

fn soul(s: wire::Soul) -> Result<Soul, WireProblem> {
    if s.subs.len() > MAX_SUBS {
        return Err(WireProblem::TooManySubs);
    }
    let subs = s
        .subs
        .iter()
        .map(|sub| {
            Ok(SubAttribute {
                attribute: attribute(sub.attribute)?,
                value: sub.value,
                enhancement_count: sub
                    .enhancement_count
                    .map(|c| u8::try_from(c).map_err(|_| WireProblem::OutOfRange))
                    .transpose()?,
            })
        })
        .collect::<Result<Vec<_>, WireProblem>>()?;
    let distinct: BTreeSet<_> = subs.iter().map(|s| s.attribute).collect();
    if distinct.len() != subs.len() {
        return Err(WireProblem::RepeatedSub);
    }
    let byte = |v: u32| u8::try_from(v).map_err(|_| WireProblem::OutOfRange);
    Ok(Soul {
        set: SoulSet::from_suit_code(byte(s.suit_code)?),
        slot: slot(s.slot)?,
        star: byte(s.star)?,
        level: byte(s.level)?,
        main: attribute(s.main)?,
        main_value: s.main_value,
        subs,
        innate: innate(s.innate)?,
    })
}

fn innate(i: Option<wire::Innate>) -> Result<Innate, WireProblem> {
    use wire::innate::State;
    // An absent message is a reading that does not say; it is never read as `Absent`.
    match i.and_then(|i| i.state) {
        None => Ok(Innate::Unknown),
        Some(State::Absent(_)) => Ok(Innate::Absent),
        Some(State::Present(a)) => attribute(a).map(Innate::Present),
    }
}

fn slot(v: i32) -> Result<SoulSlot, WireProblem> {
    use wire::SoulSlot as W;
    match W::try_from(v) {
        Ok(W::SoulSlot1) => Ok(SoulSlot::Slot1),
        Ok(W::SoulSlot2) => Ok(SoulSlot::Slot2),
        Ok(W::SoulSlot3) => Ok(SoulSlot::Slot3),
        Ok(W::SoulSlot4) => Ok(SoulSlot::Slot4),
        Ok(W::SoulSlot5) => Ok(SoulSlot::Slot5),
        Ok(W::SoulSlot6) => Ok(SoulSlot::Slot6),
        Ok(W::Unspecified) | Err(_) => Err(WireProblem::Enum { value: v }),
    }
}

pub fn attribute(v: i32) -> Result<SoulAttribute, WireProblem> {
    use wire::SoulAttribute as W;
    match W::try_from(v) {
        Ok(W::AtkFlat) => Ok(SoulAttribute::AtkFlat),
        Ok(W::AtkPercent) => Ok(SoulAttribute::AtkPercent),
        Ok(W::DefFlat) => Ok(SoulAttribute::DefFlat),
        Ok(W::DefPercent) => Ok(SoulAttribute::DefPercent),
        Ok(W::HpFlat) => Ok(SoulAttribute::HpFlat),
        Ok(W::HpPercent) => Ok(SoulAttribute::HpPercent),
        Ok(W::Spd) => Ok(SoulAttribute::Spd),
        Ok(W::EffectHit) => Ok(SoulAttribute::EffectHit),
        Ok(W::EffectRes) => Ok(SoulAttribute::EffectRes),
        Ok(W::Crit) => Ok(SoulAttribute::Crit),
        Ok(W::CritDmg) => Ok(SoulAttribute::CritDmg),
        Ok(W::Unspecified) | Err(_) => Err(WireProblem::Enum { value: v }),
    }
}

/// A slot of the wire, if the number names one.
pub fn slot_of(v: i32) -> Option<SoulSlot> {
    slot(v).ok()
}

pub fn wire_slot(k: SoulSlot) -> wire::SoulSlot {
    match k {
        SoulSlot::Slot1 => wire::SoulSlot::SoulSlot1,
        SoulSlot::Slot2 => wire::SoulSlot::SoulSlot2,
        SoulSlot::Slot3 => wire::SoulSlot::SoulSlot3,
        SoulSlot::Slot4 => wire::SoulSlot::SoulSlot4,
        SoulSlot::Slot5 => wire::SoulSlot::SoulSlot5,
        SoulSlot::Slot6 => wire::SoulSlot::SoulSlot6,
    }
}

pub fn wire_attribute(a: SoulAttribute) -> wire::SoulAttribute {
    use wire::SoulAttribute as W;
    match a {
        SoulAttribute::AtkFlat => W::AtkFlat,
        SoulAttribute::AtkPercent => W::AtkPercent,
        SoulAttribute::DefFlat => W::DefFlat,
        SoulAttribute::DefPercent => W::DefPercent,
        SoulAttribute::HpFlat => W::HpFlat,
        SoulAttribute::HpPercent => W::HpPercent,
        SoulAttribute::Spd => W::Spd,
        SoulAttribute::EffectHit => W::EffectHit,
        SoulAttribute::EffectRes => W::EffectRes,
        SoulAttribute::Crit => W::Crit,
        SoulAttribute::CritDmg => W::CritDmg,
    }
}

/// A repeated field of at most [`MAX_TEST_VALUES`] values, each converted.
fn bounded<W: Copy, T>(
    values: &[W],
    each: impl Fn(W) -> Result<T, WireProblem>,
) -> Result<Vec<T>, RequestError> {
    if values.len() > MAX_TEST_VALUES {
        return Err(too_complex(Limit::TestValues, values.len()));
    }
    values.iter().map(|&v| each(v).map_err(malformed)).collect()
}

fn suit(code: u32) -> Result<SoulSet, WireProblem> {
    u8::try_from(code)
        .map(SoulSet::from_suit_code)
        .map_err(|_| WireProblem::OutOfRange)
}

/// The query of a request, with nothing checked that `compile` checks.
pub fn query(q: wire::Query) -> Result<SoulQuery, RequestError> {
    if q.collection != wire::Collection::Souls as i32 {
        return Err(malformed(WireProblem::Collection {
            value: q.collection,
        }));
    }
    if q.sort.len() > MAX_SORT_KEYS {
        return Err(too_complex(Limit::SortKeys, q.sort.len()));
    }
    let sort = q.sort.iter().map(sort_key).collect::<Result<Vec<_>, _>>()?;
    let mut nodes = 0;
    Ok(SoulQuery {
        filter: q.filter.map(|e| expr(e, 1, &mut nodes)).transpose()?,
        sort,
        params: q.params.map(|p| ParamSetRef {
            id: p.id,
            version: p.version,
        }),
    })
}

fn sort_key(k: &wire::SortKey) -> Result<SortKey, RequestError> {
    let direction = match wire::Direction::try_from(k.direction) {
        Ok(wire::Direction::Asc) => Direction::Asc,
        Ok(wire::Direction::Desc) => Direction::Desc,
        _ => return Err(malformed(WireProblem::Enum { value: k.direction })),
    };
    let field = field(
        k.field
            .as_ref()
            .ok_or(malformed(WireProblem::Missing("SortKey.field")))?,
    )?;
    Ok(SortKey { field, direction })
}

fn field(f: &wire::Field) -> Result<Field, RequestError> {
    use wire::FieldName as N;
    let name = N::try_from(f.name).map_err(|_| RequestError::UnknownField { value: f.name })?;
    let takes_attribute = matches!(name, N::SubValue | N::HasSub);
    let given = f.attribute != wire::SoulAttribute::Unspecified as i32;
    if takes_attribute != given {
        return Err(malformed(WireProblem::FieldAttribute { field: f.name }));
    }
    let a = || attribute(f.attribute).map_err(malformed);
    Ok(match name {
        N::Unspecified => return Err(RequestError::UnknownField { value: f.name }),
        N::Set => Field::Set,
        N::Slot => Field::Slot,
        N::Star => Field::Star,
        N::Level => Field::Level,
        N::MainAttribute => Field::MainAttribute,
        N::MainValue => Field::MainValue,
        N::SubValue => Field::SubValue(a()?),
        N::HasSub => Field::HasSub(a()?),
        N::SubCount => Field::SubCount,
        N::Pristine => Field::Pristine,
        N::QualityTotal => Field::Quality(QualityComponent::Total),
        N::QualityDepth => Field::Quality(QualityComponent::Depth),
        N::QualityBreadth => Field::Quality(QualityComponent::Breadth),
    })
}

/// A filter node. The walk stops at the core's node and depth limits, so a large tree costs no
/// more to refuse than a tree at the limit; `compile` checks the same limits on what comes out.
fn expr(e: wire::Expr, depth: usize, nodes: &mut usize) -> Result<Expr, RequestError> {
    use wire::expr::Kind;
    *nodes += 1;
    if *nodes > MAX_EXPR_NODES {
        return Err(too_complex(Limit::ExprNodes, *nodes));
    }
    if depth > MAX_EXPR_DEPTH {
        return Err(too_complex(Limit::ExprDepth, depth));
    }
    let mut list = |l: wire::ExprList| {
        l.items
            .into_iter()
            .map(|e| expr(e, depth + 1, nodes))
            .collect::<Result<Vec<_>, _>>()
    };
    match e.kind.ok_or(malformed(WireProblem::Missing("Expr.kind")))? {
        Kind::And(l) => list(l).map(Expr::And),
        Kind::Or(l) => list(l).map(Expr::Or),
        Kind::Not(e) => {
            list(wire::ExprList { items: vec![*e] }).map(|mut v| Expr::Not(Box::new(v.remove(0))))
        }
        Kind::Pred(p) => predicate(p),
        Kind::Matches(s) => selection(&s).map(Expr::Matches),
        Kind::MatchesScheme(r) => Ok(Expr::MatchesScheme(SchemeRef {
            code: r.code,
            entry: r.entry.and_then(|e| usize::try_from(e).ok()),
        })),
    }
}

fn too_complex(limit: Limit, actual: usize) -> RequestError {
    RequestError::Query(QueryError::TooComplex { limit, actual })
}

fn predicate(p: wire::Predicate) -> Result<Expr, RequestError> {
    use wire::predicate::Test as W;
    let field = field(
        p.field
            .as_ref()
            .ok_or(malformed(WireProblem::Missing("Predicate.field")))?,
    )?;
    let test = match p
        .test
        .ok_or(malformed(WireProblem::Missing("Predicate.test")))?
    {
        W::In(t) => Test::In(in_values(&t)?),
        W::IntRange(r) => Test::IntRange {
            min: r.min,
            max: r.max,
        },
        W::NumberRange(r) => Test::NumberRange {
            min: r.min,
            max: r.max,
        },
        W::Is(b) => Test::Is(b),
    };
    Ok(Expr::Pred(field, test))
}

/// The values of an `In`: the one non-empty list. Values in two lists cannot fit one field, so
/// they reach `compile` together and are refused there as a type mismatch.
fn in_values(t: &wire::InTest) -> Result<Vec<EnumValue>, RequestError> {
    let total = t.suit_codes.len() + t.slots.len() + t.attributes.len();
    if total > MAX_TEST_VALUES {
        return Err(too_complex(Limit::TestValues, total));
    }
    let mut out = bounded(&t.suit_codes, |c| suit(c).map(EnumValue::Set))?;
    out.extend(bounded(&t.slots, |k| slot(k).map(EnumValue::Slot))?);
    out.extend(bounded(&t.attributes, |a| {
        attribute(a).map(EnumValue::Attribute)
    })?);
    Ok(out)
}

fn level_band(v: i32) -> Result<LevelBand, WireProblem> {
    use wire::LevelBand as W;
    match W::try_from(v) {
        Ok(W::LevelBand0To2) => Ok(LevelBand::L0to2),
        Ok(W::LevelBand3To5) => Ok(LevelBand::L3to5),
        Ok(W::LevelBand6To8) => Ok(LevelBand::L6to8),
        Ok(W::LevelBand9To11) => Ok(LevelBand::L9to11),
        Ok(W::LevelBand12To14) => Ok(LevelBand::L12to14),
        Ok(W::LevelBand15) => Ok(LevelBand::L15),
        Ok(W::Unspecified) | Err(_) => Err(WireProblem::Enum { value: v }),
    }
}

fn sub_count(v: i32) -> Result<SubCount, WireProblem> {
    use wire::SubCount as W;
    match W::try_from(v) {
        Ok(W::FewerThanTwo) => Ok(SubCount::FewerThanTwo),
        Ok(W::Two) => Ok(SubCount::Two),
        Ok(W::Three) => Ok(SubCount::Three),
        Ok(W::Four) => Ok(SubCount::Four),
        Ok(W::Unspecified) | Err(_) => Err(WireProblem::Enum { value: v }),
    }
}

/// An inline official filter.
pub fn selection(s: &wire::SoulSelection) -> Result<SoulSelection, RequestError> {
    let sets = match (s.any_set, s.suit_codes.as_slice()) {
        (true, []) => SetChoice::AnySet,
        (true, _) => return Err(malformed(WireProblem::AnySetWithSets)),
        (false, codes) => SetChoice::Sets(bounded(codes, suit)?.into_iter().collect()),
    };
    let mut out = SoulSelection::new(sets);
    out.slots = bounded(&s.slots, slot)?.into_iter().collect();
    out.stars = bounded(&s.stars, |v| {
        u8::try_from(v).map_err(|_| WireProblem::OutOfRange)
    })?
    .into_iter()
    .collect();
    out.levels = bounded(&s.levels, level_band)?.into_iter().collect();
    out.main_attributes = bounded(&s.main_attributes, attribute)?
        .into_iter()
        .collect();
    out.innate = bounded(&s.innate, |v| {
        attribute(v).and_then(|a| InnateAttribute::new(a).ok_or(WireProblem::NotInnate))
    })?
    .into_iter()
    .collect();
    let included = bounded(&s.sub_included, attribute)?;
    let excluded = bounded(&s.sub_excluded, attribute)?;
    if included.iter().any(|a| excluded.contains(a)) {
        return Err(malformed(WireProblem::IncludedAndExcluded));
    }
    for a in included {
        out.sub_attributes.set(a, SubAttributeMode::Include);
    }
    for a in excluded {
        out.sub_attributes.set(a, SubAttributeMode::Exclude);
    }
    out.sub_counts = bounded(&s.sub_counts, sub_count)?.into_iter().collect();
    Ok(out)
}
