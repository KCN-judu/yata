//! One protocol session: each `ClientMessage` in, its response and any events out, in order.
//!
//! The handler is serial and holds no I/O (`core-protocol.md`, "Requests and responses"); the
//! loop in [`super`] moves frames. Every request gets exactly one response, a failure included,
//! and every failure crosses as one typed case of `Error.kind` (`wire::Failure`).

use yata_core::scheme::code::{CodeError, decode_code};
use yata_core::scheme::layout::{LayoutError, parse};
use yata_core::scheme::transport::decode_text;
use yata_protocol::core::{
    self as pb, client_message::Kind, error::Kind as Code, response::Result as Reply,
    server_message,
};

use super::projection::Projection;
use crate::qr::{self, QrError};
use crate::query::{self, Paging, RequestError};
use crate::wire::{self, Failure, SchemeSource};

/// What the loop does after a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Next {
    Continue,
    /// The client asked to end the session; flush and exit.
    Exit,
}

/// Where a session is: before `OpenSession`, or open with the version the client declared.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase {
    AwaitingOpen,
    Open { client: pb::ProtocolVersion },
}

#[derive(Debug)]
pub struct Session {
    projection: Projection,
    phase: Phase,
}

impl Session {
    pub fn new(projection: Projection) -> Session {
        Session {
            projection,
            phase: Phase::AwaitingOpen,
        }
    }

    /// The protocol version the client declared, once the session is open.
    pub fn client_version(&self) -> Option<pb::ProtocolVersion> {
        match self.phase {
            Phase::AwaitingOpen => None,
            Phase::Open { client } => Some(client),
        }
    }

    /// The response to `m`, then any events it causes.
    pub fn handle(&mut self, m: pb::ClientMessage) -> (Vec<pb::ServerMessage>, Next) {
        let id = m.id;
        let mut events = Vec::new();
        let mut next = Next::Continue;
        let result = match (m.kind, self.phase) {
            _ if id == 0 => Err(Failure::new(
                Code::SessionInvalidRequestId(pb::SessionInvalidRequestId { id }),
                "a request id is never zero",
            )),
            (None, _) => Err(Failure::new(
                Code::SessionUnknownRequest(pb::SessionUnknownRequest {}),
                "the request kind is not one this daemon knows",
            )),
            (Some(Kind::OpenSession(_)), Phase::Open { .. }) => Err(Failure::new(
                Code::SessionAlreadyOpen(pb::SessionAlreadyOpen {}),
                "the session is already open",
            )),
            (Some(Kind::OpenSession(o)), Phase::AwaitingOpen) => {
                self.open_session(o.client_version, &mut events)
            }
            (Some(k), Phase::AwaitingOpen) => Err(Failure::new(
                Code::SessionNotOpen(pb::SessionNotOpen {
                    request: request_name(&k).to_owned(),
                }),
                "the first request of a session is OpenSession",
            )),
            (Some(Kind::Shutdown(_)), Phase::Open { .. }) => {
                next = Next::Exit;
                Ok(Reply::ShutdownAccepted(pb::ShutdownAccepted {}))
            }
            (Some(Kind::Subscribe(s)), Phase::Open { .. }) => {
                Ok(self.subscribe(s.revision, &mut events))
            }
            (Some(Kind::ListProfiles(_)), Phase::Open { .. }) => Ok(self.list_profiles()),
            (Some(Kind::SessionQuery(q)), Phase::Open { .. }) => self.query(q),
            (Some(Kind::DecodeSchemeCode(d)), Phase::Open { .. }) => decode_scheme(d),
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
        let unsupported = |why: String| {
            Failure::new(
                Code::SessionProtocolUnsupported(pb::SessionProtocolUnsupported {
                    client,
                    daemon: Some(ours),
                }),
                why,
            )
        };
        let Some(client) = client else {
            return Err(unsupported(
                "the client sent no protocol version".to_owned(),
            ));
        };
        if client.major > ours.major {
            return Err(unsupported(format!(
                "client protocol {}.{} is newer than this daemon's {}.{}",
                client.major, client.minor, ours.major, ours.minor
            )));
        }
        if client.major < ours.major {
            events.push(pb::event::Kind::Warning(pb::Warning {
                message: format!(
                    "client protocol {} is older than this daemon's; the oldest supported major is {}",
                    client.major, ours.major
                ),
                kind: Some(pb::warning::Kind::ClientOutdated(pb::ClientOutdated {
                    client: Some(client),
                    daemon: Some(ours),
                })),
            }));
        }
        self.phase = Phase::Open { client };
        Ok(Reply::SessionOpened(pb::SessionOpened {
            daemon_version: Some(ours),
            revision: wire::revision(self.projection.revision),
        }))
    }

    /// A subscription answers with the current revision, and an event at once if the client is
    /// behind it; nothing is emitted for a revision the client already holds.
    fn subscribe(&self, held: u64, events: &mut Vec<pb::event::Kind>) -> Reply {
        let current = self.projection.revision;
        if current > wire::revision_of(held) {
            events.push(pb::event::Kind::ProjectionChanged(pb::ProjectionChanged {
                revision: wire::revision(current),
            }));
        }
        Reply::Subscribed(pb::Subscribed {
            revision: wire::revision(current),
        })
    }

    fn list_profiles(&self) -> Reply {
        Reply::ProfileList(pb::ProfileList {
            revision: wire::revision(self.projection.revision),
            profiles: self
                .projection
                .profiles
                .iter()
                .map(|(&id, p)| pb::Profile {
                    id: wire::profile_id(id),
                    name: p.name.clone(),
                    capabilities: wire::capabilities(&p.held),
                })
                .collect(),
        })
    }

    /// A page of one profile's souls, through the query engine every query goes through. A later
    /// page names the revision its scan began at, and is refused if the projection moved.
    fn query(&self, q: pb::SessionQuery) -> Result<Reply, Failure> {
        let current = self.projection.revision;
        let profile = wire::profile_id_of(&q.profile_id)
            .and_then(|id| self.projection.profiles.get(&id))
            .ok_or_else(|| {
                Failure::new(
                    Code::QueryUnknownProfile(pb::QueryUnknownProfile {
                        profile_id: q.profile_id.clone(),
                    }),
                    format!("no profile '{}'", q.profile_id),
                )
            })?;
        let absent = |what: &str| {
            let problem = format!("SessionQuery.{what} is absent");
            Failure::new(
                Code::QueryMalformed(pb::QueryMalformed {
                    problem: problem.clone(),
                }),
                problem,
            )
        };
        let query = q.query.ok_or_else(|| absent("query"))?;
        let cursor = match q.position.ok_or_else(|| absent("position"))? {
            pb::session_query::Position::First(_) => None,
            pb::session_query::Position::Next(n) => {
                if wire::revision_of(n.scan) != current {
                    return Err(Failure::new(
                        Code::QueryStaleRevision(pb::QueryStaleRevision {
                            scan: n.scan,
                            current: wire::revision(current),
                        }),
                        format!(
                            "the scan began at revision {}; the projection is at {}",
                            n.scan,
                            wire::revision(current)
                        ),
                    ));
                }
                Some(n.cursor)
            }
        };
        let refused = |e: RequestError| Failure::new(e.kind(), format!("{e:?}"));
        let prepared = query::prepare(
            query,
            Paging {
                row_budget: q.row_budget,
                cursor: cursor.as_deref(),
            },
        )
        .map_err(refused)?;
        let page = query::run(&prepared, &profile.souls).map_err(refused)?;
        query::render_session(page, current, &profile.souls).map(Reply::SessionQueryPage)
    }
}

/// A request kind's field name, for the debug record of `session.not_open`.
fn request_name(k: &Kind) -> &'static str {
    match k {
        Kind::OpenSession(_) => "open_session",
        Kind::Shutdown(_) => "shutdown",
        Kind::Subscribe(_) => "subscribe",
        Kind::ListProfiles(_) => "list_profiles",
        Kind::SessionQuery(_) => "session_query",
        Kind::DecodeSchemeCode(_) => "decode_scheme_code",
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
            Code::DecodeMalformedText(pb::DecodeMalformedText {
                problem: format!("{e:?}"),
            }),
            "the scheme text does not decode",
        )
    })?;
    let layout = parse(&payload).map_err(|e| match e {
        LayoutError::UnknownFormat | LayoutError::TooShort { .. } => Failure::new(
            Code::DecodeUnknownFormat(pb::DecodeUnknownFormat {
                payload_bytes: payload.len() as u64,
            }),
            format!("not a scheme code: {e:?}"),
        ),
        e => Failure::new(
            Code::DecodeMalformedLayout(pb::DecodeMalformedLayout {
                problem: format!("{e:?}"),
            }),
            "the scheme layout is malformed",
        ),
    })?;
    let code = decode_code(&layout).map_err(|e| {
        // Every refusal names the record at fault.
        let record = match &e {
            CodeError::NameNotUtf8 { record }
            | CodeError::NameTooLong { record, .. }
            | CodeError::Selection { record, .. }
            | CodeError::DiscardCannotSelectAll { record }
            | CodeError::Layout { record, .. } => Some(*record as u32),
        };
        Failure::new(
            Code::DecodeMalformedScheme(pb::DecodeMalformedScheme {
                record,
                problem: format!("{e:?}"),
            }),
            "a scheme record does not read",
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
    let problem = format!("{e:?}");
    let kind = match e {
        QrError::NoCode => Code::DecodeNoQrCode(pb::DecodeNoQrCode {}),
        QrError::SeveralCodes { count } => Code::DecodeSeveralQrCodes(pb::DecodeSeveralQrCodes {
            count: count as u32,
        }),
        QrError::Unreadable { .. } | QrError::NotText => {
            Code::DecodeQrUnreadable(pb::DecodeQrUnreadable {
                problem: problem.clone(),
            })
        }
        QrError::ImageTooLarge { .. }
        | QrError::EmptyImage { .. }
        | QrError::FileTooLarge { .. }
        | QrError::NotAnImage { .. } => Code::DecodeImageInvalid(pb::DecodeImageInvalid {
            problem: problem.clone(),
        }),
        QrError::TooLong {
            bits,
            capacity_bits,
        } => Code::InternalQrTooLong(pb::InternalQrTooLong {
            bits: bits as u64,
            capacity_bits: capacity_bits as u64,
        }),
        QrError::SegmentTooLong { bytes } => Code::InternalQrTooLong(pb::InternalQrTooLong {
            bits: bytes as u64 * 8,
            capacity_bits: crate::qr::MAX_DATA_BITS as u64,
        }),
    };
    Failure::new(kind, problem)
}

#[cfg(test)]
mod tests {
    use yata_core::import::capability::{Availability, Capability};
    use yata_core::import::ir::{Completeness, SectionKind};
    use yata_core::nonempty::NonEmpty;

    use super::*;
    use crate::serve::projection::fixture;

    fn listed(session: &mut Session) -> pb::ProfileList {
        let request = |id, kind| pb::ClientMessage {
            id,
            kind: Some(kind),
        };
        session.handle(request(
            1,
            Kind::OpenSession(pb::OpenSession {
                client_version: Some(pb::VERSION),
            }),
        ));
        let (out, _) = session.handle(request(2, Kind::ListProfiles(pb::ListProfiles {})));
        match out.into_iter().next().and_then(|m| m.kind) {
            Some(server_message::Kind::Response(pb::Response {
                result: Some(Reply::ProfileList(l)),
                ..
            })) => l,
            other => panic!("expected a profile list, got {other:?}"),
        }
    }

    fn read(profile: &pb::Profile) -> Vec<(Capability, Availability)> {
        profile
            .capabilities
            .iter()
            .map(|c| wire::profile_capability_of(c).expect("a capability"))
            .collect()
    }

    fn lacking(s: SectionKind) -> Availability {
        Availability::Unavailable {
            missing: NonEmpty::one(s),
        }
    }

    #[test]
    fn the_fixture_offers_the_inventory_and_names_what_the_rest_lack() {
        let list = listed(&mut Session::new(fixture::projection()));
        assert_eq!(list.profiles.len(), 2);
        for profile in &list.profiles {
            assert_eq!(
                read(profile),
                vec![
                    (
                        Capability::Inventory,
                        Availability::Available {
                            completeness: Completeness::Complete
                        }
                    ),
                    (
                        Capability::ShikigamiCollection,
                        lacking(SectionKind::Shikigami)
                    ),
                    (Capability::GamePresets, lacking(SectionKind::Presets)),
                    (Capability::Assets, lacking(SectionKind::Assets)),
                    (Capability::GuildView, lacking(SectionKind::Guild)),
                ],
                "{}",
                profile.name
            );
        }
    }

    #[test]
    fn capabilities_follow_the_held_sections_not_the_souls() {
        let mut projection = fixture::projection();
        for p in projection.profiles.values_mut() {
            p.held.clear();
        }
        let list = listed(&mut Session::new(projection));
        assert_eq!(
            read(&list.profiles[0])[0],
            (Capability::Inventory, lacking(SectionKind::Souls))
        );
    }
}
