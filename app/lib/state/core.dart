/// The daemon as the views see it: its status, the session it is in, the revision the state
/// holds, the warnings it sent, and the one failure type every provider fails with.
///
/// This file is the state layer's boundary with `daemon/`: it maps the client's health into
/// values the views can show, and exposes the failure types, so no view imports the daemon client
/// (ADR-0012).
library;

import 'dart:async';

import 'package:fixnum/fixnum.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../daemon/daemon_client.dart';
import '../daemon/failure.dart';
import '../daemon/transport.dart';
import '../gen/proto/core.pb.dart' as pb;
import '../platform/io_platform_services.dart';
import '../platform/platform_services.dart';

export '../daemon/failure.dart'
    show CoreFailure, RaisedFailure, RequestFailure, SessionFailure, failureOf;

/// A game profile's id as the wire spells it: 32 lowercase hex digits, sent back unmodified.
extension type const ProfileId(String hex) {}

/// A projection revision: the `seq` of the last commit, 0 for the empty log. 0 is a real
/// revision here, not "none"; "not known yet" is a `null` revision.
extension type const Revision(Int64 seq) {
  bool isBefore(Revision other) => seq < other.seq;
}

/// The number of a session since the application started; a new one makes every cached query
/// stale. "No session yet" is a `null` epoch.
extension type const SessionEpoch(int n) {}

/// Run a daemon call, failing with a [CoreFailure] whatever went wrong, including a throw while
/// the call is being made.
Future<T> coreCall<T>(Future<T> Function() call) async {
  try {
    return await call();
  } on Object catch (e) {
    throw failureOf(e);
  }
}

final platformServicesProvider = Provider<PlatformServices>((ref) => const IoPlatformServices());

/// The daemon serves its development fixture rather than the store.
final fixtureModeProvider = Provider<bool>(
  (ref) => ref.watch(platformServicesProvider).serveFixture,
);

/// Start the daemon the platform locates, serving the fixture when asked to.
Future<DaemonTransport> launchDaemon(PlatformServices platform) async =>
    switch (platform.locateDaemon()) {
      DaemonFound(:final path) => ProcessTransport.start(path, [
        'serve',
        if (platform.serveFixture) '--fixture',
      ]),
      DaemonNotFound(:final searched) => throw RaisedFailure.daemonNotFound(searched),
    };

final daemonClientProvider = Provider<DaemonClient>((ref) {
  final platform = ref.watch(platformServicesProvider);
  final daemon = SupervisedDaemon(() => launchDaemon(platform));
  ref.onDispose(daemon.shutdown);
  return daemon;
});

/// Whether the open session reports projection changes.
sealed class ChangeFeed {
  const ChangeFeed();
}

final class ChangesLive extends ChangeFeed {
  const ChangesLive();
}

/// No change notifications: shown data may go stale without the views knowing.
final class ChangesUnavailable extends ChangeFeed {
  const ChangesUnavailable(this.failure);

  final CoreFailure failure;
}

/// What the status bar and the banners show.
sealed class CoreStatus {
  const CoreStatus();
}

final class CoreStopped extends CoreStatus {
  const CoreStopped();
}

final class CoreStarting extends CoreStatus {
  const CoreStarting();
}

/// Starting again after [cause]; the data on screen stays, read-only.
final class CoreRestarting extends CoreStatus {
  const CoreRestarting({required this.attempt, required this.cause});

  final int attempt;
  final CoreFailure cause;
}

final class CoreReady extends CoreStatus {
  const CoreReady({
    required this.version,
    required this.session,
    required this.restarted,
    required this.changes,
  });

  /// `major.minor` of the daemon's protocol.
  final String version;
  final SessionEpoch session;

  /// The core exited and this session replaced it.
  final bool restarted;
  final ChangeFeed changes;
}

final class CoreFailed extends CoreStatus {
  const CoreFailed(this.failure, this.log);

  final CoreFailure failure;

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
    DaemonStarting() => const CoreStarting(),
    DaemonRestarting(:final attempt, :final cause) => CoreRestarting(
      attempt: attempt,
      cause: cause,
    ),
    DaemonReady(:final daemonVersion, :final session, :final restarted, :final subscription) =>
      CoreReady(
        version: '${daemonVersion.major}.${daemonVersion.minor}',
        session: SessionEpoch(session),
        restarted: restarted,
        changes: switch (subscription) {
          SubscriptionLive() => const ChangesLive(),
          SubscriptionFailed(:final failure) => ChangesUnavailable(failure),
        },
      ),
    DaemonFailed(:final failure) => CoreFailed(failure, client.recentLog),
  };

  Future<void> restart() => ref.read(daemonClientProvider).restart();
}

final coreStatusProvider = NotifierProvider<CoreStatusNotifier, CoreStatus>(CoreStatusNotifier.new);

/// The open session; it changes only when a new session opens, so the data a view shows stays
/// while the core restarts, and every query refetches once it is back. `null` before the first.
class SessionEpochNotifier extends Notifier<SessionEpoch?> {
  @override
  SessionEpoch? build() {
    ref.listen(coreStatusProvider, (_, s) {
      if (s is CoreReady && s.session != state) state = s.session;
    });
    return switch (ref.read(coreStatusProvider)) {
      CoreReady(:final session) => session,
      CoreStopped() || CoreStarting() || CoreRestarting() || CoreFailed() => null,
    };
  }
}

final sessionEpochProvider = NotifierProvider<SessionEpochNotifier, SessionEpoch?>(
  SessionEpochNotifier.new,
);

/// The latest projection revision the state layer holds (ADR-0012, "Revision-driven
/// invalidation"); `null` until a session reports one. Query providers compare their page's
/// revision against it and refetch when theirs is older; nothing patches a cached value from an
/// event.
class ProjectionRevision extends Notifier<Revision?> {
  @override
  Revision? build() {
    ref.watch(sessionEpochProvider);
    final client = ref.watch(daemonClientProvider);
    final sub = client.events.listen((e) {
      if (e.hasProjectionChanged()) observe(Revision(e.projectionChanged.revision));
    });
    ref.onDispose(sub.cancel);
    return switch (client.health) {
      DaemonReady(:final revision) => Revision(revision),
      DaemonStopped() || DaemonStarting() || DaemonRestarting() || DaemonFailed() => null,
    };
  }

  /// A response or an event showed a revision; the state holds the highest seen.
  void observe(Revision revision) {
    final held = state;
    if (held == null || held.isBefore(revision)) state = revision;
  }
}

final projectionRevisionProvider = NotifierProvider<ProjectionRevision, Revision?>(
  ProjectionRevision.new,
);

/// Whether a value valid at [revision] is older than what the state layer holds.
bool isStale(Revision revision, Revision? held) => held != null && revision.isBefore(held);

/// A warning the daemon sent: something to show that changed no state. Its kind is the
/// generated `Warning.kind` case, which the views switch on.
final class CoreWarning {
  const CoreWarning(this.warning);

  final pb.Warning warning;
}

class CoreWarnings extends Notifier<List<CoreWarning>> {
  @override
  List<CoreWarning> build() {
    final client = ref.watch(daemonClientProvider);
    final sub = client.events.listen((e) {
      if (e.hasWarning()) state = [...state, CoreWarning(e.warning)];
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
