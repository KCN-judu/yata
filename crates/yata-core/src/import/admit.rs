//! Admission (`spec/snapshot-ir.md`, "Admission"): a snapshot's records become domain values, or
//! each says why it is not one. Pure, and the one place the game's value rules meet imported
//! data. A record that fails is reported and left out; the rest of its section is kept.
//!
//! Legality against the game's tables is not admission: that is [`crate::mechanics::assess`].

use crate::fact::GameSoulId;
use crate::shikigami::{ShikigamiInstance, ShikigamiLevel, Species};
use crate::soul::{
    InnateAttribute, Level, RollCount, Soul, SoulAttribute, SoulKind, SoulSet, SoulSlot, Star,
    StoredValue, SubAttribute,
};

use super::ir::{
    Assets, GamePreset, Guild, IrError, Label, Provenance, RolledSub, Section, ShikigamiRecord,
    SoulRecord, SourceId, YataSnapshot, check,
};

/// A record left out, by its position in its section, with the reason.
#[derive(Debug, Clone, PartialEq)]
pub struct Rejected<E> {
    pub index: usize,
    pub id: Option<SourceId>,
    pub reason: E,
}

/// A section's records, split into the admitted and the rejected.
#[derive(Debug, Clone, PartialEq)]
pub struct Admitted<T, E> {
    pub values: Vec<T>,
    pub rejected: Vec<Rejected<E>>,
}

/// Which value of a soul a defect is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueAt {
    Main,
    /// A rolled sub-attribute, by its position, from 0.
    Sub(usize),
}

/// Why a soul record is not a domain soul.
#[derive(Debug, Clone, PartialEq)]
pub enum SoulAdmissionError {
    UnknownSet {
        name: String,
    },
    NotASlot {
        n: i64,
    },
    NotAStar {
        n: i64,
    },
    NotALevel {
        n: i64,
    },
    /// Negative, infinite, or not a number.
    NotAValue {
        at: ValueAt,
        value: f64,
    },
    /// Every rolled sub-attribute states its roll count (the maintainer, 2026-09-25).
    MissingRolls {
        sub: usize,
    },
    NotARollCount {
        sub: usize,
        n: i64,
    },
    /// Outside the six a boss soul can carry (ADR-0029).
    NotInnate {
        attribute: SoulAttribute,
    },
}

/// Why a Shikigami record is not an owned Shikigami.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShikigamiAdmissionError {
    NotALevel { n: i64 },
    NotAStar { n: i64 },
}

/// Why a game preset is left out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresetAdmissionError {
    /// The soul at position `position` is not a soul of the snapshot.
    NotInSnapshot { position: usize, soul: SourceId },
    /// The soul at position `position` has another slot.
    WrongPosition {
        position: usize,
        soul: SourceId,
        slot: i64,
    },
}

/// A game preset whose souls sit in their own positions.
#[derive(Debug, Clone, PartialEq)]
pub struct AdmittedPreset {
    pub label: Label,
    pub souls: [Option<GameSoulId>; 6],
}

/// A snapshot after admission: its provenance and, per present section, its domain values and
/// the records left out.
#[derive(Debug, Clone, PartialEq)]
pub struct AdmittedSnapshot {
    pub provenance: Provenance,
    pub captured_at: Option<String>,
    pub souls: Section<Admitted<(GameSoulId, Soul), SoulAdmissionError>>,
    pub shikigami: Section<Admitted<(SourceId, ShikigamiInstance), ShikigamiAdmissionError>>,
    pub presets: Section<Admitted<AdmittedPreset, PresetAdmissionError>>,
    pub assets: Section<Assets>,
    pub guild: Section<Guild>,
}

fn stored(at: ValueAt, value: f64) -> Result<StoredValue, SoulAdmissionError> {
    StoredValue::new(value).ok_or(SoulAdmissionError::NotAValue { at, value })
}

fn star(n: i64) -> Option<Star> {
    u32::try_from(n).ok().and_then(|n| Star::try_from(n).ok())
}

/// Position `n`, numbered from 1 as the game shows it.
fn slot(n: i64) -> Option<SoulSlot> {
    usize::try_from(n)
        .ok()
        .and_then(|n| n.checked_sub(1))
        .and_then(|i| SoulSlot::ALL.get(i).copied())
}

fn sub(i: usize, s: &RolledSub) -> Result<SubAttribute, SoulAdmissionError> {
    let n = s.rolls.ok_or(SoulAdmissionError::MissingRolls { sub: i })?;
    let rolls = u8::try_from(n).map_err(|_| SoulAdmissionError::NotARollCount { sub: i, n })?;
    Ok(SubAttribute {
        attribute: s.valued.attribute,
        value: stored(ValueAt::Sub(i), s.valued.value)?,
        enhancement_count: Some(RollCount::new(rolls)),
    })
}

pub fn admit_soul(r: &SoulRecord) -> Result<(GameSoulId, Soul), SoulAdmissionError> {
    let set = SoulSet::from_name(r.set.as_str()).ok_or_else(|| SoulAdmissionError::UnknownSet {
        name: r.set.as_str().to_owned(),
    })?;
    let kind = match r.innate {
        None => SoulKind::Ordinary,
        Some(attribute) => SoulKind::Boss(
            InnateAttribute::new(attribute).ok_or(SoulAdmissionError::NotInnate { attribute })?,
        ),
    };
    Ok((
        GameSoulId::from_source(&r.id),
        Soul {
            set,
            slot: slot(r.slot).ok_or(SoulAdmissionError::NotASlot { n: r.slot })?,
            star: star(r.star).ok_or(SoulAdmissionError::NotAStar { n: r.star })?,
            level: u32::try_from(r.level)
                .ok()
                .and_then(|n| Level::try_from(n).ok())
                .ok_or(SoulAdmissionError::NotALevel { n: r.level })?,
            main: r.main.attribute,
            main_value: stored(ValueAt::Main, r.main.value)?,
            subs: r
                .rolled
                .iter()
                .enumerate()
                .map(|(i, s)| sub(i, s))
                .collect::<Result<_, _>>()?,
            kind,
        },
    ))
}

pub fn admit_shikigami(r: &ShikigamiRecord) -> Result<ShikigamiInstance, ShikigamiAdmissionError> {
    Ok(ShikigamiInstance {
        species: Species(r.species.0),
        level: ShikigamiLevel::new(r.level)
            .ok_or(ShikigamiAdmissionError::NotALevel { n: r.level })?,
        star: star(r.star).ok_or(ShikigamiAdmissionError::NotAStar { n: r.star })?,
        evolved: r.evolved,
        locked: r.locked,
    })
}

/// A preset is admitted when each soul it names has the slot of its position. The souls are the
/// snapshot's own: [`check`] has made sure each named soul is there.
pub fn admit_preset(
    p: &GamePreset,
    slot_of: &dyn Fn(&SourceId) -> Option<i64>,
) -> Result<AdmittedPreset, PresetAdmissionError> {
    let mut souls: [Option<GameSoulId>; 6] = Default::default();
    for (position, soul) in p.souls.iter().enumerate() {
        if let Some(soul) = soul {
            let slot = slot_of(soul).ok_or_else(|| PresetAdmissionError::NotInSnapshot {
                position,
                soul: soul.clone(),
            })?;
            if usize::try_from(slot).ok() != Some(position + 1) {
                return Err(PresetAdmissionError::WrongPosition {
                    position,
                    soul: soul.clone(),
                    slot,
                });
            }
            souls[position] = Some(GameSoulId::from_source(soul));
        }
    }
    Ok(AdmittedPreset {
        label: p.label.clone(),
        souls,
    })
}

fn split<R, T, E>(
    records: &[R],
    id: impl Fn(&R) -> Option<SourceId>,
    admit: impl Fn(&R) -> Result<T, E>,
) -> Admitted<T, E> {
    let mut out = Admitted {
        values: Vec::new(),
        rejected: Vec::new(),
    };
    for (index, r) in records.iter().enumerate() {
        match admit(r) {
            Ok(v) => out.values.push(v),
            Err(reason) => out.rejected.push(Rejected {
                index,
                id: id(r),
                reason,
            }),
        }
    }
    out
}

fn map<T, U>(s: &Section<T>, f: impl FnOnce(&T) -> U) -> Section<U> {
    match s {
        Section::Absent => Section::Absent,
        Section::Present {
            completeness,
            value,
        } => Section::Present {
            completeness: *completeness,
            value: f(value),
        },
    }
}

/// Admit a snapshot: refuse it whole if it breaks the IR's own rules, or admit it section by
/// section, record by record.
pub fn admit(s: &YataSnapshot) -> Result<AdmittedSnapshot, IrError> {
    check(s)?;
    let slot_of = |id: &SourceId| {
        s.souls
            .present()
            .and_then(|(_, souls)| souls.souls.iter().find(|r| &r.id == id))
            .map(|r| r.slot)
    };
    Ok(AdmittedSnapshot {
        provenance: s.provenance,
        captured_at: s.captured_at.clone(),
        souls: map(&s.souls, |v| {
            split(&v.souls, |r| Some(r.id.clone()), admit_soul)
        }),
        shikigami: map(&s.shikigami, |v| {
            split(
                &v.instances,
                |r| Some(r.id.clone()),
                |r| admit_shikigami(r).map(|i| (r.id.clone(), i)),
            )
        }),
        presets: map(&s.presets, |v| {
            split(&v.presets, |_| None, |p| admit_preset(p, &slot_of))
        }),
        assets: s.assets.clone(),
        guild: s.guild.clone(),
    })
}

#[cfg(test)]
#[path = "admit_tests.rs"]
mod tests;
