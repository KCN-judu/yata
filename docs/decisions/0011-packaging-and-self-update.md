---
id: ADR-0011
status: accepted
date: 2026-09-23
area: tooling
supersedes: []
superseded-by: []
related: [ADR-0004, ADR-0006, ADR-0008, ADR-0010, ADR-0015, ADR-0016, ISS-0003]
---

# ADR-0011: Per-user installs, a standalone reader download, and signed automatic updates

## Status

Accepted, 2026-09-23, as one of the initial decisions recorded before the
repository was published. It answers ISS-0003's packaging and update questions.

## Context

A Windows installation carries three programs: the Flutter application,
`yata-daemon`, and the reader `yata-reader`. A macOS installation carries the
first two only. The reader must also be downloadable on its own, because a Mac
user runs it on a Windows machine to produce an export file (ADR-0008).

The maintainer chose:

- a per-user installer on Windows
- no OS code signing and no Apple notarization for now; both are deferred until
  before the first public release, and ISS-0003 stays open for them
- full automatic updates

Automatic updates without OS code signing need their own proof that a download
is genuine. Without it, anyone who can tamper with the download or the release
feed can put code on every user's machine, and this application's reader runs
with elevation when the game does (ADR-0006).

## Decision

### Packages

| Artifact    | Platform    | Form                      |
| ----------- | ----------- | ------------------------- |
| application | Windows x64 | per-user installer        |
| application | macOS arm64 | `.app` bundle in a `.dmg` |
| reader      | Windows x64 | standalone `.zip`         |

The Windows installer puts the application in
`%LOCALAPPDATA%\Programs\io.github.kcn-judu.yata\` (the app-id, ADR-0015), needs
no administrator rights to install, and adds a Start-menu entry and an
uninstaller. The standalone reader `.zip` holds the same reader binaries the
installer carries.

Installer tooling (Inno Setup or similar) is an implementation choice. The
per-user location is not: it is what lets the updater replace files without
elevation.

### Release signing, independent of OS code signing

Every release publishes a **manifest**: version, platform, and for each file its
path, size, and SHA-256, plus the minimum store format and protocol versions the
release reads. The manifest is signed with an Ed25519 release key. The key is
held only in CI secrets. Its public half is compiled into the daemon.

An update is applied only if:

1. the manifest's signature verifies against the compiled-in public key
2. every downloaded file matches its manifest hash
3. the manifest's version is newer than the running one. Downgrades are refused.

This protects updates whether or not the binaries also carry an OS code
signature, and it stays in place after OS signing is added.

### Update flow

1. **Check.** The daemon fetches the manifest from the release feed as a job
   (ADR-0004's job category) and reports whether an update exists. It never
   downloads without the user's consent unless the user has turned automatic
   download on.
2. **Download and check.** A job downloads into `temp/` of the data directory
   (ADR-0010) and checks it as above. Nothing in the installation is touched
   yet.
3. **Apply on exit.** When the user accepts, the application shuts down. The
   daemon, started from a copy of itself in `temp/`, moves the current
   installation aside, moves the new one into place, and starts the application.
   A running executable is never overwritten.
4. **Keep the previous version until the new one starts.** If the new version
   does not complete a daemon start, the next launch restores the previous
   installation. After one successful start the previous version is deleted.
5. **Store format upgrades back up first**, as ADR-0010 requires. An update that
   raises the store format therefore always leaves a restorable backup.

There is no separate updater binary. Applying an update is a subcommand of
`yata-daemon` (the one-binary rule of ADR-0004), run from a temporary copy.

The reader is updated with the application on Windows. The standalone reader
download is not self-updating. A reader and a daemon of different versions are
handled by the probe protocol's version rules, as any mismatch is.

### The release feed

The feed must be publicly readable, because the updater runs without
credentials. The main repository is public (ADR-0016), so its own GitHub
Releases are a candidate feed. Where releases are published is decided before
the first release.

## Alternatives

- **Notify only, no automatic install.** Offered and not chosen by the
  maintainer.
- **Rely on OS code signing to authenticate updates.** Not available while
  signing is deferred, and it would still leave the feed itself unauthenticated.
- **A separate updater executable.** Rejected: a second binary that shares
  release logic with the daemon drifts, and ADR-0004 rejects that pattern for
  the CLI.
- **Framework updaters** (Sparkle on macOS, WinSparkle on Windows). Not rejected
  as implementation. If adopted, they must enforce the manifest rules above, and
  the manifest is the contract either way.

## Consequences

**Easier.** Users get updates without reinstalling. A failed update rolls itself
back. Tampered downloads are refused even while the binaries are unsigned.

**Harder.** The release key is a critical secret. Losing it means shipping a
daemon with a new public key through some channel users trust. Leaking it lets
anyone sign updates. Rotation must be designed before the first release. CI
gains a release job per platform that builds, hashes, signs, and publishes.
Without OS signing, the first install still meets SmartScreen and Gatekeeper
warnings; only updates avoid them.

**Open before the first release:**

- where releases are published: the main repository's own GitHub Releases, a
  separate public repository, or another host
- how the release key is rotated
- OS code signing and notarization (ISS-0003)

`spec/feature-scope.md` lists automated update delivery as in scope under this
record, and `spec/reader-security.md` relies on its release manifest.
