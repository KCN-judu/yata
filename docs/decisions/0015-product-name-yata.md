---
id: ADR-0015
status: accepted
date: 2026-09-23
area: process
supersedes: []
superseded-by: []
related:
  [
    ADR-0005,
    ADR-0007,
    ADR-0008,
    ADR-0010,
    ADR-0011,
    ADR-0014,
    ADR-0016,
    ISS-0005,
  ]
---

# ADR-0015: The product is named Yata (八咫镜); identifiers take the `yata` prefix

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published.

## Context

Several identifiers need a product name: the main repository, the reader's
repository and binary (ADR-0008), the crate prefix (ADR-0005), the `<app-id>`
that names the data directory (ADR-0010) and the install directory (ADR-0011),
and the website (ISS-0005). `<app-id>` must be fixed before the first release,
because it names the data directory.

阴阳师 / Onmyoji is the name of NetEase's game. The main repository, the reader,
and the website are all public (ADR-0016, ADR-0007, ISS-0005), and a public
project named after the game invites confusion with an official product and
risks a trademark complaint. The name should come from the tradition the game
draws on, not from the game itself.

A name close to that of an earlier tool for the same game would suggest this is
the same software, which ADR-0014 says it is not.

## Decision

1. **Product name.** Chinese: **八咫镜**. English and ASCII: **Yata**, lowercase
   `yata` in identifiers.
2. **Repositories.** The main repository is `yata`. The reader repository and
   its binary are `yata-reader`.
3. **Crate prefix.** Crates are named `yata-<area>`: `yata-core`,
   `yata-protocol`, and `yata-daemon`. The split rules of ADR-0005 decide when a
   crate exists.
4. **`<app-id>` is `io.github.kcn-judu.yata`.** It names the data directory
   (ADR-0010) and the install directory (ADR-0011), and is the macOS bundle
   identifier. Like any `<app-id>`, it is fixed at the first release and never
   changed afterwards. It assumes the repository is hosted under the `KCN-judu`
   GitHub account.
5. **Derived identifiers use `yata`.** Any other identifier built from the
   product name follows the same prefix: temporary file names such as
   `yata-reader-*`, a URL scheme if one is adopted (`yata://`), and log targets.
6. **Game vocabulary is unaffected.** Domain terms still follow the game
   (`SoulSet`, `Shikigami`, the glossary). This decision names the product, not
   the domain.

## Alternatives

- **式盘 `shikiban`**, the onmyōdō divination board. It fits "weigh and infer"
  well; the maintainer preferred 八咫镜, a mirror that shows the true form of
  what is set before it, which is what the tool does for a player's Souls.
- **六壬 `rikujin`**, the divination method the board serves. Accurate but
  obscure to players.
- **晴明 `seimei`** (Abe no Seimei). The most recognisable, but the game already
  uses it as a character name, so it would read as the game's.
- **A name built on the game's own name.** Rejected for the trademark risk
  above.

## Consequences

**Easier.** The repositories, the crates, the data directory, and the install
directory all have names before any code exists. Public names do not imitate the
game's trademark.

**Harder.** The game has a soul set named 八咫镜. Inside a tool about souls, the
product name and that set share a name, so user-facing text that means the
product says so explicitly, and the set is always named with its set context.
The maintainer keeps the name with this collision known.

After the first release, `<app-id>` is permanent. If the repository ever moves
away from `KCN-judu`, `<app-id>` stays as it is and no longer matches the host.
That is acceptable, because the identifier only has to be unique and stable.
