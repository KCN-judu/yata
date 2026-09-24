/// The daemon as the views see it: its status, the session it is in, the revision the state
/// holds, the warnings it sent, and the one error type every provider fails with.
///
/// This file is the state layer's boundary with `daemon/`: it maps the client's health and
/// exceptions into values the views can show, so no view imports the daemon client (ADR-0012).
library;

import 'dart:async';
import 'dart:typed_data';

import 'package:fixnum/fixnum.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../daemon/daemon_client.dart';
import '../daemon/daemon_error.dart';
import '../daemon/transport.dart';
import '../platform/io_platform_services.dart';
import '../platform/platform_services.dart';

/// A failure as the views show it: by [code], with [message] only in the detail view.
final class CoreError implements Exception {
  const CoreError(this.code, this.message, this.details);

  /// A failure in the client itself: a bug, not a daemon answer.
  static const unexpected = 'client.unexpected';

  factory CoreError.of(Object error) => switch (error) {
    CoreError e => e,
    DaemonException e => CoreError(e.code, e.message, e.details),
    _ => CoreError(unexpected, error.toString(), Uint8List(0)),
  };

  final String code;
  final String message;
  final Uint8List details;

  @override
  String toString() => 'CoreError($code: $message)';
}

/// Run a daemon call, failing with a [CoreError] whatever went wrong.
Future<T> coreCall<T>(Future<T> call) async {
  try {
    return await call;
  } on Object catch (e) {
    throw CoreError.of(e);
  }
}

final platformServicesProvider = Provider<PlatformServices>((ref) => const IoPlatformServices());

/// The daemon serves its development fixture rather than the store.
final fixtureModeProvider = Provider<bool>(
  (ref) => ref.watch(platformServicesProvider).serveFixture,
);

/// Start the daemon the platform locates, serving the fixture when asked to.
Future<DaemonTransport> launchDaemon(PlatformServices platform) async {
  final location = platform.locateDaemon();
  final path = location.path;
  if (path == null) {
    throw DaemonException(
      ClientErrorCode.daemonNotFound,
      'no yata-daemon executable; looked in: ${location.searched.join(', ')}',
    );
  }
  return ProcessTransport.start(path, ['serve', if (platform.serveFixture) '--fixture']);
}

final daemonClientProvider = Provider<DaemonClient>((ref) {
  final platform = ref.watch(platformServicesProvider);
  final daemon = SupervisedDaemon(() => launchDaemon(platform));
  ref.onDispose(daemon.shutdown);
  return daemon;
});

/// What the status bar and the banners show.
sealed class CoreStatus {
  const CoreStatus();
}

final class CoreStopped extends CoreStatus {
  const CoreStopped();
}

/// Starting; [cause] is set when this start replaces a daemon that exited.
final class CoreStarting extends CoreStatus {
  const CoreStarting({this.cause});

  final CoreError? cause;
}

final class CoreReady extends CoreStatus {
  const CoreReady({required this.version, required this.session, required this.restarted});

  /// `major.minor` of the daemon's protocol.
  final String version;
  final int session;

  /// The core exited and this session replaced it.
  final bool restarted;
}

final class CoreFailed extends CoreStatus {
  const CoreFailed(this.error, this.log);

  final CoreError error;

  /// The daemon's last stderr lines, for the detail view.
  final List<String> log;
}

class CoreStatusNotifier extends Notifier<CoreStatus> {
  @override
  CoreStatus build() {
    final client = ref.watch(daemonClientProvider);
    final sub = client.healthChanges.listen((h) => state = _status(client, h));
    ref.onDispose(sub.cancel);
    if (client.health is DaemonStopped) unawaited(Future.microtask(client.start));
    return _status(client, client.health);
  }

  static CoreStatus _status(DaemonClient client, DaemonHealth h) => switch (h) {
    DaemonStopped() => const CoreStopped(),
    DaemonStarting(:final after) => CoreStarting(cause: after == null ? null : CoreError.of(after)),
    DaemonReady(:final daemonVersion, :final session, :final restarted) => CoreReady(
      version: '${daemonVersion.major}.${daemonVersion.minor}',
      session: session,
      restarted: restarted,
    ),
    DaemonFailed(:final error) => CoreFailed(CoreError.of(error), client.recentLog),
  };

  Future<void> restart() => ref.read(daemonClientProvider).restart();
}

final coreStatusProvider = NotifierProvider<CoreStatusNotifier, CoreStatus>(CoreStatusNotifier.new);

/// The number of the open session; it changes only when a new session opens, so the data a
/// view shows stays while the core restarts, and every query refetches once it is back.
class SessionEpoch extends Notifier<int> {
  @override
  int build() {
    ref.listen(coreStatusProvider, (_, s) {
      if (s is CoreReady && s.session != state) state = s.session;
    });
    final s = ref.read(coreStatusProvider);
    return s is CoreReady ? s.session : 0;
  }
}

final sessionEpochProvider = NotifierProvider<SessionEpoch, int>(SessionEpoch.new);

/// The latest projection revision the state layer holds (ADR-0012, "Revision-driven
/// invalidation"). Query providers compare their page's revision against it and refetch when
/// theirs is older; nothing patches a cached value from an event.
class ProjectionRevision extends Notifier<Int64> {
  @override
  Int64 build() {
    ref.watch(sessionEpochProvider);
    final client = ref.watch(daemonClientProvider);
    final sub = client.events.listen((e) {
      if (e.hasProjectionChanged() && e.projectionChanged.revision > state) {
        state = e.projectionChanged.revision;
      }
    });
    ref.onDispose(sub.cancel);
    final h = client.health;
    return h is DaemonReady ? h.revision : Int64.ZERO;
  }

  /// A response showed a revision; the state holds the highest seen.
  void observe(Int64 revision) {
    if (revision > state) state = revision;
  }
}

final projectionRevisionProvider = NotifierProvider<ProjectionRevision, Int64>(
  ProjectionRevision.new,
);

/// A warning the daemon sent: something to show that changed no state.
final class CoreWarning {
  const CoreWarning(this.code, this.message);

  final String code;
  final String message;
}

class CoreWarnings extends Notifier<List<CoreWarning>> {
  @override
  List<CoreWarning> build() {
    final client = ref.watch(daemonClientProvider);
    final sub = client.events.listen((e) {
      if (e.hasWarning()) state = [...state, CoreWarning(e.warning.code, e.warning.message)];
    });
    ref.onDispose(sub.cancel);
    return const [];
  }

  void dismiss(CoreWarning w) => state = [
    for (final x in state)
      if (!identical(x, w)) x,
  ];
}

final coreWarningsProvider = NotifierProvider<CoreWarnings, List<CoreWarning>>(CoreWarnings.new);
