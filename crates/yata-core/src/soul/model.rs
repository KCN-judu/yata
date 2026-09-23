use super::{SoulAttribute, SoulSlot};

/// A soul as the game records it: `⟨k, σ, ℓ, m, S, c⟩` of `soul-mechanics.md`, plus the main
/// attribute's value.
///
/// This is the shape decoded input takes before any rule is applied, so nothing here is
/// guaranteed legal: [`crate::mechanics::assess`] decides that. Values are stored values in
/// display units, never rounded display values.
#[derive(Debug, Clone, PartialEq)]
pub struct Soul {
    pub slot: SoulSlot,
    pub star: u8,
    pub level: u8,
    pub main: SoulAttribute,
    pub main_value: f64,
    pub subs: Vec<SubAttribute>,
}

/// One sub-attribute (副属性): an element of `dom S` with its stored value `S(a)`.
#[derive(Debug, Clone, PartialEq)]
pub struct SubAttribute {
    pub attribute: SoulAttribute,
    pub value: f64,
    /// `c(a)`, the rolls this sub-attribute received after it appeared. `None` when the reading
    /// does not carry it: whether the game records it is open, and it is then inferred
    /// (`soul-mechanics.md`, § Inferring roll counts).
    pub enhancement_count: Option<u8>,
}

impl Soul {
    /// `S(a)` for an attribute in `dom S`.
    pub fn sub(&self, attribute: SoulAttribute) -> Option<&SubAttribute> {
        self.subs.iter().find(|s| s.attribute == attribute)
    }

    /// `nodes(ℓ) = ⌊ℓ / 3⌋`: the rolls a soul at this level has had.
    pub fn nodes(&self) -> u8 {
        self.level / 3
    }
}
