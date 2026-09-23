---
id: ADR-0019
status: accepted
date: 2026-09-24
area: persistence
supersedes: []
superseded-by: []
related: [ADR-0001, ADR-0002, ADR-0005, ADR-0010]
---

# ADR-0019: The store is SQLite, reached only through a pure crate that translates store instructions into SQL

## Status

Accepted, 2026-09-24. It replaces the byte-store choice in ADR-0002 (`redb`) and
adds a split trigger to ADR-0005. Both records carry an amendment pointing here.
Everything else in them stands.

## Context

ADR-0002 chose `redb` as the embeddable byte store and deferred an assessment of
its maintenance. That assessment was made on 2026-09-24
(`research/storage-engine-assessment.md`, local research, not published). Its
findings, as read on that date:

- **The file format has needed an explicit upgrade twice in three years.** redb
  1.0 promised a stable format. 2.0 broke it. 3.0 dropped the 2.x format, so an
  older file must be upgraded with 2.6 first. The 3.x format still opens in 4.x.
  A store that must stay readable on every user's disk would carry redb's
  upgrade chain on top of its own lift chain (ADR-0002).
- **Crash-recovery fixes landed as recently as 4.2.0 and 4.3.0**, in August and
  September 2026. Normal reads do not verify checksums.
- **One person writes most of the code**: 520 of 614 commits over two years.

SQLite was rejected in ADR-0002 on design grounds, not on durability: it pays
for a SQL engine the project does not query with, and it "leaves the door open
for repository code to creep back". The assessment found SQLite the strongest
option on the durability grounds that now matter most. Its file format has been
backwards compatible since 2004, and its developers state support through 2050
(<https://www.sqlite.org/lts.html>).

The maintainer chose SQLite and answered ADR-0002's objection with a structural
rule: **the project's code never writes or passes SQL. It issues store
instructions as values, and one dedicated crate translates them into SQL.**

## Decision

### The engine

1. **The store is one SQLite database file, `store.sqlite3`,** in the data
   directory (ADR-0010). It is reached through `rusqlite` with SQLite built in
   (the `bundled` feature), so every platform runs the same SQLite version,
   pinned by the lock file.
2. **Connection settings are fixed:** `journal_mode = WAL`, `synchronous = FULL`
   (a commit is durable when it returns), `locking_mode = EXCLUSIVE` (one daemon
   owns the file), and a fixed `application_id` that marks the file as a Yata
   store. `PRAGMA quick_check` runs when the store opens, and a full
   `integrity_check` runs before a backup or a compaction.
3. **SQLite stores bytes and integers only.** Tables are `STRICT`. Columns are
   `INTEGER` or `BLOB`, with one `TEXT` key column for `meta`. No value is
   interpreted by SQL: no column holds a field of a fact, and no index exists on
   anything but a key.

### The instruction crate

4. **`yata-store` is a pure crate that owns the store's instruction set and its
   translation into SQL.** It holds:
   - the instructions, as a closed Rust enum: append a commit, put a blob if
     absent, read commits from a `seq`, get a blob by digest, prune named blobs,
     replace the projection cache, read the cache, read and write `meta`
   - the schema (the `CREATE TABLE` statements) and its version
   - `plan`: a pure function from an instruction, or a list of instructions that
     must land together, to a `Transaction` of SQL statements with bound
     parameters
   - `interpret`: a pure function from the rows a statement returned, given as
     neutral values (integer, bytes, text, null), to a typed result or a typed
     error
5. **The instruction set is append-only by construction.** No instruction
   updates or deletes a commit. Pruning blobs is the one deletion, and it names
   blobs by digest (fact-format.md, Compaction). A change that needs another
   deletion adds an instruction to the enum, and that change is reviewed as a
   persistence change.
6. **No SQL passes through.** `Statement` and `Transaction` have private fields
   and no public constructor. Outside `yata-store`, code can hold a plan and
   hand it to the executor, but cannot write SQL into one. There is no
   instruction that carries an SQL string.
7. **The executor is effectful and small.** A module in `yata-daemon` opens the
   connection, runs a `Transaction` inside `BEGIN IMMEDIATE … COMMIT`, and
   returns the raw rows. It has no other entry point. It never branches on SQL
   text, and it builds no statement.
8. **`yata-store` depends on no workspace crate and on no SQLite binding.** It
   deals in bytes, `seq` numbers, and digests. Encoding facts to bytes stays
   where ADR-0002 put it, in the daemon's store module. The translation is
   therefore tested without a database, by comparing plans. An integration test
   in the daemon runs every instruction against a real SQLite file.

### Enforcement

9. **A preflight check, `sql-boundary`, rejects:**
   - an SQL keyword in a string literal outside `crates/yata-store/`
   - a dependency on `rusqlite`, or a `use rusqlite`, outside the daemon's
     executor module
   - any `execute`, `prepare`, or `query` call on a connection outside that
     module
10. **ADR-0005 gains trigger 5, language boundary.** Code that emits another
    language's text, such as SQL, lives in its own pure crate, so that the
    translation is testable alone and no other crate can emit that language.
    `yata-store` is the crate this trigger creates.

## Alternatives

- **Keep `redb`**, pinned to 4.3 with a list of restrictions. Offered and not
  chosen: the project would own redb's format upgrades for the life of every
  store.
- **A project-owned append-only log file.** Recommended by the assessment,
  because the workload needs only an append, a crash rule, and a rewrite for
  pruning, and the backup format of ADR-0010 already frames facts that way. Not
  chosen by the maintainer: the project would own crash recovery, write
  ordering, and atomic replacement on Windows, which SQLite has done for twenty
  years.
- **SQLite used directly from the daemon's store module.** Rejected by the
  maintainer: it is the "repository code creeps back" failure ADR-0002 named.
  SQL written where it is used grows into a second model of the data.
- **An ORM or a query builder crate.** Rejected: it hands the domain a general
  query language, which ADR-0002 rules out. The instruction set is deliberately
  smaller than any query language.

## Consequences

**Easier.** The file format has the longest compatibility record available.
Crash safety, atomic commits, and exclusive ownership come from SQLite. The SQL
the project issues is a fixed, reviewable list in one crate, tested without a
database.

**Harder.**

- `bundled` compiles SQLite's C source, so building the daemon needs a C
  compiler on every platform CI builds on.
- A new store operation is a new instruction variant, a plan, an interpretation,
  and a test. It is never a one-line query. This is intended.
- SQLite adds files beside the store in WAL mode. With
  `locking_mode = EXCLUSIVE` there is no shared-memory file, but the WAL file
  exists while the daemon runs. A backup never copies files; it is written from
  one read transaction (ADR-0010).

**What changes elsewhere.** `spec/fact-format.md` describes the tables in SQLite
terms. The data directory holds `store.sqlite3` in place of `store.redb`
(ADR-0010, amended). `architecture/overview.md` adds `yata-store` to the crate
table.
