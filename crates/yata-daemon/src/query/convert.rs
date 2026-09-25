//! Core-protocol messages to the core's values: the souls of a supplied inventory and a query.
//!
//! Conversion only. What a wire value can hold and a domain value cannot — an unknown enum number,
//! a missing node, a suit code past a byte — is refused here; whether a tree means something is
//! [`yata_core::query::compile`]'s. Every repeated field is bounded before it is walked.

use std::collections::{BTreeMap, BTreeSet};

use yata_core::fact::GameSoulId;
use yata_core::nonempty::NonEmptySet;
use yata_core::query::{
    Bound, Direction, EnumValue, Expr, Field, Limit, MAX_EXPR_DEPTH, MAX_EXPR_NODES, MAX_SORT_KEYS,
    MAX_TEST_VALUES, ParamSetId, ParamSetRef, QualityComponent, QueryError, SchemeCodeText,
    SchemeRef, SortKey, SoulQuery, Test,
};
use yata_core::scheme::selection::{
    InnateAttribute, LevelBand, SchemeSet, SetBit, SetChoice, SoulSelection, SubAttributeMode,
    SubCount,
};
use yata_core::soul::{
    Level, RollCount, Soul, SoulAttribute, SoulKind, SoulSet, SoulSlot, Star, StoredValue,
    SubAttribute,
};
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
pub fn inventory(souls: Vec<wire::Soul>) -> Result<BTreeMap<GameSoulId, Soul>, RequestError> {
    if souls.len() > MAX_INVENTORY_SOULS {
        return Err(RequestError::InventoryTooLarge { souls: souls.len() });
    }
    let mut out = BTreeMap::new();
    for (index, s) in souls.into_iter().enumerate() {
        let id = soul_id(&s.soul_id).map_err(|problem| RequestError::BadSoul { index, problem })?;
        let soul = soul(s).map_err(|problem| RequestError::BadSoul { index, problem })?;
        if out.insert(id, soul).is_some() {
            return Err(RequestError::RepeatedSoulId { index });
        }
    }
    Ok(out)
}

/// A soul id from the wire: the game's id, non-empty and at most [`MAX_SOUL_ID_BYTES`] long.
pub fn soul_id(id: &str) -> Result<GameSoulId, WireProblem> {
    if id.len() > MAX_SOUL_ID_BYTES {
        return Err(WireProblem::SoulId);
    }
    GameSoulId::new(id).map_err(|_| WireProblem::SoulId)
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
                value: stored(sub.value)?,
                enhancement_count: sub
                    .enhancement_count
                    .map(|c| {
                        u8::try_from(c)
                            .map(RollCount::new)
                            .map_err(|_| WireProblem::OutOfRange)
                    })
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
        star: star(s.star)?,
        level: Level::try_from(s.level).map_err(|_| WireProblem::OutOfRange)?,
        main: attribute(s.main)?,
        main_value: stored(s.main_value)?,
        subs,
        kind: kind(s.kind)?,
    })
}

/// A wire value as a stored value: finite and not negative, or out of range.
fn stored(v: f64) -> Result<StoredValue, WireProblem> {
    StoredValue::new(v).ok_or(WireProblem::OutOfRange)
}

/// A soul's kind. There is no unknown kind (ADR-0029): a soul that states neither is refused.
fn kind(k: Option<wire::soul::Kind>) -> Result<SoulKind, WireProblem> {
    use wire::soul::Kind;
    match k.ok_or(WireProblem::Missing("Soul.kind"))? {
        Kind::Ordinary(wire::OrdinarySoul {}) => Ok(SoulKind::Ordinary),
        Kind::Boss(b) => attribute(b.innate)
            .and_then(|a| InnateAttribute::new(a).ok_or(WireProblem::NotInnate))
            .map(SoulKind::Boss),
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

fn star(n: u32) -> Result<Star, WireProblem> {
    Star::try_from(n).map_err(|_| WireProblem::OutOfRange)
}

/// A suit code a scheme can choose.
fn scheme_set(code: u32) -> Result<SetBit, WireProblem> {
    let set = suit(code)?;
    SchemeSet::new(set)
        .map(SetBit::Mapped)
        .ok_or(WireProblem::UnmappedSet)
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
        params: q
            .params
            .map(|p| {
                let id = ParamSetId::new(p.id)
                    .ok_or(malformed(WireProblem::Missing("ParamSetRef.id")))?;
                Ok(ParamSetRef {
                    id,
                    version: p.version,
                })
            })
            .transpose()?,
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
    use wire::SimpleField as S;
    use wire::field::Field as F;
    let unknown = |value: i32| RequestError::UnknownField { value };
    match *f
        .field
        .as_ref()
        .ok_or(malformed(WireProblem::Missing("Field.field")))?
    {
        F::Simple(v) => Ok(match S::try_from(v) {
            Ok(S::Set) => Field::Set,
            Ok(S::Slot) => Field::Slot,
            Ok(S::Star) => Field::Star,
            Ok(S::Level) => Field::Level,
            Ok(S::MainAttribute) => Field::MainAttribute,
            Ok(S::MainValue) => Field::MainValue,
            Ok(S::SubCount) => Field::SubCount,
            Ok(S::Pristine) => Field::Pristine,
            Ok(S::Unspecified) | Err(_) => return Err(unknown(v)),
        }),
        F::SubValue(a) => attribute(a).map(Field::SubValue).map_err(malformed),
        F::HasSub(a) => attribute(a).map(Field::HasSub).map_err(malformed),
        F::Quality(q) => match wire::QualityComponent::try_from(q) {
            Ok(wire::QualityComponent::Total) => Ok(Field::Quality(QualityComponent::Total)),
            Ok(wire::QualityComponent::Depth) => Ok(Field::Quality(QualityComponent::Depth)),
            Ok(wire::QualityComponent::Breadth) => Ok(Field::Quality(QualityComponent::Breadth)),
            Ok(wire::QualityComponent::Unspecified) | Err(_) => Err(unknown(q)),
        },
    }
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
        Kind::Not(e) => expr(*e, depth + 1, nodes).map(|e| Expr::Not(Box::new(e))),
        Kind::Pred(p) => predicate(p),
        Kind::Matches(s) => selection(&s).map(Expr::Matches),
        Kind::MatchesScheme(r) => Ok(Expr::MatchesScheme(SchemeRef {
            code: SchemeCodeText(r.code),
            entry: r
                .entry
                .map(|e| usize::try_from(e).map_err(|_| malformed(WireProblem::OutOfRange)))
                .transpose()?,
        })),
    }
}

fn int_bound(b: Option<wire::int_range::Bound>) -> Result<Bound<i64>, RequestError> {
    use wire::int_range::Bound as B;
    Ok(
        match b.ok_or(malformed(WireProblem::Missing("IntRange.bound")))? {
            B::AtLeast(v) => Bound::AtLeast(v),
            B::AtMost(v) => Bound::AtMost(v),
            B::Between(b) => Bound::Between {
                min: b.min,
                max: b.max,
            },
        },
    )
}

fn number_bound(b: Option<wire::number_range::Bound>) -> Result<Bound<f64>, RequestError> {
    use wire::number_range::Bound as B;
    Ok(
        match b.ok_or(malformed(WireProblem::Missing("NumberRange.bound")))? {
            B::AtLeast(v) => Bound::AtLeast(v),
            B::AtMost(v) => Bound::AtMost(v),
            B::Between(b) => Bound::Between {
                min: b.min,
                max: b.max,
            },
        },
    )
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
        W::IntRange(r) => Test::IntRange(int_bound(r.bound)?),
        W::NumberRange(r) => Test::NumberRange(number_bound(r.bound)?),
        W::Is(b) => Test::Is(b),
    };
    Ok(Expr::Pred(field, test))
}

/// The values of an `In`: one list, of one type. An empty list reaches `compile`, which refuses it
/// as `In []`.
fn in_values(t: &wire::InTest) -> Result<Vec<EnumValue>, RequestError> {
    use wire::in_test::Values as V;
    match t
        .values
        .as_ref()
        .ok_or(malformed(WireProblem::Missing("InTest.values")))?
    {
        V::SetValues(c) => bounded(&c.codes, |c| suit(c).map(EnumValue::Set)),
        V::SlotValues(s) => bounded(&s.values, |k| slot(k).map(EnumValue::Slot)),
        V::AttributeValues(a) => bounded(&a.values, |a| attribute(a).map(EnumValue::Attribute)),
    }
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
    use wire::soul_selection::Sets;
    let sets = match s
        .sets
        .as_ref()
        .ok_or(malformed(WireProblem::Missing("SoulSelection.sets")))?
    {
        Sets::All(wire::AnySet {}) => SetChoice::AnySet,
        // "Every set" has one encoding, `all`: an empty list is not a second one.
        Sets::Chosen(c) => SetChoice::Sets(
            NonEmptySet::collect(bounded(&c.codes, scheme_set)?)
                .ok_or_else(|| malformed(WireProblem::NoSets))?,
        ),
    };
    let mut out = SoulSelection::new(sets);
    out.slots = bounded(&s.slots, slot)?.into_iter().collect();
    out.stars = bounded(&s.stars, star)?.into_iter().collect();
    out.levels = bounded(&s.levels, level_band)?.into_iter().collect();
    out.main_attributes = bounded(&s.main_attributes, attribute)?
        .into_iter()
        .collect();
    out.innate = bounded(&s.innate, |v| {
        attribute(v).and_then(|a| InnateAttribute::new(a).ok_or(WireProblem::NotInnate))
    })?
    .into_iter()
    .collect();
    if s.sub_attributes.len() > MAX_TEST_VALUES {
        return Err(too_complex(Limit::TestValues, s.sub_attributes.len()));
    }
    for c in &s.sub_attributes {
        let a = attribute(c.attribute).map_err(malformed)?;
        let mode = match wire::SubAttributeMode::try_from(c.mode) {
            Ok(wire::SubAttributeMode::Include) => SubAttributeMode::Include,
            Ok(wire::SubAttributeMode::Exclude) => SubAttributeMode::Exclude,
            Ok(wire::SubAttributeMode::Unspecified) | Err(_) => {
                return Err(malformed(WireProblem::Enum { value: c.mode }));
            }
        };
        if out.sub_attributes.get(a) != SubAttributeMode::Ignore {
            return Err(malformed(WireProblem::SubAttributeTwice));
        }
        out.sub_attributes.set(a, mode);
    }
    out.sub_counts = bounded(&s.sub_counts, sub_count)?.into_iter().collect();
    Ok(out)
}
