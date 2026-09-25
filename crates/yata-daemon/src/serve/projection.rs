//! What a session reads: the profiles and each profile's souls, at one revision.
//!
//! Until the session reads the store's fold, it serves either an empty projection — a first run,
//! no profile, nothing imported — or the development fixture of [`fixture`]. Both are queried
//! through the same path a folded projection will be: [`crate::query::prepare`] and
//! [`crate::query::run`], over souls keyed by soul id.

use std::collections::BTreeMap;

use yata_core::fact::{GameSoulId, ProfileId, Revision};
use yata_core::soul::Soul;

/// The projection a session serves.
#[derive(Debug, Clone, PartialEq)]
pub struct Projection {
    pub revision: Revision,
    /// Keyed by profile id, so a profile is found by its key and listed in one order.
    pub profiles: BTreeMap<ProfileId, ProfileEntry>,
}

/// One game account and its souls, keyed by soul id, the row identity.
#[derive(Debug, Clone, PartialEq)]
pub struct ProfileEntry {
    pub name: String,
    pub souls: BTreeMap<GameSoulId, Soul>,
}

impl Projection {
    /// A first run: no profile, nothing imported.
    pub fn empty() -> Projection {
        Projection {
            revision: Revision::EMPTY,
            profiles: BTreeMap::new(),
        }
    }
}

/// The development fixture: typed souls served through the real protocol path while no reading
/// reaches the session. Every soul is well-formed under W-Soul (tested below), so the fixture
/// never shows the application a soul the game could not hold. Suit codes are taken from the
/// scheme mapping's table; the souls themselves are invented, and their ids say so.
pub mod fixture {
    use std::collections::BTreeMap;

    use yata_core::fact::{GameSoulId, ProfileId, Revision, Seq};
    use yata_core::soul::{Soul, SoulAttribute, SoulKind, SoulSet, SoulSlot, SubAttribute};

    use super::{ProfileEntry, Projection};

    /// The fixture's profiles: their ids spell what they are, so a reader of a wire dump knows.
    pub const PROFILE_ID: ProfileId = ProfileId(*b"yata-fixture-000");
    pub const EMPTY_PROFILE_ID: ProfileId = ProfileId(*b"yata-fixture-nil");

    type Row = (
        &'static str,
        u8,
        SoulSlot,
        u8,
        SoulAttribute,
        f64,
        &'static [(SoulAttribute, f64)],
    );

    use SoulAttribute::*;
    use SoulSlot::*;

    /// `(id, suit code, slot, level, main, main value, subs)`, every soul 6★.
    #[rustfmt::skip]
    pub(super) const SOULS: [Row; 12] = [
        ("fixture-01", 30, Slot2, 15, Spd, 57.0, &[(Crit, 8.4), (CritDmg, 7.2), (AtkPercent, 5.2), (HpPercent, 5.0)]),
        ("fixture-02", 30, Slot4, 15, AtkPercent, 55.0, &[(Spd, 16.8), (Crit, 2.7), (CritDmg, 3.5), (EffectHit, 3.9)]),
        ("fixture-03", 12, Slot6, 15, CritDmg, 89.0, &[(Spd, 11.2), (Crit, 5.4), (AtkPercent, 5.5), (HpFlat, 105.0)]),
        ("fixture-04", 12, Slot1, 15, AtkFlat, 486.0, &[(Spd, 8.1), (Crit, 8.7), (CritDmg, 3.3), (EffectRes, 7.1)]),
        ("fixture-05", 20, Slot3, 15, DefFlat, 104.0, &[(Spd, 5.5), (EffectHit, 11.4), (EffectRes, 7.6), (HpPercent, 5.8)]),
        ("fixture-06", 20, Slot5, 15, HpFlat, 2052.0, &[(Spd, 13.9), (EffectHit, 3.4), (HpPercent, 2.9), (DefPercent, 5.1)]),
        ("fixture-07", 7, Slot2, 0, Spd, 12.0, &[(Crit, 2.5), (CritDmg, 3.9), (AtkPercent, 2.9), (EffectHit, 3.3)]),
        ("fixture-08", 7, Slot6, 0, Crit, 10.0, &[(Spd, 2.6), (CritDmg, 3.4), (AtkFlat, 25.0)]),
        ("fixture-09", 2, Slot4, 9, EffectHit, 37.0, &[(Spd, 5.3), (EffectRes, 3.6), (HpPercent, 5.4), (DefFlat, 9.0)]),
        ("fixture-10", 39, Slot2, 12, AtkPercent, 46.0, &[(Spd, 10.4), (Crit, 2.4), (CritDmg, 3.2), (AtkFlat, 48.0)]),
        ("fixture-11", 39, Slot6, 15, Crit, 55.0, &[(CritDmg, 19.5), (Spd, 2.4), (AtkPercent, 2.9), (HpFlat, 200.0)]),
        ("fixture-12", 48, Slot3, 3, DefFlat, 32.0, &[(Spd, 2.7), (Crit, 5.1), (HpPercent, 2.5)]),
    ];

    /// The fixture souls, all ordinary. The fixture is invented and makes no claim about which sets
    /// are boss sets; a boss soul would need a sourced one (ADR-0029, rule 5).
    pub fn souls() -> BTreeMap<GameSoulId, Soul> {
        SOULS
            .iter()
            .filter_map(|&(id, suit, slot, level, main, main_value, subs)| {
                // Every fixture id is non-empty; `every_fixture_row_is_a_soul` holds the table to it.
                let id = GameSoulId::new(id).ok()?;
                let soul = Soul {
                    set: SoulSet::from_suit_code(suit),
                    slot,
                    star: 6,
                    level,
                    main,
                    main_value,
                    subs: subs
                        .iter()
                        .map(|&(attribute, value)| SubAttribute {
                            attribute,
                            value,
                            enhancement_count: None,
                        })
                        .collect(),
                    kind: SoulKind::Ordinary,
                };
                Some((id, soul))
            })
            .collect()
    }

    /// Two profiles: one with the fixture souls, one with none, so the application can show both
    /// a filled and an empty inventory.
    pub fn projection() -> Projection {
        Projection {
            revision: Revision::at(Seq::FIRST),
            profiles: BTreeMap::from([
                (
                    PROFILE_ID,
                    ProfileEntry {
                        name: "Fixture".to_owned(),
                        souls: souls(),
                    },
                ),
                (
                    EMPTY_PROFILE_ID,
                    ProfileEntry {
                        name: "Fixture (empty)".to_owned(),
                        souls: BTreeMap::new(),
                    },
                ),
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use yata_core::mechanics::{Verdict, assess};

    use super::*;

    #[test]
    fn every_fixture_row_is_a_soul() {
        assert_eq!(fixture::souls().len(), fixture::SOULS.len());
    }

    #[test]
    fn every_fixture_soul_is_well_formed() {
        for (id, soul) in fixture::souls() {
            let a = assess(&soul);
            assert_eq!(a.verdict(), Verdict::WellFormed, "{id:?}: {a:?}");
        }
    }
}
