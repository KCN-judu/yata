---
id: PRP-0008
status: draft
date: 2026-09-25
area: import
related-issues: []
superseded-by: []
---

# PRP-0008: Import community snapshot files

## Problem

ADR-0030 removes the reader. The inventory now comes only from a file the user
imports, in a community snapshot format (`spec/import-format.md`). The import
path still has the reader's shape. A reading is a probe `Reading` with per-field
mapping evidence, and `SnapshotAcquired` records a channel, a probe build id,
and a probe protocol version (`spec/fact-format.md`). None of these describe a
file the user supplies.

## Goals and non-goals

- Goal: the daemon recognizes a file's format from its header and parses it at
  the import boundary, refusing what it cannot parse.
- Goal: more than one community format can be accepted. `mumu-snapshot-v1` is
  the first, and adding one is a new parsing module, not a change to the rest.
- Goal: `SnapshotAcquired` records an imported file's provenance, including its
  `FormatTag`. It no longer records a reader's.
- Goal: the probe schema keeps only what data reception still needs, and is
  deleted if that is nothing.
- Non-goal: producing a file. The project ships no converter and names no tool
  (ADR-0030, rules 4 and 5).

## Proposed design

The maintainer set its shape on 2026-09-25. The types and signatures are written
when the importer is implemented, types first (`style/modelling.md`).

- **The recognized formats are a closed sum type, `FormatTag`.** One case per
  section of `spec/import-format.md`.
- **Detection reads the header only.** A file that matches no format, or more
  than one, is refused. Detection never guesses.
- **One parsing module per format.** Every module produces the same
  format-independent `Snapshot`, which is then turned into `soul::Soul` values.
  Nothing after the parser knows which format a soul came from.
- **The provenance fact records the `FormatTag`.**

```text
detect  : header -> Result<FormatTag, ImportError>        -- none or several: refused
parse   : FormatTag -> bytes -> Result<Snapshot, ImportError>
souls   : Snapshot -> Result<[soul::Soul], ImportError>
```

## Compatibility and migration

The fact schema changes. No store exists outside development machines, so the
migration question is whether to keep a reader-shaped `SnapshotAcquired`
readable at all, or to refuse it with a format-version bump.

## Alternatives

- Convert a community file into a probe `ProbeExport` and keep the existing
  path. This costs the least code, but a file the user wrote would claim a
  reader's provenance. That is a second encoding of one meaning.
- Guess the format from the body when the header is ambiguous. Rejected by the
  maintainer: a guess can turn a file of one format into wrong souls of another.

## Implementation and evidence

Parsing is implemented: `FormatTag` and the format-independent souls in
`yata-core::import::snapshot`, header detection and the `mumu-snapshot-v1`
module in `yata-daemon::import`, and `yata-daemon import check`. Tests:
`import::snapshot::tests`, `yata-daemon` `import::tests`. The provenance fact
and the import job into the store are not. The shape of `mumu-snapshot-v1` comes
from a sample file the maintainer supplied (`spec/import-format.md`,
"Evidence").

## Open questions

The maintainer's sample answers these. The answers are written into the importer
and its tests when it is implemented:

- Positions are numbered from 1.
- The strengthening count of each sub-attribute is `enhancementCount`, 0–5.
- The innate attribute is an entry of `subAttributes` with
  `fixedAttribute: true`, once on each boss-set soul.
- A set is identified by its Chinese name, `setId`, which maps to a suit code.

The maintainer decided on 2026-09-25, and `spec/import-format.md` ("Import")
states it:

- Every sub-attribute's strengthening count is required; a soul without one is
  left out.
- A file whose `completeness` is not `"complete"` is refused.
- The innate attribute's value is not kept.

These stay open:

- What may `initialSubstatCount` and `equippedState` hold? They are null
  throughout the sample, and the importer does not read them.
- The provenance fact: what `SnapshotAcquired` records for an imported file, and
  what becomes of the reader-shaped fields (`channel`, `source`, the probe build
  and version).

## Outcome

Open.
