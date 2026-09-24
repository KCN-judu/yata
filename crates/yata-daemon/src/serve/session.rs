//! One protocol session: each `ClientMessage` in, its response and any events out, in order.
//!
//! The handler is serial and holds no I/O (`core-protocol.md`, "Requests and responses"); the
//! loop in [`super`] moves frames. Every request gets exactly one response, a failure included,
//! and every failure crosses as a stable code (`wire::Failure`).

use yata_core::scheme::code::decode_code;
use yata_core::scheme::layout::{LayoutError, parse};
use yata_core::scheme::transport::decode_text;
use yata_protocol::core::{
    self as pb, client_message::Kind, response::Result as Reply, server_message,
};

use super::projection::Projection;
use crate::qr::{self, QrError};
use crate::query::{self, RequestError};
use crate::wire::{self, Failure, SchemeSource};

/// What the loop does after a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Next {
    Continue,
    /// The client asked to end the session; flush and exit.
    Exit,
}

#[derive(Debug)]
pub struct Session {
    projection: Projection,
    open: bool,
}

impl Session {
    pub fn new(projection: Projection) -> Session {
        Session {
            projection,
            open: false,
        }
    }

    /// The response to `m`, then any events it causes.
    pub fn handle(&mut self, m: pb::ClientMessage) -> (Vec<pb::ServerMessage>, Next) {
        let id = m.id;
        let mut events = Vec::new();
        let mut next = Next::Continue;
        let result = match (m.kind, self.open) {
            _ if id == 0 => Err(Failure::new(
                "session.invalid_request_id",
                "a request id is never zero",
            )),
            (None, _) => Err(Failure::new(
                "session.unknown_request",
                "the request kind is not one this daemon knows",
            )),
            (Some(Kind::OpenSession(_)), true) => Err(Failure::new(
                "session.already_open",
                "the session is already open",
            )),
            (Some(Kind::OpenSession(_)), false) => {
                self.open_session(m.protocol_version, &mut events)
            }
            (Some(_), false) => Err(Failure::new(
                "session.not_open",
                "the first request of a session is OpenSession",
            )),
            (Some(Kind::Shutdown(_)), true) => {
                next = Next::Exit;
                Ok(Reply::ShutdownAccepted(pb::ShutdownAccepted {}))
            }
            (Some(Kind::Subscribe(s)), true) => Ok(self.subscribe(s.revision, &mut events)),
            (Some(Kind::ListProfiles(_)), true) => Ok(self.list_profiles()),
            (Some(Kind::Query(q)), true) => self.query(q),
            (Some(Kind::DecodeSchemeCode(d)), true) => decode_scheme(d),
        };
        let result = result.unwrap_or_else(|f| Reply::Error(wire::error(&f)));
        let response = pb::ServerMessage {
            kind: Some(server_message::Kind::Response(pb::Response {
                id,
                result: Some(result),
            })),
        };
        let mut out = vec![response];
        out.extend(events.into_iter().map(|e| pb::ServerMessage {
            kind: Some(server_message::Kind::Event(pb::Event { kind: Some(e) })),
        }));
        (out, next)
    }

    /// `core-protocol.md`, "Version": same major opens; a lower major opens with a warning; a
    /// higher major, or no version at all, is refused and the session stays closed.
    fn open_session(
        &mut self,
        client: Option<pb::ProtocolVersion>,
        events: &mut Vec<pb::event::Kind>,
    ) -> Result<Reply, Failure> {
        let ours = pb::VERSION;
        let Some(client) = client else {
            return Err(Failure::new(
                "session.protocol_unsupported",
                "the client sent no protocol version",
            ));
        };
        if client.major > ours.major {
            return Err(Failure::new(
                "session.protocol_unsupported",
                format!(
                    "client protocol {}.{} is newer than this daemon's {}.{}",
                    client.major, client.minor, ours.major, ours.minor
                ),
            ));
        }
        if client.major < ours.major {
            events.push(pb::event::Kind::Warning(pb::Warning {
                code: "session.client_outdated".to_owned(),
                message: format!(
                    "client protocol {} is older than this daemon's; the oldest supported major is {}",
                    client.major, ours.major
                ),
            }));
        }
        self.open = true;
        Ok(Reply::SessionOpened(pb::SessionOpened {
            daemon_version: Some(ours),
            revision: self.projection.revision,
        }))
    }

    /// A subscription answers with the current revision, and an event at once if the client is
    /// behind it; nothing is emitted for a revision the client already holds.
    fn subscribe(&self, held: u64, events: &mut Vec<pb::event::Kind>) -> Reply {
        let current = self.projection.revision;
        if current > held {
            events.push(pb::event::Kind::ProjectionChanged(pb::ProjectionChanged {
                revision: current,
            }));
        }
        Reply::Subscribed(pb::Subscribed { revision: current })
    }

    fn list_profiles(&self) -> Reply {
        Reply::ProfileList(pb::ProfileList {
            revision: self.projection.revision,
            profiles: self
                .projection
                .profiles
                .iter()
                .map(|p| pb::Profile {
                    id: p.id.clone(),
                    name: p.name.clone(),
                })
                .collect(),
        })
    }

    /// A page of one profile's souls through the query engine every query goes through, with
    /// each row's values and the page's revision added.
    fn query(&self, q: pb::Query) -> Result<Reply, Failure> {
        let revision = self.projection.revision;
        if let Some(scan) = q.scan_revision
            && scan != revision
        {
            return Err(Failure::new(
                "query.stale_revision",
                format!("the scan began at revision {scan}; the projection is at {revision}"),
            ));
        }
        let profile = self.projection.profile(&q.profile_id).ok_or_else(|| {
            Failure::new(
                "query.unknown_profile",
                format!("no profile '{}'", q.profile_id),
            )
        })?;
        let refused = |e: RequestError| Failure::new(e.code(), format!("{e:?}"));
        let prepared = query::prepare(q).map_err(refused)?;
        let mut page = query::run(&prepared, &profile.souls).map_err(refused)?;
        for row in &mut page.rows {
            row.soul = profile
                .souls
                .get(&row.soul_id)
                .map(|s| wire::soul(&row.soul_id, s));
        }
        page.revision = revision;
        Ok(Reply::QueryPage(page))
    }
}

/// `scheme-code.md`, "Codec": an image or the text, then the transport, the layout, and the
/// selections. The text is presented back with its QR matrix; the application never encodes.
fn decode_scheme(d: pb::DecodeSchemeCode) -> Result<Reply, Failure> {
    let text = match wire::scheme_source(d)? {
        SchemeSource::Text(t) => t,
        SchemeSource::Png(bytes) => qr::decode_png(&bytes).map_err(qr_failure)?,
    };
    let payload = decode_text(&text).map_err(|e| {
        Failure::new(
            "decode.malformed_text",
            format!("the scheme text does not decode: {e:?}"),
        )
    })?;
    let layout = parse(&payload).map_err(|e| match e {
        LayoutError::UnknownFormat | LayoutError::TooShort { .. } => {
            Failure::new("decode.unknown_format", format!("not a scheme code: {e:?}"))
        }
        e => Failure::new(
            "decode.malformed_layout",
            format!("the scheme layout is malformed: {e:?}"),
        ),
    })?;
    let code = decode_code(&layout).map_err(|e| {
        Failure::new(
            "decode.malformed_scheme",
            format!("a scheme record does not read: {e:?}"),
        )
    })?;
    let matrix = qr::encode(&text).ok();
    Ok(Reply::SchemeCodeDecoded(wire::scheme_code(
        &code,
        &text,
        matrix.as_ref(),
    )))
}

fn qr_failure(e: QrError) -> Failure {
    let code = match e {
        QrError::NoCode => "decode.no_qr_code",
        QrError::SeveralCodes { .. } => "decode.several_qr_codes",
        QrError::Unreadable { .. } | QrError::NotText => "decode.qr_unreadable",
        QrError::ImageTooLarge { .. }
        | QrError::FileTooLarge { .. }
        | QrError::NotAnImage { .. } => "decode.image_invalid",
        QrError::TooLong { .. } => "internal.qr_too_long",
    };
    Failure::new(code, format!("{e:?}"))
}
