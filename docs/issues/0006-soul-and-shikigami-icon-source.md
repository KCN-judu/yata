---
id: ISS-0006
state: resolved
area: ui
opened: 2026-09-23
resolved-by: [ADR-0017]
related: [ADR-0011, ADR-0007, ADR-0014, ADR-0016, PRP-0002]
---

# ISS-0006: Where soul and Shikigami icons come from, and how they are managed

## Problem

The interface needs an icon for every soul set and every Shikigami. The icons
are the game's artwork. The maintainer has narrowed the options to two sources:

- **NetEase's public official resources**, managed as an asset pack in a
  separate directory of the main repository and downloaded by the application
- **extraction from the user's own game client**, generating the pack locally
  with nothing redistributed

Which one is viable depends on what the official sources actually offer.

## Why it matters

- **Legal.** Redistributing game artwork is a copyright exposure. Shipping it
  inside the open-source reader repository is ruled out (ADR-0007, ADR-0014).
- **Layout.** Icons are identity marks in every table and inspector (PRP-0002).
  A missing icon needs a designed fallback, not a blank.

## Current evidence

Investigated on 2026-09-23.

**Shikigami: an official source exists.**

- `https://yys.res.netease.com/pc/zt/20161108171335/js/app/all_shishen.json`
  lists 275 Shikigami, each with an official `id` and `level` (rarity).
- `https://yys.res.netease.com/pc/zt/20161108171335/data/shishen/<id>.png`
  serves a 120 × 120 image per Shikigami. It is the same thumbnail the
  official 式神录 page uses. The sampled file is 1.8 MB, possibly animated.
- Whether this official `id` equals the `heroId` the reader sees in memory is
  not established. It must be checked against a recording.

**Souls: no public official source found.**

- The official site has no soul catalogue page with icons. `/skill/yuhun/` is a
  list of strategy articles.
- The 藏宝阁 site (`yys.cbg.163.com`) is NetEase's and shows souls on item
  pages. Its loaded bundles (`dist/main.js`, config) contain no soul-icon URL
  pattern, and guessed paths return 404. Item-page bundles loaded per route were
  not examined.
- No public source of soul icons with a license that allows redistribution is
  known.

## Candidate answers

1. **Mixed.**
   - Shikigami from the official resources, as a downloaded pack.
   - Souls extracted from the user's client, or shown with a designed text mark
     (the set's first character) until a source is found.
2. **Local extraction for both.**
   - Uniform, and nothing is redistributed.
   - Needs the game's resource-package format to be parsed. Windows only; macOS
     imports a pack generated on Windows, like the export file (ADR-0008).
3. **Keep searching** the 藏宝阁 item pages for an official soul-icon path
   before deciding.

Whatever the source, the pack keys icons by game identifier (suit code,
Shikigami id), never by name. It records each file's origin. It is verified like
an update (ADR-0011). A designed text mark stands in for any missing icon.

## Maintainer direction

**2026-09-23.** Soul icons become project-owned artwork. The maintainer produces
redrawn icons through a separate line of work (a computational paper-cut
pipeline and a model the maintainer trains, outside this repository) and hands
the finished icons over. They are then committed as source SVG with generated
app assets, like the project's own glyphs, and this issue resolves.

Until then, development uses the game's official small (emblem) soul icons:

- They are NetEase's artwork. They live only in a local, untracked folder on the
  developer's machine and are never committed, packaged, or shipped; this
  repository is public (ADR-0016).
- The application reads icons from a directory that may be empty. A missing icon
  shows the designed text mark, so a checkout without the local folder still
  runs.
- The folder's path is fixed with the application scaffolding, and `.gitignore`
  gains its entry in the same change.

## Dependencies

A reader recording, to confirm the Shikigami id mapping. For local extraction, a
study of the client's resource format. For the final soul icons, the
maintainer's redrawn set.

## Resolution

Resolved 2026-09-24 by
[ADR-0017](../decisions/0017-icon-source-and-svg-slot.md). Icons are keyed by
game identifier and have two roles, `emblem` and `portrait`. An icon resolves
from a committed project SVG, then from an untracked local pack of the official
icons, then to the designed text mark. Development uses the official large and
small soul icons through the local pack; the maintainer's redrawn SVG replace
them by adding files. The source of Shikigami icons and the delivery of the soul
SVG remain content work, not an open design question.
