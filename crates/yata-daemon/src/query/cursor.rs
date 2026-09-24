//! The page cursor's bytes. A cursor is opaque to the client (`core-protocol.md`, "Queries and
//! pages"), so its encoding is the daemon's own and is not in the schema file: a small protobuf
//! message of the last row's sort-key values and id.

use prost::{Message, Oneof};
use yata_core::query::{Cursor, SortValue};
use yata_core::soul::SoulSet;

use super::convert::{attribute, wire_attribute, wire_slot};

#[derive(Clone, PartialEq, Message)]
struct CursorBytes {
    #[prost(message, repeated, tag = "1")]
    keys: Vec<KeyBytes>,
    #[prost(string, tag = "2")]
    soul_id: String,
}

#[derive(Clone, PartialEq, Message)]
struct KeyBytes {
    #[prost(oneof = "Key", tags = "1, 2, 3, 4, 5, 6")]
    key: Option<Key>,
}

#[derive(Clone, PartialEq, Oneof)]
enum Key {
    #[prost(uint32, tag = "1")]
    Set(u32),
    #[prost(int32, tag = "2")]
    Slot(i32),
    #[prost(int32, tag = "3")]
    Attribute(i32),
    #[prost(sint64, tag = "4")]
    Int(i64),
    #[prost(double, tag = "5")]
    Number(f64),
    #[prost(bool, tag = "6")]
    Bool(bool),
}

pub fn encode(cursor: &Cursor<String>) -> Vec<u8> {
    let key = |v: &SortValue| KeyBytes {
        key: Some(match *v {
            SortValue::Set(s) => Key::Set(u32::from(s.suit_code())),
            SortValue::Slot(k) => Key::Slot(wire_slot(k) as i32),
            SortValue::Attribute(a) => Key::Attribute(wire_attribute(a) as i32),
            SortValue::Int(i) => Key::Int(i),
            SortValue::Number(n) => Key::Number(n),
            SortValue::Bool(b) => Key::Bool(b),
        }),
    };
    CursorBytes {
        keys: cursor.keys.iter().map(key).collect(),
        soul_id: cursor.id.clone(),
    }
    .encode_to_vec()
}

/// The cursor these bytes hold, if they are one this daemon wrote. Whether it continues the
/// query's order is the core's to decide.
pub fn decode(bytes: &[u8]) -> Option<Cursor<String>> {
    let c = CursorBytes::decode(bytes).ok()?;
    let value = |k: KeyBytes| {
        Some(match k.key? {
            Key::Set(code) => SortValue::Set(SoulSet::from_suit_code(u8::try_from(code).ok()?)),
            Key::Slot(v) => SortValue::Slot(super::convert::slot_of(v)?),
            Key::Attribute(v) => SortValue::Attribute(attribute(v).ok()?),
            Key::Int(i) => SortValue::Int(i),
            Key::Number(n) => SortValue::Number(n),
            Key::Bool(b) => SortValue::Bool(b),
        })
    };
    Some(Cursor {
        keys: c.keys.into_iter().map(value).collect::<Option<_>>()?,
        id: c.soul_id,
    })
}
