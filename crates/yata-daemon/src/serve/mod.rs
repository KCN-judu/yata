//! `yata-daemon serve`: the core protocol on a byte stream in and a byte stream out
//! (ADR-0004; `core-protocol.md`).
//!
//! The loop reads frames, hands each `ClientMessage` to the serial [`session::Session`], and
//! writes what it returns, one frame per message. The output carries frames and nothing else;
//! diagnostics go to stderr, which is the caller's to route. A frame that does not decode ends
//! the session: the client gets a `session_failed` event, and the loop returns the failure.

pub mod projection;
pub mod session;

use std::io::{Read, Write};

use prost::Message;
use yata_protocol::core as pb;
use yata_protocol::frame::{self, FrameDecoder, FrameError};

use self::session::{Next, Session};
use crate::wire::{self, Failure};

/// Why a session ended other than by `Shutdown` or the client closing its end cleanly.
#[derive(Debug)]
pub enum ServeError {
    /// The input broke the framing: the stream is unusable.
    Frame(FrameError),
    /// A frame's payload is not a `ClientMessage`.
    NotAMessage(prost::DecodeError),
    Io(std::io::Error),
}

impl From<std::io::Error> for ServeError {
    fn from(e: std::io::Error) -> ServeError {
        ServeError::Io(e)
    }
}

/// Bytes read from the input at a time.
const READ_CHUNK: usize = 64 * 1024;

/// Serve one session until the client shuts it down or closes the input.
pub fn serve(
    mut input: impl Read,
    mut output: impl Write,
    mut session: Session,
) -> Result<(), ServeError> {
    let mut decoder = FrameDecoder::new();
    let mut chunk = vec![0u8; READ_CHUNK];
    loop {
        let n = match input.read(&mut chunk) {
            Ok(0) => {
                return decoder
                    .finish()
                    .map_err(|e| fail(&mut output, ServeError::Frame(e)));
            }
            Ok(n) => n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e.into()),
        };
        decoder.push(&chunk[..n]);
        loop {
            let payload = match decoder.next_frame() {
                Ok(Some(p)) => p,
                Ok(None) => break,
                Err(e) => return Err(fail(&mut output, ServeError::Frame(e))),
            };
            let message = pb::ClientMessage::decode(payload.as_slice())
                .map_err(|e| fail(&mut output, ServeError::NotAMessage(e)))?;
            let (replies, next) = session.handle(message);
            for r in replies {
                write_message(&mut output, &r)?;
            }
            output.flush()?;
            if next == Next::Exit {
                return Ok(());
            }
        }
    }
}

fn write_message(output: &mut impl Write, m: &pb::ServerMessage) -> Result<(), ServeError> {
    // A server message is never empty (it always holds a kind) and far below the frame limit.
    let bytes = frame::encode(&m.encode_to_vec()).map_err(ServeError::Frame)?;
    output.write_all(&bytes)?;
    Ok(())
}

/// Tell the client the session failed, as far as the output still works, and pass the error on.
fn fail(output: &mut impl Write, e: ServeError) -> ServeError {
    let failure = match &e {
        ServeError::Frame(f) => Failure::new("session.malformed_frame", format!("{f:?}")),
        ServeError::NotAMessage(d) => Failure::new("session.malformed_message", d.to_string()),
        ServeError::Io(io) => Failure::new("internal.io", io.to_string()),
    };
    let event = pb::ServerMessage {
        kind: Some(pb::server_message::Kind::Event(pb::Event {
            kind: Some(pb::event::Kind::SessionFailed(wire::error(&failure))),
        })),
    };
    // Best effort: the session is ending either way, and the error returned is the one to report.
    let _ = write_message(output, &event).and_then(|()| output.flush().map_err(ServeError::Io));
    e
}
