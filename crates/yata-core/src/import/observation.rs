//! A reading of the game as the domain receives it: typed fields where the reader's layout maps
//! them, each with the evidence behind that mapping, and the record the reader saw, verbatim.
//!
//! Nothing here is interpreted beyond its type. A suit code is the game's code, an attribute code
//! the game's code, a value the game's stored value; turning them into [`crate::soul`] values is a
//! separate step, and one that waits on evidence (`probe-protocol.md`, "Evidence").

use std::collections::BTreeMap;

/// How a layout mapping is known (`probe-protocol.md`, "Evidence"). Ordered by strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Evidence {
    /// A hypothesis taken from the prior tool (ADR-0014), not yet re-established.
    Inherited,
    /// Re-established by this project's own recording or single-variable test.
    Established,
}

/// A typed field of [`SoulObservation`], by its name in the probe schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SoulField {
    SoulId,
    SuitCode,
    Star,
    Slot,
    Level,
    Main,
    Subs,
    Innate,
    Locked,
    Discarded,
}

impl SoulField {
    pub const ALL: [SoulField; 10] = [
        SoulField::SoulId,
        SoulField::SuitCode,
        SoulField::Star,
        SoulField::Slot,
        SoulField::Level,
        SoulField::Main,
        SoulField::Subs,
        SoulField::Innate,
        SoulField::Locked,
        SoulField::Discarded,
    ];

    /// The field's name in the probe schema, as `FieldEvidence.field` states it.
    pub fn schema_name(self) -> &'static str {
        match self {
            SoulField::SoulId => "SoulRecord.soul_id",
            SoulField::SuitCode => "SoulRecord.suit_code",
            SoulField::Star => "SoulRecord.star",
            SoulField::Slot => "SoulRecord.slot",
            SoulField::Level => "SoulRecord.level",
            SoulField::Main => "SoulRecord.main",
            SoulField::Subs => "SoulRecord.subs",
            SoulField::Innate => "SoulRecord.innate",
            SoulField::Locked => "SoulRecord.locked",
            SoulField::Discarded => "SoulRecord.discarded",
        }
    }

    pub fn from_schema_name(name: &str) -> Option<SoulField> {
        SoulField::ALL.into_iter().find(|f| f.schema_name() == name)
    }
}

/// Whether a reading covers its whole scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coverage {
    Complete,
    Partial,
    /// The reading did not say.
    Unstated,
}

/// One reading of the souls scope.
#[derive(Debug, Clone, PartialEq)]
pub struct SoulReading {
    pub coverage: Coverage,
    /// The evidence behind the rule that recognised these objects as souls, when the reader
    /// states it.
    pub recognition: Option<Evidence>,
    /// The game account the reading belongs to, when the reader read it.
    pub account: Option<String>,
    /// The evidence behind each typed field the reader fills. A field missing here is not
    /// mapped, and is `None` on every soul.
    pub evidence: BTreeMap<SoulField, Evidence>,
    pub souls: Vec<SoulObservation>,
}

impl SoulReading {
    /// The evidence for a field, or `None` when the reader does not map it.
    pub fn evidence_of(&self, field: SoulField) -> Option<Evidence> {
        self.evidence.get(&field).copied()
    }
}

/// An attribute as the game stores it: its code and its stored value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeReading {
    pub code: u32,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SubAttributeReading {
    pub code: u32,
    pub value: f64,
    /// The rolls the game records for it, when it records them.
    pub roll_count: Option<u32>,
}

/// One soul of a reading. A typed field is `None` when the reader does not map it, or when it
/// maps it and the record lacks it; [`SoulReading::evidence_of`] tells the two apart.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SoulObservation {
    pub soul_id: Option<String>,
    pub suit_code: Option<u32>,
    pub star: Option<u32>,
    pub slot: Option<u32>,
    pub level: Option<u32>,
    pub main: Option<AttributeReading>,
    pub subs: Vec<SubAttributeReading>,
    /// `None` both when not mapped and when mapped and absent. Neither says the soul has no innate
    /// attribute: until a recording establishes how a reading carries it, decode maps `None` to
    /// `Innate::Unknown`, never to `Innate::Absent` (`scheme-code.md`).
    pub innate: Option<AttributeReading>,
    pub locked: Option<bool>,
    pub discarded: Option<bool>,
    /// What the reader saw, when it sent it.
    pub observed: Option<ObservedRecord>,
}

/// A record as the game's runtime holds it: every entry, mapped or not.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservedRecord {
    pub type_name: String,
    /// The key the record is stored under in its container, when the reader found one.
    pub container_key: Option<RawValue>,
    pub entries: Vec<(RawValue, RawValue)>,
}

impl ObservedRecord {
    /// The value of the first entry whose key [`RawValue::matches_key`] `key`.
    pub fn get(&self, key: &str) -> Option<&RawValue> {
        self.entries
            .iter()
            .find(|(k, _)| k.matches_key(key))
            .map(|(_, v)| v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SequenceKind {
    List,
    Tuple,
    Unstated,
}

/// Why the reader left a value unread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnreadReason {
    UnknownKind,
    DepthLimit,
    Unreadable,
    Malformed,
    OutOfRange,
    /// The reading gave no value kind or no reason.
    Unstated,
}

/// One value as the runtime holds it.
#[derive(Debug, Clone, PartialEq)]
pub enum RawValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Text(String),
    /// `items` may be shorter than `length`: the reader's item limit cut it.
    Sequence {
        kind: SequenceKind,
        items: Vec<RawValue>,
        length: u64,
    },
    /// `entries` may be shorter than `length`: the reader's entry limit cut it.
    Mapping {
        entries: Vec<(RawValue, RawValue)>,
        length: u64,
    },
    Unread {
        type_name: String,
        reason: UnreadReason,
    },
}

/// The kind of a [`RawValue`], without its content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RawKind {
    Null,
    Bool,
    Integer,
    Float,
    Text,
    List,
    Tuple,
    Sequence,
    Mapping,
    Unread,
}

impl RawValue {
    pub fn kind(&self) -> RawKind {
        match self {
            RawValue::Null => RawKind::Null,
            RawValue::Bool(_) => RawKind::Bool,
            RawValue::Integer(_) => RawKind::Integer,
            RawValue::Float(_) => RawKind::Float,
            RawValue::Text(_) => RawKind::Text,
            RawValue::Sequence {
                kind: SequenceKind::List,
                ..
            } => RawKind::List,
            RawValue::Sequence {
                kind: SequenceKind::Tuple,
                ..
            } => RawKind::Tuple,
            RawValue::Sequence { .. } => RawKind::Sequence,
            RawValue::Mapping { .. } => RawKind::Mapping,
            RawValue::Unread { .. } => RawKind::Unread,
        }
    }

    /// Whether this value, as an entry key, is the key a user names: a text key by its text, an
    /// integer key by its decimal form.
    pub fn matches_key(&self, key: &str) -> bool {
        match self {
            RawValue::Text(t) => t == key,
            RawValue::Integer(n) => n.to_string() == key,
            _ => false,
        }
    }

    /// A one-line, deterministic rendering: the form the research tools print and group by.
    pub fn render(&self) -> String {
        let mut out = String::new();
        self.render_into(&mut out);
        out
    }

    fn render_into(&self, out: &mut String) {
        match self {
            RawValue::Null => out.push_str("null"),
            RawValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            RawValue::Integer(n) => out.push_str(&n.to_string()),
            // `{:?}` keeps a trailing `.0`, so 3.0 is not rendered as the integer 3.
            RawValue::Float(x) => out.push_str(&format!("{x:?}")),
            RawValue::Text(t) => out.push_str(&format!("{t:?}")),
            RawValue::Sequence {
                kind,
                items,
                length,
            } => {
                let (open, close) = match kind {
                    SequenceKind::Tuple => ('(', ')'),
                    SequenceKind::List | SequenceKind::Unstated => ('[', ']'),
                };
                out.push(open);
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    item.render_into(out);
                }
                if (items.len() as u64) < *length {
                    out.push_str(&format!(", …{} more", length - items.len() as u64));
                }
                out.push(close);
            }
            RawValue::Mapping { entries, length } => {
                out.push('{');
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    k.render_into(out);
                    out.push_str(": ");
                    v.render_into(out);
                }
                if (entries.len() as u64) < *length {
                    out.push_str(&format!(", …{} more", length - entries.len() as u64));
                }
                out.push('}');
            }
            RawValue::Unread { type_name, reason } => {
                out.push_str(&format!("<{type_name}: {reason:?}>"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_names_round_trip() {
        for f in SoulField::ALL {
            assert_eq!(SoulField::from_schema_name(f.schema_name()), Some(f));
        }
        assert_eq!(SoulField::from_schema_name("SoulRecord.nothing"), None);
    }

    #[test]
    fn inherited_is_weaker_than_established() {
        assert!(Evidence::Inherited < Evidence::Established);
    }

    #[test]
    fn renderings_distinguish_kinds() {
        let tuple = RawValue::Sequence {
            kind: SequenceKind::Tuple,
            items: vec![RawValue::Integer(3), RawValue::Float(3.0)],
            length: 5,
        };
        assert_eq!(tuple.render(), "(3, 3.0, …3 more)");
        let map = RawValue::Mapping {
            entries: vec![(RawValue::Text("k".into()), RawValue::Null)],
            length: 1,
        };
        assert_eq!(map.render(), "{\"k\": null}");
        let unread = RawValue::Unread {
            type_name: "set".into(),
            reason: UnreadReason::UnknownKind,
        };
        assert_eq!(unread.render(), "<set: UnknownKind>");
    }

    #[test]
    fn keys_match_by_text_or_decimal() {
        assert!(RawValue::Text("a".into()).matches_key("a"));
        assert!(RawValue::Integer(-4).matches_key("-4"));
        assert!(!RawValue::Text("4".into()).matches_key("a"));
        assert!(!RawValue::Null.matches_key("null"));
    }
}
