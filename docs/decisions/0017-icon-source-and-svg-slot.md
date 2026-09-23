---
id: ADR-0017
status: accepted
date: 2026-09-24
area: ui
supersedes: []
superseded-by: []
related: [ADR-0010, ADR-0011, ADR-0012, ADR-0016, ISS-0006, PRP-0002]
---

# ADR-0017: Icons are keyed by game id and resolved from a project SVG, then a local pack, then a text mark

## Status

Accepted, 2026-09-24.

## Context

ISS-0006 asked where soul-set and Shikigami icons come from. The maintainer has
answered in two steps:

- **In the end, soul icons are project-owned SVG.** The maintainer produces them
  in a separate line of work that turns an illustration into a layered,
  flat-colour SVG, and hands them over. That work also produces a small cropped
  emblem for each icon.
- **Until then, development uses the game's official icons.** These are a large
  portrait and a small emblem per soul set, kept in the maintainer's local
  research folder. They are NetEase's artwork. The repositories are public
  (ADR-0016), so these icons can be used on a developer's machine but never
  committed, packaged, or shipped.

The application therefore needs one way to show an icon that works with neither
source present, with the local raster icons, and later with project SVG.
Replacing an icon should need no code change.

## Decision

### Identity

1. **An icon is identified by an `IconKey`: a kind and the game's identifier.**
   For `SoulSet`, the identifier is the suit code (`spec/scheme-code.md`). For
   `Shikigami`, it is the game's Shikigami id. An icon is never looked up by
   name. Names change with translations and client versions; identifiers do not.
2. **Each key has two roles:**
   - `emblem`: the small mark used in tables, lists, filters, and anywhere an
     icon sits beside text
   - `portrait`: the large artwork used in the inspector and in detail views

   A role is a use, not a pixel size. The UI scales whatever it gets, so an SVG
   fills both roles at any size.

### Resolution

3. **An icon resolves in this order**, per key and role:
   1. a **project SVG**, committed under `app/assets/icons/`
   2. a **local raster** from the local icon pack, if one is configured
   3. the **designed text mark**: the set's or Shikigami's first character on a
      neutral tile

   The first one present wins. Adding a project SVG for a key replaces the local
   raster for that key with no code change. A key with neither falls back to the
   text mark, so a fresh checkout runs with no icons at all.

4. **Missing icons are normal.** The text mark is a designed state (PRP-0002),
   not an error. Only a file that is present and fails to decode is reported, as
   a diagnostic in the developer log, and the text mark is shown in its place.

### The SVG slot

5. **Project icons are static SVG in a fixed profile:** a square `viewBox`;
   filled paths and groups only; no `<text>`, `<script>`, `<foreignObject>`,
   `<image>`, or external reference; no CSS or SMIL animation; no filters. The
   layered paper-cut output fits this profile as it is, since every mark is a
   filled path. The profile keeps rendering predictable in Flutter and makes an
   icon file safe to open.
6. **Layout:** `app/assets/icons/<kind>/<role>/<id>.svg`, with `<kind>` one of
   `soul-set` or `shikigami`. A generated raster copy is added only if a
   measured rendering cost requires it.
7. **A preflight check tests every committed icon** against the profile and the
   layout.

### The local pack

8. **The local pack is a directory outside the repository's tracked files**,
   with the same layout as rule 6 and PNG files in place of SVG
   (`<kind>/<role>/<id>.png`). The application reads it only when the
   environment variable `YATA_LOCAL_ICONS` names it. Without the variable, it
   reads nothing.
9. **The conventional development location is `local-assets/icons/`** at the
   repository root. `.gitignore` excludes `local-assets/`. The pack is prepared
   on the developer's machine from local research and is never downloaded by the
   application.
10. **Nothing from the local pack is shipped.** The release packaging job fails
    if an artifact contains a file from `local-assets/` or any PNG under an icon
    path (ADR-0011).
11. **Icons are presentation.** The Flutter application resolves them. The
    daemon does not serve icon files, and the local pack is not in the data
    directory (ADR-0010). The daemon supplies only the identifiers: suit codes
    and Shikigami ids.

## Alternatives

- **Bundle the official icons with development builds as Flutter assets.**
  Rejected: a bundled asset is one build flag away from a release artifact, and
  the repository is public.
- **Key icons by name.** Rejected: names differ between clients and versions,
  and the soul model already keys sets by suit code.
- **Serve icons from the daemon.** Rejected: icons carry no domain meaning, and
  routing presentation assets through the protocol adds a path that exists only
  to serve the UI, which `architecture/overview.md` forbids.
- **Raster only, with SVG converted at build time.** Rejected: the final icons
  are vector, and rendering them as vector keeps them sharp at every scale on
  high-DPI displays.

## Consequences

**Easier.** The UI can be built and tested with no icons. Delivering the final
soul icons means adding files, not changing code. Shikigami icons from any later
source use the same mechanism.

**Harder.**

- The Flutter application needs an SVG renderer, a dependency decided with the
  app scaffolding.
- Preparing the local pack needs the suit code of each official icon. The
  mapping from set name to suit code is in local research, so the pack cannot be
  prepared from the public repository alone. That is intended.
- The release check in rule 10 has to exist before the first packaged build.

ISS-0006 is resolved by this record. What remains open is not the mechanism but
the source of Shikigami icons and the delivery of the soul SVGs, both of which
are content.
