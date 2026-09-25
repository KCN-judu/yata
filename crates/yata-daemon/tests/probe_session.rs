//! The daemon's probe session against a scripted reader over in-memory pipes, and the replay of
//! what it recorded: the live path and the replay path must agree (ADR-0006, rule 12).

use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use prost::Message;
use yata_daemon::probe::input::{Carrier, load_bytes, to_export};
use yata_daemon::probe::session::{
    Failed as FailedAbout, Inbound, Session, SessionError, Step, Target, Undecodable, replay,
};
use yata_protocol::discipline::{Breach, RequestId};
use yata_protocol::export;
use yata_protocol::failure::{FailureError, RequestCode, SessionReason};
use yata_protocol::frame::{self, FrameDecoder, FrameError};
use yata_protocol::probe::{
    self, Failed, HandshakeAck, ProbeError, ProbeErrorCode, ProbeMessage, Progress,
    ProtocolVersion, ReadResult, Reading, Scope, SessionLevel, SoulRecord, SoulRecords,
    failed::Subject, probe_message::Kind, reading::Records,
};

/// A recorder whose bytes the test can read after the session.
#[derive(Clone, Default)]
struct Tape(Arc<Mutex<Vec<u8>>>);

impl Write for Tape {
    #[allow(
        clippy::expect_used,
        reason = "test helper: a failure here is the test failing"
    )]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().expect("tape").extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Tape {
    #[allow(
        clippy::expect_used,
        reason = "test helper: a failure here is the test failing"
    )]
    fn bytes(&self) -> Vec<u8> {
        self.0.lock().expect("tape").clone()
    }
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn frame_of(kind: Kind) -> Vec<u8> {
    frame::encode(&ProbeMessage { kind: Some(kind) }.encode_to_vec()).expect("encodable")
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn id(n: u64) -> RequestId {
    RequestId::new(n).expect("nonzero")
}

/// The next message the scripted reader receives, or `None` at the end of its input.
#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn receive(input: &mut (impl Read + ?Sized), decoder: &mut FrameDecoder) -> Option<Kind> {
    let mut buffer = [0u8; 4096];
    loop {
        if let Some(p) = decoder.next_frame().expect("well framed") {
            return ProbeMessage::decode(p.as_slice()).expect("decodable").kind;
        }
        let n = input.read(&mut buffer).expect("readable");
        if n == 0 {
            return None;
        }
        decoder.push(&buffer[..n]);
    }
}

fn ack() -> Kind {
    Kind::HandshakeAck(HandshakeAck {
        version: Some(probe::VERSION),
        engine: "synthetic".into(),
        probe_build_id: "scripted".into(),
        channel: probe::Channel::DesktopMemory.into(),
        target: None,
    })
}

fn reading() -> Reading {
    Reading {
        coverage: probe::Coverage::Complete.into(),
        records: Some(Records::Souls(SoulRecords {
            souls: vec![SoulRecord::default(), SoulRecord::default()],
            ..SoulRecords::default()
        })),
        ..Reading::default()
    }
}

fn souls(request_id: u64) -> Kind {
    Kind::ReadResult(ReadResult {
        request_id,
        reading: Some(reading()),
    })
}

fn failed(subject: Subject, code: ProbeErrorCode) -> Kind {
    Kind::Failed(Failed {
        subject: Some(subject),
        error: Some(ProbeError {
            code: code.into(),
            ..ProbeError::default()
        }),
    })
}

/// Start a session against a reader that runs `script` on its end of the pipes.
#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn with_reader(
    script: impl FnOnce(&mut dyn Read, &mut dyn Write) + Send + 'static,
) -> (Session, Tape, std::thread::JoinHandle<()>) {
    let (daemon_in, mut reader_out) = std::io::pipe().expect("pipe");
    let (mut reader_in, daemon_out) = std::io::pipe().expect("pipe");
    let reader = std::thread::spawn(move || script(&mut reader_in, &mut reader_out));
    let tape = Tape::default();
    let session = Session::start(daemon_in, daemon_out, Some(Box::new(tape.clone())));
    (session, tape, reader)
}

/// A well-behaved reader: acknowledges, answers each read with two progress events and a
/// result, and exits at `Shutdown`.
#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn good_reader(input: &mut dyn Read, output: &mut dyn Write) {
    let mut d = FrameDecoder::new();
    while let Some(kind) = receive(input, &mut d) {
        match kind {
            Kind::Handshake(_) => output.write_all(&frame_of(ack())).expect("w"),
            Kind::ReadRequest(r) => {
                for done in 1..=2 {
                    output
                        .write_all(&frame_of(Kind::Progress(Progress {
                            request_id: r.request_id,
                            done,
                            total: Some(2),
                        })))
                        .expect("w");
                }
                output.write_all(&frame_of(souls(r.request_id))).expect("w");
            }
            Kind::Shutdown(_) => return,
            other => panic!("unexpected {other:?}"),
        }
    }
}

#[test]
fn a_framed_request_gets_its_progress_and_one_result() {
    let (mut s, tape, reader) = with_reader(good_reader);
    let ack = s.handshake(Target::Discover).expect("acknowledged");
    assert_eq!(ack.engine, "synthetic");
    let mut seen = Vec::new();
    let first = s
        .read(Scope::Souls, |done, total| {
            seen.push((done, total));
            Step::Continue
        })
        .expect("read");
    assert_eq!(seen, vec![(1, Some(2)), (2, Some(2))]);
    let second = s.read(Scope::Souls, |_, _| Step::Continue).expect("read");
    s.shutdown().expect("clean");
    reader.join().expect("reader");

    // The recording replays through the same rules to the same messages, ids included.
    let recorded = replay(&tape.bytes()).expect("a whole capture");
    assert_eq!(recorded.ack().map(|a| a.engine.as_str()), Some("synthetic"));
    let readings: Vec<&Reading> = recorded.readings().collect();
    assert_eq!(readings, vec![&first, &second]);
    let ids: Vec<RequestId> = recorded
        .messages
        .iter()
        .filter_map(|m| match m {
            Inbound::Result { id, .. } => Some(*id),
            _ => None,
        })
        .collect();
    assert_eq!(ids, vec![id(1), id(2)]);
    assert_eq!(
        recorded
            .messages
            .iter()
            .filter(|m| matches!(m, Inbound::Progress { .. }))
            .count(),
        4
    );
}

#[test]
fn a_cancel_is_sent_once_and_the_request_still_gets_one_answer() {
    let cancels = Arc::new(Mutex::new(0));
    let counted = cancels.clone();
    let (mut s, _tape, reader) = with_reader(move |input, output| {
        let mut d = FrameDecoder::new();
        let mut open = None;
        while let Some(kind) = receive(input, &mut d) {
            match kind {
                Kind::Handshake(_) => output.write_all(&frame_of(ack())).expect("w"),
                Kind::ReadRequest(r) => {
                    open = Some(r.request_id);
                    for done in 1..=3 {
                        output
                            .write_all(&frame_of(Kind::Progress(Progress {
                                request_id: r.request_id,
                                done,
                                total: None,
                            })))
                            .expect("w");
                    }
                }
                Kind::Cancel(c) => {
                    *counted.lock().expect("count") += 1;
                    assert_eq!(Some(c.request_id), open);
                    output
                        .write_all(&frame_of(failed(
                            Subject::RequestId(c.request_id),
                            ProbeErrorCode::Cancelled,
                        )))
                        .expect("w");
                }
                Kind::Shutdown(_) => return,
                other => panic!("unexpected {other:?}"),
            }
        }
    });
    s.handshake(Target::Discover).expect("acknowledged");
    let answer = s.read(Scope::Souls, |_, _| Step::Cancel);
    let Err(SessionError::RequestFailed {
        id: failed_id,
        failure,
    }) = answer
    else {
        panic!("expected the cancelled failure, got {answer:?}")
    };
    assert_eq!(failed_id, id(1));
    assert_eq!(failure.code, RequestCode::Cancelled);
    assert_eq!(failure.name(), "probe.cancelled");
    s.shutdown().expect("clean");
    reader.join().expect("reader");
    assert_eq!(*cancels.lock().expect("count"), 1);
}

#[test]
fn a_session_level_failure_ends_the_handshake() {
    let (mut s, tape, reader) = with_reader(|input, output| {
        let mut d = FrameDecoder::new();
        receive(input, &mut d);
        output
            .write_all(&frame_of(failed(
                Subject::Session(SessionLevel {}),
                ProbeErrorCode::ElevationRequired,
            )))
            .expect("w");
    });
    let answer = s.handshake(Target::Discover);
    let Err(SessionError::SessionFailed(e)) = answer else {
        panic!("expected a failure, got {answer:?}")
    };
    assert_eq!(e.reason, SessionReason::ElevationRequired);
    reader.join().expect("reader");
    // The reader has gone: shutting down is still clean.
    assert_eq!(s.shutdown(), Ok(()));
    // A capture of an attach failure is a whole capture: no request was open.
    let recorded = replay(&tape.bytes()).expect("whole");
    let failures: Vec<_> = recorded.failures().collect();
    assert_eq!(failures.len(), 1);
    assert!(matches!(failures[0], FailedAbout::Session(_)));
    assert!(recorded.ack().is_none());
}

#[test]
fn a_failure_without_its_error_or_code_is_undecodable() {
    let no_error = Kind::Failed(Failed {
        subject: Some(Subject::Session(SessionLevel {})),
        error: None,
    });
    assert_eq!(
        replay(&frame_of(no_error)),
        Err(SessionError::Undecodable(Undecodable::NoError))
    );
    let no_code = failed(
        Subject::Session(SessionLevel {}),
        ProbeErrorCode::Unspecified,
    );
    assert_eq!(
        replay(&frame_of(no_code)),
        Err(SessionError::Undecodable(Undecodable::Failure(
            FailureError::UnstatedCode
        )))
    );
}

#[test]
fn a_failure_whose_subject_disagrees_with_its_code_is_undecodable() {
    // A session-level subject with a request's code, and a request with a session's code: the
    // subject and the code each say what the failure ends, and they must agree.
    let session_cancelled = failed(Subject::Session(SessionLevel {}), ProbeErrorCode::Cancelled);
    assert_eq!(
        replay(&frame_of(session_cancelled)),
        Err(SessionError::Undecodable(Undecodable::SubjectMismatch))
    );
    let mut stream = frame_of(ack());
    stream.extend(frame_of(failed(
        Subject::RequestId(1),
        ProbeErrorCode::ElevationRequired,
    )));
    assert_eq!(
        replay(&stream),
        Err(SessionError::Undecodable(Undecodable::SubjectMismatch))
    );
}

#[test]
fn a_malformed_frame_ends_the_session() {
    let (mut s, _tape, reader) = with_reader(|input, output| {
        let mut d = FrameDecoder::new();
        receive(input, &mut d);
        // A zero length prefix is never a message.
        output.write_all(&[0, 0, 0, 0]).expect("w");
    });
    assert_eq!(
        s.handshake(Target::Discover),
        Err(SessionError::Frame(FrameError::Empty))
    );
    reader.join().expect("reader");
}

#[test]
fn a_frame_that_is_not_a_message_is_undecodable() {
    let (mut s, _tape, reader) = with_reader(|input, output| {
        let mut d = FrameDecoder::new();
        receive(input, &mut d);
        output
            .write_all(&frame::encode(&[0xff, 0xff, 0xff]).expect("encodable"))
            .expect("w");
    });
    assert!(matches!(
        s.handshake(Target::Discover),
        Err(SessionError::Undecodable(_))
    ));
    reader.join().expect("reader");
}

#[test]
fn a_reader_that_answers_twice_breaks_the_discipline() {
    let (mut s, _tape, reader) = with_reader(|input, output| {
        let mut d = FrameDecoder::new();
        receive(input, &mut d);
        output.write_all(&frame_of(ack())).expect("w");
        let Some(Kind::ReadRequest(r)) = receive(input, &mut d) else {
            panic!("request")
        };
        output.write_all(&frame_of(souls(r.request_id))).expect("w");
        output.write_all(&frame_of(souls(r.request_id))).expect("w");
        let _ = receive(input, &mut d);
    });
    s.handshake(Target::Discover).expect("acknowledged");
    s.read(Scope::Souls, |_, _| Step::Continue)
        .expect("first answer");
    // The duplicate is waiting in the stream; the next exchange meets it.
    let next = s.read(Scope::Souls, |_, _| Step::Continue);
    assert_eq!(
        next,
        Err(SessionError::Breach(Breach::AfterTerminal { id: id(1) }))
    );
    drop(s);
    reader.join().expect("reader");
}

#[test]
fn a_reader_of_another_major_version_is_refused() {
    let (mut s, _tape, reader) = with_reader(|input, output| {
        let mut d = FrameDecoder::new();
        receive(input, &mut d);
        output
            .write_all(&frame_of(Kind::HandshakeAck(HandshakeAck {
                version: Some(ProtocolVersion { major: 2, minor: 0 }),
                ..HandshakeAck::default()
            })))
            .expect("w");
    });
    assert_eq!(
        s.handshake(Target::Discover),
        Err(SessionError::ProtocolUnsupported(Some(ProtocolVersion {
            major: 2,
            minor: 0
        })))
    );
    reader.join().expect("reader");
}

#[test]
fn a_reader_that_dies_mid_frame_leaves_a_truncated_capture() {
    let (mut s, tape, reader) = with_reader(|input, output| {
        let mut d = FrameDecoder::new();
        receive(input, &mut d);
        output.write_all(&frame_of(ack())).expect("w");
        receive(input, &mut d);
        let whole = frame_of(souls(1));
        output.write_all(&whole[..whole.len() / 2]).expect("w");
    });
    s.handshake(Target::Discover).expect("acknowledged");
    let answer = s.read(Scope::Souls, |_, _| Step::Continue);
    assert!(matches!(
        answer,
        Err(SessionError::Frame(FrameError::Truncated { .. }))
    ));
    reader.join().expect("reader");
    assert!(matches!(
        replay(&tape.bytes()),
        Err(SessionError::Frame(FrameError::Truncated { .. }))
    ));
}

#[test]
fn a_capture_that_ends_with_a_request_open_is_broken() {
    let mut stream = frame_of(ack());
    stream.extend(frame_of(Kind::Progress(Progress {
        request_id: 1,
        done: 1,
        total: None,
    })));
    assert_eq!(
        replay(&stream),
        Err(SessionError::Closed { open: vec![id(1)] })
    );
}

#[test]
fn a_request_id_of_zero_is_a_breach() {
    let mut stream = frame_of(ack());
    stream.extend(frame_of(souls(0)));
    assert_eq!(replay(&stream), Err(SessionError::Breach(Breach::ZeroId)));
}

#[test]
fn a_reading_arrives_the_same_by_recording_and_by_export() {
    let mut stream = frame_of(ack());
    stream.extend(frame_of(souls(1)));
    let recorded = load_bytes(&stream).expect("a recording");
    assert!(matches!(recorded.carrier, Carrier::Recording { .. }));
    let json = export::to_json(&to_export(&recorded).expect("provenance")).expect("json");
    let exported = load_bytes(json.as_bytes()).expect("an export");
    assert_eq!(exported.carrier, Carrier::Export { captured_at: None });
    assert_eq!(
        exported.provenance.as_ref().map(|p| p.engine.as_str()),
        Some("synthetic")
    );
    assert_eq!(recorded.readings, exported.readings);
    // The same bytes, so the same blob and digest (ADR-0008, "Importing a file").
    assert_eq!(
        yata_daemon::probe::convert::blob_of(&recorded.readings[0]),
        yata_daemon::probe::convert::blob_of(&exported.readings[0])
    );
}

#[test]
fn a_recording_with_no_acknowledgement_has_no_provenance_to_export() {
    let stream = frame_of(failed(
        Subject::Session(SessionLevel {}),
        ProbeErrorCode::NotFound,
    ));
    let recorded = load_bytes(&stream).expect("a whole capture");
    assert_eq!(recorded.provenance, None);
    assert!(to_export(&recorded).is_err());
}
