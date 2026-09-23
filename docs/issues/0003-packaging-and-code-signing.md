---
id: ISS-0003
state: deferred
area: tooling
opened: 2026-09-23
resolved-by: []
related: [ADR-0008, ADR-0011]
---

# ISS-0003: How the application is packaged, signed, and updated is undecided

## Problem

There is no decision on the installer or bundle format for Windows and macOS, on
code signing, or on the update channel (already deferred in
`spec/feature-scope.md`).

## Why it matters

The reader opens another process's memory for reading. Unsigned binaries that do
this are likely to be flagged by antivirus software, and macOS will not open an
unsigned, unnotarized app without the user overriding Gatekeeper. Both affect
whether users can run the application at all.

## Current evidence

None yet. No binary exists.

## Dependencies

A buildable application on both platforms. **Deferred on 2026-09-23** by the
maintainer, to be resumed after the data-directory and backup discussion. It
must be settled before the first release, not before the first milestone.

## Resolution

Deferred, with part of it decided. **2026-09-23.** Packaging and updates are
decided by [ADR-0011](../decisions/0011-packaging-and-self-update.md): per-user
installer on Windows, `.dmg` on macOS, a standalone reader download, and
automatic updates authenticated by a signed release manifest. **OS code signing
and Apple notarization remain deferred** until before the first public release,
by the maintainer's choice; this issue stays deferred for them.
