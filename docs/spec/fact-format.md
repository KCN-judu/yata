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
| `blobs`            | `digest: [u8; 32]` | snapshot and file bytes, compressed                                | yes, prunable by policy |
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
  a command: it carries no base and always appends its `SnapshotImported`.

- **`recorded_at` never orders anything.** Order is `seq`. A clock that jumps
  backwards changes a displayed time, never a result.
- **Blob writes share the commit's transaction.** A commit that references a
  blob and the write of that blob land together, so a fact never points at bytes
  that are not there.
- **A commit is checked by the fold before it is written.** The daemon applies
  the commit to the projection first; a commit the fold refuses is never
  written, so the rules that refuse a command are the rules that would refuse
  the log. A commit is a no-op when the projection's state after it equals the
  state before it. An import always adds to the state, so importing the same
  snapshot twice writes two commits.
- **A commit's bytes carry its `seq`**, which must equal the row it is stored
  under. An encoded commit is at most 1 MiB, written or read.

## The fact envelope

```text
Fact {
  subject : Profile(ProfileId)   // a oneof; the store subject is kept for compaction
  body    : one case per kind and version, e.g. SnapshotImportedV1(SnapshotImported)
}
```

**The kind and version are the body's case** (ADR-0032, PRP-0006 option A). A
payload cannot be filed under another kind, because the case is its type.

**Every fact today is about one profile** (`feature-scope.md`). There are no
global facts: a setting that looks global is per profile, and two profiles never
share a fact. The subject is a `oneof` so that `BlobsPruned`, which names blobs
shared across profiles, can be about the store instead; that case is added with
compaction.

## Fact kinds

The catalogue below is the set that exists today. A changed payload is a new
version of the same kind, and a new kind or version moves the store format (see
"Reading old facts").

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

### Imports

| Kind                | Payload                                                   | Meaning                                            |
| ------------------- | --------------------------------------------------------- | -------------------------------------------------- |
| `SnapshotImported`  | snapshot digest, original digest, source format, sections | one imported file landed (ADR-0032, rule 4)        |
| `SnapshotRetracted` | snapshot digest, reason                                   | every current import of that snapshot is withdrawn |

```text
SnapshotImported {
  snapshot : Digest          -- the blob of the IR's canonical binary encoding (spec/snapshot-ir.md)
  original : Digest          -- the blob of the file as imported
  source   : SourceFormat    -- YataSnapshot(version) | Community(FormatTag)
  sections : Sections        -- SectionKind → Completeness; at least one entry, each kind once
}
```

**Sections are independent** (ADR-0031, rule 7; ADR-0032, rule 6). For each
`(profile, section)`, the live imports are the ones not withdrawn that list the
section, in log order:

- The **base** is the last live import whose completeness for the section is
  `Complete`. It _supersedes_ every earlier import of that section. A soul
  absent from the base is absent from the account.
- The **layers** are the live imports after the base, whose completeness for the
  section is `Partial` or `Unstated`. They supersede nothing. A soul absent from
  a layer is _not observed_, never _absent_; the inventory overlays each layer
  on the base for the souls it contains.
- With **no base**, every live import of the section is a layer, oldest first.
- An import that **does not list a section** is not part of it. Importing a file
  that holds only the guild changes nothing about the souls.

What a profile holds of each section follows, and is the input of the capability
table (`spec/snapshot-ir.md`, "Capabilities"):

```text
held(p)[k] = Complete                         if section k has a base
           = max { completeness of layers }   if it has layers only
           -- absent                          if it has neither
```

**What a retraction withdraws.** `SnapshotRetracted` withdraws every earlier
import of that snapshot in the same profile, every section of it. A later import
of the same snapshot is a new observation and stands. A retraction that
withdraws nothing — the snapshot was never imported in that profile, or already
withdrawn — does not apply.

**No account check.** A format-1 acquisition carried the account it was read
from, and an import of another account's reading was refused. The snapshot IR
states no account, so a format-2 import is not checked against one (ADR-0032,
rule 8).

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
decision about a soul the live imports no longer contain is kept, and applies
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

An import stores two blobs: the snapshot's canonical binary encoding
(`spec/snapshot-ir.md`), which the fold reads, and the file as imported, which
is kept for provenance and debugging and which nothing parses again (ADR-0031,
rule 7). The canonical encoding is deterministic, so one snapshot has one digest
however often it is imported.

- **The digest is SHA-256 over the uncompressed bytes.** Identity does not
  depend on how the bytes are stored, so the compression codec can change
  without re-keying anything.
- **Blobs are compressed at rest** with a codec named in the blob's own header:
  one byte, then the compressed bytes. The initial codec is zstd, byte `1`. An
  unknown codec byte is an error, never a guess.
- **A blob holds at most one frame's payload**, 16 MiB (`core-protocol.md`, §
  Frame), and decompression stops at that bound. Opening a blob checks its bytes
  against its digest; bytes that do not hash to their name are never used.
- **Identical bytes are stored once.** Two imports of the same snapshot produce
  two `SnapshotImported` facts pointing at the same two blobs: the observation
  happened twice, the data exists once.

A blob is never modified. It is written once and later either kept or pruned.

## Importing

An import lands a snapshot the daemon has already read and normalized
(`spec/import-format.md`, `spec/snapshot-ir.md`):

```text
import : (ProfileId, YataSnapshot, original bytes) -> Result<Imported, ImportError>
ImportError = NoSections                                -- a snapshot of nothing
            | OriginalMismatch { stated, found }        -- provenance names another file
            | Refused(IrError)                          -- the IR's own rules
            | Blob(BlobError) | Commit(CommitError)
```

- **The IR's own rules refuse the snapshot whole**: limits, a duplicate id, a
  preset naming a soul the snapshot does not hold (`spec/snapshot-ir.md`).
  Nothing is committed.
- **A record admission refuses is kept and reported** (ADR-0031, rule 6). The
  stored snapshot holds it as imported; the import's result and the inventory
  list it with its reason, and it is not a row. Nothing is guessed or repaired.

The inventory is derived, not stored: the live snapshots of the souls section
are decoded and admitted, and their souls overlaid as § Imports states.

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
  format version, and a store format above the build's is `store.newer_format`;
  the daemon opens nothing. Downgrading the application is not supported.
- **A new kind, kind version, or enum value moves the store format** (ADR-0032,
  rule 2). A build therefore never meets a case or value it does not know in a
  store it opened: an unset body or an unknown value is
  `store.malformed_commit`, never a newer format.
- **Format 1 is retired, not lifted** (ADR-0032, rule 3). Its facts held the
  retired reader's readings, which the IR cannot represent. A format-1 store is
  refused with its own error, and a developer recreates it. This is the one
  exception to lifting, and no released build wrote such a store.

The store format version is 2.

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

| Blob                                                                   | Prunable |
| ---------------------------------------------------------------------- | -------- |
| the base or a layer of any `(profile, section)`                        | never    |
| a superseded snapshot among the newest `K` of its `(profile, section)` | no       |
| any older superseded snapshot                                          | yes      |
| a retracted snapshot, and the file blob of any snapshot                | yes      |

`K` defaults to 3. It exists so that "what changed since the last import" can be
computed from two retained snapshots rather than stored as a derived fact.

**How it runs.** Compaction is a job (`core-protocol.md`, the Job category). It
removes the selected blob bytes and appends one `BlobsPruned` commit naming
them, in one transaction. The `SnapshotImported` facts stay; the fold treats a
fact whose blob is pruned as superseded history with no readable content.

**What compaction never does:** rewrite, merge, or delete a fact; prune a blob
the fold reads; run without appending the `BlobsPruned` record of what it
removed.

## Implemented

At HEAD, with the tests named in `evidence/testing.md`:

- store format 2: the fact kinds `ProfileCreated`, `ProfileRenamed`,
  `ProfileRetired`, `ProfileRestored`, `SnapshotImported`, `SnapshotRetracted`,
  `SoulMarked`, and `SoulNoted`, each at version 1, as `oneof` cases in
  `crates/yata-daemon/proto/fact.proto`; a format-1 store refused
- the codec; `store.newer_format` at open and `store.malformed_commit`
- commands with the outcomes above; imports as jobs, storing the snapshot and
  the original file
- the fold per profile and section, and `held`, as `yata-core::fact`; the soul
  inventory derived from the live snapshots through `yata-core::import::admit`
- replay on open, in the daemon's store module; `yata-daemon log` dumps the log,
  showing account ids only as present

Not yet: `SchemeAccountLearned`, `SchemeSaved`, `SchemeRemoved`,
`ParamSetActivated`, `BlobsPruned` and its store subject, compaction, and the
projection cache. The store starts with an empty cache table and replays the
whole log on open; the cache is never read. The import job and the protocol
session do not call the fact log yet.

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
- Where blobs come from: [snapshot-ir.md](snapshot-ir.md),
  [import-format.md](import-format.md)
- Typed facts and section imports:
  [ADR-0032](../decisions/0032-typed-facts-and-section-imports.md)
- Scheme payloads kept as bytes: [scheme-code.md](scheme-code.md),
  `research/scheme-code-protocol.md` (local research, not published)
- `GameProfile`, `Snapshot`, `AcquisitionEvent`: [glossary.md](glossary.md)
