use super::SoulAttribute;

/// The slot a soul occupies on a Shikigami (号位).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SoulSlot {
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
    Slot6,
}

impl SoulSlot {
    pub const ALL: [SoulSlot; 6] = [
        SoulSlot::Slot1,
        SoulSlot::Slot2,
        SoulSlot::Slot3,
        SoulSlot::Slot4,
        SoulSlot::Slot5,
        SoulSlot::Slot6,
    ];

    /// `Main(k)`: the main attributes a soul in this slot may have (glossary, "Soul slot
    /// main-attribute rules").
    pub fn main_options(self) -> &'static [SoulAttribute] {
        use SoulAttribute::*;
        match self {
            SoulSlot::Slot1 => &[AtkFlat],
            SoulSlot::Slot2 => &[Spd, AtkPercent, DefPercent, HpPercent],
            SoulSlot::Slot3 => &[DefFlat],
            SoulSlot::Slot4 => &[EffectHit, EffectRes, AtkPercent, DefPercent, HpPercent],
            SoulSlot::Slot5 => &[HpFlat],
            SoulSlot::Slot6 => &[Crit, CritDmg, AtkPercent, DefPercent, HpPercent],
        }
    }

    /// Slots 1, 3 and 5 have one fixed main attribute and draw none.
    pub fn has_fixed_main(self) -> bool {
        self.main_options().len() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speed_is_a_main_attribute_on_slot_two_only() {
        let with_spd: Vec<_> = SoulSlot::ALL
            .into_iter()
            .filter(|k| k.main_options().contains(&SoulAttribute::Spd))
            .collect();
        assert_eq!(with_spd, vec![SoulSlot::Slot2]);
    }

    #[test]
    fn odd_slots_have_a_fixed_main_attribute() {
        for k in SoulSlot::ALL {
            let odd = matches!(k, SoulSlot::Slot1 | SoulSlot::Slot3 | SoulSlot::Slot5);
            assert_eq!(k.has_fixed_main(), odd, "{k:?}");
        }
    }
}
