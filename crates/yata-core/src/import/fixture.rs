//! Small snapshots for tests, written in the IR directly.

use crate::fact::Digest;
use crate::soul::SoulAttribute::{self, *};

use super::ir::*;

pub fn id(s: &str) -> SourceId {
    SourceId::new(s).expect("an id")
}

pub fn valued(attribute: SoulAttribute, value: f64) -> Valued {
    Valued { attribute, value }
}

/// A +15 six-star 破势 in `slot` with two rolled sub-attributes.
pub fn soul(name: &str, slot: i64) -> SoulRecord {
    SoulRecord {
        id: id(name),
        set: SetName::new("破势").expect("a name"),
        slot,
        star: 6,
        level: 15,
        main: valued(AtkFlat, 486.0),
        rolled: vec![
            RolledSub {
                valued: valued(Crit, 3.0),
                rolls: Some(1),
            },
            RolledSub {
                valued: valued(Spd, 11.35),
                rolls: Some(3),
            },
        ],
        innate: None,
    }
}

pub fn complete<T>(value: T) -> Section<T> {
    Section::Present {
        completeness: Completeness::Complete,
        value,
    }
}

/// A snapshot with every section absent.
pub fn empty() -> YataSnapshot {
    YataSnapshot {
        schema: SchemaVersion::CURRENT,
        provenance: Provenance {
            format: SourceFormat::YataSnapshot(SchemaVersion::CURRENT),
            original: Digest([7; 32]),
        },
        captured_at: None,
        account: None,
        souls: Section::Absent,
        shikigami: Section::Absent,
        presets: Section::Absent,
        assets: Section::Absent,
        guild: Section::Absent,
    }
}

pub fn with_souls(souls: Vec<SoulRecord>) -> YataSnapshot {
    YataSnapshot {
        souls: complete(Souls { souls }),
        ..empty()
    }
}

pub fn preset(souls: [Option<&str>; 6]) -> GamePreset {
    GamePreset {
        label: Label::new("主力").expect("a label"),
        souls: souls.map(|s| s.map(id)),
    }
}

pub fn shikigami(name: &str, level: i64) -> ShikigamiRecord {
    ShikigamiRecord {
        id: id(name),
        species: SpeciesNumber(301),
        level,
        star: 6,
        evolved: true,
        locked: false,
    }
}
