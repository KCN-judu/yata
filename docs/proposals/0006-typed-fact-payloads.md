---
id: PRP-0006
status: accepted
date: 2026-09-25
area: persistence
related-issues: []
superseded-by: []
---

# PRP-0006: A fact's payload is typed by its envelope, not named beside it

## Problem

`fact.proto` carries a fact as an envelope and opaque bytes:

```text
Fact { profile : bytes, kind : FactKind, version : u32, payload : bytes }
```

The kind and version name a message, and the payload is expected to be that
message's bytes. Nothing in the encoding ties the two together. Several payload
messages share their first field's shape (`string soul_id = 1`, or
`string display_name = 1`), so a `SoulNoted` payload filed under `SoulMarked`,
or a `ProfileRenamed` payload filed under `ProfileCreated`, decodes without
error into a different fact. A bug in the writer, or a damaged byte in the
`kind` field, then becomes a plausible fact in the log. The fold applies it, and
the projection looks valid and is not.

The envelope also says every fact has a profile (`fact-format.md`: "there are no
global facts"). `BlobsPruned`, designed for compaction, names blobs, and blobs
are shared across profiles, so it has no profile to carry (round-2 audit C26).
The envelope cannot express it.

## Goals and non-goals

- Goal: a payload can be read only as the kind it was written as. A misfiled
  payload is not an encodable value, or it is refused on read.
- Goal: a fact about the store, not a profile, has an envelope without a
  profile.
- Goal: ADR-0002's evolution rules still hold: a kind gains a version only when
  its meaning changes, an older version is lifted, and a newer kind or version
  is `store.newer_format`.
- Non-goal: changing any kind's meaning, or the fold.

## Proposed design

Not decided; two shapes are compared here. Both use the same domain type, which
exists today in `yata-core::fact` except for the subject:

```text
Fact        = Profiled { profile : ProfileId, body : ProfileFact }
            | Store    { body : StoreFact }
ProfileFact = ProfileCreated { .. } | ProfileRenamed { .. } | ProfileRetired
            | ProfileRestored | SnapshotAcquired(Acquisition)
            | SnapshotRetracted { .. } | SoulMarked { .. } | SoulNoted { .. }
StoreFact   = BlobsPruned { digests : NonEmpty<Set<Digest>> }
```

### Option A: one oneof case per kind and version

```text
message Fact {
  oneof subject { bytes profile = 1; StoreSubject store = 2; }
  oneof body {
    ProfileCreated     profile_created_v1    = 10;
    ...
    SoulNoted          soul_noted_v1         = 17;   // read only, lifted
    SoulNotedV2        soul_noted_v2         = 18;
    BlobsPruned        blobs_pruned_v1       = 30;
  }
}
decode : Fact -> Result<yata_core::Fact, FactError>
  body unset                 -> Err(NewerFormat(UnknownBody))     -- see open question 1
  subject and body disagree  -> Err(Malformed(SubjectMismatch))   -- a store fact with a profile
```

The kind and version are the case's tag, once. A misfiled payload cannot be
written, because the case is the type. A new kind or version is a new tag.

### Option B: bytes, with the kind echoed inside the payload

```text
message Fact { oneof subject { .. } FactKind kind = 3; uint32 version = 4; bytes payload = 5; }
message SoulMarked { FactKind kind = 15; string soul_id = 1; Mark mark = 2; }  -- every payload
decode : payload.kind ≠ envelope.kind -> Err(Malformed(KindEcho))
```

The payload stays opaque, as today, and each payload message carries its kind in
a field number no payload uses otherwise. A misfiled payload decodes and is then
refused. The kind is written twice, and the two copies must agree.

### Comparison

|                      | A: oneof                              | B: kind echo                                 |
| -------------------- | ------------------------------------- | -------------------------------------------- |
| misfiled payload     | not encodable                         | encodable, refused on read                   |
| encodings of a kind  | one (the tag)                         | two, checked equal                           |
| unknown kind on read | body unset: prost drops unknown cases | `kind` unknown: already `store.newer_format` |
| store format change  | yes: new envelope                     | yes: new envelope                            |

Recommended: A. It follows the modelling rule that one meaning has one encoding,
and the only cost is open question 1.

## Compatibility and migration

Both options change the envelope, so the store format version moves from 1 to 2
(`fact-format.md`, § Reading old facts). A store at format 1 is read through the
current decoder, and each commit is lifted to the new envelope in memory;
nothing on disk is rewritten, because facts are never rewritten. A store at
format 2 is refused by a build that knows only 1, as today. No released build
has written a store, so the only format-1 stores are developers' own; the lift
from format 1 is still written and kept, as ADR-0002 requires.

## Alternatives

- **Keep the envelope and trust the writer.** Rejected: the failure is silent,
  and the log is the one place where a silent error outlives the process.
- **One message per fact, with `google.protobuf.Any`.** Rejected: a type URL per
  fact is larger, and `Any` still decodes a misfiled payload once the URL is
  wrong.

## Implementation and evidence

Not implemented. On acceptance: `fact.proto`, `yata-daemon::store::fact` (the
codec and the lift from format 1), the `Fact` type in `yata-core::fact`, and
`fact-format.md`. Tests: a misfiled payload is refused or unencodable; a
format-1 store opens and folds to the same projection; an unknown body is
`store.newer_format`.

## Open questions

1. With option A, prost drops a oneof case it does not know, so a fact of a
   newer kind decodes as "body unset". A writer bug that sets no body looks the
   same. Is "body unset" always `store.newer_format`, or does the store format
   version decide (unset in a store of this build's format is malformed)?
2. Does `BlobsPruned` need anything besides its digests, such as the retention
   `K` it was run with?
3. Is `StoreSubject` an empty message, or does it carry the store id, so a fact
   copied between stores is detected?

## Outcome

Accepted as option A by ADR-0032, 2026-09-25, in store format 2. Open question 1
is answered there: a new kind or version moves the store format, so an unset
body in a store of a known format is malformed. Questions 2 and 3 are decided
with compaction. The lift from format 1 described above is not written: ADR-0032
refuses a format-1 store instead.
