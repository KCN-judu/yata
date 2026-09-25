//! Capabilities (`spec/snapshot-ir.md`, "Capabilities"): what a profile can do follows from which
//! sections it holds, through one total table. Never from a file's format, and never from a
//! list someone wrote down.

use std::collections::BTreeMap;

use crate::nonempty::NonEmpty;

use super::ir::{Completeness, SectionKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Capability {
    /// The soul list, the filters, quality, and the recommendations.
    Inventory,
    ShikigamiCollection,
    /// The player's saved loadouts.
    GamePresets,
    Assets,
    GuildView,
}

impl Capability {
    pub const ALL: [Capability; 5] = [
        Capability::Inventory,
        Capability::ShikigamiCollection,
        Capability::GamePresets,
        Capability::Assets,
        Capability::GuildView,
    ];

    /// The sections a capability needs.
    pub fn requires(self) -> NonEmpty<SectionKind> {
        match self {
            Capability::Inventory => NonEmpty::one(SectionKind::Souls),
            Capability::ShikigamiCollection => NonEmpty::one(SectionKind::Shikigami),
            // A preset's souls must be resolvable to be shown.
            Capability::GamePresets => {
                let mut both = NonEmpty::one(SectionKind::Presets);
                both.push(SectionKind::Souls);
                both
            }
            Capability::Assets => NonEmpty::one(SectionKind::Assets),
            Capability::GuildView => NonEmpty::one(SectionKind::Guild),
        }
    }
}

/// Whether a capability can be used, and if not, which sections are missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Availability {
    Unavailable {
        missing: NonEmpty<SectionKind>,
    },
    /// The weakest completeness of the sections it needs.
    Available {
        completeness: Completeness,
    },
}

/// A capability's availability, given the sections a profile holds and how complete each is.
pub fn availability(held: &BTreeMap<SectionKind, Completeness>, c: Capability) -> Availability {
    let needed = c.requires();
    let missing: Vec<SectionKind> = needed
        .iter()
        .copied()
        .filter(|k| !held.contains_key(k))
        .collect();
    match NonEmpty::new(missing) {
        Some(missing) => Availability::Unavailable { missing },
        // Every needed section is held; `Complete` is the identity of the minimum.
        None => Availability::Available {
            completeness: needed
                .iter()
                .filter_map(|k| held.get(k).copied())
                .fold(Completeness::Complete, Ord::min),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Completeness::*;
    use SectionKind::*;

    fn held(sections: &[(SectionKind, Completeness)]) -> BTreeMap<SectionKind, Completeness> {
        sections.iter().copied().collect()
    }

    #[test]
    fn souls_alone_give_the_inventory_and_nothing_else() {
        let h = held(&[(Souls, Complete)]);
        assert_eq!(
            availability(&h, Capability::Inventory),
            Availability::Available {
                completeness: Complete
            }
        );
        for c in [
            Capability::ShikigamiCollection,
            Capability::GamePresets,
            Capability::Assets,
            Capability::GuildView,
        ] {
            assert!(
                matches!(availability(&h, c), Availability::Unavailable { .. }),
                "{c:?}"
            );
        }
    }

    #[test]
    fn a_missing_section_is_named() {
        let h = held(&[(Presets, Complete)]);
        assert_eq!(
            availability(&h, Capability::GamePresets),
            Availability::Unavailable {
                missing: NonEmpty::one(Souls)
            }
        );
    }

    #[test]
    fn the_weakest_section_decides_the_completeness() {
        let h = held(&[(Souls, Partial), (Presets, Complete)]);
        assert_eq!(
            availability(&h, Capability::GamePresets),
            Availability::Available {
                completeness: Partial
            }
        );
        assert!(Unstated < Partial && Partial < Complete);
    }
}
