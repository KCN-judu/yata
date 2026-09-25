---
id: ADR-0031
status: accepted
date: 2026-09-25
area: import
supersedes: []
superseded-by: []
related: [ADR-0002, ADR-0005, ADR-0014, ADR-0029, ADR-0030]
---

# ADR-0031: Imports end at a Yata-owned snapshot IR of independent sections

## Status

Accepted, 2026-09-25, on the maintainer's instruction. It refines ADR-0030,
rule 3. The inventory still comes only from a file the user supplies, and
community formats are still read. They now end at an intermediate representation
that Yata owns, and Yata's own snapshot format is accepted beside them. PRP-0008
is accepted in this redesigned form. PRP-0007 is withdrawn (rule 9).

## Context

The first importer (PRP-0008) parsed `mumu-snapshot-v1` straight into domain
souls. That works for one format carrying one kind of data, and fails in three
ways as soon as there are more:

- **Formats differ in what they carry.** The maintainer's sample holds souls,
  Shikigami instances, the game's equipment presets, currencies, realm cards,
  and a guild. Another format may hold souls alone. A design where every section
  is required, or where an absent section reads as an empty one, would refuse a
  useful file, or erase data it never saw.
- **Format details leak.** Without a boundary, names such as `hero_equips` or
  `fixedAttribute`, a fraction-coded rate, or a Chinese set name as the
  identifier would reach every module that needs a soul.
- **The fact log is reader-shaped.** A blob is a probe `Reading`, and
  `SnapshotAcquired` records a channel and a probe build. Nothing in production
  writes it yet (`FactLog::ingest` has test callers only), so it can be replaced
  rather than migrated.

The repository fixes one constraint: `yata-core` depends on no workspace crate
(ADR-0005), so the IR's Rust types cannot be the generated protobuf types. The
IR has a Rust definition in `yata-core` and a schema in `yata-protocol`, joined
by one conversion in `yata-daemon`, the same way the domain and `core.proto`
are.

## Decision

1. **Three stages, three owners.** A format module understands one external
   format. The snapshot IR (`spec/snapshot-ir.md`) describes what Yata received,
   in Yata's own vocabulary. The domain describes which game values are valid.

   ```text
   bytes ─detect→ Format ─parse→ SourceDocument(format) ─normalize→ YataSnapshot
         ─admit→ AdmittedSnapshot ─persist→ facts ─fold→ projection ─→ query, scoring, protocol
   ```

   A source document may hold what the domain forbids (slot 7, a set name no set
   has, a missing roll count). Normalization never forces it into domain types;
   admission decides, with typed errors.

2. **The IR is independent sections.** A section is absent or present, and a
   present section states its completeness:

   ```text
   Section<T>   = Absent | Present { completeness: Completeness, value: T }
   Completeness = Complete | Partial | Unstated
   ```

   An absent section and a present, empty one are different values. The sections
   of the first schema version are souls, Shikigami, game presets, assets, and
   guild. A section is added by a minor schema version.

3. **Capabilities are derived, never listed.** What a profile can do follows
   from which sections its projection holds, through one total function over a
   closed `Capability` type. No code outside the format modules branches on the
   source format; the format is provenance, shown only as provenance.

4. **Yata's own format is first class.** `yata-snapshot` is the IR's schema in
   `crates/yata-protocol/proto/snapshot.proto`. Its file form is proto3 JSON,
   recognised by its header. Its stored form is the protobuf binary encoding,
   which is the blob a fact names. Any imported file can be exported as one and
   imported again. The schema version moves only when the IR's meaning moves,
   never with scoring parameters.

5. **Community formats are adapters.** Each is one module under
   `yata-daemon::import`, owning its header rule, syntax, names, nulls, units,
   and normalization. It owns no game rule, fact, query, or scoring semantics.

6. **Admission is pure and separate.** `yata-core` turns a `YataSnapshot` into
   an `AdmittedSnapshot` of domain values, or names why each record is not one.
   The failures that are handled differently are typed apart: an unknown or
   ambiguous format, an unsupported version, malformed source, a failed
   normalization, a source value Yata does not support, a broken reference, and
   a record the domain refuses.

7. **An import updates only the sections it carries.** A fact records the
   sections a snapshot holds and their completeness. A complete section
   supersedes that profile's earlier state of the same section. A partial or
   unstated one adds to it. An absent section changes nothing. The original file
   is kept by digest beside the canonical IR, for provenance and debugging. No
   query reparses either.

8. **PRP-0006 lands with this.** The fact schema changes anyway, and no store
   exists outside development machines. A fact body becomes a typed `oneof`
   (PRP-0006, option A), in store format 2, recorded in its own ADR.

9. **The equipment relation is never imported.** Which Shikigami wears which
   soul is not read from any file. Yata builds simulated loadouts instead. The
   game's equipment presets are imported as their own section: they are the
   player's saved plans, not current equipment, and not Yata's plans.

10. **The probe schema is deleted.** `probe.proto`, the export file, the
    recordings, the conversion to observations, and the research commands exist
    only because the reader did. None serves the IR, and none stays as a
    parallel import path.

## Alternatives

- **Parse each format straight to domain types** (the first PRP-0008 design).
  Rejected: it cannot carry a value the domain refuses, so the reason is lost at
  the parser. It also makes each format re-implement the game's rules.
- **One account object with optional fields.** Rejected: `Option<Vec<T>>`
  conflates "not provided" with "provided, empty", and it invites fields that
  are only valid together.
- **A manually listed capability set in the file.** Rejected: it would be a
  second encoding of what the sections already say, and it could disagree.
- **Adopt the first community format as Yata's format.** Rejected: its names,
  units, and quirks (a rate as a fraction, the innate attribute inside the
  sub-attribute list) would become Yata's contract.
- **Keep the probe schema as a second import path, or as a test adapter.**
  Rejected by the maintainer. Its only real recording is the one that got the
  account restricted, and the synthetic fixtures are replaced by snapshot
  fixtures.
- **Import the equipment relation where a format carries it.** Rejected by the
  maintainer: Yata simulates loadouts and does not import the game's current
  equipment.

## Consequences

**Easier.** A new format is one module and one fixture pair, source to IR. A
file with souls alone is useful, and a file without a guild leaves the guild
alone. Tests of a format's syntax and of the game's semantics are separate: a
known source file normalizes to a committed IR fixture, and a known IR fixture
admits to known domain values.

**Harder.** The IR is written twice, in Rust and in protobuf, with a conversion
and a round-trip test. Capabilities reach the application only once the protocol
session reads the store's fold, which it does not yet (the session serves a
fixture). The fact log's reader-shaped acquisition, its admission from probe
readings, and its inventory derivation are rewritten.

**Records that change with this one.** `spec/snapshot-ir.md` is new.
`spec/import-format.md` keeps only the external formats and their mapping to the
IR. `spec/fact-format.md` records the import fact and its section rule.
`spec/feature-scope.md` drops current equipment in favour of simulated loadouts.
`spec/query.md` drops `equipped_by`. `spec/probe-protocol.md` is deleted, and
the probe schema leaves `spec/protocol-versions.md`.
