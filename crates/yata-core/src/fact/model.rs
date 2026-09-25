//! The fact catalogue of `fact-format.md`, § Fact kinds, for the kinds implemented so far.

use std::num::NonZeroU64;

/// A SHA-256 digest: the identity of a blob (`fact-format.md`, § Blobs and content addressing).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Digest(pub [u8; 32]);

/// A commit's position in the log: dense, from 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Seq(NonZeroU64);

impl Seq {
    pub const FIRST: Seq = Seq(NonZeroU64::MIN);

    /// `None` for 0.
    pub fn new(n: u64) -> Option<Seq> {
        NonZeroU64::new(n).map(Seq)
    }

    pub fn get(self) -> u64 {
        self.0.get()
    }
}

impl From<NonZeroU64> for Seq {
    fn from(n: NonZeroU64) -> Seq {
        Seq(n)
    }
}

/// How far the log has been applied: the `seq` of the last commit, or none for the empty log.
/// The core protocol's revision (`core-protocol.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Revision(Option<Seq>);

impl Revision {
    /// The revision of the empty log.
    pub const EMPTY: Revision = Revision(None);

    pub fn at(seq: Seq) -> Revision {
        Revision(Some(seq))
    }

    pub fn last(self) -> Option<Seq> {
        self.0
    }

    /// The `seq` the next commit takes; `None` only past `u64::MAX`.
    pub fn next(self) -> Option<Seq> {
        match self.0 {
            None => Some(Seq::FIRST),
            Some(s) => s.0.checked_add(1).map(Seq),
        }
    }
}

/// A `GameProfile`'s id: 128 bits the daemon draws when the profile is created. It is not the
/// game's account id, which a profile may or may not know.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProfileId(pub [u8; 16]);

/// Why a game identifier was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdError {
    /// The game never names anything with an empty id; an empty one means "not read".
    Empty,
}

/// The game's identifier for one soul, as the probe read it. The store never invents one; soul
/// identity is `(profile, game soul id)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GameSoulId(String);

/// The game's identifier for an account, as the probe read it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GameAccountId(String);

macro_rules! game_id {
    ($t:ident) => {
        impl $t {
            pub fn new(id: impl Into<String>) -> Result<$t, IdError> {
                let id = id.into();
                if id.is_empty() {
                    return Err(IdError::Empty);
                }
                Ok($t(id))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

game_id!(GameSoulId);
game_id!(GameAccountId);

/// What a reading covers. Scopes are added as the probe gains them (`GameAssets` is designed and
/// waits on the probe).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Scope {
    Souls,
}

/// Whether a snapshot covers its whole scope. A complete snapshot supersedes every earlier one of
/// the same `(profile, scope)`; a partial one supersedes nothing (`fact-format.md`, § Acquisition).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Coverage {
    Complete,
    Partial,
}

/// How the reading reached the game (ADR-0006).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    DesktopMemory,
    MumuAdb,
}

/// Whether the reading arrived from a running probe or from its export file (ADR-0008).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    Live,
    ExportFile,
}

/// The probe protocol version the reading was made under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProbeVersion {
    pub major: u32,
    pub minor: u32,
}

/// A user's mark on a soul. A `SoulMarked` fact carries `Option<Mark>`, and `None` clears it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mark {
    Keep,
    Discard,
    Strengthen,
}

/// The payload of `SnapshotAcquired`: one read from the game landed. The reading's bytes are the
/// blob named by `digest`; everything else is provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Acquisition {
    pub digest: Digest,
    pub scope: Scope,
    pub coverage: Coverage,
    pub channel: Channel,
    pub source: Source,
    pub probe_build_id: String,
    pub probe_version: ProbeVersion,
    /// The account the reading belongs to; `None` when the probe did not read it.
    pub observed_account: Option<GameAccountId>,
}

/// A note's text: never empty. Clearing a note is its own case, `None`, not an empty text.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NoteText(String);

impl NoteText {
    /// `None` for the empty string.
    pub fn new(text: impl Into<String>) -> Option<NoteText> {
        let text = text.into();
        (!text.is_empty()).then_some(NoteText(text))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One fact: a kind's body, scoped to one profile. There are no global facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fact {
    pub profile: ProfileId,
    pub body: FactBody,
}

/// The fact kinds, each in its current form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FactBody {
    /// A `GameProfile` exists from this commit on.
    ProfileCreated {
        display_name: String,
        account: Option<GameAccountId>,
    },
    ProfileRenamed {
        display_name: String,
    },
    /// Hidden from the UI; its facts remain.
    ProfileRetired,
    ProfileRestored,
    SnapshotAcquired(Acquisition),
    /// Every earlier acquisition of this blob in this profile is withdrawn.
    SnapshotRetracted {
        digest: Digest,
        /// Informational; the fold never reads it.
        reason: String,
    },
    SoulMarked {
        soul: GameSoulId,
        mark: Option<Mark>,
    },
    /// `None` clears the note.
    SoulNoted {
        soul: GameSoulId,
        note: Option<NoteText>,
    },
}

/// What caused a commit (`fact-format.md`, § Commits and sequence numbers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Command { request_id: u64 },
    Job { job_id: u64 },
    Maintenance,
}

/// The facts of one commit: at least one, in the order they apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Facts(Vec<Fact>);

impl Facts {
    /// `None` for no facts: a commit that holds nothing is not a commit.
    pub fn new(facts: Vec<Fact>) -> Option<Facts> {
        (!facts.is_empty()).then_some(Facts(facts))
    }

    pub fn one(fact: Fact) -> Facts {
        Facts(vec![fact])
    }

    pub fn as_slice(&self) -> &[Fact] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<Fact> {
        self.0
    }
}

/// One commit of the log: the facts that landed together at `seq`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// The revision is the `seq` of the last commit.
    pub seq: Seq,
    /// The daemon's wall clock in Unix milliseconds. Informational: it never orders anything.
    pub recorded_at_ms: i64,
    pub origin: Origin,
    pub facts: Facts,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_values_have_no_second_encoding() {
        assert_eq!(Seq::new(0), None);
        assert_eq!(Facts::new(vec![]), None);
        assert_eq!(NoteText::new(""), None);
        assert_eq!(Revision::EMPTY.next(), Some(Seq::FIRST));
        assert_eq!(Revision::at(Seq::FIRST).next(), Seq::new(2));
        assert_eq!(
            Seq::new(u64::MAX)
                .map(Revision::at)
                .and_then(Revision::next),
            None
        );
    }

    #[test]
    fn an_empty_game_id_is_not_an_id() {
        assert_eq!(GameSoulId::new(""), Err(IdError::Empty));
        assert_eq!(GameAccountId::new(""), Err(IdError::Empty));
        assert_eq!(
            GameSoulId::new("a1").map(|s| s.as_str().to_owned()),
            Ok("a1".to_owned())
        );
    }
}
