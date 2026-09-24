# The application starts, reaches the daemon, and shows its answers

- Date: 2026-09-25
- Area: ui
- Affected: developers
- Related: ADR-0004, ADR-0012, ADR-0017

## What changed

- The core protocol schema, `crates/yata-protocol/proto/core.proto`, gains the
  session on top of the query messages: envelopes, subscriptions, profiles, and
  scheme-code decoding with the QR matrix to draw. `Query` gains `profile_id`
  and `scan_revision`, `QueryRow` the soul's values, and `QueryPage` its
  `revision` and `total`; the headless endpoint now fills `total` too.
- `yata-daemon serve` runs the session on stdin and stdout; its soul pages go
  through the same query engine as `yata-daemon query`.
  `yata-daemon serve --fixture` serves a development fixture of two profiles,
  one with twelve invented souls, until imports reach the store.
- `app/` is the Flutter application: a daemon client that starts, restarts, and
  shuts down the daemon; Riverpod state that refetches when the revision moves;
  Chinese text by error code; and working shells for 御魂库, 方案, 式神录,
  and 设置.
- The Dart bindings are committed and checked against `protoc`; regenerate them
  with `just proto`. New preflight checks: `dart-bindings` and `error-codes`.
- The application uses `flutter_riverpod`, `protobuf`, `flutter_svg` (the SVG
  renderer ADR-0017 left to the scaffolding), and `file_selector` for picking a
  QR image.

## Compatibility and migration

Nothing to migrate. The schema is a draft: until version 1 is current, a tag may
change, and the committed bindings and recordings change with it.

## Evidence

[../../evidence/testing.md](../../evidence/testing.md), § Core protocol session
and § The application.
