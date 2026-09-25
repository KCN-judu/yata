//! The snapshot IR (`spec/snapshot-ir.md`, ADR-0031): what Yata received from one imported
//! file, in Yata's vocabulary, as independent sections.
//!
//! Every format module normalizes into a [`YataSnapshot`]; nothing past it knows the format. The
//! IR may hold what the domain refuses (a position of 7, a set name no set has, a missing roll
//! count): [`super::admit`] decides. [`check`] applies the IR's own rules — limits, uniqueness,
//! and references inside one snapshot — before admission runs.

use std::collections::BTreeSet;

use crate::fact::Digest;
use crate::soul::SoulAttribute;

/// The IR schema's version: `yata-snapshot` major and minor. It moves with the IR's meaning only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaVersion {
    pub major: u32,
    pub minor: u32,
}

impl SchemaVersion {
    /// The version this build writes.
    pub const CURRENT: SchemaVersion = SchemaVersion { major: 1, minor: 0 };
}

/// The community formats Yata reads, one per section of `spec/import-format.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FormatTag {
    MumuSnapshotV1,
}

impl FormatTag {
    pub const ALL: [FormatTag; 1] = [FormatTag::MumuSnapshotV1];

    pub fn name(self) -> &'static str {
        match self {
            FormatTag::MumuSnapshotV1 => "mumu-snapshot-v1",
        }
    }
}

/// The format a snapshot was imported from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceFormat {
    YataSnapshot(SchemaVersion),
    Community(FormatTag),
}

/// Where a snapshot came from. Shown and kept; no rule outside the format modules reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Provenance {
    pub format: SourceFormat,
    /// SHA-256 of the file's bytes, as imported.
    pub original: Digest,
}

/// Whether a present section holds everything it covers, by the source's own statement.
/// Ordered weakest first, so the weakest of several is their minimum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Completeness {
    Unstated,
    Partial,
    Complete,
}

/// A section: absent, or present with its completeness. An absent section and a present, empty
/// one are different values.
#[derive(Debug, Clone, PartialEq)]
pub enum Section<T> {
    Absent,
    Present {
        completeness: Completeness,
        value: T,
    },
}

impl<T> Section<T> {
    pub fn present(&self) -> Option<(Completeness, &T)> {
        match self {
            Section::Absent => None,
            Section::Present {
                completeness,
                value,
            } => Some((*completeness, value)),
        }
    }
}

/// The sections of the current schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SectionKind {
    Souls,
    Shikigami,
    Presets,
    Assets,
    Guild,
}

impl SectionKind {
    pub const ALL: [SectionKind; 5] = [
        SectionKind::Souls,
        SectionKind::Shikigami,
        SectionKind::Presets,
        SectionKind::Assets,
        SectionKind::Guild,
    ];
}

/// One imported file, normalized. Its identity is the digest of its canonical encoding, which is
/// not a field of it.
#[derive(Debug, Clone, PartialEq)]
pub struct YataSnapshot {
    pub schema: SchemaVersion,
    pub provenance: Provenance,
    /// As the source states it; informational.
    pub captured_at: Option<String>,
    /// The account the file says it belongs to, when it says. The fact log checks it against the
    /// profile an import goes to; nothing else reads it.
    pub account: Option<AccountRef>,
    pub souls: Section<Souls>,
    pub shikigami: Section<ShikigamiRoster>,
    pub presets: Section<GamePresets>,
    pub assets: Section<Assets>,
    pub guild: Section<Guild>,
}

impl YataSnapshot {
    /// The present sections, with their completeness.
    pub fn sections(&self) -> Vec<(SectionKind, Completeness)> {
        let c = |s: Option<Completeness>, k| s.map(|c| (k, c));
        [
            c(self.souls.present().map(|p| p.0), SectionKind::Souls),
            c(
                self.shikigami.present().map(|p| p.0),
                SectionKind::Shikigami,
            ),
            c(self.presets.present().map(|p| p.0), SectionKind::Presets),
            c(self.assets.present().map(|p| p.0), SectionKind::Assets),
            c(self.guild.present().map(|p| p.0), SectionKind::Guild),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

/// The most characters an id, a name, or a label may have (`spec/snapshot-ir.md`, "Limits").
pub const MAX_TEXT_CHARS: usize = 256;
/// The most characters a source id may have.
pub const MAX_ID_CHARS: usize = 64;

/// Why a text is not an IR text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextError {
    Empty,
    TooLong { chars: usize, limit: usize },
}

fn text(t: &str, limit: usize) -> Result<String, TextError> {
    let chars = t.chars().count();
    if chars == 0 {
        return Err(TextError::Empty);
    }
    if chars > limit {
        return Err(TextError::TooLong { chars, limit });
    }
    Ok(t.to_owned())
}

/// What a file calls a record: non-empty, at most [`MAX_ID_CHARS`] characters. Repeated imports
/// into one profile fold by it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceId(String);

impl SourceId {
    pub fn new(id: &str) -> Result<SourceId, TextError> {
        text(id, MAX_ID_CHARS).map(SourceId)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The account a file states, as its format writes it: non-empty, at most [`MAX_ID_CHARS`]
/// characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountRef(String);

impl AccountRef {
    pub fn new(id: &str) -> Result<AccountRef, TextError> {
        text(id, MAX_ID_CHARS).map(AccountRef)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A set's name as the game shows it; that it names a set is admission's question.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SetName(String);

impl SetName {
    pub fn new(name: &str) -> Result<SetName, TextError> {
        text(name, MAX_TEXT_CHARS).map(SetName)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A preset's label as the player wrote it in the game.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(String);

impl Label {
    pub fn new(label: &str) -> Result<Label, TextError> {
        text(label, MAX_TEXT_CHARS).map(Label)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An attribute with its value in display units: a rate in percentage points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Valued {
    pub attribute: SoulAttribute,
    pub value: f64,
}

/// A rolled sub-attribute, with its strengthening count if the source states it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RolledSub {
    pub valued: Valued,
    pub rolls: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoulRecord {
    pub id: SourceId,
    pub set: SetName,
    /// Position, meant 1..6.
    pub slot: i64,
    pub star: i64,
    pub level: i64,
    pub main: Valued,
    pub rolled: Vec<RolledSub>,
    /// 固有属性, without its value (ADR-0029, the maintainer 2026-09-25).
    pub innate: Option<SoulAttribute>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Souls {
    pub souls: Vec<SoulRecord>,
}

/// The game's number for a kind of Shikigami, unresolved: no catalogue maps it yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpeciesNumber(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct ShikigamiRecord {
    /// This owned copy.
    pub id: SourceId,
    pub species: SpeciesNumber,
    pub level: i64,
    pub star: i64,
    /// 觉醒.
    pub evolved: bool,
    /// 锁定 in the game.
    pub locked: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShikigamiRoster {
    pub instances: Vec<ShikigamiRecord>,
}

/// A loadout the player saved in the game (御魂方案): not current equipment, not a Yata plan.
#[derive(Debug, Clone, PartialEq)]
pub struct GamePreset {
    pub label: Label,
    /// By position: index `i` is slot `i + 1`.
    pub souls: [Option<SourceId>; 6],
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GamePresets {
    pub presets: Vec<GamePreset>,
}

/// The currencies of the first format, named after its keys. Which game item each is, is not
/// established for every name (`spec/snapshot-ir.md`, "Assets").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Currency {
    ActionPoint,
    ArAmulet,
    AutoPoint,
    BrokenAmulet,
    Coin,
    Contrib,
    DemonSoul,
    FooleryPass,
    GoldOfuda,
    Honor,
    Jade,
    Medal,
    MysteryAmulet,
    Ofuda,
    RealmRaidPass,
    ReverseScale,
    SJade,
    Scale,
    SkinToken,
    SpSkinToken,
    TotemPass,
}

impl Currency {
    pub const ALL: [Currency; 21] = [
        Currency::ActionPoint,
        Currency::ArAmulet,
        Currency::AutoPoint,
        Currency::BrokenAmulet,
        Currency::Coin,
        Currency::Contrib,
        Currency::DemonSoul,
        Currency::FooleryPass,
        Currency::GoldOfuda,
        Currency::Honor,
        Currency::Jade,
        Currency::Medal,
        Currency::MysteryAmulet,
        Currency::Ofuda,
        Currency::RealmRaidPass,
        Currency::ReverseScale,
        Currency::SJade,
        Currency::Scale,
        Currency::SkinToken,
        Currency::SpSkinToken,
        Currency::TotemPass,
    ];
}

/// The game's number for a kind of realm card, unresolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RealmCardKind(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct RealmCardRecord {
    pub id: SourceId,
    pub kind: RealmCardKind,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Assets {
    pub currencies: Vec<(Currency, u64)>,
    pub realm_cards: Vec<RealmCardRecord>,
}

/// The guild itself; nothing about its members (`spec/snapshot-ir.md`, "Guild").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guild {
    pub level: u32,
    pub member_count: u32,
}

/// The IR's limits (`spec/snapshot-ir.md`, "Limits"). A format module checks them before it
/// allocates; [`check`] checks them again on any snapshot.
pub mod limits {
    /// The fact log keeps the file as a blob, whose limit is 16 MiB.
    pub const FILE_BYTES: u64 = 16 * 1024 * 1024;
    pub const SOULS: usize = 20_000;
    pub const SUBS_PER_SOUL: usize = 8;
    pub const SHIKIGAMI: usize = 20_000;
    pub const PRESETS: usize = 1_000;
    pub const CURRENCIES: usize = 64;
    pub const REALM_CARDS: usize = 10_000;
}

/// Why a snapshot breaks the IR's own rules. A snapshot that does is refused whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrError {
    TooMany {
        section: SectionKind,
        count: usize,
        limit: usize,
    },
    TooManySubs {
        soul: SourceId,
        count: usize,
    },
    DuplicateId {
        section: SectionKind,
        id: SourceId,
    },
    DuplicateCurrency(Currency),
    /// Presets without souls in the same snapshot: their references could name nothing.
    PresetsWithoutSouls,
    /// A preset names a soul the snapshot's souls do not hold.
    InconsistentReference {
        preset: usize,
        position: usize,
        soul: SourceId,
    },
}

fn within(section: SectionKind, count: usize, limit: usize) -> Result<(), IrError> {
    if count > limit {
        return Err(IrError::TooMany {
            section,
            count,
            limit,
        });
    }
    Ok(())
}

fn unique<'a>(
    section: SectionKind,
    ids: impl Iterator<Item = &'a SourceId>,
) -> Result<BTreeSet<&'a SourceId>, IrError> {
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id) {
            return Err(IrError::DuplicateId {
                section,
                id: id.clone(),
            });
        }
    }
    Ok(seen)
}

/// The IR's own rules over one snapshot: limits, unique ids, each currency once, and every
/// preset reference naming a soul of the same snapshot.
pub fn check(s: &YataSnapshot) -> Result<(), IrError> {
    let souls = match s.souls.present() {
        Some((_, souls)) => {
            within(SectionKind::Souls, souls.souls.len(), limits::SOULS)?;
            if let Some(r) = souls
                .souls
                .iter()
                .find(|r| r.rolled.len() > limits::SUBS_PER_SOUL)
            {
                return Err(IrError::TooManySubs {
                    soul: r.id.clone(),
                    count: r.rolled.len(),
                });
            }
            Some(unique(
                SectionKind::Souls,
                souls.souls.iter().map(|r| &r.id),
            )?)
        }
        None => None,
    };
    if let Some((_, roster)) = s.shikigami.present() {
        within(
            SectionKind::Shikigami,
            roster.instances.len(),
            limits::SHIKIGAMI,
        )?;
        unique(
            SectionKind::Shikigami,
            roster.instances.iter().map(|r| &r.id),
        )?;
    }
    if let Some((_, assets)) = s.assets.present() {
        within(
            SectionKind::Assets,
            assets.currencies.len(),
            limits::CURRENCIES,
        )?;
        within(
            SectionKind::Assets,
            assets.realm_cards.len(),
            limits::REALM_CARDS,
        )?;
        let mut seen = BTreeSet::new();
        for (c, _) in &assets.currencies {
            if !seen.insert(*c) {
                return Err(IrError::DuplicateCurrency(*c));
            }
        }
        unique(
            SectionKind::Assets,
            assets.realm_cards.iter().map(|r| &r.id),
        )?;
    }
    if let Some((_, presets)) = s.presets.present() {
        within(SectionKind::Presets, presets.presets.len(), limits::PRESETS)?;
        let souls = souls.ok_or(IrError::PresetsWithoutSouls)?;
        for (preset, p) in presets.presets.iter().enumerate() {
            for (position, soul) in p.souls.iter().enumerate() {
                if let Some(soul) = soul
                    && !souls.contains(soul)
                {
                    return Err(IrError::InconsistentReference {
                        preset,
                        position,
                        soul: soul.clone(),
                    });
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "ir_tests.rs"]
mod tests;
