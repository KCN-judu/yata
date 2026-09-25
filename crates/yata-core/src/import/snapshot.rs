//! A snapshot file, once a format's parser has read it, and the souls it holds
//! (`spec/import-format.md`, PRP-0008).
//!
//! Every format parses into the same [`SnapshotSoul`]s: names resolved to the domain's attributes,
//! values in display units, and nothing else decided. [`soul_of`] then turns each into a
//! [`Soul`], or says why it cannot be one. Nothing here knows which format a soul came from, and
//! nothing reads bytes: the daemon's format modules do.
//!
//! A snapshot is always a complete inventory: a format's parser refuses a file that does not say
//! it is complete (the maintainer, 2026-09-25).

use crate::fact::{GameSoulId, IdError};
use crate::soul::{
    InnateAttribute, Level, RollCount, Soul, SoulAttribute, SoulKind, SoulSet, SoulSlot, Star,
    StoredValue, SubAttribute,
};

/// The formats Yata reads, one per section of `spec/import-format.md`. A file is recognised as
/// exactly one of them from its header, or refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FormatTag {
    MumuSnapshotV1,
}

impl FormatTag {
    pub const ALL: [FormatTag; 1] = [FormatTag::MumuSnapshotV1];

    /// The format's name in `spec/import-format.md`.
    pub fn name(self) -> &'static str {
        match self {
            FormatTag::MumuSnapshotV1 => "mumu-snapshot-v1",
        }
    }
}

/// An attribute with its value, in display units: a percentage attribute in percentage points
/// (`Crit` 2.4 is 2.4%), whatever unit the file used.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Valued {
    pub attribute: SoulAttribute,
    pub value: f64,
}

/// A sub-attribute as the file states it. `rolls` is the file's count as written; `None` when
/// the file leaves it out.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnapshotSub {
    pub valued: Valued,
    pub rolls: Option<u64>,
}

/// One soul as a format's parser reads it: every field present and of the right JSON kind, every
/// name a domain attribute, and no domain rule applied yet.
#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotSoul {
    pub id: String,
    /// The set's name as the file writes it.
    pub set: String,
    pub slot: i64,
    pub star: i64,
    pub level: i64,
    pub main: Valued,
    pub subs: Vec<SnapshotSub>,
    /// The innate attributes the file lists: none for an ordinary soul, one for a boss soul.
    /// Their values are not kept (the maintainer, 2026-09-25).
    pub innate: Vec<SoulAttribute>,
}

/// Which value of a soul a defect is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueAt {
    Main,
    /// A sub-attribute, by its position among the soul's sub-attributes, from 0.
    Sub(usize),
}

/// Why a [`SnapshotSoul`] is not a [`Soul`].
#[derive(Debug, Clone, PartialEq)]
pub enum SoulDefect {
    EmptyId,
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
    /// A value that is negative, infinite, or not a number.
    NotAValue {
        at: ValueAt,
        value: f64,
    },
    /// A sub-attribute without its roll count: the maintainer requires every one (2026-09-25).
    MissingRolls {
        sub: usize,
    },
    /// A roll count too large to be one.
    NotARollCount {
        sub: usize,
        n: u64,
    },
    /// An innate attribute outside the six a boss soul can carry (ADR-0029).
    NotInnate {
        attribute: SoulAttribute,
    },
    /// More than one innate attribute: a boss soul has exactly one.
    SeveralInnate {
        count: usize,
    },
}

/// Turn a parsed soul into a domain soul, or say why it is not one. The soul's legality (its
/// main attribute for its slot, its values against the game's tables) is not decided here:
/// that is [`crate::mechanics::assess`].
pub fn soul_of(s: &SnapshotSoul) -> Result<(GameSoulId, Soul), SoulDefect> {
    let id = GameSoulId::new(s.id.clone()).map_err(|IdError::Empty| SoulDefect::EmptyId)?;
    let set = SoulSet::from_name(&s.set).ok_or_else(|| SoulDefect::UnknownSet {
        name: s.set.clone(),
    })?;
    let slot = slot_of(s.slot).ok_or(SoulDefect::NotASlot { n: s.slot })?;
    let star = u32::try_from(s.star)
        .ok()
        .and_then(|n| Star::try_from(n).ok())
        .ok_or(SoulDefect::NotAStar { n: s.star })?;
    let level = u32::try_from(s.level)
        .ok()
        .and_then(|n| Level::try_from(n).ok())
        .ok_or(SoulDefect::NotALevel { n: s.level })?;
    let stored = |at: ValueAt, value: f64| {
        StoredValue::new(value).ok_or(SoulDefect::NotAValue { at, value })
    };
    let main_value = stored(ValueAt::Main, s.main.value)?;
    let subs = s
        .subs
        .iter()
        .enumerate()
        .map(|(i, sub)| {
            let n = sub.rolls.ok_or(SoulDefect::MissingRolls { sub: i })?;
            let rolls = u8::try_from(n).map_err(|_| SoulDefect::NotARollCount { sub: i, n })?;
            Ok(SubAttribute {
                attribute: sub.valued.attribute,
                value: stored(ValueAt::Sub(i), sub.valued.value)?,
                enhancement_count: Some(RollCount::new(rolls)),
            })
        })
        .collect::<Result<Vec<_>, SoulDefect>>()?;
    let kind = match s.innate.as_slice() {
        [] => SoulKind::Ordinary,
        [attribute] => SoulKind::Boss(InnateAttribute::new(*attribute).ok_or(
            SoulDefect::NotInnate {
                attribute: *attribute,
            },
        )?),
        several => {
            return Err(SoulDefect::SeveralInnate {
                count: several.len(),
            });
        }
    };
    Ok((
        id,
        Soul {
            set,
            slot,
            star,
            level,
            main: s.main.attribute,
            main_value,
            subs,
            kind,
        },
    ))
}

/// Position `n`, numbered from 1 as the game shows it.
fn slot_of(n: i64) -> Option<SoulSlot> {
    usize::try_from(n)
        .ok()
        .and_then(|n| n.checked_sub(1))
        .and_then(|i| SoulSlot::ALL.get(i).copied())
}

#[cfg(test)]
mod tests {
    use super::*;
    use SoulAttribute::*;

    fn sample() -> SnapshotSoul {
        SnapshotSoul {
            id: "0123456789abcdef01234567".into(),
            set: "荒骷髅".into(),
            slot: 2,
            star: 6,
            level: 15,
            main: Valued {
                attribute: Spd,
                value: 57.0,
            },
            subs: vec![SnapshotSub {
                valued: Valued {
                    attribute: Crit,
                    value: 2.7,
                },
                rolls: Some(1),
            }],
            innate: vec![EffectRes],
        }
    }

    #[test]
    fn a_complete_soul_becomes_a_domain_soul() {
        let (id, soul) = soul_of(&sample()).expect("a soul");
        assert_eq!(id.as_str(), "0123456789abcdef01234567");
        assert_eq!(soul.set, SoulSet::from_suit_code(52));
        assert_eq!(soul.slot, SoulSlot::Slot2);
        assert_eq!(soul.star, Star::Six);
        assert_eq!(soul.level, Level::MAX);
        assert_eq!(soul.main, Spd);
        assert_eq!(soul.subs[0].enhancement_count, Some(RollCount::new(1)));
        assert_eq!(
            soul.kind,
            SoulKind::Boss(InnateAttribute::new(EffectRes).expect("innate"))
        );
        let ordinary = SnapshotSoul {
            innate: vec![],
            ..sample()
        };
        assert_eq!(
            soul_of(&ordinary).map(|(_, s)| s.kind),
            Ok(SoulKind::Ordinary)
        );
    }

    #[test]
    fn positions_count_from_one() {
        let at = |n| {
            soul_of(&SnapshotSoul {
                slot: n,
                ..sample()
            })
            .map(|(_, s)| s.slot)
        };
        assert_eq!(at(1), Ok(SoulSlot::Slot1));
        assert_eq!(at(6), Ok(SoulSlot::Slot6));
        for n in [0, 7, -1, i64::MAX] {
            assert_eq!(at(n), Err(SoulDefect::NotASlot { n }));
        }
    }

    #[test]
    fn each_defect_is_named() {
        let with = |f: fn(&mut SnapshotSoul)| {
            let mut s = sample();
            f(&mut s);
            soul_of(&s).map(|_| ())
        };
        assert_eq!(with(|s| s.id.clear()), Err(SoulDefect::EmptyId));
        assert_eq!(
            with(|s| s.set = "破势x".into()),
            Err(SoulDefect::UnknownSet {
                name: "破势x".into()
            })
        );
        assert_eq!(with(|s| s.star = 7), Err(SoulDefect::NotAStar { n: 7 }));
        assert_eq!(with(|s| s.level = 16), Err(SoulDefect::NotALevel { n: 16 }));
        assert_eq!(
            with(|s| s.main.value = -1.0),
            Err(SoulDefect::NotAValue {
                at: ValueAt::Main,
                value: -1.0
            })
        );
        assert_eq!(
            with(|s| s.subs[0].rolls = None),
            Err(SoulDefect::MissingRolls { sub: 0 })
        );
        assert_eq!(
            with(|s| s.subs[0].rolls = Some(256)),
            Err(SoulDefect::NotARollCount { sub: 0, n: 256 })
        );
        assert_eq!(
            with(|s| s.innate = vec![Spd]),
            Err(SoulDefect::NotInnate { attribute: Spd })
        );
        assert_eq!(
            with(|s| s.innate = vec![Crit, EffectHit]),
            Err(SoulDefect::SeveralInnate { count: 2 })
        );
    }
}
