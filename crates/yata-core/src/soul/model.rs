use super::{SoulAttribute, SoulSet, SoulSlot};

/// A soul as the game records it: `⟨k, σ, ℓ, m, S, c⟩` of `soul-mechanics.md`, plus its set, the
/// main attribute's value, and its innate attribute.
///
/// This is the shape decoded input takes before any rule is applied, so nothing here is
/// guaranteed legal: [`crate::mechanics::assess`] decides that. Values are stored values in
/// display units, never rounded display values.
#[derive(Debug, Clone, PartialEq)]
pub struct Soul {
    /// The set the soul belongs to. The roll rules do not depend on it; a scheme's 类型 group
    /// does (`scheme-code.md`, "Evaluation").
    pub set: SoulSet,
    pub slot: SoulSlot,
    pub star: u8,
    pub level: u8,
    pub main: SoulAttribute,
    pub main_value: f64,
    pub subs: Vec<SubAttribute>,
    /// 固有属性: held beside `subs`, never among them.
    pub innate: Innate,
}

/// A soul's 固有属性, as far as the input says.
///
/// Every boss soul (首领御魂) carries exactly one, and no other soul does (the maintainer,
/// 2026-09-25). It is not a sub-attribute: neither 副属性 nor 数量 sees it (`scheme-code.md`,
/// "Evaluation").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Innate {
    /// The soul carries none: it is not a boss soul.
    Absent,
    /// The soul's innate attribute.
    Present(SoulAttribute),
    /// The input does not say whether the soul carries one, or which. How a reading carries it
    /// is not established by a recording yet, so decode never infers `Absent` from a missing
    /// field.
    Unknown,
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
