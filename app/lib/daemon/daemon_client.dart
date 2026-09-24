/// The daemon as the state layer sees it: one client that starts the process, opens the session,
/// restarts the daemon when it exits, and reports its health (ADR-0012, "Package layout").
///
/// Health is never an endless "starting": a daemon that cannot be found, is refused by version,
/// or keeps exiting ends in [DaemonFailed] with the error that stopped it, and waits for an
/// explicit [DaemonClient.restart].
library;

import 'dart:async';

import 'package:fixnum/fixnum.dart';

import '../gen/proto/core.pb.dart' as pb;
import 'connection.dart';
import 'daemon_error.dart';
import 'transport.dart';

/// Where the daemon is in its lifecycle.
sealed class DaemonHealth {
  const DaemonHealth();
}

/// Not started yet, or shut down on request.
final class DaemonStopped extends DaemonHealth {
  const DaemonStopped();
}

/// Starting or restarting. [after] is the failure that caused a restart, if this is one.
final class DaemonStarting extends DaemonHealth {
  const DaemonStarting({required this.attempt, this.after});

  final int attempt;
  final DaemonException? after;
}

/// A session is open.
final class DaemonReady extends DaemonHealth {
  const DaemonReady({
    required this.daemonVersion,
    required this.revision,
    required this.session,
    required this.restarted,
  });

  final pb.ProtocolVersion daemonVersion;

  /// The projection's revision when the session opened.
  final Int64 revision;

  /// Counts sessions since the client started: a new one means every cached query is stale.
  final int session;

  /// This session replaced one whose daemon exited.
  final bool restarted;
}

/// Stopped for good until [DaemonClient.restart]: [error] says why.
final class DaemonFailed extends DaemonHealth {
  const DaemonFailed(this.error);

  final DaemonException error;
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

  Future<pb.QueryPage> query(pb.Query query);

  Future<pb.SchemeCodeDecoded> decodeSchemeCode(pb.DecodeSchemeCode request);
}

/// Codes after which starting again cannot help, so no restart is attempted.
const _permanent = {
  ClientErrorCode.daemonNotFound,
  ClientErrorCode.daemonStartFailed,
  'session.protocol_unsupported',
};

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
  DaemonHealth _current = const DaemonStopped();
  DaemonConnection? _connection;
  Completer<DaemonConnection>? _ready;
  int _sessions = 0;
  int _consecutiveRestarts = 0;
  bool _stopping = false;
  // Bumped by every stop: a start still in progress from before it is abandoned.
  int _generation = 0;

  @override
  DaemonHealth get health => _current;

  @override
  Stream<DaemonHealth> get healthChanges => _health.stream;

  @override
  Stream<pb.Event> get events => _events.stream;

  @override
  List<String> get recentLog => _connection?.recentLog ?? const [];

  void _set(DaemonHealth h) {
    _current = h;
    _health.add(h);
  }

  @override
  Future<void> start() async {
    if (_current is DaemonReady || _current is DaemonStarting) return;
    _stopping = false;
    _consecutiveRestarts = 0;
    await _connect(attempt: 1, after: null);
  }

  @override
  Future<void> restart() async {
    await _stop();
    _stopping = false;
    _consecutiveRestarts = 0;
    await _connect(attempt: 1, after: null);
  }

  @override
  Future<void> shutdown() async {
    await _stop();
    _set(const DaemonStopped());
  }

  Future<void> _stop() async {
    _stopping = true;
    _generation += 1;
    final c = _connection;
    _connection = null;
    _ready?.completeError(DaemonException(ClientErrorCode.notConnected, 'the daemon was stopped'));
    _ready?.future.ignore();
    _ready = null;
    await c?.shutdown();
  }

  Future<void> _connect({required int attempt, required DaemonException? after}) async {
    final generation = _generation;
    _set(DaemonStarting(attempt: attempt, after: after));
    final ready = _ready ??= Completer<DaemonConnection>();
    final DaemonConnection connection;
    final pb.SessionOpened opened;
    try {
      connection = DaemonConnection(await _launch(), requestTimeout: requestTimeout);
    } on DaemonException catch (e) {
      if (generation != _generation) return;
      return _failed(e, ready);
    }
    if (generation != _generation) return connection.shutdown();
    try {
      opened = await connection.open(timeout: openTimeout);
    } on DaemonException catch (e) {
      unawaited(connection.shutdown());
      if (generation != _generation) return;
      return _permanent.contains(e.code) ? _failed(e, ready) : _exitedUnexpectedly(e);
    }
    if (generation != _generation) {
      await connection.shutdown();
      return;
    }
    _connection = connection;
    _sessions += 1;
    connection.events.listen(_events.add);
    final openedAt = DateTime.now();
    unawaited(
      connection.closed.then((why) async {
        if (identical(_connection, connection) && !_stopping) {
          _connection = null;
          if (DateTime.now().difference(openedAt) >= stableAfter) _consecutiveRestarts = 0;
          await _exitedUnexpectedly(why);
        }
      }),
    );
    _ready = null;
    ready.complete(connection);
    _set(
      DaemonReady(
        daemonVersion: opened.daemonVersion,
        revision: opened.revision,
        session: _sessions,
        restarted: after != null,
      ),
    );
    try {
      await connection.subscribe(opened.revision);
    } on DaemonException {
      // A session that cannot subscribe still answers queries; its end is reported by `closed`.
    }
  }

  void _failed(DaemonException e, Completer<DaemonConnection> ready) {
    _ready = null;
    ready.completeError(e);
    ready.future.ignore();
    _set(DaemonFailed(e));
  }

  Future<void> _exitedUnexpectedly(DaemonException why) async {
    // Requests made while the daemon restarts wait for the new session.
    final ready = _ready ??= Completer<DaemonConnection>();
    if (_consecutiveRestarts >= restartDelays.length) return _failed(why, ready);
    final delay = restartDelays[_consecutiveRestarts];
    _consecutiveRestarts += 1;
    _set(DaemonStarting(attempt: _consecutiveRestarts + 1, after: why));
    final generation = _generation;
    await Future<void>.delayed(delay);
    if (generation != _generation) return;
    await _connect(attempt: _consecutiveRestarts + 1, after: why);
  }

  /// The open session, waiting for one that is starting; throws when there is none. A request
  /// made before the daemon was ever started starts it, so it fails with the real cause, not
  /// with "not running".
  Future<DaemonConnection> _session() {
    final c = _connection;
    if (c != null && !c.isClosed) {
      return Future.value(c);
    }
    if (_current is DaemonStopped && !_stopping) unawaited(start());
    final pending = _ready;
    if (pending != null) return pending.future;
    final h = _current;
    return Future.error(
      h is DaemonFailed
          ? h.error
          : DaemonException(ClientErrorCode.notConnected, 'the daemon is not running'),
    );
  }

  @override
  Future<pb.ProfileList> listProfiles() async => (await _session()).listProfiles();

  @override
  Future<pb.QueryPage> query(pb.Query query) async => (await _session()).query(query);

  @override
  Future<pb.SchemeCodeDecoded> decodeSchemeCode(pb.DecodeSchemeCode request) async =>
      (await _session()).decodeSchemeCode(request);
}
