/// The daemon as the state layer sees it: one client that starts the process, opens the session,
/// restarts the daemon when it exits, and reports its health (ADR-0012, "Package layout").
///
/// Health is never an endless "starting": a daemon that cannot be found, is refused by version,
/// or keeps exiting ends in [DaemonFailed] with the failure that stopped it, and waits for an
/// explicit [DaemonClient.restart].
library;

import 'dart:async';

import 'package:fixnum/fixnum.dart';

import '../gen/proto/core.pb.dart' as pb;
import 'connection.dart';
import 'failure.dart';
import 'transport.dart';

/// Where the daemon is in its lifecycle.
sealed class DaemonHealth {
  const DaemonHealth();
}

/// Not started yet, or shut down on request.
final class DaemonStopped extends DaemonHealth {
  const DaemonStopped();
}

/// The first start after [DaemonClient.start] or [DaemonClient.restart].
final class DaemonStarting extends DaemonHealth {
  const DaemonStarting();
}

/// Starting again because the daemon exited or could not open a session: [attempt] counts the
/// consecutive restarts, [cause] is why.
final class DaemonRestarting extends DaemonHealth {
  const DaemonRestarting({required this.attempt, required this.cause});

  final int attempt;
  final CoreFailure cause;
}

/// Whether the open session tells the client when the projection moves.
sealed class Subscription {
  const Subscription();
}

final class SubscriptionLive extends Subscription {
  const SubscriptionLive();
}

/// The session answers queries but sends no `ProjectionChanged`: views go stale without knowing.
final class SubscriptionFailed extends Subscription {
  const SubscriptionFailed(this.failure);

  final CoreFailure failure;
}

/// A session is open.
final class DaemonReady extends DaemonHealth {
  const DaemonReady({
    required this.daemonVersion,
    required this.revision,
    required this.session,
    required this.restarted,
    this.subscription = const SubscriptionLive(),
  });

  final pb.ProtocolVersion daemonVersion;

  /// The projection's revision when the session opened.
  final Int64 revision;

  /// Counts sessions since the client started: a new one means every cached query is stale.
  final int session;

  /// This session replaced one whose daemon exited.
  final bool restarted;

  final Subscription subscription;
}

/// Stopped for good until [DaemonClient.restart]: [failure] says why.
final class DaemonFailed extends DaemonHealth {
  const DaemonFailed(this.failure);

  final CoreFailure failure;
}

/// What the state layer calls. The widget tests and the state tests supply their own.
abstract interface class DaemonClient {
  DaemonHealth get health;

  /// Every change of [health], starting with the next one.
  Stream<DaemonHealth> get healthChanges;

  /// Events from whichever session is open: projection changes and warnings.
  Stream<pb.Event> get events;

  /// The last lines the daemon wrote to stderr.
  List<String> get recentLog;

  Future<void> start();

  /// Start again after a failure, or replace a running daemon.
  Future<void> restart();

  Future<void> shutdown();

  Future<pb.ProfileList> listProfiles();

  Future<pb.SessionQueryPage> query(pb.SessionQuery query);

  Future<pb.SchemeCodeDecoded> decodeSchemeCode(pb.DecodeSchemeCode request);
}

/// Whether starting again can help after [failure]: not for a missing executable, one the
/// operating system refuses to start, or a daemon that refuses this client's version.
bool _worthRestarting(CoreFailure failure) => switch (failure) {
  RaisedFailure(:final failure) => switch (failure.whichKind()) {
    pb.ClientFailure_Kind.clientDaemonNotFound ||
    pb.ClientFailure_Kind.clientDaemonStartFailed => false,
    pb.ClientFailure_Kind.clientDaemonExited ||
    pb.ClientFailure_Kind.clientTimeout ||
    pb.ClientFailure_Kind.clientProtocolError ||
    pb.ClientFailure_Kind.clientNotConnected ||
    pb.ClientFailure_Kind.clientUnexpected ||
    pb.ClientFailure_Kind.notSet => true,
  },
  RequestFailure(:final error) => error.whichKind() != pb.Error_Kind.sessionProtocolUnsupported,
  SessionFailure() => true,
};

/// One run of starting a daemon. Stopping cancels it, so a start still in progress from before a
/// stop is abandoned when it resumes.
final class _Attempt {
  bool cancelled = false;
}

/// The supervisor's own state: one value, replaced on every transition, never several fields
/// filled in together.
sealed class _Phase {
  const _Phase();
}

final class _Idle extends _Phase {
  const _Idle();
}

/// A daemon is being started, or waited for before a restart; requests made meanwhile wait on
/// [ready].
final class _Starting extends _Phase {
  _Starting() : attempt = _Attempt(), ready = Completer<DaemonConnection>();

  final _Attempt attempt;
  final Completer<DaemonConnection> ready;
}

final class _Running extends _Phase {
  const _Running(this.attempt, this.connection);

  final _Attempt attempt;
  final DaemonConnection connection;
}

final class _Stopped extends _Phase {
  const _Stopped(this.failure);

  /// Why it stopped: absent when it was asked to.
  final CoreFailure? failure;
}

/// The [DaemonClient] over real daemon processes.
class SupervisedDaemon implements DaemonClient {
  SupervisedDaemon(
    this._launch, {
    this.restartDelays = const [
      Duration(milliseconds: 250),
      Duration(seconds: 1),
      Duration(seconds: 4),
    ],
    this.requestTimeout = const Duration(seconds: 30),
    this.openTimeout = const Duration(seconds: 10),
    this.stableAfter = const Duration(seconds: 30),
  });

  final DaemonLauncher _launch;

  /// The pause before each consecutive restart; one restart per entry, then [DaemonFailed].
  final List<Duration> restartDelays;
  final Duration requestTimeout;
  final Duration openTimeout;

  /// A session that lived this long ends the run of consecutive restarts.
  final Duration stableAfter;

  final StreamController<DaemonHealth> _health = StreamController.broadcast();
  final StreamController<pb.Event> _events = StreamController.broadcast();
  _Phase _phase = const _Idle();
  DaemonHealth _current = const DaemonStopped();
  List<String> _lastLog = const [];
  int _sessions = 0;
  int _restarts = 0;

  @override
  DaemonHealth get health => _current;

  @override
  Stream<DaemonHealth> get healthChanges => _health.stream;

  @override
  Stream<pb.Event> get events => _events.stream;

  @override
  List<String> get recentLog => switch (_phase) {
    _Running(:final connection) => connection.recentLog,
    _Idle() || _Starting() || _Stopped() => _lastLog,
  };

  void _set(DaemonHealth h) {
    _current = h;
    _health.add(h);
  }

  @override
  Future<void> start() async {
    switch (_phase) {
      case _Starting() || _Running():
        return;
      case _Idle() || _Stopped():
        _restarts = 0;
        final starting = _Starting();
        _phase = starting;
        _set(const DaemonStarting());
        await _connect(starting, restarted: false);
    }
  }

  @override
  Future<void> restart() async {
    await _stop();
    _restarts = 0;
    final starting = _Starting();
    _phase = starting;
    _set(const DaemonStarting());
    await _connect(starting, restarted: false);
  }

  @override
  Future<void> shutdown() async {
    await _stop();
    _set(const DaemonStopped());
  }

  /// Leave whatever phase this is for [_Stopped], cancelling a start in progress.
  Future<void> _stop() async {
    final previous = _phase;
    _phase = const _Stopped(null);
    switch (previous) {
      case _Starting(:final attempt, :final ready):
        attempt.cancelled = true;
        ready.completeError(RaisedFailure.notConnected());
        ready.future.ignore();
      case _Running(:final attempt, :final connection):
        attempt.cancelled = true;
        _lastLog = connection.recentLog;
        await connection.shutdown();
      case _Idle() || _Stopped():
        break;
    }
  }

  /// Start a daemon and open its session under [starting].
  Future<void> _connect(_Starting starting, {required bool restarted}) async {
    final attempt = starting.attempt;
    final DaemonConnection connection;
    final pb.SessionOpened opened;
    try {
      connection = DaemonConnection(await _launch(), requestTimeout: requestTimeout);
    } on CoreFailure catch (e) {
      if (!attempt.cancelled) _fail(starting, e);
      return;
    }
    if (attempt.cancelled) return connection.shutdown();
    try {
      opened = await connection.open(timeout: openTimeout);
    } on CoreFailure catch (e) {
      _lastLog = connection.recentLog;
      unawaited(connection.shutdown());
      if (attempt.cancelled) return;
      return _worthRestarting(e) ? _restart(starting, e) : _fail(starting, e);
    }
    if (attempt.cancelled) return connection.shutdown();
    _phase = _Running(attempt, connection);
    _sessions += 1;
    connection.events.listen(_events.add);
    final openedAt = DateTime.now();
    unawaited(
      connection.closed.then((why) async {
        // A stop or a replacement cancelled this attempt; its session's end then changes nothing.
        if (attempt.cancelled) return;
        _lastLog = connection.recentLog;
        if (DateTime.now().difference(openedAt) >= stableAfter) _restarts = 0;
        await _restart(_Starting(), why);
      }),
    );
    starting.ready.complete(connection);
    Subscription subscription = const SubscriptionLive();
    try {
      await connection.subscribe(opened.revision);
    } on CoreFailure catch (e) {
      subscription = SubscriptionFailed(e);
    }
    if (attempt.cancelled) return;
    _set(
      DaemonReady(
        daemonVersion: opened.daemonVersion,
        revision: opened.revision,
        session: _sessions,
        restarted: restarted,
        subscription: subscription,
      ),
    );
  }

  void _fail(_Starting starting, CoreFailure failure) {
    _phase = _Stopped(failure);
    starting.ready.completeError(failure);
    starting.ready.future.ignore();
    _set(DaemonFailed(failure));
  }

  /// Start again after [cause] under [starting], unless the consecutive restarts are used up.
  /// Requests made meanwhile wait on [starting]'s completer for the new session.
  Future<void> _restart(_Starting starting, CoreFailure cause) async {
    if (_restarts >= restartDelays.length) return _fail(starting, cause);
    final delay = restartDelays[_restarts];
    _restarts += 1;
    _phase = starting;
    _set(DaemonRestarting(attempt: _restarts, cause: cause));
    await Future<void>.delayed(delay);
    if (starting.attempt.cancelled) return;
    await _connect(starting, restarted: true);
  }

  /// The open session, waiting for one that is starting. A request made before the daemon was
  /// ever started starts it, so it fails with the real cause, not with "not running".
  Future<DaemonConnection> _session() {
    switch (_phase) {
      case _Running(:final connection) when !connection.isClosed:
        return Future.value(connection);
      case _Starting(:final ready):
        return ready.future;
      case _Idle():
        unawaited(start());
        return _session();
      case _Stopped(:final failure):
        return Future.error(failure ?? RaisedFailure.notConnected());
      case _Running():
        return Future.error(RaisedFailure.notConnected());
    }
  }

  @override
  Future<pb.ProfileList> listProfiles() async => (await _session()).listProfiles();

  @override
  Future<pb.SessionQueryPage> query(pb.SessionQuery query) async => (await _session()).query(query);

  @override
  Future<pb.SchemeCodeDecoded> decodeSchemeCode(pb.DecodeSchemeCode request) async =>
      (await _session()).decodeSchemeCode(request);
}
