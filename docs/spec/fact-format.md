---
kind: spec
status: current
area: persistence
---

# Fact format

What the durable store contains: the records in the fact log, how raw snapshot
bytes are addressed, how a record written by an older build is read by a newer
one, and what compaction may and may not remove. ADR-0002 decides that
persistence is an append-only fact log plus a rebuildable projection; this page
defines the records.

What exists is listed in § Implemented; everything else here is _designed_.

## Tables

The store is one SQLite database, `store.sqlite3` (ADR-0019), holding four
tables. Only the first three are durable truth; the fourth is a cache. Every
table is `STRICT`, and SQLite sees only integers and bytes: no column holds a
field of a fact.

| Table              | Key                | Value                                                              | Durable                 |
| ------------------ | ------------------ | ------------------------------------------------------------------ | ----------------------- |
| `meta`             | fixed keys         | store format version, store id                                     | yes                     |
| `log`              | `seq: u64`         | one encoded commit                                                 | yes                     |
| `blobs`            | `digest: [u8; 32]` | raw snapshot bytes, compressed                                     | yes, prunable by policy |
| `projection_cache` | `seq: u64`         | the fold code version and a serialized projection as of that `seq` | no                      |

The tables, and every statement that touches them, exist only in `yata-store`,
which translates the store's instructions into SQL (ADR-0019). The daemon's
store module encodes facts and runs `yata-store`'s plans; the rest of the code
sees a log of facts and a blob lookup.

- `meta` values are bytes: the store format version as a 4-byte big-endian
  integer, the store id as 16 random bytes. Both are written once, when the
  store is created.
- A file is a Yata store when SQLite's `application_id` header field is
  `0x59415441` ("YATA"). An empty file is initialized; any other file is refused
  and left untouched.
- **Appending keeps the log dense.** A commit is inserted only if its `seq` is
  the last `seq` plus one; otherwise nothing is written and the append fails.
  Commits read back are checked to be dense as well.
- **The log is append-only in the database itself.** Triggers abort any `UPDATE`
  or `DELETE` on `log`, beneath the instruction set, which offers neither.
- **A write that fails voids its whole commit.** Every instruction of a commit
  runs in one transaction; if the append is refused, the blobs written with it
  are rolled back too.

## Commits and sequence numbers

The log is a sequence of **commits**. A commit is one SQLite write transaction
and holds one or more facts; it either lands whole or not at all.

```text
Commit {
  seq         : u64          // position in the log, dense, starts at 1
  recorded_at : timestamp    // daemon wall clock; informational only
  origin      : Origin       // Command(request id) | Job(job id) | Maintenance
  facts       : [Fact]       // at least one
}
```

- **`seq` is the revision.** The core protocol's revision (`core-protocol.md`)
  is the `seq` of the last commit. A commit is by definition a semantic change,
  and the revision advances by exactly one per commit.
- **A command maps to at most one commit.** A command carries the revision it
  was formed against (`core-protocol.md`, § Commands), and has one of three
  outcomes. The log never records a no-op.

  ```text
  CommandOutcome = Applied { revision : Revision }
                 | Unchanged { revision : Revision }
                 | Stale { base : Revision, current : Revision }   -- command.stale_revision

   current = r    base ≠ r
   ---------------------------------- (Stale: nothing applied)
   command(base, facts) = Stale { base, r }

   current = r    base = r    fold(s, facts) = s
   ---------------------------------------------- (Unchanged: no commit)
   command(base, facts) = Unchanged { r }

   current = r    base = r    fold(s, facts) = s' ≠ s
   ------------------------------------------------ (Applied: one commit at r + 1)
   command(base, facts) = Applied { r + 1 }
  ```

  Locking a soul that is already locked is `Unchanged`. An import is a job, not
  a command: it carries no base and always appends its acquisition.

- **`recorded_at` never orders anything.** Order is `seq`. A clock that jumps
  backwards changes a displayed time, never a result.
- **Blob writes share the commit's transaction.** A commit that references a
  blob and the write of that blob land together, so a fact never points at bytes
  that are not there.
- **A commit is checked by the fold before it is written.** The daemon applies
  the commit to the projection first; a commit the fold refuses is never
  written, so the rules that refuse a command are the rules that would refuse
  the log. A commit is a no-op when the projection's state after it equals the
  state before it. An acquisition always adds to the state, so importing the
  same bytes twice writes two commits.
- **A commit's bytes carry its `seq`**, which must equal the row it is stored
  under. An encoded commit is at most 1 MiB, written or read.

## The fact envelope

```text
Fact {
  profile : ProfileId     // every fact is scoped to one GameProfile
  kind    : FactKind      // the tag
  version : u16           // the format version of this kind's payload
  payload : bytes         // the kind's body at that version
}
```

**Every fact carries a `ProfileId`** (`feature-scope.md`). There are no global
facts: a setting that looks global is per profile, and two profiles never share
a fact.

**`version` is per kind, not per store.** Changing the payload of `SoulMarked`
bumps `SoulMarked`'s version and nothing else.

## Fact kinds

The catalogue below is the set that exists today. A new kind is additive; a
changed payload is a new version of the same kind (see "Reading old facts").

### Profile

| Kind                   | Payload                                             | Meaning                                                                                                                                                                                                                            |
| ---------------------- | --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ProfileCreated`       | display name, optional observed game account id     | a `GameProfile` exists from this commit on                                                                                                                                                                                         |
| `ProfileRenamed`       | display name                                        |                                                                                                                                                                                                                                    |
| `ProfileRetired`       | —                                                   | hidden from the UI; its facts remain and it can be restored by `ProfileRestored`                                                                                                                                                   |
| `ProfileRestored`      | —                                                   |                                                                                                                                                                                                                                    |
| `SchemeAccountLearned` | the 14-byte account segment of a scheme-code header | the profile's own account segment, learned from a code the user shared (`scheme-code.md`); identifies the profile's codes, and is never the header of a code encoded for the profile to import (ISS-0007); a later one replaces it |

A profile is never deleted. `ProfileId` is a 128-bit id the daemon generates; it
is not the game's account id, which a profile may or may not know.

The account segment in `SchemeAccountLearned` is account-derived data. It is
stored because encoding needs it, and like every fact it never leaves the
machine except in a backup the user makes; the daemon never logs it.

### Acquisition

| Kind                | Payload                                                                                                                                  | Meaning                                             |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| `SnapshotAcquired`  | blob digest, scope, coverage, channel, source (`Live` or `ExportFile`), probe build id, probe protocol version, observed game account id | one read from the game landed                       |
| `SnapshotRetracted` | blob digest, reason                                                                                                                      | a landed snapshot is withdrawn; the fold ignores it |

`scope` names what was read: `Souls`, `GameAssets`, and further scopes as the
probe gains them. `coverage` is `Complete` or `Partial`, and it carries the rule
the legacy design left to importer discipline:

- A **complete** snapshot of a scope _supersedes_ every earlier snapshot of the
  same `(profile, scope)`. A soul absent from it is absent from the account.
- A **partial** snapshot supersedes nothing. A soul absent from it is _not
  observed_, never _absent_. The fold overlays a partial on the live complete
  snapshot for the souls it contains.

The **live snapshot** of a `(profile, scope)` is the latest complete,
non-retracted one; the projection is built from it, the partials after it, and
the non-acquisition facts.

**An import never crosses profiles.** If a snapshot's observed game account id
differs from the one its target profile already knows, the import is refused
with `import.profile_mismatch` and nothing is committed. The account a profile
knows is the one named in `ProfileCreated`, else the observed account of its
earliest acquisition that carries one and is not retracted. A reading that did
not read the account is never a mismatch.

**No complete snapshot yet.** Until a `(profile, scope)` has a live complete
snapshot, every non-retracted partial one is live, oldest first, and the
inventory holds only the souls they observed.

**What a retraction withdraws.** `SnapshotRetracted` withdraws every earlier
acquisition of that blob in the same profile. A later acquisition of the same
bytes is a new observation and stands. A retraction that withdraws nothing — the
blob was never acquired in that profile, or already withdrawn — does not apply.

### User decisions

These are the only durable things the user authors about the inventory
(ADR-0002: "the only durable derived thing is the user's decisions").

| Kind                | Payload                                 | Meaning                                             |
| ------------------- | --------------------------------------- | --------------------------------------------------- |
| `SoulMarked`        | game soul id, mark                      | `Keep`, `Discard`, `Strengthen`, or `None` to clear |
| `SoulNoted`         | game soul id, note: text or cleared     | a text is never empty; clearing is its own case     |
| `SchemeSaved`       | scheme id, name, official payload bytes | a scheme code imported or authored and kept         |
| `SchemeRemoved`     | scheme id                               |                                                     |
| `ParamSetActivated` | param set id, version                   | the scoring parameter set this profile uses         |

Soul identity is `(profile, game soul id)`. The store never invents a soul id; a
decision about a soul the live snapshot no longer contains is kept, and applies
again if the soul reappears.

`SchemeSaved` stores the **decompressed official payload**, not a
`SoulSelection`. The wire page marks some bits as not yet understood, and a
scheme stored as the AST would lose them; stored as bytes it round-trips exactly
(`scheme-code.md`, "Round-trip is lossless").

Need profiles are not in this catalogue. Their authoring is undecided
(`scoring.md`), and the kinds are added when it is.

### Maintenance

| Kind          | Payload         | Meaning                                       |
| ------------- | --------------- | --------------------------------------------- |
| `BlobsPruned` | list of digests | these blobs' bytes were removed by compaction |

## Blobs and content addressing

A blob is the protobuf serialization of one probe `ReadResult` with its request
id left out, which belongs to the session and not to the reading
(`probe-protocol.md`, § Export file). A reading from the pipe and the same
reading from an export file are serialized by `yata-protocol` alike, so they
have the same digest.

- **The digest is SHA-256 over the uncompressed bytes.** Identity does not
  depend on how the bytes are stored, so the compression codec can change
  without re-keying anything.
- **Blobs are compressed at rest** with a codec named in the blob's own header:
  one byte, then the compressed bytes. The initial codec is zstd, byte `1`. An
  unknown codec byte is an error, never a guess.
- **A blob holds at most one frame's payload**, 16 MiB (`core-protocol.md`, §
  Frame), and decompression stops at that bound. Opening a blob checks its bytes
  against its digest; bytes that do not hash to their name are never used.
- **Identical bytes are stored once.** Two reads that return the same bytes
  produce two `SnapshotAcquired` facts pointing at one blob: the observation
  happened twice, the data exists once.

A blob is never modified. It is written once and later either kept or pruned.

## Ingestion

A reading reaches the fact log as a soul observation: the typed fields the
reader fills, each with the evidence behind its mapping (`probe-protocol.md`, §
Evidence), and the records as the reader saw them. Importing it checks two
levels, which fail differently:

- **A reading without soul identity is refused**, and nothing is committed.
  Unless the reading states `SoulRecord.soul_id` as established, no record has
  an identity the inventory can key on, and the import is
  `import.unestablished_identity`, naming whether the soul id is unmapped or
  inherited. Two records of one soul are `import.duplicate_soul`. A record
  without a soul id or with an empty one, an empty account id, a reading that
  does not state its coverage, and a result that is not a soul reading are
  `import.malformed_reading`.
- **A record that cannot be a row is kept and reported.** A row needs its set,
  slot, star, level, main attribute, and sub-attributes, each established by the
  reading and present on the record, and its innate attribute established,
  present or not, because it decides whether the soul is a boss soul (ADR-0029);
  then the premises of W-Soul that need no code table (`soul-mechanics.md`:
  star, level, at most four sub-attributes, no attribute twice, finite
  non-negative values, recorded rolls within the nodes reached). The lock and
  discard flags are carried as read, with their evidence. A record that fails is
  not a row (`query.md`) and is listed, by its position in the reading, with the
  import and the inventory. The blob keeps it as read; nothing is guessed or
  repaired.

Admission parses rather than checks: what passes is an admitted row whose fields
are present and established by type, so no later step checks them again.

```text
admit : SoulReading -> Result<Admitted, AdmissionError>
Admitted       { coverage, account : GameAccountId?, rows : [AdmittedSoul], defects : [RecordDefect] }
AdmittedSoul   { id, suit, star, slot, level, main, subs, innate : InnateReading, locked : Field<bool>, discarded : Field<bool> }
RecordDefect   { index : usize, soul : GameSoulId, kind : SoulDefectKind }
NotEstablished = Unmapped | Inherited
```

Until the reader establishes these fields, every real reading is refused at the
first level. That is the intended state: the store holds no soul whose identity
rests on an inherited hypothesis (ADR-0014).

## Reading old facts

A store written by an older build is always readable by a newer one. The
mechanism is ADR-0002's: a fact carries the version that wrote it, and a pure
function lifts it.

```text
lift : (kind, version, payload) -> current payload of that kind
```

- Lifting is a chain of one-step functions, `v1 → v2 → … → current`, each total
  and pure. A lift step is never deleted, because facts are never rewritten and
  a v1 fact may exist in any store forever.
- A payload that fails to lift is a load error for the whole store, reported
  with the commit's `seq`. It is never skipped: a fold that silently drops a
  fact produces a projection that looks valid and is not.
- **A store written by a newer build is refused.** `meta` carries the store
  format version; an unknown `FactKind`, a kind version above the build's
  current, or a store format above the build's is `store.newer_format`, and the
  daemon opens nothing. Downgrading the application is not supported.

The store format version moves only when the envelope or the table layout
changes. Adding a kind or a kind version does not move it.

## The projection cache

`projection_cache` holds serialized projections to make startup cheap. It is a
cache in the strict sense: deleting the table costs time and nothing else.

- An entry is valid only if its fold code version equals the running build's.
  Any mismatch discards it, and the projection is rebuilt from the log.
- Startup loads the newest valid entry at `seq = k` and folds the commits after
  `k`.
- At most one entry is kept. Writing a new one deletes the old in the same
  transaction.

## Compaction

The log is small — facts are tens of bytes and a user produces few of them. The
growth is in blobs, one full inventory read per import. Compaction bounds blobs
and never touches facts.

**What may be pruned.** A blob may be pruned only if the fold does not read it:

| Blob                                                                 | Prunable |
| -------------------------------------------------------------------- | -------- |
| the live snapshot of any `(profile, scope)`                          | never    |
| a partial snapshot after the live one                                | never    |
| a superseded snapshot among the newest `K` of its `(profile, scope)` | no       |
| any older superseded snapshot                                        | yes      |
| a retracted snapshot                                                 | yes      |

`K` defaults to 3. It exists so that "what changed since the last import" can be
computed from two retained snapshots rather than stored as a derived fact.

**How it runs.** Compaction is a job (`core-protocol.md`, the Job category). It
removes the selected blob bytes and appends one `BlobsPruned` commit naming
them, in one transaction. The `SnapshotAcquired` facts stay; the fold treats a
fact whose blob is pruned as superseded history with no readable content.

**What compaction never does:** rewrite, merge, or delete a fact; prune a blob
the fold reads; run without appending the `BlobsPruned` record of what it
removed.

## Implemented

At HEAD, with the tests named in `evidence/testing.md`:

- the fact kinds `ProfileCreated`, `ProfileRenamed`, `ProfileRetired`,
  `ProfileRestored`, `SnapshotAcquired`, `SnapshotRetracted`, and `SoulMarked`
  at version 1, and `SoulNoted` at version 2, in
  `crates/yata-daemon/proto/fact.proto`
- the codec and the lift chain, whose one step lifts `SoulNoted` version 1
  (where the empty text meant "cleared") to version 2; `store.newer_format` and
  `store.malformed_commit`
- commands with the outcomes above; imports as jobs
- the fold, as `yata-core::fact`; the soul inventory derived from the live
  snapshots over `yata-core::import`'s observations, carrying the game's raw
  codes, because decode to `SoulSet`, `SoulSlot`, and `SoulAttribute` waits on
  the game's code tables
- blobs, ingestion, and replay on open, in the daemon's store module;
  `yata-daemon log` dumps the log, showing account ids only as present

Not yet: `SchemeAccountLearned`, `SchemeSaved`, `SchemeRemoved`,
`ParamSetActivated`, `BlobsPruned`, compaction, and the projection cache. The
store starts with an empty cache table and replays the whole log on open; the
cache is never read.

## Not decided here

- the byte encoding is decided: protobuf, in `fact.proto`, with the evolution
  rules of [ADR-0002](../decisions/0002-fact-log-projection-and-fact-schema.md)
- backups are self-contained files written and verified by the daemon; the data
  directory, the backup format, restore, and the automatic triggers are
  [ADR-0010](../decisions/0010-data-directory-and-backup.md)
- when compaction runs: on the user's request only, or also on a schedule

## Related

- The persistence shape:
  [ADR-0002](../decisions/0002-fact-log-projection-and-fact-schema.md)
- The premise:
  [ADR-0001](../decisions/0001-pure-core-effects-at-the-boundary.md)
- Revisions and jobs on the wire: [core-protocol.md](core-protocol.md)
- Where blobs come from: [probe-protocol.md](probe-protocol.md)
- Scheme payloads kept as bytes: [scheme-code.md](scheme-code.md),
  `research/scheme-code-protocol.md` (local research, not published)
- `GameProfile`, `Snapshot`, `AcquisitionEvent`: [glossary.md](glossary.md)
