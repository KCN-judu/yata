---
id: ADR-0029
status: accepted
date: 2026-09-25
area: domain
supersedes: []
superseded-by: []
related: [ADR-0009, ADR-0026, ADR-0014, PRP-0005]
---

# ADR-0029: A soul is ordinary or a boss soul with its innate attribute, and a reading that cannot tell is bad data

## Status

Accepted, 2026-09-25.

## Context

The maintainer's observation of 2026-09-25 (`scheme-code.md`, "Evaluation") is
that every boss soul (首领御魂) carries exactly one innate attribute, one of
six, and no other soul carries any. `Soul` holds this as
`innate: Absent | Present(SoulAttribute) | Unknown`. That type admits three
states the game does not have:

- `Present` of an attribute outside the six innate attributes;
- `Unknown`, which is not a state of any soul but of a reading that could not
  tell;
- on the core wire, `Innate { absent: false }`, a marker whose value means
  nothing and which the daemon read as `Absent`.

`Unknown` was introduced because no reader recording yet shows how a reading
carries the innate attribute. It lets an undecided reading into the inventory,
where each such soul turns a chosen 固有属性 into an open verdict. The reader
does not produce uncertain souls: a reading that cannot say whether a soul is a
boss soul has failed to read it.

## Decision

1. **A soul is one of two kinds.** `Soul` carries
   `kind: SoulKind = Ordinary | Boss(InnateAttribute)`. The innate attribute is
   an `InnateAttribute`, so only the six can be held. There is no unknown kind.
   `Soul::innate`, `Innate`, and its three variants are removed.
2. **A reading that does not determine the kind is bad data.** A record whose
   innate field the reader does not establish cannot be an inventory row: the
   fact log keeps the reading as read and reports the record, as it does for any
   row field that is not established (`fact-format.md`, § Ingestion). Decode
   from an admitted record to a `Soul` refuses an innate attribute outside the
   six. An established record with no innate attribute is `Ordinary`; one with
   an innate attribute is `Boss`. A refused soul is never a row (`query.md`,
   "Collections").
3. **The 固有属性 group matches on the kind.** With something chosen: an
   ordinary soul passes; a boss soul passes when its innate attribute is chosen;
   a boss soul whose own set is chosen and whose innate attribute is not fails;
   a boss soul whose own set is not chosen (`AnySet`, or no set chosen) and
   whose innate attribute is not chosen stays `Undetermined(Innate)`, because
   the game's reading of that case is not observed. `OpenRule::Innate` means
   that case and nothing else.
4. **The core wire carries the kind as a oneof of messages.**
   `Soul.kind = oneof { OrdinarySoul ordinary; BossSoul boss }`, with
   `OrdinarySoul {}` empty and `BossSoul { SoulAttribute innate }`. A soul with
   neither, or a boss innate attribute outside the six, is `query.malformed`.
   Field 9 and the `Innate` message are retired. Core protocol version 1 is
   still reserved, so the change needs no version bump.
5. **Boss-ness is carried, not derived.** Each soul states its kind, and the
   kind is not inferred from the set. A check of kind against set is PRP-0005;
   it waits on evidence that imported souls carry the suit codes the scheme's
   bit table gives them.

## Alternatives

- **Keep `Unknown` in the domain.** Rejected: it models the reader's failure as
  a state of the soul, and every such soul makes filtering less exact while
  looking like data.
- **Keep the three states and fix the types only** (`InnateAttribute`, an empty
  marker message). Rejected for the same reason; it removes the illegal values
  and keeps the illegal state.
- **One enum for the whole soul**, `Soul = Ordinary(Body) | Boss(Body, Innate)`.
  Equivalent; rejected only because every field read would first match the
  constructor. A struct with a `kind` is the same sum, with the shared fields
  once.
- **Derive the kind from the set.** Deferred to PRP-0005: a sourced list exists,
  but the suit codes of imported souls are not yet confirmed to match it.

## Consequences

**Easier.** The evaluator is one `match` on the kind, total, with no case for a
soul that is not a game state. The only open verdict of 固有属性 is the one the
game has not been observed deciding. `absent: false` cannot be written.

**Harder.** Decode must know how a reading carries the innate attribute before
any soul can be admitted, so the reader's mapping of that field becomes a
prerequisite of a non-empty inventory. The probe schema's `SoulRecord.innate`
must say "no innate attribute" distinctly from "not mapped"; its field evidence
already tells the two apart, and decode relies on that.
