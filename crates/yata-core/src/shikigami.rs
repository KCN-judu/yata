//! Owned Shikigami (式神), as far as the imported data establishes them: an instance's kind, level,
//! star, and whether it is evolved and locked. No catalogue maps a species number to a Shikigami
//! yet, so the species stays a number. Which souls a Shikigami wears is never part of it
//! (ADR-0031, rule 9).

use crate::soul::Star;

/// The game's number for a kind of Shikigami.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Species(pub u32);

/// A Shikigami's level, 1 to 40. Forty is the highest level the maintainer's sample shows, and the
/// game's cap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShikigamiLevel(u8);

impl ShikigamiLevel {
    pub const MAX: u8 = 40;

    /// `n`, or `None` outside 1 to 40.
    pub fn new(n: i64) -> Option<ShikigamiLevel> {
        u8::try_from(n)
            .ok()
            .filter(|n| (1..=ShikigamiLevel::MAX).contains(n))
            .map(ShikigamiLevel)
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

/// One owned copy of a Shikigami.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShikigamiInstance {
    pub species: Species,
    pub level: ShikigamiLevel,
    pub star: Star,
    pub evolved: bool,
    pub locked: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_level_is_one_to_forty() {
        assert_eq!(ShikigamiLevel::new(1).map(ShikigamiLevel::get), Some(1));
        assert_eq!(ShikigamiLevel::new(40).map(ShikigamiLevel::get), Some(40));
        for n in [0, 41, -1, i64::MAX] {
            assert_eq!(ShikigamiLevel::new(n), None, "{n}");
        }
    }
}
