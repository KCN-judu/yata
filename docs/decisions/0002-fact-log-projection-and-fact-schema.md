---
id: ADR-0002
status: accepted
date: 2026-09-23
area: persistence
supersedes: []
superseded-by: []
related:
  [
    ADR-0001,
    ADR-0003,
    ADR-0004,
    ADR-0005,
    ADR-0006,
    ADR-0008,
    ADR-0010,
    ADR-0019,
  ]
---

# ADR-0002: Persistence is an append-only fact log of protobuf records plus a rebuildable in-memory projection; there is no query language

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published. The choice of embeddable byte store inside this shape
(`redb`) is the one part of it designed to be replaceable; see Consequences.

## Context

An earlier tool for the same game persisted through a large SQL repository
layer, a long migration chain that rebuilt several tables more than once, and a
backup chain of its own. Two of its design assumptions came from wanting a
database rather than from the domain:

- **Filtering and pagination were pushed into SQL** because the inventory was
  treated as a remote data source. The front end never held the full inventory.
- **Current state was stored, not derived.** The inventory was a mutable table
  whose rows carried a presence flag written by the importer, so "not read this
  time" and "deleted" were distinguished by importer discipline rather than by
  the data model.

The scale does not justify any of this. Ten thousand souls at roughly sixteen
fields each is on the order of a few megabytes: the entire inventory fits in
memory comfortably, and a filter-sort-group over it in Rust runs in
microseconds. There is no dataset here large enough for a query planner to have
anything to plan.

SQL is also the wrong shape for ADR-0001's premise. Its three-valued `NULL`
logic, its implicit row ordering and grouping, its mutation-as-default
`UPDATE`/`DELETE`, and its non-composable expression grammar all push against
"state is a value, computation is a pure function." Language-distaste alone
would not decide a persistence format — but here the discomfort and the
requirements point the same way, and the requirements are the reason.

One part of that tool's design was correct and is adopted wholesale: raw
snapshot bytes are content-addressed by SHA-256, and each import is an
acquisition event. The defect was not the fact log; it was that the fact log was
treated as an implementation detail of a mutable projection instead of as the
only durable truth.

Making the log the only durable truth turns schema evolution into a
serialization problem, and the byte encoding of a fact becomes the most
expensive encoding in the project to get wrong. A wire format can be bumped,
because both ends ship together. A fact written today must stay readable on
every user's disk, in every later build. The encoding needs tagged fields with a
documented evolution rule, so that lifting old facts is needed only when a
fact's _meaning_ changes, not every time a field is added.

## Decision

Persistence has three layers, and only the first is durable.

1. **The fact log is the truth.** A snapshot is an immutable, content-addressed
   record (SHA-256 over the payload) plus its provenance. Nothing is ever
   updated or deleted in place: a re-read appends a new snapshot that supersedes
   an earlier one for the same scope. Removing data is expressed by appending a
   fact, never by rewriting an old one.
2. **The projection is derived, and rebuildable.** The inventory, scores,
   head/tail results — every current-state view — is a pure fold of the fact
   log, held in memory as an immutable value (ADR-0001). It is a cache and is
   treated as one: deleting it costs time, never information. Its staleness is
   not tracked by version counters; it is a function of the log's high-water
   mark.
3. **Queries are typed Rust expressions over in-memory values.** There is no
   query language, no schema, and no migration chain. Filtering, sorting,
   grouping, and aggregation are iterator composition over typed values; the
   search stage of matching (`spec/scoring.md`) is Rust code, not a query.

Schema evolution is serialization evolution: a fact carries the version of the
format that wrote it, and older facts are lifted to the current form by a pure
function at load time. There is no DDL and no table rebuild.

### The byte store

**The embeddable byte store is `redb`.** Its responsibilities are narrow: an
append-only key-value store, crash-safe, with a write transaction that either
commits or does not. It stores fact payloads by digest, the log's ordering
metadata, and serialized projection snapshots. It is reached only through a
small trait, because it is the one reversible part of this decision. `redb` is
dual-licensed `MIT OR Apache-2.0`. Its maintenance state is assessed before it
appears in `Cargo.toml`; this record does not assert it.

### The fact schema

**Commits and fact payloads are Protocol Buffers, defined in their own schema
file, `fact.proto`,** separate from the core and probe wire schemas (ADR-0004,
ADR-0006). It is owned by the store module in `yata-daemon` (ADR-0005) and
generated with `protox`, like the Rust side of the wire schemas. No Dart
bindings are generated: nothing outside the daemon reads facts.

The evolution rules are protobuf's, made strict:

- **A tag number is never reused.** A removed field's tag and name go into
  `reserved`.
- **Adding a field does not bump the kind's `version`.** An older payload
  decodes with the field at its default, and the kind's definition states what
  that default means. This is the case the lift chain does not have to handle.
- **The kind's `version` is bumped only when meaning changes.** A field that now
  means something different, a unit change, or a split or merged field gets a
  new version and a lift step ([fact-format.md](../spec/fact-format.md),
  "Reading old facts").
- **Enum values are never renumbered or reused.** An unknown enum value on load
  is a load error, not a default.
- **The store format version moves only for envelope or table changes**, as
  [fact-format.md](../spec/fact-format.md) states.

`fact.proto` has its own version line. A change to either wire schema never
forces a fact change, and a fact change never forces a protocol bump.

## Alternatives

- **SQLite as a dumb byte store** (no schema beyond a fact table, no SQL in the
  domain). Rejected: it satisfies the log requirement while still paying for a
  SQL engine we would not query with, and it leaves the door open for repository
  code to creep back.
- **Cozo, an embeddable Datalog engine.** The strongest candidate, and the one
  that answers the stated objection about SQL: Datalog is declarative, its rules
  compose, it has no `NULL` semantics problem, and recursion is native. Rejected
  on fit rather than on quality. The access patterns here are filter, sort,
  group, and the combinatorial search of matching — none of which a query engine
  expresses better than Rust does, and the search it cannot help with at all.
  Adopting it buys a second language to learn and a young engine to depend on,
  in exchange for expressiveness this system has no query that needs.
- **Datomic / XTDB.** The origin of the immutable-datom, speculative-`with`
  model that layer 3 above borrows from. Rejected as a dependency: both are JVM,
  which a desktop bundle cannot carry.
- **SurrealDB.** Rejected: multi-model document and graph capability is unused
  here, and its licensing is not a fit for a freely distributed desktop
  application.
- **A document store with Mongo-style CRUD** (`PoloDB` and similar). Rejected:
  "CRUD" is the vocabulary this decision exists to avoid. A mutable document
  store models current state as stored data, which is the mutable-inventory
  problem described in Context with different syntax.
- **A Rust-native fact encoding** (`postcard`, `bincode`). Rejected: neither has
  field-level evolution. Every added field would be a new kind version and a
  lift step, which is the exact cost the encoding constraint exists to avoid.
  Both also tie the on-disk format to a serde representation of Rust types, so
  an innocent refactor of a type could change bytes on disk.
- **Reuse the wire schemas' messages as fact payloads.** Rejected: it couples
  the durable format to the wire's version line, and a protocol minor bump would
  become a persistence migration.
- **JSON facts.** Rejected for the log: larger, slower, and no enforced tags.
  JSON is the export format for a reading from `yata-reader` (ADR-0008), a
  different job.

## Consequences

**Easier.** Backup and restore reduce to carrying the log and its blobs, with no
migration step on the restore path (the backup file is ADR-0010). Compaction is
possible and safe: the projection can be discarded and rebuilt, so storage
growth is bounded by a policy rather than by careful incremental updates. There
is no repository layer, no migration chain, and no set of independent
cache-invalidation schemes to maintain. Adding a field to a fact is a one-line
schema change with no migration. One toolchain covers all three protobuf
schemas, and its compatibility rules are ones every contributor can look up.

**Harder.** Every access pattern a SQL design gets from the engine, group-bys
included, must be written in Rust. Compaction must actually be implemented,
because an append-only log grows without bound if nobody prunes it; this
decision pays for bounded storage with a compaction job rather than with a
migration chain. Large inventories must be loaded into memory at startup, and
this design depends on the scale estimate above staying true. Facts cannot be
inspected with a text editor, so the daemon needs a subcommand that dumps the
log in readable form, and it is part of the store's first implementation, not an
extra. The protobuf rule about defaults means a new field's default must be a
correct reading of every older fact; when no default is correct, the change is a
meaning change and takes a version bump.

**The reversible part.** `redb` is chosen, but nothing outside one module knows.
If it proves unsuitable, substituting another crash-safe key-value store is a
change to that module and needs no new decision record, because the shape above
does not depend on which store implements it.

**Where the details live.** The tables, commits, fact kinds, lift steps, and
compaction policy are specification: [fact-format.md](../spec/fact-format.md).
The typed query values are [query.md](../spec/query.md).

## Amendment 2026-09-24

The byte store is SQLite, not `redb`
([ADR-0019](0019-sqlite-store-behind-an-instruction-crate.md)). The maintenance
assessment this record deferred found that redb's file format had needed an
explicit upgrade twice in three years. SQLite is reached only through
`yata-store`, a pure crate that translates store instructions into SQL, which
answers this record's objection that SQLite "leaves the door open for repository
code to creep back". Everything else in this record stands: the fact log, the
projection, the absence of a query language, and `fact.proto`.
