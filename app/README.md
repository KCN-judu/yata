# Yata application

The Flutter desktop application. Its architecture is
[ADR-0012](../docs/decisions/0012-flutter-app-architecture.md): `lib/ui/` →
`lib/state/` → `lib/daemon/`, with the generated protocol types in
`lib/gen/proto/` and the Chinese text in `lib/l10n/app_zh.arb`.

Run it against the daemon's development fixture:

```text
cargo build -p yata-daemon
cd app
flutter run -d windows --dart-define=YATA_FIXTURE=true
```

A debug build finds the daemon in the working copy's `target/`; `YATA_DAEMON`
names another executable. Regenerate the protocol bindings with `just proto`
after changing `core.proto`.
