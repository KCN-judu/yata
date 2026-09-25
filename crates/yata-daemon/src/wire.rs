//! Domain values as the session's core-protocol messages (ADR-0004, rule 1): souls, selections,
//! scheme codes, QR matrices, and failures.
//!
//! Domain → wire is total: every value renders. Wire → domain validates and returns a `Result`,
//! because a client can send anything; for queries and souls that direction is
//! [`crate::query::convert`], whose attribute and slot mappings this module reuses, so each enum
//! is mapped in one place.

use yata_core::fact::{AdmissionError, FoldError, ProfileId, Revision, Seq};
use yata_core::scheme::code::{DiscardScheme, SchemeCode, StrengtheningPlan};
use yata_core::scheme::selection::{
    LevelBand, SetChoice, SoulSelection, SubAttributeMode, SubCount,
};
use yata_core::soul::{Soul, SoulKind};
use yata_protocol::core as pb;

use crate::qr::QrMatrix;
use crate::query::convert::{wire_attribute, wire_slot};
use crate::store::fact::FactError;
use crate::store::{CommitError, IngestError, LoadError, OpenError};

/// The stable dotted code of a failure (`core-protocol.md`, "Errors"): the failure oneof's field
/// name with its first `_` read as `.`. Implemented for the three failure sets by generated code.
pub trait Code {
    fn code(&self) -> &'static str;
}

include!(concat!(env!("OUT_DIR"), "/core_codes.rs"));

/// A request's failure as the session answers it: its kind, which is its code and the debug
/// record of that code, and an English message for developers (`core-protocol.md`, "Errors").
#[derive(Debug, Clone, PartialEq)]
pub struct Failure {
    pub kind: pb::error::Kind,
    pub message: String,
}

impl Failure {
    pub fn new(kind: pb::error::Kind, message: impl Into<String>) -> Failure {
        Failure {
            kind,
            message: message.into(),
        }
    }

    /// The dotted code, for logs and tests.
    pub fn code(&self) -> &'static str {
        self.kind.code()
    }
}

pub fn error(f: &Failure) -> pb::Error {
    pb::Error {
        message: f.message.clone(),
        kind: Some(f.kind.clone()),
    }
}

/// A failure's debug record where the record is the typed failure rendered in English.
fn problem(e: &impl std::fmt::Debug) -> String {
    format!("{e:?}")
}

/// Why a store could not be opened, as its failure (`store.*`).
pub fn open_failure(e: &OpenError) -> pb::error::Kind {
    use pb::error::Kind;
    match e {
        OpenError::Missing { path } => Kind::StoreMissing(pb::StoreMissing {
            path: path.display().to_string(),
        }),
        OpenError::NotADatabase => Kind::StoreNotADatabase(pb::StoreNotADatabase {}),
        OpenError::Uninitialized => Kind::StoreUninitialized(pb::StoreUninitialized {}),
        OpenError::Foreign {
            application_id,
            objects,
        } => Kind::StoreForeign(pb::StoreForeign {
            application_id: *application_id,
            objects: *objects,
        }),
        OpenError::NewerFormat { .. } => Kind::StoreNewerFormat(pb::StoreNewerFormat {
            problem: problem(e),
        }),
        OpenError::NoFormatVersion => Kind::StoreNoFormatVersion(pb::StoreNoFormatVersion {}),
        OpenError::Damaged { problems } => Kind::StoreDamaged(pb::StoreDamaged {
            problems: problems.clone(),
        }),
        OpenError::Failure(f) => Kind::StoreFailure(pb::StoreFailure {
            problem: problem(f),
        }),
    }
}

/// Why a stored commit did not decode (`store.newer_format`, `store.malformed_commit`).
pub fn fact_failure(e: &FactError) -> pb::error::Kind {
    use pb::error::Kind;
    match e {
        FactError::NewerFormat { .. } => Kind::StoreNewerFormat(pb::StoreNewerFormat {
            problem: problem(e),
        }),
        FactError::TooLarge { .. }
        | FactError::Malformed { .. }
        | FactError::SeqMismatch { .. } => Kind::StoreMalformedCommit(pb::StoreMalformedCommit {
            problem: problem(e),
        }),
    }
}

/// Why the log could not be read or replayed.
pub fn load_failure(e: &LoadError) -> pb::error::Kind {
    use pb::error::Kind;
    match e {
        LoadError::Store(f) => Kind::StoreFailure(pb::StoreFailure {
            problem: problem(f),
        }),
        LoadError::Fact(f) => fact_failure(f),
        LoadError::Fold(f) => Kind::StoreInvalidLog(pb::StoreInvalidLog {
            problem: problem(f),
        }),
    }
}

/// Why a write was refused.
pub fn commit_failure(e: &CommitError) -> pb::error::Kind {
    use pb::error::Kind;
    match e {
        CommitError::Refused(f @ FoldError::ProfileMismatch { .. }) => {
            Kind::ImportProfileMismatch(pb::ImportProfileMismatch {
                problem: problem(f),
            })
        }
        CommitError::Refused(f) => Kind::CommandRefused(pb::CommandRefused {
            problem: problem(f),
        }),
        CommitError::Encode(f) => Kind::CommandTooLarge(pb::CommandTooLarge {
            problem: problem(f),
        }),
        CommitError::LogFull | CommitError::Store(_) => Kind::StoreFailure(pb::StoreFailure {
            problem: problem(e),
        }),
    }
}

/// Why the fact log refused a reading. The core names the failure; its code is the wire's.
pub fn admission_failure(e: &AdmissionError) -> pb::error::Kind {
    use pb::error::Kind;
    match e {
        AdmissionError::UnestablishedIdentity { evidence } => {
            Kind::ImportUnestablishedIdentity(pb::ImportUnestablishedIdentity {
                evidence: problem(evidence),
            })
        }
        AdmissionError::DuplicateSoul { soul } => {
            Kind::ImportDuplicateSoul(pb::ImportDuplicateSoul {
                soul_id: soul.as_str().to_owned(),
            })
        }
        AdmissionError::MissingSoulId { .. }
        | AdmissionError::EmptySoulId { .. }
        | AdmissionError::EmptyAccount => {
            Kind::ImportMalformedReading(pb::ImportMalformedReading {
                problem: problem(e),
            })
        }
    }
}

/// Why a reading could not be imported.
pub fn ingest_failure(e: &IngestError) -> pb::error::Kind {
    use pb::error::Kind;
    match e {
        IngestError::Convert(f) => Kind::ImportMalformedReading(pb::ImportMalformedReading {
            problem: problem(f),
        }),
        IngestError::Refused(f) => admission_failure(f),
        IngestError::Blob(f) => Kind::ImportReadingTooLarge(pb::ImportReadingTooLarge {
            problem: problem(f),
        }),
        IngestError::Commit(f) => commit_failure(f),
    }
}

/// A revision on the wire: the `seq` of the last commit, 0 for the empty log. That 0 is the
/// empty log's own revision, not "none"; the wire has one encoding for each revision.
pub fn revision(r: Revision) -> u64 {
    r.last().map_or(0, Seq::get)
}

/// A wire revision as the domain's. Total: every `u64` names a revision.
pub fn revision_of(n: u64) -> Revision {
    Seq::new(n).map_or(Revision::EMPTY, Revision::at)
}

/// A profile id on the wire: its 16 bytes as 32 lowercase hex digits.
pub fn profile_id(id: ProfileId) -> String {
    id.0.iter().map(|b| format!("{b:02x}")).collect()
}

/// A wire profile id as the domain's; `None` for anything but 32 lowercase hex digits, so each
/// id has one spelling.
pub fn profile_id_of(text: &str) -> Option<ProfileId> {
    let nibble = |c: u8| match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    };
    let digits: [u8; 32] = text.as_bytes().try_into().ok()?;
    let mut bytes = [0u8; 16];
    for (b, pair) in bytes.iter_mut().zip(digits.chunks_exact(2)) {
        *b = nibble(pair[0])? << 4 | nibble(pair[1])?;
    }
    Some(ProfileId(bytes))
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

/// A soul of the projection, under its row identity, with its kind (ADR-0029).
pub fn soul(id: &str, s: &Soul) -> pb::Soul {
    let kind = match s.kind {
        SoulKind::Ordinary => pb::soul::Kind::Ordinary(pb::OrdinarySoul {}),
        SoulKind::Boss(a) => pb::soul::Kind::Boss(pb::BossSoul {
            innate: attribute(a.attribute()),
        }),
    };
    pb::Soul {
        soul_id: id.to_owned(),
        suit_code: u32::from(s.set.suit_code()),
        slot: wire_slot(s.slot).into(),
        star: u32::from(s.star.get()),
        level: u32::from(s.level.get()),
        main: attribute(s.main),
        main_value: s.main_value.get(),
        subs: s
            .subs
            .iter()
            .map(|sub| pb::SubAttribute {
                attribute: attribute(sub.attribute),
                value: sub.value.get(),
                enhancement_count: sub.enhancement_count.map(|c| u32::from(c.get())),
            })
            .collect(),
        kind: Some(kind),
    }
}

pub fn selection(s: &SoulSelection) -> pb::SoulSelection {
    use pb::soul_selection::Sets;
    let sets = match &s.sets {
        SetChoice::AnySet => Sets::All(pb::AnySet {}),
        SetChoice::Sets(sets) => Sets::Chosen(pb::SuitCodes {
            codes: sets.iter().map(|x| u32::from(x.suit_code())).collect(),
        }),
        // Souls the model does not map: no suit code can name them. The entry reports its unknown
        // conditions, and the selection is shown, not sent back; a query decoder refuses it.
        SetChoice::OnlyUnmapped => Sets::Chosen(pb::SuitCodes { codes: Vec::new() }),
    };
    let marked = |mode, wire_mode: pb::SubAttributeMode| {
        s.sub_attributes
            .with(mode)
            .map(move |a| pb::SubAttributeChoice {
                attribute: attribute(a),
                mode: wire_mode.into(),
            })
    };
    pb::SoulSelection {
        sets: Some(sets),
        slots: s.slots.iter().map(|&k| wire_slot(k).into()).collect(),
        stars: s.stars.iter().map(|&n| u32::from(n.get())).collect(),
        levels: s.levels.iter().map(|&b| level_band(b).into()).collect(),
        main_attributes: s.main_attributes.iter().map(|&a| attribute(a)).collect(),
        innate: s.innate.iter().map(|i| attribute(i.attribute())).collect(),
        sub_attributes: marked(SubAttributeMode::Include, pb::SubAttributeMode::Include)
            .chain(marked(
                SubAttributeMode::Exclude,
                pb::SubAttributeMode::Exclude,
            ))
            .collect(),
        sub_counts: s.sub_counts.iter().map(|&c| sub_count(c).into()).collect(),
    }
}

fn plan(p: &StrengtheningPlan) -> pb::SchemeEntry {
    pb::SchemeEntry {
        name: p.name.to_string(),
        selection: Some(selection(&p.selection)),
        has_unknown_conditions: p.has_unknown_conditions(),
    }
}

fn discard(d: &DiscardScheme) -> pb::SchemeEntry {
    pb::SchemeEntry {
        name: d.name.to_string(),
        selection: Some(selection(d.selection())),
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
            pb::error::Kind::DecodeNoInput(pb::DecodeNoInput {}),
            "DecodeSchemeCode carries neither text nor an image",
        )),
    }
}

#[cfg(test)]
mod tests {
    use yata_core::soul::{
        InnateAttribute, Level, SoulAttribute, SoulSet, SoulSlot, Star, StoredValue,
    };

    use super::*;

    fn a_soul(kind: SoulKind) -> Soul {
        Soul {
            set: SoulSet::from_suit_code(30),
            slot: SoulSlot::Slot2,
            star: Star::Six,
            level: Level::MAX,
            main: SoulAttribute::Spd,
            main_value: StoredValue::from_tenths(570),
            subs: vec![],
            kind,
        }
    }

    fn crit_boss() -> SoulKind {
        SoulKind::Boss(InnateAttribute::new(SoulAttribute::Crit).expect("innate"))
    }

    #[test]
    fn every_soul_states_its_kind() {
        assert_eq!(
            soul("s", &a_soul(SoulKind::Ordinary)).kind,
            Some(pb::soul::Kind::Ordinary(pb::OrdinarySoul {}))
        );
        assert_eq!(
            soul("s", &a_soul(crit_boss())).kind,
            Some(pb::soul::Kind::Boss(pb::BossSoul {
                innate: pb::SoulAttribute::Crit.into()
            }))
        );
    }

    #[test]
    fn a_soul_reads_back_through_the_query_conversion() {
        for kind in [SoulKind::Ordinary, crit_boss()] {
            let wire = soul("s-1", &a_soul(kind));
            let back = crate::query::convert::inventory(vec![wire]).expect("valid");
            let id = yata_core::fact::GameSoulId::new("s-1").expect("non-empty");
            assert_eq!(back.get(&id), Some(&a_soul(kind)));
        }
    }

    #[test]
    fn every_failure_set_names_its_cases_by_the_field_rule() {
        let request = pb::error::Kind::QueryStaleRevision(pb::QueryStaleRevision::default());
        assert_eq!(request.code(), "query.stale_revision");
        let stream = pb::session_failed::Kind::InternalIo(pb::InternalIo::default());
        assert_eq!(stream.code(), "internal.io");
        let client =
            pb::client_failure::Kind::ClientDaemonNotFound(pb::ClientDaemonNotFound::default());
        assert_eq!(client.code(), "client.daemon_not_found");
        let store = pb::error::Kind::StoreNotADatabase(pb::StoreNotADatabase::default());
        assert_eq!(store.code(), "store.not_a_database");
    }

    #[test]
    fn a_profile_id_has_one_spelling() {
        let id = ProfileId(*b"yata-fixture-000");
        let text = profile_id(id);
        assert_eq!(text, "796174612d666978747572652d303030");
        assert_eq!(profile_id_of(&text), Some(id));
        assert_eq!(profile_id_of(&text.to_uppercase()), None);
        assert_eq!(profile_id_of(&text[1..]), None);
        assert_eq!(profile_id_of(&format!("{text}0")), None);
        assert_eq!(profile_id_of(&format!("+{}", &text[1..])), None);
        assert_eq!(profile_id_of(""), None);
    }

    #[test]
    fn revision_zero_is_the_empty_log() {
        assert_eq!(revision(Revision::EMPTY), 0);
        assert_eq!(revision_of(0), Revision::EMPTY);
        let first = revision_of(1);
        assert_eq!(revision(first), 1);
        assert_ne!(first, Revision::EMPTY);
    }
}
