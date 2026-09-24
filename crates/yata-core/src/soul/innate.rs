use super::SoulAttribute;

/// 固有属性: one of the six attributes a boss soul can carry, and the scheme editor can choose. It
/// is a [`SoulAttribute`], restricted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InnateAttribute(pub(crate) SoulAttribute);

impl InnateAttribute {
    pub const ALL: [InnateAttribute; 6] = [
        InnateAttribute(SoulAttribute::AtkPercent),
        InnateAttribute(SoulAttribute::DefPercent),
        InnateAttribute(SoulAttribute::HpPercent),
        InnateAttribute(SoulAttribute::EffectHit),
        InnateAttribute(SoulAttribute::EffectRes),
        InnateAttribute(SoulAttribute::Crit),
    ];

    pub fn new(attribute: SoulAttribute) -> Option<InnateAttribute> {
        InnateAttribute::ALL.into_iter().find(|i| i.0 == attribute)
    }

    pub fn attribute(self) -> SoulAttribute {
        self.0
    }
}
