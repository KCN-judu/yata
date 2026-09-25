/// [PlatformServices] on a desktop operating system. The one file that asks which platform it
/// runs on.
library;

import 'dart:io';

import 'package:file_selector/file_selector.dart';
import 'package:flutter/foundation.dart';

import 'platform_services.dart';

class IoPlatformServices implements PlatformServices {
  const IoPlatformServices();

  static String get _daemonFileName => Platform.isWindows ? 'yata-daemon.exe' : 'yata-daemon';

  /// Tried in order: `YATA_DAEMON`; beside the application executable, where a release puts it;
  /// and in a debug build, the Cargo output of the working copy the app is run from.
  @override
  DaemonLocation locateDaemon() {
    final candidates = <String>[
      ?Platform.environment['YATA_DAEMON'],
      _join(File(Platform.resolvedExecutable).parent.path, _daemonFileName),
      if (!kReleaseMode) ..._workingCopyBuilds(),
    ];
    for (final c in candidates) {
      if (File(c).existsSync()) return DaemonFound(c, searched: candidates);
    }
    return DaemonNotFound(searched: candidates);
  }

  /// `target/debug` and `target/release` in the current directory and its ancestors.
  Iterable<String> _workingCopyBuilds() sync* {
    Directory? dir = Directory.current;
    for (var depth = 0; depth < 4 && dir != null; depth++) {
      for (final profile in ['debug', 'release']) {
        yield _join(_join(_join(dir.path, 'target'), profile), _daemonFileName);
      }
      final parent = dir.parent;
      dir = parent.path == dir.path ? null : parent;
    }
  }

  static String _join(String dir, String name) => '$dir${Platform.pathSeparator}$name';

  @override
  bool get serveFixture =>
      const bool.fromEnvironment('YATA_FIXTURE') || Platform.environment['YATA_FIXTURE'] == '1';

  @override
  String? localIconPath(IconKind kind, IconRole role, int id) {
    final pack = Platform.environment['YATA_LOCAL_ICONS'];
    if (pack == null || pack.isEmpty) return null;
    final path = [pack, kind.folder, role.folder, '$id.png'].join(Platform.pathSeparator);
    return File(path).existsSync() ? path : null;
  }

  @override
  Future<Uint8List?> pickPngImage() async {
    const png = XTypeGroup(
      label: 'PNG',
      extensions: ['png'],
      uniformTypeIdentifiers: ['public.png'],
    );
    final file = await openFile(acceptedTypeGroups: [png]);
    return file?.readAsBytes();
  }
}
