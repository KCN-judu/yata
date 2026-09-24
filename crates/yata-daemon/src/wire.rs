//! Domain values as the session's core-protocol messages (ADR-0004, rule 1): souls, selections,
//! scheme codes, QR matrices, and failures.
//!
//! Domain → wire is total: every value renders. Wire → domain validates and returns a `Result`,
//! because a client can send anything; for queries and souls that direction is
//! [`crate::query::convert`], whose attribute and slot mappings this module reuses, so each enum
//! is mapped in one place.

use yata_core::scheme::code::{DiscardScheme, SchemeCode, StrengtheningPlan};
use yata_core::scheme::selection::{
    LevelBand, SetChoice, SoulSelection, SubAttributeMode, SubCount,
};
use yata_core::soul::{Innate, Soul};
use yata_protocol::core as pb;

use crate::qr::QrMatrix;
use crate::query::convert::{wire_attribute, wire_slot};

/// A failure the session reports to the client: a stable code and an English message
/// (`core-protocol.md`, "Errors"). The code is the contract; the message is never parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    pub code: &'static str,
    pub message: String,
}

impl Failure {
    pub fn new(code: &'static str, message: impl Into<String>) -> Failure {
        Failure {
            code,
            message: message.into(),
        }
    }
}

pub fn error(f: &Failure) -> pb::Error {
    pb::Error {
        code: f.code.to_owned(),
        message: f.message.clone(),
        details: Vec::new(),
    }
}

fn attribute(a: yata_core::soul::SoulAttribute) -> i32 {
    wire_attribute(a).into()
}

fn level_band(b: LevelBand) -> pb::LevelBand {
    match b {
        LevelBand::L0to2 => pb::LevelBand::LevelBand0To2,
        LevelBand::L3to5 => pb::LevelBand::LevelBand3To5,
        LevelBand::L6to8 => pb::LevelBand::LevelBand6To8,
        LevelBand::L9to11 => pb::LevelBand::LevelBand9To11,
        LevelBand::L12to14 => pb::LevelBand::LevelBand12To14,
        LevelBand::L15 => pb::LevelBand::LevelBand15,
    }
}

fn sub_count(c: SubCount) -> pb::SubCount {
    match c {
        SubCount::FewerThanTwo => pb::SubCount::FewerThanTwo,
        SubCount::Two => pb::SubCount::Two,
        SubCount::Three => pb::SubCount::Three,
        SubCount::Four => pb::SubCount::Four,
    }
}

/// A soul of the projection, under its row identity. An unknown innate attribute is an absent
/// message, as `core.proto` reads it.
pub fn soul(id: &str, s: &Soul) -> pb::Soul {
    use pb::innate::State;
    let innate = match s.innate {
        Innate::Unknown => None,
        Innate::Absent => Some(State::Absent(true)),
        Innate::Present(a) => Some(State::Present(attribute(a))),
    };
    pb::Soul {
        soul_id: id.to_owned(),
        suit_code: u32::from(s.set.suit_code()),
        slot: wire_slot(s.slot).into(),
        star: u32::from(s.star),
        level: u32::from(s.level),
        main: attribute(s.main),
        main_value: s.main_value,
        subs: s
            .subs
            .iter()
            .map(|sub| pb::SubAttribute {
                attribute: attribute(sub.attribute),
                value: sub.value,
                enhancement_count: sub.enhancement_count.map(u32::from),
            })
            .collect(),
        innate: innate.map(|state| pb::Innate { state: Some(state) }),
    }
}

pub fn selection(s: &SoulSelection) -> pb::SoulSelection {
    let (any_set, suit_codes) = match &s.sets {
        SetChoice::AnySet => (true, Vec::new()),
        SetChoice::Sets(sets) => (
            false,
            sets.iter().map(|x| u32::from(x.suit_code())).collect(),
        ),
    };
    let marked = |mode| s.sub_attributes.with(mode).map(attribute).collect();
    pb::SoulSelection {
        any_set,
        suit_codes,
        slots: s.slots.iter().map(|&k| wire_slot(k).into()).collect(),
        stars: s.stars.iter().map(|&n| u32::from(n)).collect(),
        levels: s.levels.iter().map(|&b| level_band(b).into()).collect(),
        main_attributes: s.main_attributes.iter().map(|&a| attribute(a)).collect(),
        innate: s.innate.iter().map(|i| attribute(i.attribute())).collect(),
        sub_included: marked(SubAttributeMode::Include),
        sub_excluded: marked(SubAttributeMode::Exclude),
        sub_counts: s.sub_counts.iter().map(|&c| sub_count(c).into()).collect(),
    }
}

fn plan(p: &StrengtheningPlan) -> pb::SchemeEntry {
    pb::SchemeEntry {
        name: p.name.clone(),
        selection: Some(selection(&p.selection)),
        has_unknown_conditions: p.has_unknown_conditions(),
    }
}

fn discard(d: &DiscardScheme) -> pb::SchemeEntry {
    pb::SchemeEntry {
        name: d.name.clone(),
        selection: Some(selection(&d.selection)),
        has_unknown_conditions: d.has_unknown_conditions(),
    }
}

pub fn qr_matrix(m: &QrMatrix) -> pb::QrMatrix {
    let size = m.size();
    let modules = (0..size)
        .flat_map(|y| (0..size).map(move |x| (x, y)))
        .map(|(x, y)| u8::from(m.is_dark(x, y)))
        .collect();
    pb::QrMatrix { size, modules }
}

/// A decoded scheme code, with the text it was read from and that text's QR matrix.
pub fn scheme_code(code: &SchemeCode, text: &str, qr: Option<&QrMatrix>) -> pb::SchemeCodeDecoded {
    let (kind, entries) = match code {
        SchemeCode::Strengthening(set) => (
            pb::SchemeKind::Strengthening,
            set.plans.iter().map(plan).collect(),
        ),
        SchemeCode::Discard(schemes) => (
            pb::SchemeKind::Discard,
            schemes.iter().map(discard).collect(),
        ),
    };
    pb::SchemeCodeDecoded {
        kind: kind.into(),
        entries,
        encoded: Some(pb::EncodedScheme {
            text: text.to_owned(),
            qr: qr.map(qr_matrix),
        }),
    }
}

/// Where a scheme code to decode comes from, validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemeSource {
    Text(String),
    Png(Vec<u8>),
}

pub fn scheme_source(m: pb::DecodeSchemeCode) -> Result<SchemeSource, Failure> {
    match m.source {
        Some(pb::decode_scheme_code::Source::Text(t)) => Ok(SchemeSource::Text(t)),
        Some(pb::decode_scheme_code::Source::Png(p)) => Ok(SchemeSource::Png(p)),
        None => Err(Failure::new(
            "decode.no_input",
            "DecodeSchemeCode carries neither text nor an image",
        )),
    }
}

#[cfg(test)]
mod tests {
    use yata_core::soul::{SoulAttribute, SoulSet, SoulSlot};

    use super::*;

    fn a_soul(innate: Innate) -> Soul {
        Soul {
            set: SoulSet::from_suit_code(30),
            slot: SoulSlot::Slot2,
            star: 6,
            level: 15,
            main: SoulAttribute::Spd,
            main_value: 57.0,
            subs: vec![],
            innate,
        }
    }

    #[test]
    fn an_unknown_innate_attribute_is_an_absent_message_not_absent() {
        assert_eq!(soul("s", &a_soul(Innate::Unknown)).innate, None);
        assert_eq!(
            soul("s", &a_soul(Innate::Absent)).innate,
            Some(pb::Innate {
                state: Some(pb::innate::State::Absent(true))
            })
        );
    }

    #[test]
    fn a_soul_reads_back_through_the_query_conversion() {
        let wire = soul("s-1", &a_soul(Innate::Present(SoulAttribute::Crit)));
        let back = crate::query::convert::inventory(vec![wire]).expect("valid");
        assert_eq!(
            back.get("s-1"),
            Some(&a_soul(Innate::Present(SoulAttribute::Crit)))
        );
    }
}
