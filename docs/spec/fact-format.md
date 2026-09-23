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

Everything here is _designed_. Nothing is _implemented_.

## Tables

The store is one SQLite database, `store.sqlite3` (ADR-0019), holding four
tables. Only the first three are durable truth; the fourth is a cache. Every
table is `STRICT`, and SQLite sees only integers and bytes: no column holds a
field of a fact.

| Table              | Key                | Value                                    | Durable                 |
| ------------------ | ------------------ | ---------------------------------------- | ----------------------- |
| `meta`             | fixed keys         | store format version, store id           | yes                     |
| `log`              | `seq: u64`         | one encoded commit                       | yes                     |
| `blobs`            | `digest: [u8; 32]` | raw snapshot bytes, compressed           | yes, prunable by policy |
| `projection_cache` | `seq: u64`         | a serialized projection as of that `seq` | no                      |

The tables, and every statement that touches them, exist only in `yata-store`,
which translates the store's instructions into SQL (ADR-0019). The daemon's
store module encodes facts and runs `yata-store`'s plans; the rest of the code
sees a log of facts and a blob lookup.

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
- **A command maps to exactly one commit.** A command that would change nothing
  — locking a soul that is already locked — is acknowledged without a commit and
  without advancing the revision. The log never records a no-op.
- **`recorded_at` never orders anything.** Order is `seq`. A clock that jumps
  backwards changes a displayed time, never a result.
- **Blob writes share the commit's transaction.** A commit that references a
  blob and the write of that blob land together, so a fact never points at bytes
  that are not there.

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

| Kind              | Payload                                         | Meaning                                                                          |
| ----------------- | ----------------------------------------------- | -------------------------------------------------------------------------------- |
| `ProfileCreated`  | display name, optional observed game account id | a `GameProfile` exists from this commit on                                       |
| `ProfileRenamed`  | display name                                    |                                                                                  |
| `ProfileRetired`  | —                                               | hidden from the UI; its facts remain and it can be restored by `ProfileRestored` |
| `ProfileRestored` | —                                               |                                                                                  |

A profile is never deleted. `ProfileId` is a 128-bit id the daemon generates; it
is not the game's account id, which a profile may or may not know.

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
with `import.profile_mismatch` and nothing is committed.

### User decisions

These are the only durable things the user authors about the inventory
(ADR-0002: "the only durable derived thing is the user's decisions").

| Kind                | Payload                                 | Meaning                                             |
| ------------------- | --------------------------------------- | --------------------------------------------------- |
| `SoulMarked`        | game soul id, mark                      | `Keep`, `Discard`, `Strengthen`, or `None` to clear |
| `SoulNoted`         | game soul id, text                      | empty text clears the note                          |
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

A blob is the protobuf serialization of one probe `ReadResult`: the bytes as
received from the pipe, or, for an export file, the result re-serialized after
parsing the JSON (ADR-0008). Both paths serialize with `yata-protocol`, so the
same reading has the same digest.

- **The digest is SHA-256 over the uncompressed bytes.** Identity does not
  depend on how the bytes are stored, so the compression codec can change
  without re-keying anything.
- **Blobs are compressed at rest** with a codec named in the blob's own header.
  The initial codec is zstd.
- **Identical bytes are stored once.** Two reads that return the same bytes
  produce two `SnapshotAcquired` facts pointing at one blob: the observation
  happened twice, the data exists once.

A blob is never modified. It is written once and later either kept or pruned.

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
