//! The daemon's side of a probe session (`probe-protocol.md`), over any byte stream: the named
//! pipe live, a recording on replay.
//!
//! [`Inbox`] holds the rules for what the reader may send, and both paths feed it the same way:
//! bytes into the one frame decoder, each payload decoded and checked (ADR-0006, rule 12). The
//! live [`Session`] writes every byte it reads to its recorder before decoding it, so a recording
//! is the stream verbatim, cut exactly where the stream was cut.

use std::io::{self, Read, Write};
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::Duration;

use prost::Message;
use yata_protocol::discipline::{Breach, Ledger};
use yata_protocol::frame::{self, FrameDecoder, FrameError};
use yata_protocol::probe::{
    self, Cancel, Failed, Handshake, HandshakeAck, Log, ProbeError, ProbeMessage, Progress,
    ProtocolVersion, ReadRequest, ReadResult, Scope, Shutdown, probe_message::Kind,
};

/// Why a session cannot go on.
#[derive(Debug, Clone, PartialEq)]
pub enum SessionError {
    /// The stream failed.
    Io(String),
    /// The framing failed, or the stream ended inside a frame.
    Frame(FrameError),
    /// A frame that is not a `ProbeMessage`, or one with no kind.
    Undecodable(String),
    /// A message only the daemon sends.
    WrongDirection(&'static str),
    /// A message before the `HandshakeAck` that needs one, or a second `HandshakeAck`.
    OutOfOrder(&'static str),
    Breach(Breach),
    /// The reader speaks a protocol major version this build does not.
    ProtocolUnsupported(ProtocolVersion),
    /// The reader answered with `Failed`: for the request, or for the session (request id 0).
    Failed(ProbeError),
    /// The stream ended cleanly with requests unanswered, or before the handshake was answered.
    Closed {
        open: Vec<u64>,
    },
    /// No frame arrived within the session's wait.
    TimedOut,
}

/// One checked message from the reader.
#[derive(Debug, Clone, PartialEq)]
pub enum Inbound {
    Ack(HandshakeAck),
    Progress(Progress),
    Result(ReadResult),
    Failed(Failed),
    Log(Log),
}

/// The rules for the reader's messages. On replay the daemon's requests are not in the stream,
/// so a request counts as issued when its id first appears.
#[derive(Debug, Default)]
pub struct Inbox {
    ledger: Ledger,
    acked: bool,
    replay: bool,
    highest: u64,
}

impl Inbox {
    pub fn live() -> Inbox {
        Inbox::default()
    }

    pub fn replay() -> Inbox {
        Inbox {
            replay: true,
            ..Inbox::default()
        }
    }

    /// A request the daemon sends.
    pub fn issue(&mut self, id: u64) -> Result<(), SessionError> {
        self.ledger.issue(id).map_err(SessionError::Breach)?;
        self.highest = self.highest.max(id);
        Ok(())
    }

    /// Requests issued and not answered.
    pub fn open(&self) -> Vec<u64> {
        self.ledger.open().collect()
    }

    pub fn acked(&self) -> bool {
        self.acked
    }

    /// Decode and check one payload.
    pub fn accept(&mut self, payload: &[u8]) -> Result<Inbound, SessionError> {
        let message =
            ProbeMessage::decode(payload).map_err(|e| SessionError::Undecodable(e.to_string()))?;
        let kind = message
            .kind
            .ok_or_else(|| SessionError::Undecodable("a message with no kind".to_owned()))?;
        match kind {
            Kind::Handshake(_) => Err(SessionError::WrongDirection("Handshake")),
            Kind::ReadRequest(_) => Err(SessionError::WrongDirection("ReadRequest")),
            Kind::Cancel(_) => Err(SessionError::WrongDirection("Cancel")),
            Kind::Shutdown(_) => Err(SessionError::WrongDirection("Shutdown")),
            Kind::Log(l) => Ok(Inbound::Log(l)),
            Kind::HandshakeAck(a) => {
                if self.acked {
                    return Err(SessionError::OutOfOrder("a second HandshakeAck"));
                }
                let version = a.version.unwrap_or_default();
                if !probe::accepts(version) {
                    return Err(SessionError::ProtocolUnsupported(version));
                }
                self.acked = true;
                Ok(Inbound::Ack(a))
            }
            Kind::Failed(f) if f.request_id == 0 => Ok(Inbound::Failed(f)),
            Kind::Progress(p) => {
                self.before(p.request_id)?;
                self.ledger
                    .progress(p.request_id)
                    .map_err(SessionError::Breach)?;
                Ok(Inbound::Progress(p))
            }
            Kind::ReadResult(r) => {
                self.before(r.request_id)?;
                self.ledger
                    .answer(r.request_id)
                    .map_err(SessionError::Breach)?;
                Ok(Inbound::Result(r))
            }
            Kind::Failed(f) => {
                self.before(f.request_id)?;
                self.ledger
                    .answer(f.request_id)
                    .map_err(SessionError::Breach)?;
                Ok(Inbound::Failed(f))
            }
        }
    }

    /// A message about a request: only after the handshake, and on replay the first sight of an
    /// id issues it.
    fn before(&mut self, id: u64) -> Result<(), SessionError> {
        if !self.acked {
            return Err(SessionError::OutOfOrder(
                "a request message before HandshakeAck",
            ));
        }
        if self.replay && id > self.highest {
            self.issue(id)?;
        }
        Ok(())
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

    pub fn results(&self) -> impl Iterator<Item = &ReadResult> {
        self.messages.iter().filter_map(|m| match m {
            Inbound::Result(r) => Some(r),
            _ => None,
        })
    }

    pub fn failures(&self) -> impl Iterator<Item = &Failed> {
        self.messages.iter().filter_map(|m| match m {
            Inbound::Failed(f) => Some(f),
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

/// A live session. A thread pumps the incoming stream into frames, so the daemon can write a
/// `Cancel` while a request is running.
pub struct Session {
    outgoing: Box<dyn Write + Send>,
    events: mpsc::Receiver<Event>,
    pump: Option<JoinHandle<()>>,
    inbox: Inbox,
    next_id: u64,
    wait: Option<Duration>,
    ended: Option<Result<(), SessionError>>,
    /// `Log` messages received, in order.
    pub logs: Vec<Log>,
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
            events,
            pump: Some(pump),
            inbox: Inbox::live(),
            next_id: 1,
            wait: None,
            ended: None,
            logs: Vec::new(),
        }
    }

    /// Give up when no frame arrives for this long.
    pub fn with_wait(mut self, wait: Duration) -> Session {
        self.wait = Some(wait);
        self
    }

    /// Open the session. `target_pid` 0 lets the reader choose the process.
    pub fn handshake(&mut self, target_pid: u32) -> Result<HandshakeAck, SessionError> {
        self.send(Kind::Handshake(Handshake {
            version: Some(probe::VERSION),
            expected_engine: String::new(),
            target_pid,
        }))?;
        loop {
            match self.next()? {
                Inbound::Ack(a) => return Ok(a),
                Inbound::Failed(f) => {
                    return Err(SessionError::Failed(f.error.unwrap_or_default()));
                }
                Inbound::Log(l) => self.logs.push(l),
                Inbound::Progress(_) | Inbound::Result(_) => {
                    return Err(SessionError::OutOfOrder(
                        "a request message before HandshakeAck",
                    ));
                }
            }
        }
    }

    /// Read one scope. `observe` sees each `Progress` and may ask for a cancel.
    pub fn read(
        &mut self,
        scope: Scope,
        mut observe: impl FnMut(&Progress) -> Step,
    ) -> Result<ReadResult, SessionError> {
        let id = self.next_id;
        self.next_id += 1;
        self.inbox.issue(id)?;
        self.send(Kind::ReadRequest(ReadRequest {
            request_id: id,
            scope: scope.into(),
        }))?;
        let mut cancelled = false;
        loop {
            match self.next()? {
                Inbound::Progress(p) => {
                    if observe(&p) == Step::Cancel && !cancelled {
                        cancelled = true;
                        self.send(Kind::Cancel(Cancel { request_id: id }))?;
                    }
                }
                Inbound::Result(r) => return Ok(r),
                Inbound::Failed(f) => {
                    return Err(SessionError::Failed(f.error.unwrap_or_default()));
                }
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
            other => other.map_err(|e| SessionError::Io(e.to_string())),
        };
        let Session {
            outgoing,
            events,
            pump,
            ended,
            ..
        } = self;
        drop(outgoing);
        let end = match ended {
            Some(end) => end,
            None => loop {
                match events.recv() {
                    Ok(Event::Frame(_)) => continue,
                    Ok(Event::End(end)) => break end,
                    Err(_) => break Ok(()),
                }
            },
        };
        if let Some(p) = pump {
            let _ = p.join();
        }
        sent.and(end)
    }

    fn send(&mut self, kind: Kind) -> Result<(), SessionError> {
        self.write(kind)
            .map_err(|e| SessionError::Io(e.to_string()))
    }

    fn write(&mut self, kind: Kind) -> io::Result<()> {
        let payload = ProbeMessage { kind: Some(kind) }.encode_to_vec();
        // Every message this session builds is far below the frame limit and never empty.
        let bytes = frame::encode(&payload).map_err(|e| io::Error::other(format!("{e:?}")))?;
        self.outgoing.write_all(&bytes)?;
        self.outgoing.flush()
    }

    fn next(&mut self) -> Result<Inbound, SessionError> {
        if let Some(end) = &self.ended {
            return Err(self.after_end(end.clone()));
        }
        let event = match self.wait {
            Some(w) => self.events.recv_timeout(w).map_err(|e| match e {
                mpsc::RecvTimeoutError::Timeout => SessionError::TimedOut,
                mpsc::RecvTimeoutError::Disconnected => SessionError::Closed {
                    open: self.inbox.open(),
                },
            })?,
            None => self.events.recv().map_err(|_| SessionError::Closed {
                open: self.inbox.open(),
            })?,
        };
        match event {
            Event::Frame(payload) => self.inbox.accept(&payload),
            Event::End(end) => {
                self.ended = Some(end.clone());
                Err(self.after_end(end))
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
            Err(e) => break Err(SessionError::Io(e.to_string())),
        };
        let bytes = &buffer[..n];
        if let Some(r) = recorder.as_mut()
            && let Err(e) = r.write_all(bytes).and_then(|()| r.flush())
        {
            break Err(SessionError::Io(format!("recorder: {e}")));
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
