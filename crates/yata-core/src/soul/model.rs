use super::{InnateAttribute, SoulAttribute, SoulSet, SoulSlot};

/// A soul as the game records it: `⟨k, σ, ℓ, m, S, c⟩` of `soul-mechanics.md`, plus its set, the
/// main attribute's value, and whether it is a boss soul.
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
    /// Ordinary, or a boss soul with its 固有属性, held beside `subs`, never among them.
    pub kind: SoulKind,
}

/// Whether a soul is a boss soul (首领御魂), and if it is, its 固有属性 (ADR-0029).
///
/// Every boss soul carries exactly one innate attribute, one of six, and no other soul carries any
/// (the maintainer, 2026-09-25). It is not a sub-attribute: neither 副属性 nor 数量 sees it
/// (`scheme-code.md`, "Evaluation"). There is no unknown kind: a reading that cannot tell which a
/// soul is, is refused before it becomes a soul.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoulKind {
    Ordinary,
    Boss(InnateAttribute),
}

/// One sub-attribute (副属性): an element of `dom S` with its stored value `S(a)`.
#[derive(Debug, Clone, PartialEq)]
pub struct SubAttribute {
    pub attribute: SoulAttribute,
    pub value: f64,
    /// `c(a)`, the rolls this sub-attribute received after it appeared. `None` when the reading
    /// does not carry it: whether the game records it is open, and it is then inferred
    /// (`soul-mechanics.md`, § Inferring roll counts).
    pub enhancement_count: Option<RollCount>,
}

/// `c(a)`: how many of a soul's rolls landed on one sub-attribute after it appeared. Any count can
/// be held; whether it fits the soul's level is legality's question (`RollsExceedNodes`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RollCount(u8);

impl RollCount {
    pub const fn new(c: u8) -> RollCount {
        RollCount(c)
    }

    pub const fn get(self) -> u8 {
        self.0
    }
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
