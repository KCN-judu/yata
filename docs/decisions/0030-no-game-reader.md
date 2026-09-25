---
id: ADR-0030
status: accepted
date: 2026-09-25
area: import
supersedes: [ADR-0006, ADR-0007, ADR-0008]
superseded-by: []
related: [ADR-0002, ADR-0005, ADR-0011, ADR-0014, ADR-0016]
---

# ADR-0030: Yata does not read the game; the user supplies the data, as a file in a community snapshot format

## Status

Accepted, 2026-09-25, on the maintainer's instruction. It supersedes ADR-0006
(the reader channel), ADR-0007 (the open-source reader), and ADR-0008 (the
platforms and the reader's export file).

ADR-0006 and ADR-0007 were **wrong on the same evidence.** They weighed rights,
transport, and auditability, and did not weigh the legal exposure of publishing
and maintaining a program that reads the game. That exposure existed when they
were written. What changed on 2026-09-25 is that the game flagged a MuMu-channel
read as an injection, which made the risk concrete. ADR-0008 was **right at the
time and overtaken by circumstances**: its platform split existed only because
the reader ran on Windows alone.

## Context

The reader, `yata-reader`, was a separate public repository that read the game's
memory from outside, on the desktop client and in the MuMu emulator (ADR-0007).
The daemon started it, talked to it over a named pipe, and elevated it through
UAC when the game required it (ADR-0006).

**Maintaining the reader carries a legal risk the project will not accept.**

- The game's operator does not permit third-party programs that access the
  client, and it enforces this with anti-cheat. On 2026-09-25 the game treated a
  read through the MuMu channel as an injection. The users of such a program
  risk their accounts.
- In the maintainer's assessment, a public program that reads a game client's
  memory can also expose its author and distributor to civil claims from the
  operator. Under PRC law it can expose them to criminal liability: courts have
  applied Criminal Law Article 285, paragraph 3 (providing programs or tools for
  intruding into or illegally controlling computer information systems) to tools
  for online games. Reading from outside, without writing, does not reliably
  place a program outside that provision. This is the maintainer's risk
  assessment, not legal advice.
- Publishing the source under the maintainer's name identifies the maintainer as
  its author and distributor. A reader must also follow every change to the
  game's in-memory layout, so the risk would recur with every release.

**The project's value is the analysis, not the acquisition.** Scoring,
selection, scheme codes, and queries only need the inventory as data. None of
them depend on how the data was obtained.

**Some existing tools already write common file formats for this data.** They
come from outside this project and do not comply with the operator's rules.
Their files abstract a soul well: set, position, star, level, main attribute,
and sub-attributes with their values and strengthening counts, with no emulator
or process detail. Parsing a file that a user already has is not reading the
game.

**The CSV route is the safe one.** A user can write the inventory by hand in a
spreadsheet and turn it into that format without running any tool against the
game. It is slow, and it is the only route that involves no third-party program.

## Decision

1. **No part of this project reads, attaches to, injects into, or automates the
   game, or an emulator running it.** The project maintains no reader. The
   `KCN-judu/yata-reader` repository is deleted, and nothing in this repository
   builds, starts, downloads, or links to a reader.
2. **The inventory comes only from a file the user imports.** Where the data
   came from is the user's concern. The daemon's concern is to parse it and
   refuse what it cannot parse.
3. **The import formats are community snapshot formats, a closed set.** The
   daemon recognizes a file's format from its header, refuses a file that
   matches none or several, and parses it at the import boundary into the
   domain's values. `mumu-snapshot-v1` is the first format, and more may be
   added. Each is specified by its shape in `spec/import-format.md`, one section
   per format. Where a file's values meet the domain, for example a set name
   meeting `SoulSet`, the mapping is a hypothesis until this project's own
   evidence establishes it (ADR-0014).
4. **The project never names, links, recommends, or bundles a program that
   produces a format from the game.** Public documentation identifies a format
   by its shape and its header tag, and a tag is only a name. The application
   does not suggest a way to obtain a file. The sample files used to specify a
   format stay on the maintainer's machine (ADR-0016).
5. **The documented route is a CSV the user writes.** `spec/import-format.md`
   publishes a CSV template: one row per soul, with its columns and their units.
   It also states how each column becomes a field of the first format. The user
   does the conversion. The project ships no converter.
6. **Windows and macOS have the same data path.** There is no read channel on
   either platform, so neither has one to lack. The daemon no longer contains
   pipe, elevation, or reader-spawning code on any platform.
7. **The probe protocol is kept only as far as data reception uses it.** The
   parts that exist to talk to a live reader are removed: the MuMu channel, the
   transport and elevation rules, and `spec/reader-security.md`. The reading
   records and the export file remain until the community-format import replaces
   them as the representation of an imported file. The replacement, and what
   then remains of the probe schema, is a proposal, not this record.

## Alternatives

- **Keep the reader and add a stronger disclaimer.** Rejected: a disclaimer
  moves the risk to the user. It does not remove the maintainer's exposure as
  author and distributor.
- **Keep the reader but stop maintaining it in public.** Rejected: a private
  reader is still a reader, and ADR-0007's reason for publishing it, that users
  must be able to audit an elevated binary, still holds. The only
  risk-consistent option is not to have one.
- **Read the game's on-disk cache instead of its memory.** Rejected for the same
  reason. It is still a program built against the game's private data, and its
  format follows the game's changes.
- **Define a new file format of our own.** Rejected as the primary format. The
  community formats already describe the data well, and one more format would
  split the files users already have. A format of our own remains possible
  later, as a new ADR, if the community formats prove too weak.
- **Ship a CSV-to-format converter.** Offered and not chosen. The maintainer
  wants the template and the description only, so that the project publishes no
  data-producing tool of any kind.
- **Point users to an existing tool that produces the format.** Rejected: that
  would be recommending a non-compliant program, which rule 4 forbids.

## Consequences

**Easier.** The project touches no process but its own. Nothing runs elevated.
Windows and macOS behave the same. The daemon loses its only Windows-specific
module and the dependencies that came with it. Testing the import path needs
only files, so CI can see all of it.

**Harder.** Users must produce a file themselves, and the CSV route takes time.
The project controls none of the community formats. Their fields and values are
facts to verify from samples, not a contract anyone keeps with us. The fact
log's `SnapshotAcquired` provenance was shaped around the reader (a channel, a
probe build), and it must be redesigned for an imported file. That redesign goes
through the proposal in rule 7.

**Not superseded, but emptied.** ADR-0005's trigger 3 (the reader depends on
`yata-protocol`) and ADR-0011's standalone reader download lose their subject.
Their other rules stand, so they are not superseded. A later record that touches
them removes the reader rows.

**Records that change with this one.** `spec/reader-security.md` is deleted. The
MuMu channel leaves `spec/probe-protocol.md`, `probe.proto`, and `fact.proto`.
`spec/feature-scope.md`, `architecture/overview.md`, `project/status.md`,
`project/roadmap.md`, and the front door stop describing a reader.
`spec/import-format.md` is new.
