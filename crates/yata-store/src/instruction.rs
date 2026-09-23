/// A SHA-256 digest: the identity of a blob (`fact-format.md`, § Blobs and content addressing).
pub type Digest = [u8; 32];

/// The store's own random identity, written once when the store is created.
pub type StoreId = [u8; 16];

/// The fixed keys of the `meta` table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaKey {
    /// The store format version, which moves only when the envelope or the table layout changes.
    StoreFormatVersion,
    StoreId,
}

impl MetaKey {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            MetaKey::StoreFormatVersion => "store_format_version",
            MetaKey::StoreId => "store_id",
        }
    }
}

/// Everything the project can ask of the store. The set is closed: a new need is a new variant,
/// reviewed as a persistence change (ADR-0019).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    /// Whether the file is empty, a Yata store, or something else.
    Inspect,
    /// Create the tables of an empty file and write its identity.
    Initialize {
        store_id: StoreId,
        store_format_version: u32,
    },
    /// SQLite's fast structural check, run when the store opens.
    QuickCheck,
    /// SQLite's full check, run before a backup or a compaction.
    IntegrityCheck,
    ReadMeta {
        key: MetaKey,
    },
    /// Append a commit at `seq`, which must be the last `seq` plus one.
    AppendCommit {
        seq: u64,
        commit: Vec<u8>,
    },
    /// The `seq` of the last commit, or 0 for an empty log.
    LastSeq,
    /// Up to `limit` commits from `from` on, in order.
    ReadCommits {
        from: u64,
        limit: u32,
    },
    /// Store a blob's bytes under its digest. Identical bytes are stored once, so a blob already
    /// present is left as it is.
    PutBlob {
        digest: Digest,
        bytes: Vec<u8>,
    },
    GetBlob {
        digest: Digest,
    },
    /// Remove the named blobs' bytes (compaction). Always planned together with the
    /// `BlobsPruned` commit that records it.
    PruneBlobs {
        digests: Vec<Digest>,
    },
    /// Replace the one projection cache entry.
    ReplaceCache {
        seq: u64,
        fold_version: u32,
        projection: Vec<u8>,
    },
    ReadCache,
}
