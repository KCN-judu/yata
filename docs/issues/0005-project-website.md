---
id: ISS-0005
state: deferred
area: tooling
opened: 2026-09-23
resolved-by: []
related: [ADR-0011, ADR-0007, ADR-0016, ISS-0003]
---

# ISS-0005: The project website: what it carries is set, how it is built and hosted is not

## Problem

The maintainer expects the project to need a repository website later. Its
purpose, pages, visual direction, host, and repository are now set (see
Maintainer direction). When it can go live, how it is built, and where the user
guide lives are not.

## Why it matters

Several existing decisions need a public, trusted place:

- **Release feed.** ADR-0011 requires a publicly readable feed and leaves its
  location open. A website and the feed may share a host, or may not.
- **Release public key.** ADR-0011 says losing the release key means shipping a
  new public key "through some channel users trust". A website that publishes
  the key fingerprint is one candidate for that channel.
- **Reader transparency.** ADR-0007 makes the reader open source so that anyone
  can confirm it does not harm their system. Users need a place that links the
  reader's source, its `reader-security.md` guarantees, and the matching
  download.
- **First install without OS signing.** Until ISS-0003 is settled, users meet
  SmartScreen and Gatekeeper warnings. A download page is where that is
  explained.

The main repository is public from its creation (ADR-0016), which settles where
the site is published from.

## Maintainer direction

**2026-09-23.** The site's first job is a **concept showcase**: a product page
that presents what the application is. From the showcase, visitors can **go to
the download** and **go to the docs**; both are entry points the page must
carry.

**"The docs" means the user guide** (maintainer, 2026-09-23): how players
install and use the application. It is not the engineering records under
`docs/`, which stay English and internal. The user guide is written in Chinese,
since the application is Chinese-only (ADR-0012).

**The download entry goes to the site's own download page** (maintainer,
2026-09-23), not straight to the public release location. The page links to the
files, which stay hosted at the release location (ADR-0011), so moving that
location changes only the page's links. It carries:

- one download per artifact: the Windows installer, the macOS `.dmg`, the
  standalone reader zip (ADR-0011), with guidance on which one a visitor needs
- the current version and its release notes
- what the first install looks like while OS signing is deferred (ISS-0003): the
  SmartScreen and Gatekeeper warnings and how to proceed
- a link to the open-source reader and its security guarantees (ADR-0007,
  `spec/reader-security.md`)

**The site is deployed as a GitHub Pages static site** (maintainer, 2026-09-23),
which matches the choice of GitHub and Actions for the code. Consequences for
the design:

- The site is static: no server code. Anything dynamic, such as reading the
  current version, is done at build time or from a public static file.
- **The main repository is public from its creation** (ADR-0016), so the site is
  published by Pages from the main repository. GitHub Free allows Pages only
  from public repositories, so the site cannot go live before the repository is
  public, short of a paid plan. The reader stays in its own repository
  (ADR-0007).
- Release files are not stored in the Pages site. Pages limits site size and
  bandwidth; the download page links to the release location (ADR-0011), as
  decided above.

Visual direction, as the maintainer stated it:

- **Layered paper-cut animation**: several cut-paper layers stacked in depth and
  moving relative to each other
- **References**: Japanese (和风) and Chinese (国风) design
- **Emphasis**: gilding (鎏金) and similar classical accents mark what matters
- **Texture**: surfaces carry matching material texture, not flat fills

As with the application (ADR-0012), the **UI agent owns the site's information
architecture, interaction, visual design, and wording**. The engineering side
(host, build, CI, feed) is decided here. The direction above is the maintainer's
brief to the UI agent, not a design.

## Open questions

- Where the user guide lives: on the same site, or elsewhere; how it is authored
  and built
- When the main repository goes public relative to the first public release. If
  it is public first, the site and the release feed can both live in it at no
  cost; if not, an interim host is needed for whichever of them ships before it
- Whether `github.io`, or a custom domain in front of it, is reliably reachable
  for the intended players; to be checked before launch, not assumed
- Where the site's source lives inside the main repository (a top-level
  directory of its own)
- Build: a static site generator, or plain pages; published to Pages by a GitHub
  Actions job
- Language of the showcase page: Chinese only, like the application and the user
  guide, or also another language
- Domain name: the product is named Yata (ADR-0015); whether a custom domain is
  used at all is open

## Current evidence

None. No release and no public repository exist yet; the product name is Yata
(ADR-0015).

## Dependencies

The release-feed location (ADR-0011). The product name and `<app-id>` are
settled (ADR-0015).

## Resolution

Deferred on 2026-09-23 by the maintainer's note that it is needed "later". It
should be settled no later than the release-feed location, before the first
public release.
