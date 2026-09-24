/// A soul set (套装), identified by the game's suit code: the id every soul record carries,
/// `suitId = 300000 + suit code` (`scheme-code.md`, "SoulSet identity is the game's suit code").
///
/// The code is the identity and nothing else is: the scheme bit and the UI order are mappings
/// from it, never the reverse. Any code can be held; whether the game uses it is a question for
/// the mapping that consumes it, which refuses a code it does not know.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SoulSet(u8);

impl SoulSet {
    pub const fn from_suit_code(code: u8) -> SoulSet {
        SoulSet(code)
    }

    pub const fn suit_code(self) -> u8 {
        self.0
    }
}
