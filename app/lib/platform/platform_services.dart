/// What differs between Windows and macOS, and what touches the machine outside the daemon:
/// where the daemon executable is, the development switches in the environment, and the
/// system's file picker. Views and state ask this interface; only
/// `io_platform_services.dart` knows which platform it is on.
library;

import 'dart:typed_data';

/// Where the daemon executable was found, or every place that was searched.
final class DaemonLocation {
  const DaemonLocation({required this.path, required this.searched});

  /// The executable, or `null` when none was found.
  final String? path;

  /// The candidate paths, in the order they were tried.
  final List<String> searched;
}

abstract interface class PlatformServices {
  DaemonLocation locateDaemon();

  /// Serve the daemon's development fixture instead of the store: `YATA_FIXTURE=1` in the
  /// environment, or `--dart-define=YATA_FIXTURE=true`.
  bool get serveFixture;

  /// The file for an icon in the local pack of ADR-0017 (`<kind>/<role>/<id>.png` under the
  /// folder `YATA_LOCAL_ICONS` names), if the variable is set and the file exists.
  String? localIconPath(String kind, String role, int id);

  /// A PNG image the user picks, as bytes; `null` when the user cancels.
  Future<Uint8List?> pickPngImage();
}
