---
id: ADR-0010
status: accepted
date: 2026-09-23
area: persistence
supersedes: []
superseded-by: []
related: [ADR-0002, ADR-0006, ADR-0008, ADR-0015, ADR-0019]
---

# ADR-0010: One data directory per user; backups are self-contained files the daemon writes and verifies

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published.

## Context

The store is one `redb` database holding the log, the blobs, and a projection
cache (`spec/fact-format.md`). ADR-0002 made copying the log and its blobs the
whole of backup and restore, and kept `redb` replaceable behind a trait. Two
questions remain: whether a backup is the raw database file or an export format,
and when backups happen without the user asking.

Lessons from an earlier tool for the same game:

- **Paths must survive Chinese user names and spaces.** The failure is real on
  Windows.
- **Every path derives from one root.** No module assembles a path of its own.
- **A backup that restores the database without its raw objects restores an
  empty shell.** A restore must check that every object the database references
  is present in the backup, re-hash every object, and bring the restored store
  to a startable state before switching to it.

## Decision

### Data directory

| Platform | Default root                                             |
| -------- | -------------------------------------------------------- |
| Windows  | `%LOCALAPPDATA%\io.github.kcn-judu.yata\`                |
| macOS    | `~/Library/Application Support/io.github.kcn-judu.yata/` |

`%LOCALAPPDATA%` rather than `%APPDATA%`: the store grows with every import, and
roaming profiles would sync it.

The directory name is the app-id, `io.github.kcn-judu.yata` (ADR-0015). It is
fixed at the first release and never changed after. The display name and the
repository name may change; the directory does not follow them.

```text
<root>/
  store.redb      the store (spec/fact-format.md)
  backups/        backup files
  logs/           daemon logs; never included in a backup
  temp/           emptied at daemon start
```

- **Only the daemon touches the directory.** The Flutter application never
  builds a path into it. The reader has its own directory (ADR-0006).
- **Every path derives from one root value.** A module receives a path and never
  assembles one.
- **Non-ASCII and spaces are tested.** A Windows CI test runs the store under a
  root containing Chinese characters and a space.
- **The root can move later.** A user setting can point the data directory
  elsewhere, for example to another drive. Moving is a backup and restore, not a
  file copy. The setting is not part of milestone 1.

### Backup file

**A backup is a self-contained file the daemon writes from one read
transaction.** It does not depend on `redb`: it is a stream of frames in the
same 4-byte length-prefixed framing as both wires, carrying `fact.proto`
messages (ADR-0002).

```text
BackupHeader  { backup_format_version, store_format_version, store_id,
                created_at, last_seq, commit_count, blob_count }
Commit × commit_count       // seq 1 … last_seq, dense, as stored
Blob   × blob_count         // digest, codec, stored bytes
BackupTrailer { sha256 over every byte before the trailer }
```

- It contains everything durable (`meta`, the whole log, every blob not yet
  pruned) and nothing that is not (the projection cache, logs).
- It is platform-independent. A backup made on Windows restores on macOS, and
  carries the whole history, including the user's marks and notes, which an
  export file (ADR-0008) does not.
- A build refuses a backup whose `backup_format_version` or
  `store_format_version` is newer than it knows, as it refuses a newer store.

### Restore

Restore replaces the whole store, all profiles. It is checked before anything is
replaced:

1. The trailer hash matches.
2. Commits are dense from 1 to `last_seq`.
3. Every blob's digest, recomputed over its decompressed bytes, matches.
4. Every blob a fact references is present, unless a `BlobsPruned` fact names
   it.
5. Every fact lifts to its current form (`spec/fact-format.md`, "Reading old
   facts").
6. The staging store is written into `temp/` and opened once, successfully.

Only then is the current store renamed aside and the staging store moved into
its place. The previous store is kept until the next successful daemon start,
then deleted. A failure at any step leaves the current store untouched.

### When backups happen

| Trigger                        | Automatic |
| ------------------------------ | --------- |
| the user asks                  | —         |
| before a store format upgrade  | yes       |
| before compaction prunes blobs | yes       |

Automatic backups keep the newest 5 and delete older automatic ones. A backup
the user made is never deleted automatically.

## Alternatives

- **Copy `store.redb`.** Rejected: the copy is consistent only while the daemon
  is stopped, and it ties the backup to the storage engine ADR-0002 keeps
  replaceable.
- **A database file plus a directory of object files.** Rejected: the store
  already holds blobs, and one file per blob would bring back the reference
  check between two storage places that such a restore has to perform.
- **Scheduled backups.** Not chosen by the maintainer. The two automatic
  triggers cover the moments where data is at risk from the application itself.

## Consequences

**Easier.** Backup, restore, and moving a whole history to another machine or
platform are one mechanism. A restore that fails leaves nothing half-applied.
The backup format reuses the frame codec and the fact schema, so it adds no new
encoding.

**Harder.** Restore has six checks, and each needs a test with a deliberately
broken backup. The app-id can never be changed after the first release without a
migration.

`spec/fact-format.md` defines the store this record backs up and points here for
the data directory, the backup format, restore, and the automatic triggers.

## Amendment 2026-09-24

The store is a SQLite file, `store.sqlite3`, in place of `store.redb`
([ADR-0019](0019-sqlite-store-behind-an-instruction-crate.md)). While the daemon
runs, SQLite keeps its write-ahead log beside it as `store.sqlite3-wal`. The
backup format, restore, and the automatic triggers are unchanged: a backup is
written from one read transaction and never copies the store's files, which is
the reason the "Copy `store.redb`" alternative was rejected, and it applies to
the SQLite file in the same way.
