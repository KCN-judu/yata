---
id: ISS-0004
state: resolved
area: ui
opened: 2026-09-23
resolved-by: [ADR-0012]
related: [ADR-0004, ADR-0005]
---

# ISS-0004: The Flutter application's internal architecture and who decides it are undecided

## Problem

The Flutter side's state management, package layout, and navigation structure
are not decided. The UI/UX design is being handed to a separate agent, and it is
not settled whether that handoff includes these engineering choices or whether
they are decided here.

## Why it matters

Milestone 1 (`project/roadmap.md`) has a basic UI, so some Flutter code is
written early. Whatever is chosen then becomes the default by accident unless it
is decided.

## Current evidence

The constraints already fixed: the Dart side holds no domain model beyond the
generated protobuf types (ADR-0004), and it computes no score and decides no
domain fact (`architecture/overview.md`, layer table). ADR-0005's split triggers
are recommended for the package layout.

## Dependencies

**Deferred on 2026-09-23** by the maintainer, to be resumed after ISS-0003. It
must be settled before milestone 1's UI work starts.

## Resolution

**Resolved 2026-09-23 by
[ADR-0012](../decisions/0012-flutter-app-architecture.md).** Engineering is
decided here; the UI agent owns information architecture, interaction, visual
design, and wording. One package with three layers, Riverpod, revision-driven
invalidation, `gen-l10n` with Chinese only, error text by code. Routing waits
for the UI agent's information architecture.
