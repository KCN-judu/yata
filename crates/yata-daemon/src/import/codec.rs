//! The one conversion between the snapshot IR and its schema (`snapshot.proto`, ADR-0031), and
//! the stored form the fact log keeps.
//!
//! [`encode_snapshot`] is the canonical binary encoding: the schema has no map fields and prost
//! writes fields in tag order, so equivalent snapshots encode to identical bytes, and the digest
//! of those bytes is the snapshot's identity. [`decode_snapshot`] reads a stored snapshot back,
//! strictly: a stored blob was written by this code, so any record it cannot convert is an
//! error. [`from_proto`] is the lenient reading a yata-snapshot file gets on import: a record it
//! cannot convert is left out and reported, and its section becomes partial, as for any format.

use prost::Message;
use yata_core::fact::Digest;
use yata_core::import::ir::{
    Assets, Completeness, Currency, FormatTag, GamePreset, GamePresets, Guild, Label, Provenance,
    RealmCardKind, RealmCardRecord, RolledSub, SchemaVersion, Section, SectionKind, SetName,
    ShikigamiRecord, ShikigamiRoster, SoulRecord, Souls, SourceFormat, SourceId, SpeciesNumber,
    Valued, YataSnapshot,
};
use yata_core::soul::SoulAttribute;
use yata_protocol::snapshot as pb;

use super::{Kind, Problem, SourceDefect, SourceReason, partial_if};

/// Why stored or file bytes are not a snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotDecodeError {
    /// Not the binary encoding of a `YataSnapshot`.
    Protobuf { reason: String },
    /// A major version this build does not read.
    UnsupportedVersion { found: SchemaVersion },
    /// A field every snapshot has is unset.
    Missing { field: &'static str },
    /// An enum field holds its zero value, or a name this build does not know.
    Unspecified { field: &'static str },
    /// The provenance digest is not 32 bytes.
    BadDigest { len: usize },
    /// A record that a stored snapshot must never hold.
    Record(SourceDefect),
}

// ---- IR → schema ----------------------------------------------------------------------------

fn version(v: SchemaVersion) -> pb::SchemaVersion {
    pb::SchemaVersion {
        major: v.major,
        minor: v.minor,
    }
}

fn completeness(c: Completeness) -> i32 {
    match c {
        Completeness::Unstated => pb::Completeness::Unstated,
        Completeness::Partial => pb::Completeness::Partial,
        Completeness::Complete => pb::Completeness::Complete,
    }
    .into()
}

fn attribute(a: SoulAttribute) -> i32 {
    use SoulAttribute::*;
    match a {
        AtkFlat => pb::Attribute::AtkFlat,
        AtkPercent => pb::Attribute::AtkPercent,
        DefFlat => pb::Attribute::DefFlat,
        DefPercent => pb::Attribute::DefPercent,
        HpFlat => pb::Attribute::HpFlat,
        HpPercent => pb::Attribute::HpPercent,
        Spd => pb::Attribute::Spd,
        EffectHit => pb::Attribute::EffectHit,
        EffectRes => pb::Attribute::EffectRes,
        Crit => pb::Attribute::Crit,
        CritDmg => pb::Attribute::CritDmg,
    }
    .into()
}

fn currency(c: Currency) -> i32 {
    use Currency::*;
    match c {
        ActionPoint => pb::Currency::ActionPoint,
        ArAmulet => pb::Currency::ArAmulet,
        AutoPoint => pb::Currency::AutoPoint,
        BrokenAmulet => pb::Currency::BrokenAmulet,
        Coin => pb::Currency::Coin,
        Contrib => pb::Currency::Contrib,
        DemonSoul => pb::Currency::DemonSoul,
        FooleryPass => pb::Currency::FooleryPass,
        GoldOfuda => pb::Currency::GoldOfuda,
        Honor => pb::Currency::Honor,
        Jade => pb::Currency::Jade,
        Medal => pb::Currency::Medal,
        MysteryAmulet => pb::Currency::MysteryAmulet,
        Ofuda => pb::Currency::Ofuda,
        RealmRaidPass => pb::Currency::RealmRaidPass,
        ReverseScale => pb::Currency::ReverseScale,
        SJade => pb::Currency::SJade,
        Scale => pb::Currency::Scale,
        SkinToken => pb::Currency::SkinToken,
        SpSkinToken => pb::Currency::SpSkinToken,
        TotemPass => pb::Currency::TotemPass,
    }
    .into()
}

fn valued(v: &Valued) -> pb::Valued {
    pb::Valued {
        attribute: attribute(v.attribute),
        value: v.value,
    }
}

fn soul(r: &SoulRecord) -> pb::Soul {
    pb::Soul {
        id: r.id.as_str().to_owned(),
        set: r.set.as_str().to_owned(),
        slot: r.slot,
        star: r.star,
        level: r.level,
        main: Some(valued(&r.main)),
        rolled: r
            .rolled
            .iter()
            .map(|s| pb::RolledSub {
                valued: Some(valued(&s.valued)),
                rolls: s.rolls,
            })
            .collect(),
        innate: r.innate.map(attribute),
    }
}

/// The schema's form of a snapshot.
pub fn to_proto(s: &YataSnapshot) -> pb::YataSnapshot {
    let c = |c: &Completeness| completeness(*c);
    pb::YataSnapshot {
        yata_snapshot: Some(version(s.schema)),
        provenance: Some(pb::Provenance {
            format: Some(match s.provenance.format {
                SourceFormat::YataSnapshot(v) => pb::provenance::Format::YataSnapshot(version(v)),
                SourceFormat::Community(FormatTag::MumuSnapshotV1) => {
                    pb::provenance::Format::Community(pb::FormatTag::MumuSnapshotV1.into())
                }
            }),
            original: s.provenance.original.0.to_vec(),
        }),
        captured_at: s.captured_at.clone(),
        souls: s.souls.present().map(|(k, v)| pb::SoulsSection {
            completeness: c(&k),
            souls: v.souls.iter().map(soul).collect(),
        }),
        shikigami: s.shikigami.present().map(|(k, v)| pb::ShikigamiSection {
            completeness: c(&k),
            instances: v
                .instances
                .iter()
                .map(|r| pb::Shikigami {
                    id: r.id.as_str().to_owned(),
                    species: r.species.0,
                    level: r.level,
                    star: r.star,
                    evolved: r.evolved,
                    locked: r.locked,
                })
                .collect(),
        }),
        presets: s.presets.present().map(|(k, v)| pb::PresetsSection {
            completeness: c(&k),
            presets: v
                .presets
                .iter()
                .map(|p| pb::Preset {
                    label: p.label.as_str().to_owned(),
                    positions: p
                        .souls
                        .iter()
                        .map(|s| pb::PresetPosition {
                            soul: s.as_ref().map(|id| id.as_str().to_owned()),
                        })
                        .collect(),
                })
                .collect(),
        }),
        assets: s.assets.present().map(|(k, v)| pb::AssetsSection {
            completeness: c(&k),
            currencies: v
                .currencies
                .iter()
                .map(|(cur, amount)| pb::CurrencyAmount {
                    currency: currency(*cur),
                    amount: *amount,
                })
                .collect(),
            realm_cards: v
                .realm_cards
                .iter()
                .map(|r| pb::RealmCard {
                    id: r.id.as_str().to_owned(),
                    kind: r.kind.0,
                })
                .collect(),
        }),
        guild: s.guild.present().map(|(k, g)| pb::GuildSection {
            completeness: c(&k),
            level: g.level,
            member_count: g.member_count,
        }),
    }
}

/// The canonical binary encoding: its SHA-256 is the snapshot's identity.
pub fn encode_snapshot(s: &YataSnapshot) -> Vec<u8> {
    to_proto(s).encode_to_vec()
}

// ---- schema → IR ----------------------------------------------------------------------------

fn from_version(v: &pb::SchemaVersion) -> SchemaVersion {
    SchemaVersion {
        major: v.major,
        minor: v.minor,
    }
}

fn from_completeness(n: i32) -> Result<Completeness, SnapshotDecodeError> {
    match pb::Completeness::try_from(n) {
        Ok(pb::Completeness::Unstated) => Ok(Completeness::Unstated),
        Ok(pb::Completeness::Partial) => Ok(Completeness::Partial),
        Ok(pb::Completeness::Complete) => Ok(Completeness::Complete),
        Ok(pb::Completeness::Unspecified) | Err(_) => Err(SnapshotDecodeError::Unspecified {
            field: "completeness",
        }),
    }
}

fn from_attribute(n: i32) -> Option<SoulAttribute> {
    use SoulAttribute::*;
    Some(match pb::Attribute::try_from(n).ok()? {
        pb::Attribute::Unspecified => return None,
        pb::Attribute::AtkFlat => AtkFlat,
        pb::Attribute::AtkPercent => AtkPercent,
        pb::Attribute::DefFlat => DefFlat,
        pb::Attribute::DefPercent => DefPercent,
        pb::Attribute::HpFlat => HpFlat,
        pb::Attribute::HpPercent => HpPercent,
        pb::Attribute::Spd => Spd,
        pb::Attribute::EffectHit => EffectHit,
        pb::Attribute::EffectRes => EffectRes,
        pb::Attribute::Crit => Crit,
        pb::Attribute::CritDmg => CritDmg,
    })
}

fn from_currency(n: i32) -> Option<Currency> {
    let c = pb::Currency::try_from(n).ok()?;
    Currency::ALL
        .into_iter()
        .find(|&k| currency(k) == i32::from(c))
}

fn text<T, E>(field: &str, t: &str, make: fn(&str) -> Result<T, E>) -> Result<T, SourceReason> {
    make(t).map_err(|_| SourceReason::Text {
        field: field.to_owned(),
    })
}

fn unsupported(field: &str, n: i32) -> SourceReason {
    SourceReason::UnsupportedValue {
        field: field.to_owned(),
        value: n.to_string(),
    }
}

fn missing(field: &str) -> SourceReason {
    SourceReason::Malformed {
        field: field.to_owned(),
        problem: Problem::Missing,
    }
}

fn from_valued(at: &str, v: Option<&pb::Valued>) -> Result<Valued, SourceReason> {
    let v = v.ok_or_else(|| missing(at))?;
    Ok(Valued {
        attribute: from_attribute(v.attribute)
            .ok_or_else(|| unsupported(&format!("{at}.attribute"), v.attribute))?,
        value: v.value,
    })
}

fn from_soul(r: &pb::Soul) -> Result<SoulRecord, SourceReason> {
    Ok(SoulRecord {
        id: text("id", &r.id, SourceId::new)?,
        set: text("set", &r.set, SetName::new)?,
        slot: r.slot,
        star: r.star,
        level: r.level,
        main: from_valued("main", r.main.as_ref())?,
        rolled: r
            .rolled
            .iter()
            .enumerate()
            .map(|(i, s)| {
                Ok(RolledSub {
                    valued: from_valued(&format!("rolled[{i}]"), s.valued.as_ref())?,
                    rolls: s.rolls,
                })
            })
            .collect::<Result<_, SourceReason>>()?,
        innate: r
            .innate
            .map(|n| from_attribute(n).ok_or_else(|| unsupported("innate", n)))
            .transpose()?,
    })
}

fn from_preset(p: &pb::Preset) -> Result<GamePreset, SourceReason> {
    let ids: Vec<Option<SourceId>> = p
        .positions
        .iter()
        .enumerate()
        .map(|(i, pos)| {
            pos.soul
                .as_deref()
                .map(|s| text(&format!("positions[{i}]"), s, SourceId::new))
                .transpose()
        })
        .collect::<Result<_, _>>()?;
    let souls: [Option<SourceId>; 6] = ids.try_into().map_err(|_| SourceReason::Malformed {
        field: "positions".to_owned(),
        problem: Problem::WrongKind {
            expected: Kind::Array,
        },
    })?;
    Ok(GamePreset {
        label: text("label", &p.label, Label::new)?,
        souls,
    })
}

/// Convert each record, keeping those that convert and reporting the rest.
fn records<R, T>(
    section: SectionKind,
    items: &[R],
    defects: &mut Vec<SourceDefect>,
    f: impl Fn(&R) -> Result<T, SourceReason>,
) -> Vec<T> {
    let mut out = Vec::new();
    for (index, r) in items.iter().enumerate() {
        match f(r) {
            Ok(v) => out.push(v),
            Err(reason) => defects.push(SourceDefect {
                section,
                index,
                reason,
            }),
        }
    }
    out
}

/// A present section, partial if a record was left out.
fn section<P, T>(
    p: Option<&P>,
    completeness_of: impl Fn(&P) -> i32,
    defects: &mut Vec<SourceDefect>,
    build: impl FnOnce(&P, &mut Vec<SourceDefect>) -> T,
) -> Result<Section<T>, SnapshotDecodeError> {
    let Some(p) = p else {
        return Ok(Section::Absent);
    };
    let completeness = from_completeness(completeness_of(p))?;
    let before = defects.len();
    let value = build(p, defects);
    let lost = defects.len() > before;
    Ok(partial_if(
        Section::Present {
            completeness,
            value,
        },
        lost,
    ))
}

/// The IR a schema value holds, with the records left out. The version is not checked here.
pub fn from_proto(
    p: &pb::YataSnapshot,
) -> Result<(YataSnapshot, Vec<SourceDefect>), SnapshotDecodeError> {
    let schema =
        p.yata_snapshot
            .as_ref()
            .map(from_version)
            .ok_or(SnapshotDecodeError::Missing {
                field: "yataSnapshot",
            })?;
    let prov = p.provenance.as_ref().ok_or(SnapshotDecodeError::Missing {
        field: "provenance",
    })?;
    let format = match prov.format.as_ref() {
        Some(pb::provenance::Format::YataSnapshot(v)) => {
            SourceFormat::YataSnapshot(from_version(v))
        }
        Some(pb::provenance::Format::Community(n)) => match pb::FormatTag::try_from(*n) {
            Ok(pb::FormatTag::MumuSnapshotV1) => SourceFormat::Community(FormatTag::MumuSnapshotV1),
            Ok(pb::FormatTag::Unspecified) | Err(_) => {
                return Err(SnapshotDecodeError::Unspecified {
                    field: "provenance.community",
                });
            }
        },
        None => {
            return Err(SnapshotDecodeError::Missing {
                field: "provenance.format",
            });
        }
    };
    let original: [u8; 32] =
        prov.original
            .as_slice()
            .try_into()
            .map_err(|_| SnapshotDecodeError::BadDigest {
                len: prov.original.len(),
            })?;
    let mut d = Vec::new();
    let souls = section(
        p.souls.as_ref(),
        |s| s.completeness,
        &mut d,
        |s, d| Souls {
            souls: records(SectionKind::Souls, &s.souls, d, from_soul),
        },
    )?;
    let shikigami = section(
        p.shikigami.as_ref(),
        |s| s.completeness,
        &mut d,
        |s, d| ShikigamiRoster {
            instances: records(SectionKind::Shikigami, &s.instances, d, |r| {
                Ok(ShikigamiRecord {
                    id: text("id", &r.id, SourceId::new)?,
                    species: SpeciesNumber(r.species),
                    level: r.level,
                    star: r.star,
                    evolved: r.evolved,
                    locked: r.locked,
                })
            }),
        },
    )?;
    let presets = section(
        p.presets.as_ref(),
        |s| s.completeness,
        &mut d,
        |s, d| GamePresets {
            presets: records(SectionKind::Presets, &s.presets, d, from_preset),
        },
    )?;
    let assets = section(
        p.assets.as_ref(),
        |s| s.completeness,
        &mut d,
        |s, d| Assets {
            currencies: records(SectionKind::Assets, &s.currencies, d, |c| {
                let cur =
                    from_currency(c.currency).ok_or_else(|| unsupported("currency", c.currency))?;
                Ok((cur, c.amount))
            }),
            realm_cards: records(SectionKind::Assets, &s.realm_cards, d, |r| {
                Ok(RealmCardRecord {
                    id: text("id", &r.id, SourceId::new)?,
                    kind: RealmCardKind(r.kind),
                })
            }),
        },
    )?;
    let guild = section(
        p.guild.as_ref(),
        |s| s.completeness,
        &mut d,
        |g, _| Guild {
            level: g.level,
            member_count: g.member_count,
        },
    )?;
    Ok((
        YataSnapshot {
            schema,
            provenance: Provenance {
                format,
                original: Digest(original),
            },
            captured_at: p.captured_at.clone(),
            souls,
            shikigami,
            presets,
            assets,
            guild,
        },
        d,
    ))
}

/// A stored snapshot, read back. Any record it cannot convert is an error: this code wrote it.
pub fn decode_snapshot(bytes: &[u8]) -> Result<YataSnapshot, SnapshotDecodeError> {
    let p = pb::YataSnapshot::decode(bytes).map_err(|e| SnapshotDecodeError::Protobuf {
        reason: e.to_string(),
    })?;
    if let Some(v) = &p.yata_snapshot
        && !yata_protocol::snapshot::accepts(*v)
    {
        return Err(SnapshotDecodeError::UnsupportedVersion {
            found: from_version(v),
        });
    }
    let (snapshot, mut left_out) = from_proto(&p)?;
    match left_out.pop() {
        None => Ok(snapshot),
        Some(d) => Err(SnapshotDecodeError::Record(d)),
    }
}
