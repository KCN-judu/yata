/// 等级: a soul's strengthening level, +0 to +15. The only levels a soul holds; a raw number from
/// the game or the wire becomes one through [`Level::try_from`], where a number above 15 is
/// refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Level(u8);

/// A number that is not a level, 0 to 15.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotALevel(pub u32);

impl Level {
    pub const ZERO: Level = Level(0);
    /// +15, the highest level.
    pub const MAX: Level = Level(15);

    /// `+n`, or `None` above 15.
    pub const fn new(n: u8) -> Option<Level> {
        if n <= 15 { Some(Level(n)) } else { None }
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    /// `nodes(ℓ) = ⌊ℓ / 3⌋`: the rolls a soul at this level has had.
    pub const fn nodes(self) -> u8 {
        self.0 / 3
    }
}

impl TryFrom<u32> for Level {
    type Error = NotALevel;

    fn try_from(n: u32) -> Result<Level, NotALevel> {
        u8::try_from(n)
            .ok()
            .and_then(Level::new)
            .ok_or(NotALevel(n))
    }
}

impl TryFrom<u8> for Level {
    type Error = NotALevel;

    fn try_from(n: u8) -> Result<Level, NotALevel> {
        Level::try_from(u32::from(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_zero_to_fifteen_are_levels_both_ways() {
        for n in 0..=15u8 {
            assert_eq!(Level::try_from(n).map(Level::get), Ok(n));
        }
        for n in [16u32, 255, 256, u32::MAX] {
            assert_eq!(Level::try_from(n), Err(NotALevel(n)));
        }
        assert_eq!(Level::MAX.nodes(), 5);
        assert_eq!(Level::ZERO.nodes(), 0);
    }
}
