/// 星级: one to six stars. The only stars a soul, a scheme, or a query holds; a raw number from the
/// game or the wire becomes one through [`Star::try_from`], where a number outside 1 to 6 is
/// refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Star {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

/// A number that is not a star, 1 to 6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotAStar(pub u32);

impl Star {
    pub const ALL: [Star; 6] = [
        Star::One,
        Star::Two,
        Star::Three,
        Star::Four,
        Star::Five,
        Star::Six,
    ];

    /// The number of stars, 1 to 6.
    pub const fn get(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u32> for Star {
    type Error = NotAStar;

    fn try_from(n: u32) -> Result<Star, NotAStar> {
        Star::ALL
            .into_iter()
            .find(|s| u32::from(s.get()) == n)
            .ok_or(NotAStar(n))
    }
}

impl TryFrom<u8> for Star {
    type Error = NotAStar;

    fn try_from(n: u8) -> Result<Star, NotAStar> {
        Star::try_from(u32::from(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_one_to_six_are_stars_both_ways() {
        for s in Star::ALL {
            assert_eq!(Star::try_from(s.get()), Ok(s));
        }
        for n in [0u32, 7, 255, 256, u32::MAX] {
            assert_eq!(Star::try_from(n), Err(NotAStar(n)));
        }
    }
}
