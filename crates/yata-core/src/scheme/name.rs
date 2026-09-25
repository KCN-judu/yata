//! The name of a strengthening plan or a discard scheme.

use std::fmt;

/// The longest plan name the game imports, in characters. Names of 11 characters or more were
/// refused on import, and the game's own exports never exceed 10 (2026-09-24).
pub const MAX_NAME_CHARS: usize = 10;

/// The longest plan name, in UTF-8 bytes, known to import: 26. Whether the game's limit counts
/// characters or bytes is not yet told apart, so a name must meet both.
pub const MAX_NAME_BYTES: usize = 26;

/// A name the game imports: at most [`MAX_NAME_CHARS`] characters and [`MAX_NAME_BYTES`] bytes of
/// UTF-8. Checked once, here; everything that holds one may write it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemeName(String);

/// A name the game would refuse on import, with its length both ways.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NameTooLong {
    pub chars: usize,
    pub bytes: usize,
}

impl SchemeName {
    pub fn new(name: impl Into<String>) -> Result<SchemeName, NameTooLong> {
        let name = name.into();
        let (chars, bytes) = (name.chars().count(), name.len());
        if chars <= MAX_NAME_CHARS && bytes <= MAX_NAME_BYTES {
            Ok(SchemeName(name))
        } else {
            Err(NameTooLong { chars, bytes })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SchemeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_the_game_refuses_are_refused() {
        // Imported on 2026-09-24: 10 characters, 26 bytes.
        assert!(SchemeName::new("基准-不要改-雪幽魂").is_ok());
        // Refused on import: 11 characters, 29 bytes.
        assert_eq!(
            SchemeName::new("攻击固定值-排除-蝠翼"),
            Err(NameTooLong {
                chars: 11,
                bytes: 29
            })
        );
        // Ten characters, but past the longest byte length known to import.
        assert!(matches!(
            SchemeName::new("攻击攻击攻击攻击攻击"),
            Err(NameTooLong { chars: 10, .. })
        ));
    }
}
