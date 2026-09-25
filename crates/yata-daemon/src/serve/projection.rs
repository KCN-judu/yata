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
    use yata_core::soul::{
        Level, Soul, SoulAttribute, SoulKind, SoulSet, SoulSlot, Star, StoredValue, SubAttribute,
    };

    use super::{ProfileEntry, Projection};

    /// The fixture's profiles: their ids spell what they are, so a reader of a wire dump knows.
    pub const PROFILE_ID: ProfileId = ProfileId(*b"yata-fixture-000");
    pub const EMPTY_PROFILE_ID: ProfileId = ProfileId(*b"yata-fixture-nil");

    type Row = (
        &'static str,
        u8,
        SoulSlot,
        Level,
        SoulAttribute,
        StoredValue,
        &'static [(SoulAttribute, StoredValue)],
    );

    /// A literal as a level, checked at compile time like [`v`].
    const fn lv(n: u8) -> Level {
        match Level::new(n) {
            Some(l) => l,
            None => panic!("a fixture level is not a level"),
        }
    }

    /// A literal as a stored value. Only called in the constant below, so a literal that is not
    /// one fails the build, not the daemon.
    const fn v(x: f64) -> StoredValue {
        match StoredValue::new(x) {
            Some(v) => v,
            None => panic!("a fixture value is not a stored value"),
        }
    }

    use SoulAttribute::*;
    use SoulSlot::*;

    /// `(id, suit code, slot, level, main, main value, subs)`, every soul 6★.
    #[rustfmt::skip]
    pub(super) const SOULS: [Row; 12] = [
        ("fixture-01", 30, Slot2, lv(15), Spd, v(57.0), &[(Crit, v(8.4)), (CritDmg, v(7.2)), (AtkPercent, v(5.2)), (HpPercent, v(5.0))]),
        ("fixture-02", 30, Slot4, lv(15), AtkPercent, v(55.0), &[(Spd, v(16.8)), (Crit, v(2.7)), (CritDmg, v(3.5)), (EffectHit, v(3.9))]),
        ("fixture-03", 12, Slot6, lv(15), CritDmg, v(89.0), &[(Spd, v(11.2)), (Crit, v(5.4)), (AtkPercent, v(5.5)), (HpFlat, v(105.0))]),
        ("fixture-04", 12, Slot1, lv(15), AtkFlat, v(486.0), &[(Spd, v(8.1)), (Crit, v(8.7)), (CritDmg, v(3.3)), (EffectRes, v(7.1))]),
        ("fixture-05", 20, Slot3, lv(15), DefFlat, v(104.0), &[(Spd, v(5.5)), (EffectHit, v(11.4)), (EffectRes, v(7.6)), (HpPercent, v(5.8))]),
        ("fixture-06", 20, Slot5, lv(15), HpFlat, v(2052.0), &[(Spd, v(13.9)), (EffectHit, v(3.4)), (HpPercent, v(2.9)), (DefPercent, v(5.1))]),
        ("fixture-07", 7, Slot2, lv(0), Spd, v(12.0), &[(Crit, v(2.5)), (CritDmg, v(3.9)), (AtkPercent, v(2.9)), (EffectHit, v(3.3))]),
        ("fixture-08", 7, Slot6, lv(0), Crit, v(10.0), &[(Spd, v(2.6)), (CritDmg, v(3.4)), (AtkFlat, v(25.0))]),
        ("fixture-09", 2, Slot4, lv(9), EffectHit, v(37.0), &[(Spd, v(5.3)), (EffectRes, v(3.6)), (HpPercent, v(5.4)), (DefFlat, v(9.0))]),
        ("fixture-10", 39, Slot2, lv(12), AtkPercent, v(46.0), &[(Spd, v(10.4)), (Crit, v(2.4)), (CritDmg, v(3.2)), (AtkFlat, v(48.0))]),
        ("fixture-11", 39, Slot6, lv(15), Crit, v(55.0), &[(CritDmg, v(19.5)), (Spd, v(2.4)), (AtkPercent, v(2.9)), (HpFlat, v(200.0))]),
        ("fixture-12", 48, Slot3, lv(3), DefFlat, v(32.0), &[(Spd, v(2.7)), (Crit, v(5.1)), (HpPercent, v(2.5))]),
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
                    star: Star::Six,
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
