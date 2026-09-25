//! The fact log against a real SQLite file: snapshots imported as blobs and section facts, the
//! inventory and the held sections derived from them, and the whole projection rebuilt from the
//! log alone (ADR-0002, ADR-0032).

use std::collections::BTreeMap;
use std::path::PathBuf;

use yata_core::fact::{
    Fact, FactBody, Facts, GameSoulId, Inventory, Mark, NoteText, Origin, ProfileId, Revision,
};
use yata_core::import::capability::{Availability, Capability, availability};
use yata_core::import::ir::{
    Completeness, Guild, Provenance, RolledSub, SchemaVersion, Section, SectionKind, SetName,
    ShikigamiRecord, ShikigamiRoster, SoulRecord, Souls, SourceFormat, SourceId, SpeciesNumber,
    Valued, YataSnapshot,
};
use yata_core::soul::SoulAttribute;
use yata_daemon::store::blob::digest_of;
use yata_daemon::store::{
    CanonicalCodec, CommandOutcome, FactLog, Imported, SnapshotCodec, Store, read_commits,
};
use yata_store::{GetBlob, ReplaceCache, StoreId};

use Completeness::{Complete, Partial, Unstated};
use SectionKind::{Guild as GuildKind, Shikigami, Souls as SoulsKind};

const P: ProfileId = ProfileId([1; 16]);
const Q: ProfileId = ProfileId([2; 16]);

struct Scratch(PathBuf);

impl Scratch {
    #[allow(
        clippy::expect_used,
        reason = "test helper: a failure here is the test failing"
    )]
    fn new(name: &str) -> Scratch {
        let dir = std::env::temp_dir()
            .join(format!("yata-facts-{}-{name}", std::process::id()))
            .join("数据 目录");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        Scratch(dir)
    }

    #[allow(
        clippy::expect_used,
        reason = "test helper: a failure here is the test failing"
    )]
    fn log(&self) -> FactLog {
        let store = Store::create_or_open(&self.0.join("store.sqlite3"), || StoreId([7; 16]))
            .expect("store");
        let mut log = FactLog::open(store).expect("log");
        if log.projection().revision() == Revision::EMPTY {
            let created = |profile| Fact {
                profile,
                body: FactBody::ProfileCreated {
                    display_name: "main".into(),
                    account: None,
                },
            };
            log.command(
                Revision::EMPTY,
                Origin::Maintenance,
                0,
                Facts::new(vec![created(P), created(Q)]).expect("facts"),
            )
            .expect("profiles");
        }
        log
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Some(parent) = self.0.parent() {
            let _ = std::fs::remove_dir_all(parent);
        }
    }
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn soul(id: &str, level: i64) -> SoulRecord {
    SoulRecord {
        id: SourceId::new(id).expect("an id"),
        set: SetName::new("破势").expect("a name"),
        slot: 2,
        star: 6,
        level,
        main: Valued {
            attribute: SoulAttribute::Spd,
            value: 57.0,
        },
        rolled: vec![RolledSub {
            valued: Valued {
                attribute: SoulAttribute::Crit,
                value: 3.0,
            },
            rolls: Some(1),
        }],
        innate: None,
    }
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn shikigami(id: &str) -> ShikigamiRecord {
    ShikigamiRecord {
        id: SourceId::new(id).expect("an id"),
        species: SpeciesNumber(301),
        level: 40,
        star: 6,
        evolved: true,
        locked: false,
    }
}

fn present<T>(completeness: Completeness, value: T) -> Section<T> {
    Section::Present {
        completeness,
        value,
    }
}

fn souls(completeness: Completeness, records: Vec<SoulRecord>) -> Section<Souls> {
    present(completeness, Souls { souls: records })
}

fn guild(completeness: Completeness) -> Section<Guild> {
    present(
        completeness,
        Guild {
            level: 10,
            member_count: 30,
        },
    )
}

/// The bytes of the file a snapshot was read from: any bytes, named so each snapshot has its own.
fn original(name: &str) -> Vec<u8> {
    format!("file {name}").into_bytes()
}

/// A snapshot read from the file `name`, with every section absent.
fn file(name: &str) -> YataSnapshot {
    YataSnapshot {
        schema: SchemaVersion::CURRENT,
        provenance: Provenance {
            format: SourceFormat::YataSnapshot(SchemaVersion::CURRENT),
            original: digest_of(&original(name)),
        },
        captured_at: None,
        souls: Section::Absent,
        shikigami: Section::Absent,
        presets: Section::Absent,
        assets: Section::Absent,
        guild: Section::Absent,
    }
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn import(log: &mut FactLog, profile: ProfileId, name: &str, snapshot: &YataSnapshot) -> Imported {
    log.import(
        profile,
        snapshot,
        &original(name),
        &CanonicalCodec,
        Origin::Job { job_id: 1 },
        0,
    )
    .expect("imports")
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn inventory(log: &mut FactLog, profile: ProfileId) -> Inventory {
    log.inventory(profile, &CanonicalCodec).expect("derives")
}

fn levels(inv: &Inventory) -> Vec<(String, u32)> {
    inv.souls()
        .map(|s| (s.id.as_str().to_owned(), u32::from(s.soul.level.get())))
        .collect()
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn held(log: &FactLog, profile: ProfileId) -> BTreeMap<SectionKind, Completeness> {
    log.held(profile).expect("a profile")
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn id(s: &str) -> GameSoulId {
    GameSoulId::new(s).expect("non-empty")
}

#[test]
fn souls_alone_give_the_inventory_and_nothing_else() {
    let dir = Scratch::new("souls-only");
    let mut log = dir.log();
    let s = YataSnapshot {
        souls: souls(Complete, vec![soul("a", 12), soul("b", 3)]),
        ..file("1")
    };
    import(&mut log, P, "1", &s);
    assert_eq!(
        levels(&inventory(&mut log, P)),
        vec![("a".into(), 12), ("b".into(), 3)]
    );
    let held = held(&log, P);
    assert_eq!(held, BTreeMap::from([(SoulsKind, Complete)]));
    assert_eq!(
        availability(&held, Capability::Inventory),
        Availability::Available {
            completeness: Complete
        }
    );
    assert!(matches!(
        availability(&held, Capability::ShikigamiCollection),
        Availability::Unavailable { .. }
    ));
}

#[test]
fn souls_and_shikigami_give_both_capabilities() {
    let dir = Scratch::new("souls-shikigami");
    let mut log = dir.log();
    let s = YataSnapshot {
        souls: souls(Complete, vec![soul("a", 15)]),
        shikigami: present(
            Partial,
            ShikigamiRoster {
                instances: vec![shikigami("s1")],
            },
        ),
        ..file("1")
    };
    import(&mut log, P, "1", &s);
    let held = held(&log, P);
    assert_eq!(
        held,
        BTreeMap::from([(SoulsKind, Complete), (Shikigami, Partial)])
    );
    assert_eq!(
        availability(&held, Capability::ShikigamiCollection),
        Availability::Available {
            completeness: Partial
        }
    );
    assert_eq!(levels(&inventory(&mut log, P)), vec![("a".into(), 15)]);
}

#[test]
fn a_guild_alone_holds_the_guild_and_no_souls() {
    let dir = Scratch::new("guild-only");
    let mut log = dir.log();
    import(
        &mut log,
        P,
        "1",
        &YataSnapshot {
            guild: guild(Complete),
            ..file("1")
        },
    );
    assert_eq!(held(&log, P), BTreeMap::from([(GuildKind, Complete)]));
    assert!(inventory(&mut log, P).is_empty());
    assert!(matches!(
        availability(&held(&log, P), Capability::Inventory),
        Availability::Unavailable { .. }
    ));
}

#[test]
fn a_present_guild_is_held_and_an_absent_one_is_not() {
    let dir = Scratch::new("guild-present-absent");
    let mut log = dir.log();
    let with = YataSnapshot {
        souls: souls(Complete, vec![]),
        guild: guild(Unstated),
        ..file("1")
    };
    let without = YataSnapshot {
        souls: souls(Complete, vec![]),
        ..file("2")
    };
    import(&mut log, P, "1", &with);
    import(&mut log, Q, "2", &without);
    assert_eq!(
        held(&log, P),
        BTreeMap::from([(SoulsKind, Complete), (GuildKind, Unstated)])
    );
    assert_eq!(held(&log, Q), BTreeMap::from([(SoulsKind, Complete)]));
}

#[test]
fn a_partial_import_updates_souls_and_removes_none() {
    let dir = Scratch::new("partial");
    let mut log = dir.log();
    let full = YataSnapshot {
        souls: souls(Complete, vec![soul("a", 12), soul("b", 3)]),
        ..file("1")
    };
    let partial = YataSnapshot {
        souls: souls(Partial, vec![soul("b", 6), soul("c", 0)]),
        ..file("2")
    };
    import(&mut log, P, "1", &full);
    import(&mut log, P, "2", &partial);
    assert_eq!(
        levels(&inventory(&mut log, P)),
        vec![("a".into(), 12), ("b".into(), 6), ("c".into(), 0)]
    );
    assert_eq!(held(&log, P), BTreeMap::from([(SoulsKind, Complete)]));
}

#[test]
fn an_import_without_the_guild_keeps_the_guild() {
    let dir = Scratch::new("keeps-guild");
    let mut log = dir.log();
    let both = YataSnapshot {
        souls: souls(Complete, vec![soul("a", 12)]),
        guild: guild(Complete),
        ..file("1")
    };
    let souls_only = YataSnapshot {
        souls: souls(Complete, vec![soul("b", 15)]),
        ..file("2")
    };
    let first = import(&mut log, P, "1", &both);
    import(&mut log, P, "2", &souls_only);
    let state = log.projection().profile(P).expect("P");
    assert_eq!(
        state.live(GuildKind).base,
        Some((first.seq, first.snapshot))
    );
    assert_eq!(
        held(&log, P),
        BTreeMap::from([(SoulsKind, Complete), (GuildKind, Complete)])
    );
    assert_eq!(levels(&inventory(&mut log, P)), vec![("b".into(), 15)]);
}

#[test]
fn importing_the_same_snapshot_again_changes_nothing_but_the_log() {
    let dir = Scratch::new("repeat");
    let mut log = dir.log();
    let s = YataSnapshot {
        souls: souls(Complete, vec![soul("a", 12)]),
        guild: guild(Partial),
        ..file("1")
    };
    let first = import(&mut log, P, "1", &s);
    let (inv, held_once) = (inventory(&mut log, P), held(&log, P));
    let second = import(&mut log, P, "1", &s);
    assert_eq!(first.snapshot, second.snapshot);
    assert_eq!(levels(&inventory(&mut log, P)), levels(&inv));
    assert_eq!(held(&log, P), held_once);
    // Two observations, one blob of each kind.
    let mut store = log.into_store();
    assert_eq!(read_commits(&mut store).map(|c| c.len()), Ok(3));
    for digest in [first.snapshot, digest_of(&original("1"))] {
        assert!(matches!(
            store.apply(GetBlob {
                digest: yata_store::Digest(digest.0)
            }),
            Ok(Some(_))
        ));
    }
}

#[test]
fn a_retraction_withdraws_every_section_of_its_snapshot() {
    let dir = Scratch::new("retract");
    let mut log = dir.log();
    let kept = YataSnapshot {
        souls: souls(Complete, vec![soul("a", 12)]),
        ..file("1")
    };
    let wrong = YataSnapshot {
        souls: souls(Complete, vec![soul("z", 0)]),
        guild: guild(Complete),
        ..file("2")
    };
    import(&mut log, P, "1", &kept);
    let landed = import(&mut log, P, "2", &wrong);
    let revision = log.projection().revision();
    let retract = Facts::one(Fact {
        profile: P,
        body: FactBody::SnapshotRetracted {
            snapshot: landed.snapshot,
            reason: "another account".into(),
        },
    });
    assert!(matches!(
        log.command(revision, Origin::Command { request_id: 9 }, 0, retract),
        Ok(CommandOutcome::Applied { .. })
    ));
    assert_eq!(levels(&inventory(&mut log, P)), vec![("a".into(), 12)]);
    assert_eq!(held(&log, P), BTreeMap::from([(SoulsKind, Complete)]));
}

#[test]
fn the_projection_rebuilds_from_the_log_alone() {
    let dir = Scratch::new("rebuild");
    let mut log = dir.log();
    import(
        &mut log,
        P,
        "1",
        &YataSnapshot {
            souls: souls(Complete, vec![soul("a", 12), soul("b", 3)]),
            guild: guild(Complete),
            ..file("1")
        },
    );
    import(
        &mut log,
        P,
        "2",
        &YataSnapshot {
            souls: souls(Partial, vec![soul("b", 6)]),
            ..file("2")
        },
    );
    import(
        &mut log,
        Q,
        "3",
        &YataSnapshot {
            souls: souls(Complete, vec![soul("a", 15)]),
            ..file("3")
        },
    );
    let revision = log.projection().revision();
    let decisions = Facts::new(vec![
        Fact {
            profile: P,
            body: FactBody::SoulMarked {
                soul: id("a"),
                mark: Some(Mark::Keep),
            },
        },
        Fact {
            profile: P,
            body: FactBody::SoulNoted {
                soul: id("b"),
                note: NoteText::new("双速"),
            },
        },
    ])
    .expect("facts");
    log.command(revision, Origin::Command { request_id: 3 }, 0, decisions)
        .expect("decisions");
    let projection = log.projection().clone();
    let (p, q) = (inventory(&mut log, P), inventory(&mut log, Q));
    assert_eq!(levels(&p), vec![("a".into(), 12), ("b".into(), 6)]);
    assert_eq!(p.get(&id("a")).and_then(|s| s.mark), Some(Mark::Keep));
    assert_eq!(
        p.get(&id("b")).and_then(|s| s.note.clone()),
        NoteText::new("双速")
    );

    // Destroy every derived thing: the in-memory projection goes with the log, and a projection
    // cache entry that disagrees with the log is planted. Neither is consulted on replay.
    let mut store = log.into_store();
    store
        .apply(ReplaceCache {
            seq: projection
                .revision()
                .last()
                .and_then(|s| yata_store::Seq::new(s.get()))
                .expect("a revision"),
            fold_version: 0,
            projection: b"not a projection".to_vec(),
        })
        .expect("cache");
    drop(store);

    let mut rebuilt = dir.log();
    assert_eq!(rebuilt.projection(), &projection);
    assert_eq!(rebuilt.held(P), Some(held_of(&projection, P)));
    assert_eq!(inventory(&mut rebuilt, P), p);
    assert_eq!(inventory(&mut rebuilt, Q), q);
}

fn held_of(
    p: &yata_core::fact::Projection,
    profile: ProfileId,
) -> BTreeMap<SectionKind, Completeness> {
    p.profile(profile).map(|s| s.held()).unwrap_or_default()
}

#[test]
fn the_canonical_codec_gives_one_snapshot_one_digest() {
    let s = YataSnapshot {
        souls: souls(Complete, vec![soul("a", 12)]),
        ..file("1")
    };
    let codec = CanonicalCodec;
    let bytes = codec.encode(&s);
    assert_eq!(codec.encode(&s.clone()), bytes);
    assert_eq!(codec.decode(&bytes), Ok(s));
}
