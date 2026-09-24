/// A value as the game stores it, in display units (显示值 / 实际值, `glossary.md`): finite and not
/// negative. Every sub-attribute value and main-attribute value of a [`super::Soul`] is one, so a
/// rule never meets a NaN, an infinity, or a negative value (`soul-mechanics.md`, § Values).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct StoredValue(f64);

impl StoredValue {
    pub const ZERO: StoredValue = StoredValue(0.0);

    /// `v`, or `None` if it is not finite or is negative.
    pub const fn new(v: f64) -> Option<StoredValue> {
        if v.is_finite() && v >= 0.0 {
            Some(StoredValue(v))
        } else {
            None
        }
    }

    /// `n` tenths of a display unit, the step of the game's value tables.
    pub fn from_tenths(n: u32) -> StoredValue {
        StoredValue(f64::from(n) / 10.0)
    }

    pub const fn get(self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_finite_non_negative_values_are_stored_values() {
        for v in [0.0, 0.1, 17.0, 2052.0] {
            assert_eq!(StoredValue::new(v).map(StoredValue::get), Some(v));
        }
        for v in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.1] {
            assert_eq!(StoredValue::new(v), None, "{v}");
        }
        assert_eq!(StoredValue::from_tenths(148).get(), 14.8);
    }
}
