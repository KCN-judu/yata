/// What differs between Windows and macOS, and what touches the machine outside the daemon:
/// where the daemon executable is, the development switches in the environment, the local icon
/// pack, and the system's file picker. Views and state ask this interface; only
/// `io_platform_services.dart` knows which platform it is on.
library;

import 'dart:typed_data';

/// Where the daemon executable is: found at a path, or not found after every candidate.
sealed class DaemonLocation {
  const DaemonLocation({required this.searched});

  /// The candidate paths, in the order they were tried.
  final List<String> searched;
}

final class DaemonFound extends DaemonLocation {
  const DaemonFound(this.path, {required super.searched});

  final String path;
}

final class DaemonNotFound extends DaemonLocation {
  const DaemonNotFound({required super.searched});
}

/// What an icon is of, with the folder name of ADR-0017's layout (`<kind>/<role>/<id>`).
enum IconKind {
  soulSet('soul-set'),
  shikigami('shikigami');

  const IconKind(this.folder);

  final String folder;
}

/// How an icon is used, with its folder name.
enum IconRole {
  /// Beside text: tables, lists, filters.
  emblem('emblem'),

  /// Large: the inspector and detail views.
  portrait('portrait');

  const IconRole(this.folder);

  final String folder;
}

abstract interface class PlatformServices {
  DaemonLocation locateDaemon();

  /// Serve the daemon's development fixture instead of the store: `YATA_FIXTURE=1` in the
  /// environment, or `--dart-define=YATA_FIXTURE=true`.
  bool get serveFixture;

  /// The file for an icon in the local pack of ADR-0017 (`<kind>/<role>/<id>.png` under the
  /// folder `YATA_LOCAL_ICONS` names), if the variable is set and the file exists.
  String? localIconPath(IconKind kind, IconRole role, int id);

  /// A PNG image the user picks, as bytes; `null` when the user cancels.
  Future<Uint8List?> pickPngImage();
}
