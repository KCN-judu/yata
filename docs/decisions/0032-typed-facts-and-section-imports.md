---
id: ADR-0032
status: accepted
date: 2026-09-25
area: persistence
supersedes: []
superseded-by: []
related: [ADR-0002, ADR-0030, ADR-0031, PRP-0006]
---

# ADR-0032: Facts are a typed oneof in store format 2, and an import is recorded by its sections

## Status

Accepted, 2026-09-25, on the maintainer's instruction. It accepts PRP-0006 as
option A, and it makes ADR-0031 rules 7, 8, and 10 exact for the fact log. It
supersedes nothing. It makes one exception to ADR-0002's rule that every older
store format is lifted: format 1 is refused (rule 3).

## Context

Store format 1 carries a fact as an envelope of kind, version, and opaque
payload bytes. Nothing ties the payload to the kind, so a misfiled payload
decodes into a different, plausible fact (PRP-0006).

Its one import fact, `SnapshotAcquired`, is shaped by the reader that ADR-0030
removed: a channel, a probe build, a probe protocol version, and a blob that is
a probe `Reading`. ADR-0031 replaces that reading with the snapshot IR, whose
sections are independent: an import that carries the guild and not the souls
must not change the souls.

No released build has written a store, and `FactLog::ingest` has only test
callers. The only format-1 stores are developers' own, and their acquisitions
hold probe readings that the IR cannot represent.

## Decision

1. **A fact body is a typed `oneof`, one case per kind and version** (PRP-0006,
   option A). The kind and version are the case's tag, written once, so a
   misfiled payload cannot be encoded. The envelope's subject is a `oneof` as
   well. It has one case today, the profile. The store subject, for
   `BlobsPruned`, is reserved and is decided with compaction.
2. **Store format 2.** `STORE_FORMAT_VERSION` is 2. Adding a fact kind, a
   version of one, or a value to a fact enum moves the store format. A build
   therefore meets an unknown case or value only in a store it has already
   refused as `store.newer_format`, and an unset body or an unknown value in a
   store of a format it knows is a malformed commit, not a newer one. This
   answers PRP-0006's open question 1.
3. **A format-1 store is refused, not lifted.** Opening one fails with its own
   error, `RetiredFormat`, distinct from a newer or a missing format. This is an
   exception to ADR-0002, for this format only. No released build wrote one, and
   its acquisitions cannot be lifted without inventing the provenance of an
   import that never happened. A developer's format-1 store is recreated.
4. **`SnapshotImported` replaces `SnapshotAcquired`.**

   ```text
   SnapshotImported {
     snapshot : Digest                  -- the blob of the IR's canonical binary encoding
     original : Digest                  -- the blob of the file as imported
     source   : SourceFormat            -- YataSnapshot(version) | Community(FormatTag)
     sections : Sections                -- SectionKind → Completeness, at least one entry
   }
   ```

   `Sections` is a map with at least one entry, so a section cannot be listed
   twice and an import of nothing cannot be written. The channel, the source
   (`Live` or `ExportFile`), the probe build, the probe version, and the
   observed account are gone. Both blobs land in the commit's transaction.

5. **`SnapshotRetracted` names a snapshot digest** and withdraws every section
   of every current import of that snapshot in the profile. A retraction that
   withdraws nothing is refused.
6. **The fold is per profile and per section.** For each section kind, the live
   imports are the ones not withdrawn, in log order. The base is the last one
   whose completeness for that section is `Complete`, and the layers are the
   imports after it whose completeness for that section is `Partial` or
   `Unstated`. With no `Complete` import, every live import of the section is a
   layer. An import that does not list a section is not part of it, so it
   changes nothing there.

   ```text
   held : ProfileState -> Map SectionKind Completeness
   held(p)[k] = Complete                              if section k has a base
              = max { completeness of its layers }    if it has layers only
              -- absent                               if it has neither
   ```

   `held` is the input of `capability::availability` (ADR-0031, rule 3).

7. **The soul inventory is derived from IR snapshots.** The daemon decodes the
   live snapshot blobs, and `yata-core` admits each one (ADR-0031, rule 6) and
   overlays its souls section: the base's souls, then each layer's souls
   replacing the records of the souls they contain. A soul the base does not
   contain is absent. Marks and notes attach by game soul id, as before. The
   probe reading, its admission, and its row type are deleted.
8. **The observed account is not recorded.** The IR carries no account id, so
   the check that refused a reading of another account
   (`import.profile_mismatch`) has nothing to compare. Binding an import to an
   account is reopened as a question for the IR, not for the fact log.

## Alternatives

- **PRP-0006 option B, the kind echoed inside the payload.** Rejected in
  PRP-0006: two encodings of one kind, checked equal on read.
- **Lift format 1 to format 2.** Rejected: a probe reading has no IR form, so
  the lift would have to invent a snapshot, or drop the acquisitions and
  silently change what the log says.
- **One completeness per import instead of per section.** Rejected: ADR-0031
  makes sections independent, and a file complete for its souls and partial for
  its guild is an ordinary file.
- **Keep the observed account as an optional field.** Rejected for now: no
  import fills it, and a field that is always absent is a second encoding of
  "unknown".

## Consequences

**Easier.** A misfiled fact cannot be written. An import of the guild alone
cannot erase souls, by construction of the fold, and the capability computation
reads one map. Replaying the log is the only way the projection is built, so
rebuilding it gives the same result.

**Harder.** Every developer store is recreated. A profile can no longer refuse
another account's file until the IR states an account. The blob store now holds
two blobs per distinct import.

**Records that change with this one.** `spec/fact-format.md` (the envelope, the
import fact, the section rule, the retired format), `architecture/overview.md`
(the fact log and the store), `project/status.md`, and `evidence/testing.md`.
PRP-0006 is accepted.
