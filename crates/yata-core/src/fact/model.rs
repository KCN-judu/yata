//! The fact catalogue of `fact-format.md`, § Fact kinds, for the kinds implemented so far.

use std::collections::BTreeMap;
use std::num::NonZeroU64;

use crate::import::ir::{Completeness, SectionKind, SourceFormat, YataSnapshot};

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

/// The game's identifier for one soul, as an imported snapshot states it. The store never invents
/// one; soul identity is `(profile, game soul id)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GameSoulId(String);

/// The game's identifier for an account, as the user named it when creating a profile.
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

impl GameSoulId {
    /// The soul id an imported snapshot states. A source id is never empty, so this cannot fail.
    pub fn from_source(id: &crate::import::ir::SourceId) -> GameSoulId {
        GameSoulId(id.as_str().to_owned())
    }
}

/// A user's mark on a soul. A `SoulMarked` fact carries `Option<Mark>`, and `None` clears it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mark {
    Keep,
    Discard,
    Strengthen,
}

/// The sections one import holds, each with its completeness (ADR-0032, rule 4): at least one,
/// and each kind once.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sections(BTreeMap<SectionKind, Completeness>);

impl Sections {
    /// `None` for no sections: an import of nothing is not an import.
    pub fn new(sections: BTreeMap<SectionKind, Completeness>) -> Option<Sections> {
        (!sections.is_empty()).then_some(Sections(sections))
    }

    /// The sections a snapshot holds; `None` when it holds none.
    pub fn of(snapshot: &YataSnapshot) -> Option<Sections> {
        Sections::new(snapshot.sections().into_iter().collect())
    }

    /// The completeness this import states for `kind`, or `None` when it does not carry it.
    pub fn get(&self, kind: SectionKind) -> Option<Completeness> {
        self.0.get(&kind).copied()
    }

    pub fn as_map(&self) -> &BTreeMap<SectionKind, Completeness> {
        &self.0
    }
}

/// The payload of `SnapshotImported`: one imported file landed (ADR-0032, rule 4). The snapshot's
/// canonical encoding and the file as imported are blobs; the rest is what the fold reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotImport {
    /// The blob of the IR's canonical binary encoding.
    pub snapshot: Digest,
    /// The blob of the file as imported.
    pub original: Digest,
    pub source: SourceFormat,
    pub sections: Sections,
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
    SnapshotImported(SnapshotImport),
    /// Every earlier import of this snapshot in this profile is withdrawn, all its sections.
    SnapshotRetracted {
        snapshot: Digest,
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
    fn an_import_holds_at_least_one_section() {
        use crate::import::ir::{Completeness, SectionKind};
        assert_eq!(Sections::new(BTreeMap::new()), None);
        let one = Sections::new(BTreeMap::from([(
            SectionKind::Guild,
            Completeness::Unstated,
        )]))
        .expect("one section");
        assert_eq!(one.get(SectionKind::Guild), Some(Completeness::Unstated));
        assert_eq!(one.get(SectionKind::Souls), None);
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
