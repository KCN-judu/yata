//! The core protocol session end to end, through the real loop: frames in, frames out.
//!
//! Each test drives `serve` over in-memory streams, so what is asserted is exactly what the
//! application reads from the daemon's stdout. The last test records one whole session as shared
//! fixtures, which the Dart client decodes and re-encodes byte for byte (`app/test/daemon/`).

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    reason = "a failure here is the test failing"
)]

use std::collections::BTreeSet;
use std::path::PathBuf;

use prost::Message;
use yata_core::scheme::code::{SchemeCode, StrengtheningPlan, StrengtheningSchemeSet, encode_code};
use yata_core::scheme::layout::{AccountSegment, serialize};
use yata_core::scheme::selection::{SetChoice, SoulSelection, SubAttributeMode};
use yata_core::scheme::transport::encode_text;
use yata_core::soul::{SoulAttribute, SoulSet, SoulSlot};
use yata_daemon::qr;
use yata_daemon::serve::projection::{Projection, fixture};
use yata_daemon::serve::session::Session;
use yata_daemon::serve::{ServeError, serve};
use yata_protocol::core::{
    self as pb, client_message::Kind, event, response::Result as Reply, server_message,
};
use yata_protocol::frame::{decode_all, encode};

fn request(id: u64, kind: Kind) -> pb::ClientMessage {
    pb::ClientMessage {
        protocol_version: Some(pb::VERSION),
        id,
        kind: Some(kind),
    }
}

fn open(id: u64) -> pb::ClientMessage {
    request(id, Kind::OpenSession(pb::OpenSession {}))
}

fn frames(messages: &[pb::ClientMessage]) -> Vec<u8> {
    messages
        .iter()
        .flat_map(|m| encode(&m.encode_to_vec()).expect("encodable"))
        .collect()
}

/// Run a session over `input` and decode everything it wrote.
fn run(
    projection: Projection,
    input: &[u8],
) -> (Result<(), ServeError>, Vec<pb::ServerMessage>, Vec<u8>) {
    let mut out = Vec::new();
    let result = serve(input, &mut out, Session::new(projection));
    let messages = decode_all(&out)
        .expect("the daemon writes whole frames")
        .iter()
        .map(|p| pb::ServerMessage::decode(p.as_slice()).expect("a server message"))
        .collect();
    (result, messages, out)
}

fn reply(m: &pb::ServerMessage) -> (u64, &Reply) {
    match &m.kind {
        Some(server_message::Kind::Response(r)) => (r.id, r.result.as_ref().expect("a result")),
        other => panic!("expected a response, got {other:?}"),
    }
}

fn error_code(m: &pb::ServerMessage) -> &str {
    match reply(m).1 {
        Reply::Error(e) => &e.code,
        other => panic!("expected an error, got {other:?}"),
    }
}

fn souls_query(
    id: u64,
    profile: &str,
    budget: u32,
    cursor: Option<Vec<u8>>,
    scan: Option<u64>,
) -> pb::ClientMessage {
    request(
        id,
        Kind::Query(pb::Query {
            profile_id: profile.to_owned(),
            collection: pb::Collection::Souls.into(),
            page: Some(pb::PageRequest {
                row_budget: (budget != 0).then_some(budget),
                cursor,
            }),
            scan_revision: scan,
            ..pb::Query::default()
        }),
    )
}

#[test]
fn a_session_opens_answers_every_request_once_and_shuts_down() {
    let input = frames(&[
        open(1),
        request(2, Kind::ListProfiles(pb::ListProfiles {})),
        request(3, Kind::Shutdown(pb::Shutdown {})),
        request(4, Kind::ListProfiles(pb::ListProfiles {})),
    ]);
    let (result, out, _) = run(fixture::projection(), &input);
    assert!(result.is_ok());
    // Nothing is read after Shutdown: three requests, three responses.
    assert_eq!(out.len(), 3);
    let (id, opened) = reply(&out[0]);
    assert_eq!(id, 1);
    let Reply::SessionOpened(opened) = opened else {
        panic!("{opened:?}")
    };
    assert_eq!(opened.daemon_version, Some(pb::VERSION));
    assert_eq!(opened.revision, 1);
    let Reply::ProfileList(list) = reply(&out[1]).1 else {
        panic!()
    };
    let ids: Vec<&str> = list.profiles.iter().map(|p| p.id.as_str()).collect();
    assert_eq!(ids, [fixture::PROFILE_ID, fixture::EMPTY_PROFILE_ID]);
    assert!(matches!(reply(&out[2]), (3, Reply::ShutdownAccepted(_))));
}

#[test]
fn a_request_before_open_session_is_refused_by_code() {
    let (_, out, _) = run(
        Projection::empty(),
        &frames(&[request(1, Kind::ListProfiles(pb::ListProfiles {}))]),
    );
    assert_eq!(error_code(&out[0]), "session.not_open");
}

#[test]
fn a_newer_major_is_refused_and_the_session_stays_closed() {
    let mut newer = open(1);
    newer.protocol_version = Some(pb::ProtocolVersion {
        major: pb::VERSION.major + 1,
        minor: 0,
    });
    let input = frames(&[newer, request(2, Kind::ListProfiles(pb::ListProfiles {}))]);
    let (_, out, _) = run(Projection::empty(), &input);
    assert_eq!(error_code(&out[0]), "session.protocol_unsupported");
    assert_eq!(error_code(&out[1]), "session.not_open");
}

#[test]
fn an_older_major_opens_with_a_warning_event() {
    let mut older = open(1);
    older.protocol_version = Some(pb::ProtocolVersion { major: 0, minor: 9 });
    let (_, out, _) = run(Projection::empty(), &frames(&[older]));
    assert!(matches!(reply(&out[0]).1, Reply::SessionOpened(_)));
    let Some(server_message::Kind::Event(pb::Event {
        kind: Some(event::Kind::Warning(w)),
    })) = &out[1].kind
    else {
        panic!("{:?}", out[1])
    };
    assert_eq!(w.code, "session.client_outdated");
}

#[test]
fn a_scan_pages_by_cursor_to_the_end() {
    let (_, out, _) = run(
        fixture::projection(),
        &frames(&[open(1), souls_query(2, fixture::PROFILE_ID, 5, None, None)]),
    );
    let Reply::QueryPage(first) = reply(&out[1]).1 else {
        panic!()
    };
    assert_eq!(
        (first.total, first.next_cursor.is_some(), first.revision),
        (12, true, 1)
    );
    let mut seen = Vec::new();
    let mut cursor = first.next_cursor.clone();
    // Every row carries its values under its own id.
    let rows = |r: &pb::QueryPage| {
        r.rows
            .iter()
            .map(|row| {
                assert_eq!(
                    row.soul.as_ref().map(|s| s.soul_id.as_str()),
                    Some(row.soul_id.as_str())
                );
                row.soul_id.clone()
            })
            .collect::<Vec<_>>()
    };
    seen.extend(rows(first));
    let mut id = 3;
    loop {
        let (_, out, _) = run(
            fixture::projection(),
            &frames(&[
                open(1),
                souls_query(id, fixture::PROFILE_ID, 5, cursor.clone(), Some(1)),
            ]),
        );
        let Reply::QueryPage(page) = reply(&out[1]).1 else {
            panic!("{:?}", out[1])
        };
        seen.extend(rows(page));
        id += 1;
        match &page.next_cursor {
            None => break,
            Some(next) => cursor = Some(next.clone()),
        }
    }
    let unique: BTreeSet<&String> = seen.iter().collect();
    assert_eq!((seen.len(), unique.len()), (12, 12));
    assert!(seen.windows(2).all(|w| w[0] < w[1]));
}

#[test]
fn query_failures_carry_their_codes() {
    let input = frames(&[
        open(1),
        souls_query(2, "nobody", 5, None, None),
        souls_query(3, fixture::PROFILE_ID, 5, None, Some(0)),
        souls_query(4, fixture::PROFILE_ID, 5, Some(vec![7, 7]), None),
        request(
            5,
            Kind::Query(pb::Query {
                profile_id: fixture::PROFILE_ID.to_owned(),
                ..pb::Query::default()
            }),
        ),
    ]);
    let (_, out, _) = run(fixture::projection(), &input);
    let codes: Vec<&str> = out[1..].iter().map(error_code).collect();
    assert_eq!(
        codes,
        [
            "query.unknown_profile",
            "query.stale_revision",
            "query.malformed_cursor",
            "query.malformed"
        ]
    );
}

#[test]
fn the_empty_projection_has_no_profile() {
    let input = frames(&[open(1), request(2, Kind::ListProfiles(pb::ListProfiles {}))]);
    let (_, out, _) = run(Projection::empty(), &input);
    let Reply::ProfileList(list) = reply(&out[1]).1 else {
        panic!()
    };
    assert!(list.profiles.is_empty());
    assert_eq!(list.revision, 0);
}

#[test]
fn subscribing_behind_the_projection_emits_its_revision_once() {
    let input = frames(&[
        open(1),
        request(2, Kind::Subscribe(pb::Subscribe { revision: 0 })),
        request(3, Kind::Subscribe(pb::Subscribe { revision: 1 })),
    ]);
    let (_, out, _) = run(fixture::projection(), &input);
    assert!(matches!(
        reply(&out[1]).1,
        Reply::Subscribed(pb::Subscribed { revision: 1 })
    ));
    assert!(matches!(
        &out[2].kind,
        Some(server_message::Kind::Event(pb::Event {
            kind: Some(event::Kind::ProjectionChanged(pb::ProjectionChanged {
                revision: 1
            }))
        }))
    ));
    // A client that holds the current revision gets no event.
    assert_eq!(out.len(), 4);
}

/// A strengthening set of two plans, built through the codec, and its scheme text.
fn sample_scheme_text() -> String {
    let mut spd = SoulSelection::new(SetChoice::Sets(BTreeSet::from([SoulSet::from_suit_code(
        30,
    )])));
    spd.slots.insert(SoulSlot::Slot2);
    spd.main_attributes.insert(SoulAttribute::Spd);
    spd.sub_attributes
        .set(SoulAttribute::Crit, SubAttributeMode::Include);
    spd.sub_attributes
        .set(SoulAttribute::HpFlat, SubAttributeMode::Exclude);
    let mut all = SoulSelection::new(SetChoice::AnySet);
    all.stars.insert(6);
    let code = SchemeCode::Strengthening(StrengtheningSchemeSet {
        plans: vec![
            StrengtheningPlan::new("spd", spd),
            StrengtheningPlan::new("six", all),
        ],
    });
    let layout = encode_code(&code, AccountSegment::from_bytes([0; 14])).expect("encodable");
    let payload = serialize(&layout).expect("serializable");
    encode_text(&payload).expect("encodable").into_string()
}

fn decode(id: u64, source: pb::decode_scheme_code::Source) -> pb::ClientMessage {
    request(
        id,
        Kind::DecodeSchemeCode(pb::DecodeSchemeCode {
            source: Some(source),
        }),
    )
}

#[test]
fn a_scheme_code_decodes_from_text_and_from_its_qr_image() {
    let text = sample_scheme_text();
    let matrix = qr::encode(&text).expect("fits a QR code");
    let png = qr::render_png(&matrix, 4).expect("renders");
    let input = frames(&[
        open(1),
        decode(2, pb::decode_scheme_code::Source::Text(text.clone())),
        decode(3, pb::decode_scheme_code::Source::Png(png)),
    ]);
    let (_, out, _) = run(Projection::empty(), &input);
    for m in &out[1..] {
        let Reply::SchemeCodeDecoded(d) = reply(m).1 else {
            panic!("{m:?}")
        };
        assert_eq!(d.kind(), pb::SchemeKind::Strengthening);
        let names: Vec<&str> = d.entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["spd", "six"]);
        let spd = d.entries[0].selection.as_ref().expect("selection");
        assert_eq!(
            spd.sets,
            Some(pb::soul_selection::Sets::Chosen(pb::SuitCodes {
                codes: vec![30]
            }))
        );
        let modes: Vec<i32> = spd.sub_attributes.iter().map(|c| c.mode).collect();
        assert_eq!(
            modes,
            [
                pb::SubAttributeMode::Include as i32,
                pb::SubAttributeMode::Exclude as i32
            ]
        );
        assert_eq!(
            d.entries[1].selection.as_ref().expect("selection").sets,
            Some(pb::soul_selection::Sets::All(pb::AnySet {}))
        );
        let encoded = d.encoded.as_ref().expect("encoded");
        assert_eq!(encoded.text, text);
        let qr = encoded.qr.as_ref().expect("a QR matrix");
        assert_eq!(qr.modules.len(), (qr.size * qr.size) as usize);
    }
}

#[test]
fn scheme_decode_failures_carry_their_codes() {
    let input = frames(&[
        open(1),
        request(
            2,
            Kind::DecodeSchemeCode(pb::DecodeSchemeCode { source: None }),
        ),
        decode(
            3,
            pb::decode_scheme_code::Source::Text("not base64!".to_owned()),
        ),
        decode(
            4,
            pb::decode_scheme_code::Source::Png(b"not a png".to_vec()),
        ),
    ]);
    let (_, out, _) = run(Projection::empty(), &input);
    let codes: Vec<&str> = out[1..].iter().map(error_code).collect();
    assert_eq!(
        codes,
        [
            "decode.no_input",
            "decode.malformed_text",
            "decode.image_invalid"
        ]
    );
}

#[test]
fn a_broken_frame_ends_the_session_with_an_event() {
    let mut input = frames(&[open(1)]);
    input.extend_from_slice(&[0xff, 0xff, 0xff, 0xff]);
    let (result, out, _) = run(Projection::empty(), &input);
    assert!(matches!(result, Err(ServeError::Frame(_))));
    let Some(server_message::Kind::Event(pb::Event {
        kind: Some(event::Kind::SessionFailed(e)),
    })) = &out[1].kind
    else {
        panic!("{:?}", out[1])
    };
    assert_eq!(e.code, "session.malformed_frame");
}

#[test]
fn a_payload_that_is_not_a_message_ends_the_session() {
    let mut input = frames(&[open(1)]);
    input.extend(encode(&[0xff, 0xff]).expect("encodable"));
    let (result, _, _) = run(Projection::empty(), &input);
    assert!(matches!(result, Err(ServeError::NotAMessage(_))));
}

// ---------------------------------------------------------------- shared fixtures

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../yata-protocol/fixtures/core")
}

/// Compare `bytes` with the committed fixture, or rewrite it when `YATA_BLESS` is set.
fn golden(name: &str, bytes: &[u8]) {
    let path = fixtures().join(name);
    if std::env::var_os("YATA_BLESS").is_some() {
        std::fs::create_dir_all(fixtures()).expect("fixture folder");
        std::fs::write(&path, bytes).expect("fixture written");
        return;
    }
    let committed = std::fs::read(&path).unwrap_or_else(|e| {
        panic!(
            "{}: {e}; run with YATA_BLESS=1 to record it",
            path.display()
        )
    });
    assert!(
        committed == bytes,
        "{} differs from what the daemon writes; rerun with YATA_BLESS=1 if the change is intended",
        path.display()
    );
}

/// The cursor after the first eight fixture souls, as the daemon writes it.
fn first_page_cursor() -> Vec<u8> {
    let input = frames(&[open(1), souls_query(2, fixture::PROFILE_ID, 8, None, None)]);
    let (_, out, _) = run(fixture::projection(), &input);
    let Reply::QueryPage(page) = reply(&out[1]).1 else {
        panic!("{:?}", out[1])
    };
    page.next_cursor.clone().expect("more than one page")
}

/// The recorded session: every request kind the application sends, and the daemon's frames in
/// reply. `session.in` is what the Dart client must encode byte for byte; `session.out` is what
/// it must decode.
#[test]
fn the_recorded_session_matches_the_shared_fixtures() {
    let requests = [
        open(1),
        request(2, Kind::Subscribe(pb::Subscribe { revision: 0 })),
        request(3, Kind::ListProfiles(pb::ListProfiles {})),
        souls_query(4, fixture::PROFILE_ID, 8, None, None),
        souls_query(
            5,
            fixture::PROFILE_ID,
            8,
            Some(first_page_cursor()),
            Some(1),
        ),
        souls_query(6, "nobody", 0, None, None),
        decode(
            7,
            pb::decode_scheme_code::Source::Text(sample_scheme_text()),
        ),
        request(8, Kind::Shutdown(pb::Shutdown {})),
    ];
    let input = frames(&requests);
    let (result, out, bytes) = run(fixture::projection(), &input);
    assert!(result.is_ok());
    assert_eq!(
        out.len(),
        requests.len() + 1,
        "one response each, and one ProjectionChanged"
    );
    // The recording reads back through the frame decoder as exactly the messages sent.
    let decoded: Vec<pb::ClientMessage> = decode_all(&input)
        .expect("frames")
        .iter()
        .map(|p| pb::ClientMessage::decode(p.as_slice()).expect("a client message"))
        .collect();
    assert_eq!(decoded, requests);
    golden("session.in", &input);
    golden("session.out", &bytes);
}
