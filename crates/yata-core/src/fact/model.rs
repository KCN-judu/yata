//! The fact catalogue of `fact-format.md`, § Fact kinds, for the kinds implemented so far.

/// A SHA-256 digest: the identity of a blob (`fact-format.md`, § Blobs and content addressing).
pub type Digest = [u8; 32];

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
    /// Empty text clears the note.
    SoulNoted {
        soul: GameSoulId,
        text: String,
    },
}

/// What caused a commit (`fact-format.md`, § Commits and sequence numbers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Command { request_id: u64 },
    Job { job_id: u64 },
    Maintenance,
}

/// One commit of the log: the facts that landed together at `seq`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// Dense, from 1. The revision is the `seq` of the last commit.
    pub seq: u64,
    /// The daemon's wall clock in Unix milliseconds. Informational: it never orders anything.
    pub recorded_at_ms: i64,
    pub origin: Origin,
    /// At least one.
    pub facts: Vec<Fact>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
