---
kind: spec
status: current
area: domain
---

# Feature scope

What this application does and does not do. This page is the authoritative
ledger for inclusion and exclusion decisions. When a feature is cut, the reason
is recorded here so the decision is not relitigated. When a feature is added
back, the old row is updated rather than deleted — the reversal is the note.

Everything on this page is _designed_ or _accepted_. Nothing is _implemented_ —
see `../project/status.md`.

## In scope

### Two-pass soul scoring

The application scores every soul in the user's inventory using two passes
(ADR-0003):

- **Pass 1 (quality):** target-independent. Answers whether a soul is good in
  itself, with no holder in mind. Produces a `QualityScore` with depth, breadth,
  slot-fit, and roll-headroom. See `scoring.md`.
- **Pass 2 (affinity/matching):** target-relative. Given a set of Shikigami need
  profiles, produces per-soul affinity scores, candidate loadout assignments,
  and the delta against what the account currently has equipped. The inversion
  list — souls with low quality but high affinity for a specific need — is a
  first-class output and the feature a single-pass model cannot produce.

### Scheme code import and export

The application can produce a scheme code from any strengthening scheme or
discard scheme the user authors, and can ingest scheme codes published by
streamers or exported by other accounts.

Encoding: the game's official format only; there is no project-defined format
(ADR-0009). The game presents a scheme code as a QR code, so import reads a QR
image as well as text, and export produces one. The model and codec rules are
[scheme-code.md](scheme-code.md). Design: ADR-0009.

### Soul import via two read channels

Souls are read from the running game via exactly two channels (probe-boundary
decision, recorded in `architecture/overview.md`):

1. **Desktop memory** — `yata-reader` reads the game's memory from outside the
   process, without injecting code (ADR-0007). Authoritative and fast; requires
   the game to be running on the same machine.
2. **MuMu over ADB** — reads the emulated game's memory through
   `/proc/<pid>/mem` under a temporary adbd root, without injecting code
   (ADR-0007). Covers the case where the user plays on emulator rather than (or
   in addition to) the PC client.

A third channel is an ADR decision, not a feature addition.

Both channels run on Windows only. The application also runs on macOS, where the
inventory comes from an **export file**: the probe writes its reading as JSON,
and the application imports it on either platform (ADR-0008). The same file is a
portable backup and a documented format other tools can read.

### GameProfile — multi-profile support

The application supports multiple game accounts (`GameProfile`). Every fact in
the store carries a `ProfileId` dimension; scoring, inventory, and scheme codes
are always profile-scoped. The user can switch between profiles without data
loss and can run comparisons across profiles.

### Item / GameAsset import

The probe reads the user's item inventory alongside souls. The 21-category item
list from the legacy application is the starting point; the authoritative
category list must be sourced from the game or EN wiki before this feature is
finalized (see `glossary.md`). Tracked as `GameAsset` (the union of items and
realm cards the probe can read).

### Shikigami collection (式神录)

The application shows the Shikigami the account owns — level, star, evolution —
and the six souls each one has equipped. It is a place of its own and the usual
way into matching, because matching answers "how much better than what this
Shikigami wears now". Added 2026-09-23 (PRP-0002).

### Guild view (寮) — view only

The application shows the guild the account belongs to and its member roster, as
the reader reads it (ADR-0007): other players' names, online state, and
contribution, kept on this machine. The view is read-only; exporting the roster
is deferred (below). Added 2026-09-23 (PRP-0002).

### Head/tail analysis

Speed-head detection: a soul on Slot 2 with SPD as its main attribute is a speed
head. The scoring pass's `slot_fit` field surfaces this directly. The feature
answers which souls in the inventory are speed heads and how they rank by
quality and affinity for speed- dependent needs.

### Discard and strengthen recommendations

The application produces a list of souls to discard (exchange at the Altar) and
a list to strengthen, both expressed as `SoulSelection` values (ADR-0009). These
can be exported as scheme codes (QR codes), imported directly into the game's
panel, and shared with other players.

### Automatic updates

The application updates itself: a signed release manifest, download and
verification in the background, apply on exit, and rollback if the new version
does not start ([ADR-0011](../decisions/0011-packaging-and-self-update.md)).

## Cut — will not be built

### Simulated strengthening

Simulating how a soul's sub-attributes would roll through each +3 node, to
estimate its future value. Cut for two reasons:

1. Four-sub-attribute detection (`PristineSoul`) plus pre-filtering by attribute
   type already answers the question users were using simulation for.
2. The game's own strengthening scheme answers it better, inside the game, with
   the actual rolls.

Bringing this back would require a new proposal that explains what the game's
own panel cannot answer.

### CBG integration

The legacy application had a CBG feature for cross-account soul exchange. Cut
because CBG was removed from the game client before this project started. Any
feature involving it would require a platform that no longer exists.

### Leaderboard and community sharing

No server-side component. The application is a local desktop tool. Scheme codes
are the sharing mechanism; there is no upload, ranking, or cloud sync.

## Deferred — out of scope for the initial release, may reconsider

### Derived need profiles from the game's damage model

`NeedProfile`s may be derived from Shikigami damage formulas rather than
authored by hand. Deferred because the damage model needs a separate spec, and
authored profiles are sufficient for the initial release. When this lands, it is
an addition to `scoring.md`, not a new feature page.

### Guild roster export

Exporting the guild member roster to a file. Deferred on 2026-09-23: it hands
other players' data to a file that leaves the application, and the view alone
serves the current need. Adding it back is an update to this row.

## Related

- Why two passes: [ADR-0003](../decisions/0003-two-pass-scoring.md)
- Scheme code design:
  [ADR-0009](../decisions/0009-scheme-code-model-and-format.md)
- Architecture and channel boundaries:
  [../architecture/overview.md](../architecture/overview.md)
- Term definitions: [glossary.md](glossary.md)
- What is currently built: [../project/status.md](../project/status.md)
