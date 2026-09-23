/// An attribute a soul can carry, as a main attribute or a sub-attribute (属性类型).
///
/// The eleven values of the glossary's `SoulAttribute` table. Percentage attributes are measured
/// in percentage points: `Crit` 2.4 means 2.4%.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SoulAttribute {
    AtkFlat,
    AtkPercent,
    DefFlat,
    DefPercent,
    HpFlat,
    HpPercent,
    Spd,
    EffectHit,
    EffectRes,
    Crit,
    CritDmg,
}

/// The glossary's two main-attribute groups: 特殊属性 and 普通属性.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeCategory {
    Special,
    Ordinary,
}

/// The three classes of the official probability notice (攻击类, 防御类, 功能类), which weight
/// the attribute a roll draws (`soul-mechanics.md`, § Roll distribution).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RollClass {
    Attack,
    Defense,
    Utility,
}

impl SoulAttribute {
    pub const ALL: [SoulAttribute; 11] = [
        SoulAttribute::AtkFlat,
        SoulAttribute::AtkPercent,
        SoulAttribute::DefFlat,
        SoulAttribute::DefPercent,
        SoulAttribute::HpFlat,
        SoulAttribute::HpPercent,
        SoulAttribute::Spd,
        SoulAttribute::EffectHit,
        SoulAttribute::EffectRes,
        SoulAttribute::Crit,
        SoulAttribute::CritDmg,
    ];

    pub fn category(self) -> AttributeCategory {
        use SoulAttribute::*;
        match self {
            Spd | EffectHit | EffectRes | Crit | CritDmg => AttributeCategory::Special,
            AtkFlat | AtkPercent | DefFlat | DefPercent | HpFlat | HpPercent => {
                AttributeCategory::Ordinary
            }
        }
    }

    pub fn roll_class(self) -> RollClass {
        use SoulAttribute::*;
        match self {
            AtkFlat | AtkPercent | Crit | CritDmg => RollClass::Attack,
            DefFlat | DefPercent | HpFlat | HpPercent => RollClass::Defense,
            Spd | EffectHit | EffectRes => RollClass::Utility,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_special_and_six_ordinary_attributes() {
        let special = SoulAttribute::ALL
            .iter()
            .filter(|a| a.category() == AttributeCategory::Special)
            .count();
        assert_eq!(special, 5);
        assert_eq!(SoulAttribute::ALL.len() - special, 6);
    }

    #[test]
    fn roll_classes_hold_four_four_and_three_members() {
        let count = |c| {
            SoulAttribute::ALL
                .iter()
                .filter(|a| a.roll_class() == c)
                .count()
        };
        assert_eq!(count(RollClass::Attack), 4);
        assert_eq!(count(RollClass::Defense), 4);
        assert_eq!(count(RollClass::Utility), 3);
    }
}
