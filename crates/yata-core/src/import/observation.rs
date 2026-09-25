//! A reading of the game as the domain receives it: typed fields where the reader's layout maps
//! them, each with the evidence behind that mapping, and the record the reader saw, verbatim.
//!
//! Nothing here is interpreted beyond its type. A suit code is the game's code, an attribute code
//! the game's code, a value the game's stored value; turning them into [`crate::soul`] values is a
//! separate step, and one that waits on evidence (`probe-protocol.md`, "Evidence").
//!
//! A [`SoulReading`] is built only by [`SoulReading::new`], from the reading's mappings and each
//! record's raw values ([`RawSoul`]). That is where "not mapped" and "mapped but missing" are told
//! apart, once: every [`SoulObservation`] field is a [`Field`] that says which it is, so a value
//! for an unmapped field cannot exist.

/// How a layout mapping is known (`probe-protocol.md`, "Evidence"). Ordered by strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Evidence {
    /// A hypothesis taken from the prior tool (ADR-0014), not yet re-established.
    Inherited,
    /// Re-established by this project's own recording or single-variable test.
    Established,
}

/// A mapping as the reading states it: its evidence, and for an established one where that
/// evidence is recorded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mapping {
    Inherited,
    Established { basis: String },
}

impl Mapping {
    pub fn evidence(&self) -> Evidence {
        match self {
            Mapping::Inherited => Evidence::Inherited,
            Mapping::Established { .. } => Evidence::Established,
        }
    }
}

/// One typed field of one soul.
#[derive(Debug, Clone, PartialEq)]
pub enum Field<T> {
    /// The reader does not map this field.
    Unmapped,
    /// The reader maps it with this evidence; `None` is a record that lacks it.
    Mapped {
        evidence: Evidence,
        value: Option<T>,
    },
}

impl<T> Field<T> {
    /// The field a record holds under its mapping. A value on a field the reading does not map
    /// is a contradiction, refused with the field's name.
    fn from_mapping(
        field: SoulField,
        mapping: Option<&Mapping>,
        value: Option<T>,
    ) -> Result<Field<T>, SoulField> {
        match (mapping, value) {
            (None, None) => Ok(Field::Unmapped),
            (None, Some(_)) => Err(field),
            (Some(m), value) => Ok(Field::Mapped {
                evidence: m.evidence(),
                value,
            }),
        }
    }

    /// The evidence behind the mapping, or `None` when unmapped.
    pub fn evidence(&self) -> Option<Evidence> {
        match self {
            Field::Unmapped => None,
            Field::Mapped { evidence, .. } => Some(*evidence),
        }
    }

    /// The value, when the field is mapped and the record holds it.
    pub fn value(&self) -> Option<&T> {
        match self {
            Field::Unmapped => None,
            Field::Mapped { value, .. } => value.as_ref(),
        }
    }
}

/// A typed field of [`SoulObservation`].
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

    /// The field's name in the probe schema's `SoulRecord`.
    pub fn name(self) -> &'static str {
        match self {
            SoulField::SoulId => "soul_id",
            SoulField::SuitCode => "suit_code",
            SoulField::Star => "star",
            SoulField::Slot => "slot",
            SoulField::Level => "level",
            SoulField::Main => "main",
            SoulField::Subs => "subs",
            SoulField::Innate => "innate",
            SoulField::Locked => "locked",
            SoulField::Discarded => "discarded",
        }
    }

    pub fn from_name(name: &str) -> Option<SoulField> {
        SoulField::ALL.into_iter().find(|f| f.name() == name)
    }
}

/// The reading's mapping for each typed field; `None` is a field the reader does not map.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SoulMappings {
    pub soul_id: Option<Mapping>,
    pub suit_code: Option<Mapping>,
    pub star: Option<Mapping>,
    pub slot: Option<Mapping>,
    pub level: Option<Mapping>,
    pub main: Option<Mapping>,
    pub subs: Option<Mapping>,
    pub innate: Option<Mapping>,
    pub locked: Option<Mapping>,
    pub discarded: Option<Mapping>,
}

impl SoulMappings {
    pub fn get(&self, field: SoulField) -> Option<&Mapping> {
        match field {
            SoulField::SoulId => self.soul_id.as_ref(),
            SoulField::SuitCode => self.suit_code.as_ref(),
            SoulField::Star => self.star.as_ref(),
            SoulField::Slot => self.slot.as_ref(),
            SoulField::Level => self.level.as_ref(),
            SoulField::Main => self.main.as_ref(),
            SoulField::Subs => self.subs.as_ref(),
            SoulField::Innate => self.innate.as_ref(),
            SoulField::Locked => self.locked.as_ref(),
            SoulField::Discarded => self.discarded.as_ref(),
        }
    }
}

/// Whether a reading covers its whole scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coverage {
    Complete,
    Partial,
}

/// The game's suit code, as read: not yet a [`crate::soul::SoulSet`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameSuitCode(pub u32);

/// The game's attribute code, as read: not yet a [`crate::soul::SoulAttribute`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameAttributeCode(pub u32);

/// The game's star value, as read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameStar(pub u32);

/// The game's slot number, as read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameSlot(pub u32);

/// The game's level value, as read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameLevel(pub u32);

/// An attribute as the game stores it: its code and its stored value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeReading {
    pub code: GameAttributeCode,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SubAttributeReading {
    pub code: GameAttributeCode,
    pub value: f64,
    /// The rolls the game records for it, when it records them.
    pub roll_count: Option<u32>,
}

/// A soul's innate attribute as its record holds it. Decode maps `None` to an ordinary soul and
/// `Present` to a boss soul (ADR-0029); a soul whose field is unmapped or missing has no kind to
/// decode, and decode refuses it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InnateReading {
    None,
    Present(AttributeReading),
}

/// One record's raw values, before the reading's mappings are applied: what conversion from the
/// wire produces, and the input of [`SoulReading::new`].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RawSoul {
    pub soul_id: Option<String>,
    pub suit_code: Option<GameSuitCode>,
    pub star: Option<GameStar>,
    pub slot: Option<GameSlot>,
    pub level: Option<GameLevel>,
    pub main: Option<AttributeReading>,
    pub subs: Option<Vec<SubAttributeReading>>,
    /// `None` when the record lacks the field. Only a mapped field decides a soul's kind:
    /// [`InnateReading::None`] is an ordinary soul, [`InnateReading::Present`] a boss soul, and
    /// a record whose field is unmapped or lacking is not a row (ADR-0029).
    pub innate: Option<InnateReading>,
    pub locked: Option<bool>,
    pub discarded: Option<bool>,
    pub observed: Option<ObservedRecord>,
}

/// One soul of a reading.
#[derive(Debug, Clone, PartialEq)]
pub struct SoulObservation {
    pub soul_id: Field<String>,
    pub suit_code: Field<GameSuitCode>,
    pub star: Field<GameStar>,
    pub slot: Field<GameSlot>,
    pub level: Field<GameLevel>,
    pub main: Field<AttributeReading>,
    pub subs: Field<Vec<SubAttributeReading>>,
    pub innate: Field<InnateReading>,
    pub locked: Field<bool>,
    pub discarded: Field<bool>,
    /// What the reader saw, when it sent it.
    pub observed: Option<ObservedRecord>,
}

impl SoulObservation {
    /// The evidence behind one of this soul's fields, or `None` when unmapped.
    pub fn evidence_of(&self, field: SoulField) -> Option<Evidence> {
        match field {
            SoulField::SoulId => self.soul_id.evidence(),
            SoulField::SuitCode => self.suit_code.evidence(),
            SoulField::Star => self.star.evidence(),
            SoulField::Slot => self.slot.evidence(),
            SoulField::Level => self.level.evidence(),
            SoulField::Main => self.main.evidence(),
            SoulField::Subs => self.subs.evidence(),
            SoulField::Innate => self.innate.evidence(),
            SoulField::Locked => self.locked.evidence(),
            SoulField::Discarded => self.discarded.evidence(),
        }
    }
}

/// Why a reading's records contradict what the reading states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadingDefect {
    /// A record holds a value for a field the reading says it does not map.
    ValueOnUnmappedField { index: usize, field: SoulField },
    /// The reading names an account with an empty id.
    EmptyAccount,
    /// A record's soul id is empty.
    EmptySoulId { index: usize },
}

/// One reading of the souls scope.
#[derive(Debug, Clone, PartialEq)]
pub struct SoulReading {
    coverage: Coverage,
    account: Option<String>,
    recognition: Option<Mapping>,
    mappings: SoulMappings,
    souls: Vec<SoulObservation>,
}

impl SoulReading {
    /// A reading from its mappings and its records' raw values. Each field of each soul is
    /// unmapped exactly when its mapping is; a value on an unmapped field, an empty account id,
    /// or an empty soul id contradicts the reading and is refused.
    pub fn new(
        coverage: Coverage,
        account: Option<String>,
        recognition: Option<Mapping>,
        mappings: SoulMappings,
        souls: Vec<RawSoul>,
    ) -> Result<SoulReading, ReadingDefect> {
        if account.as_deref() == Some("") {
            return Err(ReadingDefect::EmptyAccount);
        }
        let m = &mappings;
        let souls = souls
            .into_iter()
            .enumerate()
            .map(|(index, r)| {
                if r.soul_id.as_deref() == Some("") {
                    return Err(ReadingDefect::EmptySoulId { index });
                }
                fn take<T>(
                    index: usize,
                    field: SoulField,
                    mapping: &Option<Mapping>,
                    value: Option<T>,
                ) -> Result<Field<T>, ReadingDefect> {
                    Field::from_mapping(field, mapping.as_ref(), value)
                        .map_err(|field| ReadingDefect::ValueOnUnmappedField { index, field })
                }
                Ok(SoulObservation {
                    soul_id: take(index, SoulField::SoulId, &m.soul_id, r.soul_id)?,
                    suit_code: take(index, SoulField::SuitCode, &m.suit_code, r.suit_code)?,
                    star: take(index, SoulField::Star, &m.star, r.star)?,
                    slot: take(index, SoulField::Slot, &m.slot, r.slot)?,
                    level: take(index, SoulField::Level, &m.level, r.level)?,
                    main: take(index, SoulField::Main, &m.main, r.main)?,
                    subs: take(index, SoulField::Subs, &m.subs, r.subs)?,
                    innate: take(index, SoulField::Innate, &m.innate, r.innate)?,
                    locked: take(index, SoulField::Locked, &m.locked, r.locked)?,
                    discarded: take(index, SoulField::Discarded, &m.discarded, r.discarded)?,
                    observed: r.observed,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(SoulReading {
            coverage,
            account,
            recognition,
            mappings,
            souls,
        })
    }

    pub fn coverage(&self) -> Coverage {
        self.coverage
    }

    /// The game account the reading belongs to, when the reader read it.
    pub fn account(&self) -> Option<&str> {
        self.account.as_deref()
    }

    /// The rule that recognised these objects as souls, when the reader states it.
    pub fn recognition(&self) -> Option<&Mapping> {
        self.recognition.as_ref()
    }

    /// Which fields the reader maps, with the evidence and its basis.
    pub fn mappings(&self) -> &SoulMappings {
        &self.mappings
    }

    pub fn souls(&self) -> &[SoulObservation] {
        &self.souls
    }
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
}

/// Why the reader left a value unread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnreadReason {
    UnknownKind,
    DepthLimit,
    Unreadable,
    Malformed,
    OutOfRange,
}

/// One value as the runtime holds it.
#[derive(Debug, Clone, PartialEq)]
pub enum RawValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Text(String),
    /// `full_length` is set only when the reader's item limit cut the sequence, and is then
    /// above the number of items.
    Sequence {
        kind: SequenceKind,
        items: Vec<RawValue>,
        full_length: Option<u64>,
    },
    /// `full_length` as for [`RawValue::Sequence`].
    Mapping {
        entries: Vec<(RawValue, RawValue)>,
        full_length: Option<u64>,
    },
    Unread {
        /// The runtime's name for the value's type, when it could be read.
        type_name: Option<String>,
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
            RawValue::Mapping { .. } => RawKind::Mapping,
            RawValue::Unread { .. } => RawKind::Unread,
        }
    }

    /// Whether this value, as an entry key, is the key a user names: a text key by its text, an
    /// integer key by its decimal form.
    pub fn matches_key(&self, key: &str) -> bool {
        match self {
            RawValue::Text(t) => t == key,
            RawValue::Integer(n) => key.parse::<i64>() == Ok(*n),
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
        let more = |shown: usize, full: Option<u64>| {
            full.map(|f| format!(", …{} more", f.saturating_sub(shown as u64)))
                .unwrap_or_default()
        };
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
                full_length,
            } => {
                let (open, close) = match kind {
                    SequenceKind::Tuple => ('(', ')'),
                    SequenceKind::List => ('[', ']'),
                };
                out.push(open);
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    item.render_into(out);
                }
                out.push_str(&more(items.len(), *full_length));
                out.push(close);
            }
            RawValue::Mapping {
                entries,
                full_length,
            } => {
                out.push('{');
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    k.render_into(out);
                    out.push_str(": ");
                    v.render_into(out);
                }
                out.push_str(&more(entries.len(), *full_length));
                out.push('}');
            }
            RawValue::Unread { type_name, reason } => {
                let name = type_name.as_deref().unwrap_or("?");
                out.push_str(&format!("<{name}: {reason:?}>"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_names_round_trip() {
        for f in SoulField::ALL {
            assert_eq!(SoulField::from_name(f.name()), Some(f));
        }
        assert_eq!(SoulField::from_name("nothing"), None);
    }

    #[test]
    fn inherited_is_weaker_than_established() {
        assert!(Evidence::Inherited < Evidence::Established);
    }

    #[test]
    fn a_value_on_an_unmapped_field_contradicts_the_reading() {
        let mappings = SoulMappings {
            star: Some(Mapping::Inherited),
            ..SoulMappings::default()
        };
        let mapped = RawSoul {
            star: Some(GameStar(6)),
            ..RawSoul::default()
        };
        let r = SoulReading::new(
            Coverage::Partial,
            None,
            None,
            mappings.clone(),
            vec![mapped],
        )
        .expect("consistent");
        let soul = &r.souls()[0];
        assert_eq!(soul.star.value(), Some(&GameStar(6)));
        assert_eq!(soul.star.evidence(), Some(Evidence::Inherited));
        assert_eq!(soul.level, Field::Unmapped);
        let stray = RawSoul {
            star: Some(GameStar(6)),
            level: Some(GameLevel(15)),
            ..RawSoul::default()
        };
        assert_eq!(
            SoulReading::new(
                Coverage::Partial,
                None,
                None,
                mappings,
                vec![RawSoul::default(), stray]
            ),
            Err(ReadingDefect::ValueOnUnmappedField {
                index: 1,
                field: SoulField::Level
            })
        );
    }

    #[test]
    fn an_empty_account_or_soul_id_is_refused() {
        let ids = SoulMappings {
            soul_id: Some(Mapping::Inherited),
            ..SoulMappings::default()
        };
        assert_eq!(
            SoulReading::new(
                Coverage::Partial,
                Some(String::new()),
                None,
                SoulMappings::default(),
                vec![]
            ),
            Err(ReadingDefect::EmptyAccount)
        );
        let empty = RawSoul {
            soul_id: Some(String::new()),
            ..RawSoul::default()
        };
        assert_eq!(
            SoulReading::new(Coverage::Partial, None, None, ids, vec![empty]),
            Err(ReadingDefect::EmptySoulId { index: 0 })
        );
    }

    #[test]
    fn a_mapped_field_a_record_lacks_is_mapped_and_empty() {
        let mappings = SoulMappings {
            subs: Some(Mapping::Established {
                basis: "exp".into(),
            }),
            ..SoulMappings::default()
        };
        let r = SoulReading::new(
            Coverage::Complete,
            None,
            None,
            mappings,
            vec![RawSoul::default()],
        )
        .expect("consistent");
        assert_eq!(
            r.souls()[0].subs,
            Field::Mapped {
                evidence: Evidence::Established,
                value: None
            }
        );
    }

    #[test]
    fn renderings_distinguish_kinds() {
        let tuple = RawValue::Sequence {
            kind: SequenceKind::Tuple,
            items: vec![RawValue::Integer(3), RawValue::Float(3.0)],
            full_length: Some(5),
        };
        assert_eq!(tuple.render(), "(3, 3.0, …3 more)");
        let map = RawValue::Mapping {
            entries: vec![(RawValue::Text("k".into()), RawValue::Null)],
            full_length: None,
        };
        assert_eq!(map.render(), "{\"k\": null}");
        let unread = RawValue::Unread {
            type_name: Some("set".into()),
            reason: UnreadReason::UnknownKind,
        };
        assert_eq!(unread.render(), "<set: UnknownKind>");
    }

    #[test]
    fn keys_match_by_text_or_integer_value() {
        assert!(RawValue::Text("a".into()).matches_key("a"));
        assert!(RawValue::Integer(-4).matches_key("-4"));
        assert!(RawValue::Integer(4).matches_key("04"));
        assert!(!RawValue::Text("4".into()).matches_key("a"));
        assert!(!RawValue::Null.matches_key("null"));
    }
}
