//! Domain values as the session's core-protocol messages (ADR-0004, rule 1): souls, selections,
//! scheme codes, QR matrices, a profile's capabilities, and failures.
//!
//! Domain → wire is total: every value renders. Wire → domain validates and returns a `Result`,
//! because a client can send anything; for queries and souls that direction is
//! [`crate::query::convert`], whose attribute and slot mappings this module reuses, so each enum
//! is mapped in one place.

use std::collections::{BTreeMap, BTreeSet};

use yata_core::fact::{ProfileId, Revision, Seq};
use yata_core::import::capability::{Availability, Capability, availability};
use yata_core::import::ir::{Completeness, IrError, SectionKind};
use yata_core::nonempty::NonEmpty;
use yata_core::scheme::code::{DiscardScheme, SchemeCode, StrengtheningPlan};
use yata_core::scheme::selection::{
    LevelBand, SetBit, SetChoice, SoulSelection, SubAttributeMode, SubCount,
};
use yata_core::soul::{Soul, SoulKind};
use yata_protocol::core as pb;

use crate::import::{Format, ImportError as FileError};
use crate::qr::QrMatrix;
use crate::query::convert::{wire_attribute, wire_slot};
use crate::store::fact::FactError;
use crate::store::{CommitError, ImportError, LoadError, OpenError};

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
        OpenError::RetiredFormat { found } => {
            Kind::StoreRetiredFormat(pb::StoreRetiredFormat { found: *found })
        }
        OpenError::NoFormatVersion => Kind::StoreNoFormatVersion(pb::StoreNoFormatVersion {}),
        OpenError::Damaged { problems } => Kind::StoreDamaged(pb::StoreDamaged {
            problems: problems.clone(),
        }),
        OpenError::Failure(f) => Kind::StoreFailure(pb::StoreFailure {
            problem: problem(f),
        }),
    }
}

/// Why a stored commit did not decode (`store.malformed_commit`). A newer format is refused when
/// the store is opened, before any commit is read (ADR-0032, rule 2).
pub fn fact_failure(e: &FactError) -> pb::error::Kind {
    use pb::error::Kind;
    match e {
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

/// Why the fact log refused a snapshot: by the stage that refused it, as a file is (ADR-0031,
/// rule 6).
pub fn import_failure(e: &ImportError) -> pb::error::Kind {
    use pb::error::Kind;
    match e {
        // An import of nothing: the snapshot holds no section of the IR.
        ImportError::NoSections => Kind::ImportNormalizationFailed(pb::ImportNormalizationFailed {
            problem: problem(e),
        }),
        // The daemon supplied the wrong original bytes: its own inconsistency, not the file's.
        ImportError::OriginalMismatch { .. } => {
            Kind::InternalImportMismatch(pb::InternalImportMismatch {
                problem: problem(e),
            })
        }
        ImportError::Refused(f) => ir_failure(f),
        ImportError::Blob(f) => Kind::ImportMalformedSource(pb::ImportMalformedSource {
            problem: problem(f),
        }),
        ImportError::Commit(f) => commit_failure(f),
    }
}

/// Why a snapshot breaks the IR's own rules (`snapshot-ir.md`): a reference that names nothing,
/// or anything else the normalized snapshot may not hold. Shared by every path that checks an IR.
pub fn ir_failure(e: &IrError) -> pb::error::Kind {
    use pb::error::Kind;
    match e {
        IrError::InconsistentReference { .. } | IrError::PresetsWithoutSouls => {
            Kind::ImportInconsistentReference(pb::ImportInconsistentReference {
                problem: problem(e),
            })
        }
        IrError::TooMany { .. }
        | IrError::TooManySubs { .. }
        | IrError::DuplicateId { .. }
        | IrError::DuplicateCurrency(_) => {
            Kind::ImportNormalizationFailed(pb::ImportNormalizationFailed {
                problem: problem(e),
            })
        }
    }
}

/// A format's name, as `spec/import-format.md` and `spec/snapshot-ir.md` spell it.
fn format_name(f: &Format) -> &'static str {
    match f {
        Format::YataSnapshot => "yata-snapshot",
        Format::Community(tag) => tag.name(),
    }
}

/// Why an imported file was refused whole, by the stage that refused it (ADR-0031, rule 6).
pub fn import_file_failure(e: &FileError) -> pb::error::Kind {
    use pb::error::Kind;
    match e {
        FileError::UnknownFormat { stated } => Kind::ImportUnknownFormat(pb::ImportUnknownFormat {
            stated: stated.clone(),
        }),
        FileError::AmbiguousFormat { formats } => {
            Kind::ImportAmbiguousFormat(pb::ImportAmbiguousFormat {
                formats: formats.iter().map(|f| format_name(f).to_owned()).collect(),
            })
        }
        FileError::UnsupportedVersion { found } => {
            Kind::ImportUnsupportedVersion(pb::ImportUnsupportedVersion {
                format: format_name(&Format::YataSnapshot).to_owned(),
                version: format!("{}.{}", found.major, found.minor),
            })
        }
        // Over the file limit, not JSON, a file-level field missing, or a yata-snapshot whose
        // body is not a snapshot: nothing of the file is a snapshot yet.
        FileError::TooLarge { .. }
        | FileError::MalformedSource { .. }
        | FileError::Decode(_)
        | FileError::Shape { .. } => Kind::ImportMalformedSource(pb::ImportMalformedSource {
            problem: problem(e),
        }),
        FileError::UnsupportedSourceValue { field, value } => {
            Kind::ImportUnsupportedSourceValue(pb::ImportUnsupportedSourceValue {
                field: (*field).to_owned(),
                value: value.clone(),
            })
        }
        FileError::Ir(f) => ir_failure(f),
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

// ---- capabilities (`snapshot-ir.md`, "Capabilities") ----

pub fn capability(c: Capability) -> pb::Capability {
    match c {
        Capability::Inventory => pb::Capability::Inventory,
        Capability::ShikigamiCollection => pb::Capability::ShikigamiCollection,
        Capability::GamePresets => pb::Capability::GamePresets,
        Capability::Assets => pb::Capability::Assets,
        Capability::GuildView => pb::Capability::GuildView,
    }
}

pub fn section_kind(k: SectionKind) -> pb::SectionKind {
    match k {
        SectionKind::Souls => pb::SectionKind::Souls,
        SectionKind::Shikigami => pb::SectionKind::Shikigami,
        SectionKind::Presets => pb::SectionKind::Presets,
        SectionKind::Assets => pb::SectionKind::Assets,
        SectionKind::Guild => pb::SectionKind::Guild,
    }
}

pub fn completeness(c: Completeness) -> pb::Completeness {
    match c {
        Completeness::Unstated => pb::Completeness::Unstated,
        Completeness::Partial => pb::Completeness::Partial,
        Completeness::Complete => pb::Completeness::Complete,
    }
}

/// One capability and its availability.
pub fn profile_capability(c: Capability, a: &Availability) -> pb::ProfileCapability {
    use pb::profile_capability::Availability as Wire;
    let availability = match a {
        Availability::Unavailable { missing } => Wire::Unavailable(pb::CapabilityUnavailable {
            missing: missing.iter().map(|&k| section_kind(k).into()).collect(),
        }),
        Availability::Available { completeness: c } => Wire::Available(pb::CapabilityAvailable {
            completeness: completeness(*c).into(),
        }),
    };
    pb::ProfileCapability {
        capability: capability(c).into(),
        availability: Some(availability),
    }
}

/// Every capability of a profile that holds `held`, in [`Capability::ALL`] order. The one place
/// the session derives what a profile can do, whatever the held sections came from.
pub fn capabilities(held: &BTreeMap<SectionKind, Completeness>) -> Vec<pb::ProfileCapability> {
    Capability::ALL
        .into_iter()
        .map(|c| profile_capability(c, &availability(held, c)))
        .collect()
}

/// Why a `ProfileCapability` is not a capability and its availability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityWireError {
    /// An enum field holds its `UNSPECIFIED` value: the sender left it unstated.
    Unspecified { field: &'static str },
    /// An enum field holds a value this build does not know.
    UnknownValue { field: &'static str, value: i32 },
    /// Neither `unavailable` nor `available` is set.
    NoAvailability,
    /// `unavailable` names no section.
    NothingMissing,
    /// `unavailable` names a section twice.
    MissingTwice { section: SectionKind },
}

fn capability_of(v: i32) -> Result<Capability, CapabilityWireError> {
    use pb::Capability as W;
    let field = "capability";
    match W::try_from(v) {
        Ok(W::Inventory) => Ok(Capability::Inventory),
        Ok(W::ShikigamiCollection) => Ok(Capability::ShikigamiCollection),
        Ok(W::GamePresets) => Ok(Capability::GamePresets),
        Ok(W::Assets) => Ok(Capability::Assets),
        Ok(W::GuildView) => Ok(Capability::GuildView),
        Ok(W::Unspecified) => Err(CapabilityWireError::Unspecified { field }),
        Err(_) => Err(CapabilityWireError::UnknownValue { field, value: v }),
    }
}

fn section_kind_of(v: i32) -> Result<SectionKind, CapabilityWireError> {
    use pb::SectionKind as W;
    let field = "missing";
    match W::try_from(v) {
        Ok(W::Souls) => Ok(SectionKind::Souls),
        Ok(W::Shikigami) => Ok(SectionKind::Shikigami),
        Ok(W::Presets) => Ok(SectionKind::Presets),
        Ok(W::Assets) => Ok(SectionKind::Assets),
        Ok(W::Guild) => Ok(SectionKind::Guild),
        Ok(W::Unspecified) => Err(CapabilityWireError::Unspecified { field }),
        Err(_) => Err(CapabilityWireError::UnknownValue { field, value: v }),
    }
}

fn completeness_of(v: i32) -> Result<Completeness, CapabilityWireError> {
    use pb::Completeness as W;
    let field = "completeness";
    match W::try_from(v) {
        Ok(W::Unstated) => Ok(Completeness::Unstated),
        Ok(W::Partial) => Ok(Completeness::Partial),
        Ok(W::Complete) => Ok(Completeness::Complete),
        Ok(W::Unspecified) => Err(CapabilityWireError::Unspecified { field }),
        Err(_) => Err(CapabilityWireError::UnknownValue { field, value: v }),
    }
}

/// A wire capability as the domain's, refusing anything left unstated.
pub fn profile_capability_of(
    m: &pb::ProfileCapability,
) -> Result<(Capability, Availability), CapabilityWireError> {
    use pb::profile_capability::Availability as Wire;
    let c = capability_of(m.capability)?;
    let a = match &m.availability {
        None => return Err(CapabilityWireError::NoAvailability),
        Some(Wire::Available(a)) => Availability::Available {
            completeness: completeness_of(a.completeness)?,
        },
        Some(Wire::Unavailable(u)) => {
            let mut seen = BTreeSet::new();
            let missing = u
                .missing
                .iter()
                .map(|&v| {
                    let k = section_kind_of(v)?;
                    if seen.insert(k) {
                        Ok(k)
                    } else {
                        Err(CapabilityWireError::MissingTwice { section: k })
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            Availability::Unavailable {
                missing: NonEmpty::new(missing).ok_or(CapabilityWireError::NothingMissing)?,
            }
        }
    };
    Ok((c, a))
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

/// A selection on the wire, or `None` while it chooses a soul bit beyond the mapped sets.
///
/// INTERIM, until the wire carries unmapped soul bits (the query session's follow-up to
/// `SuitCodes`): such a bit has no suit code, so listing the mapped sets alone would show a
/// narrower choice than the scheme's, and none at all would read as nothing chosen. The entry is
/// sent without a selection, and its `has_unknown_conditions` is true.
pub fn selection(s: &SoulSelection) -> Option<pb::SoulSelection> {
    use pb::soul_selection::Sets;
    let sets = match &s.sets {
        SetChoice::AnySet => Sets::All(pb::AnySet {}),
        SetChoice::Sets(bits) => Sets::Chosen(pb::SuitCodes {
            codes: bits
                .iter()
                .map(|b| match b {
                    SetBit::Mapped(set) => Some(u32::from(set.set().suit_code())),
                    SetBit::Unmapped(_) => None,
                })
                .collect::<Option<Vec<_>>>()?,
        }),
    };
    let marked = |mode, wire_mode: pb::SubAttributeMode| {
        s.sub_attributes
            .with(mode)
            .map(move |a| pb::SubAttributeChoice {
                attribute: attribute(a),
                mode: wire_mode.into(),
            })
    };
    Some(pb::SoulSelection {
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
    })
}

fn plan(p: &StrengtheningPlan) -> pb::SchemeEntry {
    pb::SchemeEntry {
        name: p.name.to_string(),
        selection: selection(&p.selection),
        has_unknown_conditions: p.has_unknown_conditions(),
    }
}

fn discard(d: &DiscardScheme) -> pb::SchemeEntry {
    pb::SchemeEntry {
        name: d.name.to_string(),
        selection: selection(d.selection()),
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

    use prost::Message;

    use super::*;
    use pb::error::Kind;

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
        let retired = open_failure(&OpenError::RetiredFormat { found: 1 });
        assert_eq!(retired.code(), "store.retired_format");
        assert_eq!(
            retired,
            pb::error::Kind::StoreRetiredFormat(pb::StoreRetiredFormat { found: 1 })
        );
        let account = pb::error::Kind::ImportAccountMismatch(pb::ImportAccountMismatch::default());
        assert_eq!(account.code(), "import.account_mismatch");
    }

    #[test]
    fn the_fact_log_refuses_a_snapshot_by_whose_fault_it_is() {
        use yata_core::fact::Digest;
        let mismatch = ImportError::OriginalMismatch {
            stated: Digest([1; 32]),
            found: Digest([2; 32]),
        };
        assert_eq!(import_failure(&mismatch).code(), "internal.import_mismatch");
        assert_eq!(
            import_failure(&ImportError::NoSections).code(),
            "import.normalization_failed"
        );
    }

    #[test]
    fn each_import_refusal_has_the_code_of_its_stage() {
        use yata_core::import::ir::{FormatTag, SchemaVersion, SectionKind, SourceId};

        let id = || SourceId::new("s-1").expect("an id");
        let cases = [
            (
                FileError::UnknownFormat { stated: None },
                "import.unknown_format",
            ),
            (
                FileError::UnsupportedVersion {
                    found: SchemaVersion { major: 2, minor: 0 },
                },
                "import.unsupported_version",
            ),
            (
                FileError::AmbiguousFormat {
                    formats: vec![
                        Format::YataSnapshot,
                        Format::Community(FormatTag::MumuSnapshotV1),
                    ],
                },
                "import.ambiguous_format",
            ),
            (FileError::TooLarge { bytes: 1 }, "import.malformed_source"),
            (
                FileError::MalformedSource {
                    reason: "not JSON".to_owned(),
                },
                "import.malformed_source",
            ),
            (
                FileError::Shape {
                    field: "data",
                    problem: crate::import::Problem::Missing,
                },
                "import.malformed_source",
            ),
            (
                FileError::UnsupportedSourceValue {
                    field: "scope",
                    value: "x".to_owned(),
                },
                "import.unsupported_source_value",
            ),
            (
                FileError::Ir(IrError::InconsistentReference {
                    preset: 0,
                    position: 0,
                    soul: id(),
                }),
                "import.inconsistent_reference",
            ),
            (
                FileError::Ir(IrError::PresetsWithoutSouls),
                "import.inconsistent_reference",
            ),
            (
                FileError::Ir(IrError::DuplicateId {
                    section: SectionKind::Souls,
                    id: id(),
                }),
                "import.normalization_failed",
            ),
            (
                FileError::Ir(IrError::TooMany {
                    section: SectionKind::Souls,
                    count: 2,
                    limit: 1,
                }),
                "import.normalization_failed",
            ),
        ];
        for (e, code) in cases {
            assert_eq!(import_file_failure(&e).code(), code, "{e:?}");
        }
        let Kind::ImportUnknownFormat(record) = import_file_failure(&FileError::UnknownFormat {
            stated: Some("other".to_owned()),
        }) else {
            panic!("an unknown format");
        };
        assert_eq!(record.stated.as_deref(), Some("other"));
    }

    fn held(sections: &[(SectionKind, Completeness)]) -> BTreeMap<SectionKind, Completeness> {
        sections.iter().copied().collect()
    }

    #[test]
    fn every_capability_reads_back_as_it_was_sent() {
        use Completeness::*;
        use SectionKind::*;
        let cases = [
            held(&[]),
            held(&[(Souls, Complete)]),
            held(&[
                (Souls, Partial),
                (Presets, Unstated),
                (Shikigami, Complete),
                (Assets, Partial),
                (Guild, Complete),
            ]),
            held(&[(Presets, Complete)]),
        ];
        for h in cases {
            let wire = capabilities(&h);
            assert_eq!(wire.len(), Capability::ALL.len());
            for (c, m) in Capability::ALL.into_iter().zip(&wire) {
                let decoded =
                    pb::ProfileCapability::decode(m.encode_to_vec().as_slice()).expect("decodes");
                assert_eq!(
                    profile_capability_of(&decoded),
                    Ok((c, availability(&h, c))),
                    "{h:?}"
                );
            }
        }
    }

    #[test]
    fn an_unstated_capability_field_is_refused() {
        use pb::profile_capability::Availability as Wire;
        let inventory = || {
            profile_capability(
                Capability::Inventory,
                &Availability::Available {
                    completeness: Completeness::Complete,
                },
            )
        };
        let refusal = |edit: &dyn Fn(&mut pb::ProfileCapability)| {
            let mut m = inventory();
            edit(&mut m);
            profile_capability_of(&m).expect_err("refused")
        };
        assert_eq!(
            refusal(&|m| m.capability = pb::Capability::Unspecified.into()),
            CapabilityWireError::Unspecified {
                field: "capability"
            }
        );
        assert_eq!(
            refusal(&|m| m.capability = 99),
            CapabilityWireError::UnknownValue {
                field: "capability",
                value: 99
            }
        );
        assert_eq!(
            refusal(&|m| m.availability = Some(Wire::Available(pb::CapabilityAvailable::default()))),
            CapabilityWireError::Unspecified {
                field: "completeness"
            }
        );
        assert_eq!(
            refusal(&|m| m.availability = None),
            CapabilityWireError::NoAvailability
        );
        let missing = |sections: Vec<i32>| {
            refusal(&move |m| {
                m.availability = Some(Wire::Unavailable(pb::CapabilityUnavailable {
                    missing: sections.clone(),
                }));
            })
        };
        assert_eq!(missing(vec![]), CapabilityWireError::NothingMissing);
        assert_eq!(
            missing(vec![pb::SectionKind::Unspecified.into()]),
            CapabilityWireError::Unspecified { field: "missing" }
        );
        let souls: i32 = pb::SectionKind::Souls.into();
        assert_eq!(
            missing(vec![souls, souls]),
            CapabilityWireError::MissingTwice {
                section: SectionKind::Souls
            }
        );
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

    #[test]
    fn a_selection_of_unmapped_soul_bits_is_sent_without_one_for_now() {
        // INTERIM: remove with the None arm of `selection` once unmapped bits are on the wire.
        use yata_core::scheme::layout::{AccountSegment, Record, SchemeLayout};
        let unmapped_only =
            Record::new("x", vec![0, 0, 0, 0, 0, 0, 0, 0, 0x40], vec![1]).expect("short");
        let layout = SchemeLayout::Strengthening {
            account: AccountSegment::from_bytes([7; 14]),
            plans: vec![unmapped_only],
        };
        let code = yata_core::scheme::code::decode_code(&layout).expect("decodes");
        let SchemeCode::Strengthening(set) = &code else {
            panic!("a strengthening set");
        };
        let entry = plan(&set.plans[0]);
        assert_eq!(entry.selection, None);
        assert!(entry.has_unknown_conditions);
        let mapped = SoulSelection::new(SetChoice::AnySet);
        assert!(
            selection(&mapped)
                .is_some_and(|s| matches!(s.sets, Some(pb::soul_selection::Sets::All(_))))
        );
    }
}
