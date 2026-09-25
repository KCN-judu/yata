//! The daemon's side of a probe session (`probe-protocol.md`), over any byte stream: the named
//! pipe live, a recording on replay.
//!
//! [`Inbox`] holds the rules for what the reader may send, and both paths feed it the same way:
//! bytes into the one frame decoder, each payload decoded and checked (ADR-0006, rule 12). The
//! live [`Session`] writes every byte it reads to its recorder before decoding it, so a recording
//! is the stream verbatim, cut exactly where the stream was cut.

use std::io::{self, Read, Write};
use std::num::NonZeroU32;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::Duration;

use prost::Message;
use yata_protocol::discipline::{Breach, Ledger, RequestId};
use yata_protocol::failure::{Failure, FailureError, RequestFailure, SessionFailure};
use yata_protocol::frame::{self, FrameDecoder, FrameError};
use yata_protocol::probe::{
    self, Cancel, Discover, Handshake, HandshakeAck, Log, ProbeMessage, ProtocolVersion,
    ReadRequest, Reading, Scope, Shutdown, failed::Subject, handshake, probe_message::Kind,
};

/// Which process the reader reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Let the reader choose by its discovery rules.
    Discover,
    /// The process the user chose.
    Pid(NonZeroU32),
}

/// A failure the reader reported, with what it was about.
#[derive(Debug, Clone, PartialEq)]
pub enum Failed {
    Session(SessionFailure),
    Request {
        id: RequestId,
        failure: RequestFailure,
    },
}

impl Failed {
    /// The code's stable dotted name.
    pub fn name(&self) -> &'static str {
        match self {
            Failed::Session(f) => f.name(),
            Failed::Request { failure, .. } => failure.name(),
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Failed::Session(f) => &f.message,
            Failed::Request { failure, .. } => &failure.message,
        }
    }
}

/// What made a frame unreadable as a probe message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Undecodable {
    /// Not a `ProbeMessage`.
    Protobuf(String),
    /// A message, or a oneof inside one, with no case set.
    NoKind,
    /// A `Failed` without its error.
    NoError,
    /// An error the shared failure types refuse: an unstated code, or a detail or system error
    /// the code does not have.
    Failure(FailureError),
    /// A `Failed` whose subject is the session and whose code answers one request, or the
    /// other way round.
    SubjectMismatch,
    /// A result without its reading.
    NoReading,
}

/// Why a session cannot go on.
#[derive(Debug, Clone, PartialEq)]
pub enum SessionError {
    /// The stream failed.
    Io {
        kind: io::ErrorKind,
        message: String,
    },
    /// The framing failed, or the stream ended inside a frame.
    Frame(FrameError),
    Undecodable(Undecodable),
    /// A message only the daemon sends.
    WrongDirection(&'static str),
    /// A message before the `HandshakeAck` that needs one, or a second `HandshakeAck`.
    OutOfOrder(&'static str),
    Breach(Breach),
    /// The reader speaks a protocol major version this build does not, or states none.
    ProtocolUnsupported(Option<ProtocolVersion>),
    /// The reader ended the session with `Failed`.
    SessionFailed(SessionFailure),
    /// The reader answered a request with `Failed`.
    RequestFailed {
        id: RequestId,
        failure: RequestFailure,
    },
    /// The thread reading the stream panicked.
    PumpPanicked,
    /// The stream ended cleanly with requests unanswered, or before the handshake was answered.
    Closed {
        open: Vec<RequestId>,
    },
    /// No frame arrived within the session's wait.
    TimedOut,
    /// More requests than ids.
    IdsExhausted,
}

impl SessionError {
    fn io(e: &io::Error) -> SessionError {
        SessionError::Io {
            kind: e.kind(),
            message: e.to_string(),
        }
    }
}

/// One checked message from the reader.
#[derive(Debug, Clone, PartialEq)]
pub enum Inbound {
    Ack(HandshakeAck),
    Progress {
        id: RequestId,
        done: u64,
        total: Option<u64>,
    },
    Result {
        id: RequestId,
        reading: Box<Reading>,
    },
    RequestFailed {
        id: RequestId,
        failure: RequestFailure,
    },
    SessionFailed(SessionFailure),
    Log(Log),
}

/// Where requests come from: the daemon live, or the stream itself on replay, where a request
/// counts as issued when its id first appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Live,
    Replay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    AwaitingAck,
    Acked,
}

/// The rules for the reader's messages.
#[derive(Debug)]
pub struct Inbox {
    ledger: Ledger,
    mode: Mode,
    phase: Phase,
}

impl Inbox {
    pub fn live() -> Inbox {
        Inbox {
            ledger: Ledger::new(),
            mode: Mode::Live,
            phase: Phase::AwaitingAck,
        }
    }

    pub fn replay() -> Inbox {
        Inbox {
            mode: Mode::Replay,
            ..Inbox::live()
        }
    }

    /// A request the daemon sends.
    pub fn issue(&mut self, id: RequestId) -> Result<(), SessionError> {
        self.ledger.issue(id).map_err(SessionError::Breach)
    }

    /// Requests issued and not answered.
    pub fn open(&self) -> Vec<RequestId> {
        self.ledger.open().collect()
    }

    /// Decode and check one payload.
    pub fn accept(&mut self, payload: &[u8]) -> Result<Inbound, SessionError> {
        let message = ProbeMessage::decode(payload)
            .map_err(|e| SessionError::Undecodable(Undecodable::Protobuf(e.to_string())))?;
        match message
            .kind
            .ok_or(SessionError::Undecodable(Undecodable::NoKind))?
        {
            Kind::Handshake(_) => Err(SessionError::WrongDirection("Handshake")),
            Kind::ReadRequest(_) => Err(SessionError::WrongDirection("ReadRequest")),
            Kind::Cancel(_) => Err(SessionError::WrongDirection("Cancel")),
            Kind::Shutdown(_) => Err(SessionError::WrongDirection("Shutdown")),
            Kind::Log(l) => Ok(Inbound::Log(l)),
            Kind::HandshakeAck(a) => {
                if self.phase == Phase::Acked {
                    return Err(SessionError::OutOfOrder("a second HandshakeAck"));
                }
                if !a.version.is_some_and(probe::accepts) {
                    return Err(SessionError::ProtocolUnsupported(a.version));
                }
                self.phase = Phase::Acked;
                Ok(Inbound::Ack(a))
            }
            Kind::Failed(f) => {
                let undecodable = |u| SessionError::Undecodable(u);
                let error = f.error.ok_or(undecodable(Undecodable::NoError))?;
                let failure =
                    Failure::from_wire(error).map_err(|e| undecodable(Undecodable::Failure(e)))?;
                match (f.subject, failure) {
                    (Some(Subject::Session(_)), Failure::Session(failure)) => {
                        Ok(Inbound::SessionFailed(failure))
                    }
                    (Some(Subject::RequestId(id)), Failure::Request(failure)) => {
                        let id = self.about(id)?;
                        self.ledger.answer(id).map_err(SessionError::Breach)?;
                        Ok(Inbound::RequestFailed { id, failure })
                    }
                    (Some(_), _) => Err(undecodable(Undecodable::SubjectMismatch)),
                    (None, _) => Err(undecodable(Undecodable::NoKind)),
                }
            }
            Kind::Progress(p) => {
                let id = self.about(p.request_id)?;
                self.ledger.progress(id).map_err(SessionError::Breach)?;
                Ok(Inbound::Progress {
                    id,
                    done: p.done,
                    total: p.total,
                })
            }
            Kind::ReadResult(r) => {
                let id = self.about(r.request_id)?;
                let reading = r
                    .reading
                    .ok_or(SessionError::Undecodable(Undecodable::NoReading))?;
                self.ledger.answer(id).map_err(SessionError::Breach)?;
                Ok(Inbound::Result {
                    id,
                    reading: Box::new(reading),
                })
            }
        }
    }

    /// The id of a message about a request: only after the handshake, and on replay the first
    /// sight of an id issues it.
    fn about(&mut self, id: u64) -> Result<RequestId, SessionError> {
        if self.phase == Phase::AwaitingAck {
            return Err(SessionError::OutOfOrder(
                "a request message before HandshakeAck",
            ));
        }
        let id = RequestId::new(id).map_err(SessionError::Breach)?;
        if self.mode == Mode::Replay && self.ledger.highest().is_none_or(|h| id > h) {
            self.issue(id)?;
        }
        Ok(id)
    }
}

/// A recording read back: every message, in order, checked by the live rules.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Replay {
    pub messages: Vec<Inbound>,
}

impl Replay {
    pub fn ack(&self) -> Option<&HandshakeAck> {
        self.messages.iter().find_map(|m| match m {
            Inbound::Ack(a) => Some(a),
            _ => None,
        })
    }

    pub fn readings(&self) -> impl Iterator<Item = &Reading> {
        self.messages.iter().filter_map(|m| match m {
            Inbound::Result { reading, .. } => Some(&**reading),
            _ => None,
        })
    }

    /// Every failure, with what it was about.
    pub fn failures(&self) -> impl Iterator<Item = Failed> + '_ {
        self.messages.iter().filter_map(|m| match m {
            Inbound::SessionFailed(f) => Some(Failed::Session(f.clone())),
            Inbound::RequestFailed { id, failure } => Some(Failed::Request {
                id: *id,
                failure: failure.clone(),
            }),
            _ => None,
        })
    }
}

/// Replay a recording through the live decoder and rules. A recording cut inside a frame, or
/// ending with a request unanswered, is a broken capture (`probe-protocol.md`, "Recording
/// format").
pub fn replay(recording: &[u8]) -> Result<Replay, SessionError> {
    let mut decoder = FrameDecoder::new();
    decoder.push(recording);
    let mut inbox = Inbox::replay();
    let mut messages = Vec::new();
    while let Some(payload) = decoder.next_frame().map_err(SessionError::Frame)? {
        messages.push(inbox.accept(&payload)?);
    }
    decoder.finish().map_err(SessionError::Frame)?;
    let open = inbox.open();
    if !open.is_empty() {
        return Err(SessionError::Closed { open });
    }
    Ok(Replay { messages })
}

/// What the caller wants after a `Progress`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Continue,
    /// Send `Cancel` once; the request still ends with its one answer.
    Cancel,
}

enum Event {
    Frame(Vec<u8>),
    /// The stream ended: cleanly, or with the error that ended it.
    End(Result<(), SessionError>),
}

/// The incoming side: frames still arriving, or the end the stream reached.
enum Stream {
    Open(mpsc::Receiver<Event>),
    Ended(Result<(), SessionError>),
}

/// A live session. A thread pumps the incoming stream into frames, so the daemon can write a
/// `Cancel` while a request is running.
pub struct Session {
    outgoing: Box<dyn Write + Send>,
    stream: Stream,
    pump: JoinHandle<()>,
    inbox: Inbox,
    next_id: Option<RequestId>,
    wait: Option<Duration>,
    logs: Vec<Log>,
}

impl Session {
    /// A session over a stream's two directions. Every byte read is written to `recorder` first.
    pub fn start(
        incoming: impl Read + Send + 'static,
        outgoing: impl Write + Send + 'static,
        recorder: Option<Box<dyn Write + Send>>,
    ) -> Session {
        let (tx, events) = mpsc::channel();
        let pump = std::thread::spawn(move || pump(incoming, recorder, &tx));
        Session {
            outgoing: Box::new(outgoing),
            stream: Stream::Open(events),
            pump,
            inbox: Inbox::live(),
            next_id: Some(RequestId::FIRST),
            wait: None,
            logs: Vec::new(),
        }
    }

    /// Give up when no frame arrives for this long.
    pub fn with_wait(mut self, wait: Duration) -> Session {
        self.wait = Some(wait);
        self
    }

    /// The `Log` messages received so far, in order.
    pub fn logs(&self) -> &[Log] {
        &self.logs
    }

    /// Open the session.
    pub fn handshake(&mut self, target: Target) -> Result<HandshakeAck, SessionError> {
        self.send(Kind::Handshake(Handshake {
            version: Some(probe::VERSION),
            expected_engine: None,
            target: Some(match target {
                Target::Discover => handshake::Target::Discover(Discover {}),
                Target::Pid(pid) => handshake::Target::Pid(pid.get()),
            }),
        }))?;
        loop {
            match self.next()? {
                Inbound::Ack(a) => return Ok(a),
                Inbound::SessionFailed(f) => return Err(SessionError::SessionFailed(f)),
                Inbound::Log(l) => self.logs.push(l),
                Inbound::Progress { .. }
                | Inbound::Result { .. }
                | Inbound::RequestFailed { .. } => {
                    return Err(SessionError::OutOfOrder(
                        "a request message before HandshakeAck",
                    ));
                }
            }
        }
    }

    /// Read one scope. `observe` sees each progress `(done, total)` and may ask for a cancel.
    pub fn read(
        &mut self,
        scope: Scope,
        mut observe: impl FnMut(u64, Option<u64>) -> Step,
    ) -> Result<Reading, SessionError> {
        let id = self.next_id.ok_or(SessionError::IdsExhausted)?;
        self.next_id = id.next();
        self.inbox.issue(id)?;
        self.send(Kind::ReadRequest(ReadRequest {
            request_id: id.get(),
            scope: scope.into(),
        }))?;
        let mut cancelled = false;
        loop {
            match self.next()? {
                Inbound::Progress { done, total, .. } => {
                    if observe(done, total) == Step::Cancel && !cancelled {
                        cancelled = true;
                        self.send(Kind::Cancel(Cancel {
                            request_id: id.get(),
                        }))?;
                    }
                }
                Inbound::Result { reading, .. } => return Ok(*reading),
                Inbound::RequestFailed { id, failure } => {
                    return Err(SessionError::RequestFailed { id, failure });
                }
                Inbound::SessionFailed(f) => return Err(SessionError::SessionFailed(f)),
                Inbound::Log(l) => self.logs.push(l),
                // The inbox refuses a second HandshakeAck before it gets here.
                Inbound::Ack(_) => return Err(SessionError::OutOfOrder("a second HandshakeAck")),
            }
        }
    }

    /// Ask the reader to exit, close the outgoing direction, and wait for the stream to end. A
    /// reader that has already gone is not an error.
    pub fn shutdown(mut self) -> Result<(), SessionError> {
        let sent = match self.write(Kind::Shutdown(Shutdown {})) {
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
            other => other.map_err(|e| SessionError::io(&e)),
        };
        let Session {
            outgoing,
            stream,
            pump,
            ..
        } = self;
        drop(outgoing);
        let end = match stream {
            Stream::Ended(end) => end,
            Stream::Open(events) => loop {
                match events.recv() {
                    Ok(Event::Frame(_)) => continue,
                    Ok(Event::End(end)) => break end,
                    // The pump always sends its end before it returns, so a disconnect without
                    // one is a panic; the join below says so.
                    Err(_) => break Ok(()),
                }
            },
        };
        let joined = pump.join().map_err(|_| SessionError::PumpPanicked);
        sent.and(end).and(joined)
    }

    fn send(&mut self, kind: Kind) -> Result<(), SessionError> {
        self.write(kind).map_err(|e| SessionError::io(&e))
    }

    fn write(&mut self, kind: Kind) -> io::Result<()> {
        let payload = ProbeMessage { kind: Some(kind) }.encode_to_vec();
        let bytes = frame::encode(&payload).map_err(|e| io::Error::other(format!("{e:?}")))?;
        self.outgoing.write_all(&bytes)?;
        self.outgoing.flush()
    }

    fn next(&mut self) -> Result<Inbound, SessionError> {
        let events = match &self.stream {
            Stream::Ended(end) => return Err(self.after_end(end.clone())),
            Stream::Open(events) => events,
        };
        let event = match self.wait {
            Some(w) => events.recv_timeout(w).map_err(|e| match e {
                mpsc::RecvTimeoutError::Timeout => SessionError::TimedOut,
                mpsc::RecvTimeoutError::Disconnected => SessionError::Closed {
                    open: self.inbox.open(),
                },
            })?,
            None => events.recv().map_err(|_| SessionError::Closed {
                open: self.inbox.open(),
            })?,
        };
        match event {
            Event::Frame(payload) => self.inbox.accept(&payload),
            Event::End(end) => {
                let e = self.after_end(end.clone());
                self.stream = Stream::Ended(end);
                Err(e)
            }
        }
    }

    fn after_end(&self, end: Result<(), SessionError>) -> SessionError {
        match end {
            Ok(()) => SessionError::Closed {
                open: self.inbox.open(),
            },
            Err(e) => e,
        }
    }
}

/// The pump thread: bytes to the recorder, then to the decoder, then frames to the session.
fn pump(
    mut incoming: impl Read,
    mut recorder: Option<Box<dyn Write + Send>>,
    tx: &mpsc::Sender<Event>,
) {
    let mut decoder = FrameDecoder::new();
    let mut buffer = vec![0u8; 64 * 1024];
    let end = loop {
        let n = match incoming.read(&mut buffer) {
            Ok(0) => break Ok(()),
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            // A pipe whose writer has gone reads as broken on Windows: that is the end of stream.
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => break Ok(()),
            Err(e) => break Err(SessionError::io(&e)),
        };
        let bytes = &buffer[..n];
        if let Some(r) = recorder.as_mut()
            && let Err(e) = r.write_all(bytes).and_then(|()| r.flush())
        {
            break Err(SessionError::io(&e));
        }
        decoder.push(bytes);
        loop {
            match decoder.next_frame() {
                Ok(Some(payload)) => {
                    if tx.send(Event::Frame(payload)).is_err() {
                        return;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    let _ = tx.send(Event::End(Err(SessionError::Frame(e))));
                    return;
                }
            }
        }
    };
    let end = end.and_then(|()| decoder.finish().map_err(SessionError::Frame));
    let _ = tx.send(Event::End(end));
}
