/// One protocol session with one daemon process: request ids, correlation, events, and the end.
///
/// Every request gets exactly one outcome: its response, the daemon's refusal as a
/// [RequestFailure], or a [RaisedFailure] when the daemon exits, stays silent, or breaks the
/// protocol (`core-protocol.md`, "Requests and responses"). A protocol break is fatal: the process
/// is killed and every request in flight fails.
library;

import 'dart:async';
import 'dart:collection';

import 'package:fixnum/fixnum.dart';

import '../gen/proto/core.pb.dart' as pb;
import 'failure.dart';
import 'frame_codec.dart';
import 'transport.dart';

/// The core protocol version this client was built against (`protocol-versions.md`): 1.0.
///
/// Requests follow one encoding rule per field kind, so the client's bytes equal the Rust
/// encoder's, which the recorded-session test checks byte for byte:
/// - a scalar with implicit presence is set only when it differs from its default: the Dart
///   runtime writes a scalar that was set, even to its default, and prost never does;
/// - a field with explicit presence (`optional`, a message, a oneof case) is set exactly when it
///   is present, whatever its value: `optional` 0 is written, and means 0.
final pb.ProtocolVersion clientProtocolVersion = pb.ProtocolVersion(major: 1);

/// What a request in flight is waiting for.
final class _RequestSlot {
  _RequestSlot(this.completer, this.timer);

  final Completer<pb.Response> completer;
  final Timer timer;
}

class DaemonConnection {
  DaemonConnection(this._transport, {this.requestTimeout = const Duration(seconds: 30)}) {
    _subscription = _transport.output.listen(
      _receive,
      onError: (Object e) => _break('the daemon output failed: $e'),
      onDone: _outputEnded,
    );
    unawaited(_transport.exitCode.then(_exited));
  }

  /// How many timed-out ids are remembered, so a late answer to one is dropped, not fatal.
  static const abandonedMemory = 256;

  final DaemonTransport _transport;
  final Duration requestTimeout;
  final FrameDecoder _decoder = FrameDecoder();
  final Map<Int64, _RequestSlot> _inFlight = {};
  final Queue<Int64> _abandoned = Queue();
  final StreamController<pb.Event> _events = StreamController.broadcast();
  final Completer<CoreFailure> _closed = Completer();
  late final StreamSubscription<List<int>> _subscription;
  Int64 _nextId = Int64.ONE;
  bool _shuttingDown = false;

  /// Events the daemon sends unasked: projection changes and warnings.
  Stream<pb.Event> get events => _events.stream;

  /// Completes once when the session has ended, with why.
  Future<CoreFailure> get closed => _closed.future;

  bool get isClosed => _closed.isCompleted;

  List<String> get recentLog => _transport.recentLog;

  /// The first request of every session: it declares the version the client was built against.
  Future<pb.SessionOpened> open({Duration timeout = const Duration(seconds: 10)}) async {
    final r = await _request(
      pb.ClientMessage(openSession: pb.OpenSession(clientVersion: clientProtocolVersion)),
      timeout,
    );
    return r.sessionOpened;
  }

  Future<pb.ProfileList> listProfiles() async =>
      (await _request(pb.ClientMessage(listProfiles: pb.ListProfiles()))).profileList;

  Future<pb.SessionQueryPage> query(pb.SessionQuery query) async =>
      (await _request(pb.ClientMessage(sessionQuery: query))).sessionQueryPage;

  Future<pb.SchemeCodeDecoded> decodeSchemeCode(pb.DecodeSchemeCode request) async =>
      (await _request(pb.ClientMessage(decodeSchemeCode: request))).schemeCodeDecoded;

  Future<pb.Subscribed> subscribe(Int64 revision) async {
    final s = pb.Subscribe();
    if (revision != Int64.ZERO) s.revision = revision;
    return (await _request(pb.ClientMessage(subscribe: s))).subscribed;
  }

  /// End the session cleanly: ask, close stdin, and wait for the exit; kill after [grace].
  Future<void> shutdown({Duration grace = const Duration(seconds: 3)}) async {
    if (isClosed) return;
    _shuttingDown = true;
    try {
      await _request(pb.ClientMessage(shutdown: pb.Shutdown()), grace);
    } on CoreFailure {
      // Shutting down either way; the exit below is what matters.
    }
    await _transport.closeInput();
    try {
      await _closed.future.timeout(grace);
    } on TimeoutException {
      _transport.kill();
      await _closed.future;
    }
  }

  Future<pb.Response> _request(pb.ClientMessage message, [Duration? timeout]) {
    if (isClosed) return Future.error(RaisedFailure.notConnected());
    final id = _nextId;
    _nextId += 1;
    message.id = id;
    final limit = timeout ?? requestTimeout;
    final completer = Completer<pb.Response>();
    _inFlight[id] = _RequestSlot(completer, Timer(limit, () => _timedOut(id, limit)));
    _transport.send(encodeFrame(message.writeToBuffer()));
    return completer.future.then((r) => r.hasError() ? throw RequestFailure(r.error) : r);
  }

  void _timedOut(Int64 id, Duration limit) {
    final slot = _inFlight.remove(id);
    if (slot == null) return;
    _abandoned.addLast(id);
    if (_abandoned.length > abandonedMemory) _abandoned.removeFirst();
    slot.completer.completeError(RaisedFailure.timeout(id.toInt(), limit));
  }

  void _receive(List<int> bytes) {
    if (isClosed) return;
    _decoder.push(bytes);
    try {
      for (var p = _decoder.nextFrame(); p != null; p = _decoder.nextFrame()) {
        _dispatch(pb.ServerMessage.fromBuffer(p));
        if (isClosed) return;
      }
    } on FrameError catch (e) {
      _break('the daemon wrote a malformed frame: $e');
    } on FormatException catch (e) {
      _break('the daemon wrote a frame that is not a server message: $e');
    }
  }

  void _dispatch(pb.ServerMessage m) {
    switch (m.whichKind()) {
      case pb.ServerMessage_Kind.response:
        final id = m.response.id;
        final slot = _inFlight.remove(id);
        if (slot != null) {
          slot.timer.cancel();
          slot.completer.complete(m.response);
        } else if (!_abandoned.remove(id)) {
          _break('the daemon answered request $id, which was never sent');
        }
      case pb.ServerMessage_Kind.event:
        if (m.event.hasSessionFailure()) {
          _close(SessionFailure(m.event.sessionFailure));
          _transport.kill();
        } else {
          _events.add(m.event);
        }
      case pb.ServerMessage_Kind.notSet:
        _break('the daemon sent a message of no kind this client knows');
    }
  }

  void _break(String why) {
    _close(RaisedFailure.protocolError(why));
    _transport.kill();
  }

  void _outputEnded() {
    try {
      _decoder.finish();
    } on FrameError catch (e) {
      _close(RaisedFailure.protocolError('the daemon output ended inside a frame: $e'));
    }
  }

  void _exited(int code) {
    final log = _transport.recentLog;
    _close(RaisedFailure.daemonExited(code, log.isEmpty ? null : log.last, asked: _shuttingDown));
  }

  /// End the session once: fail everything in flight with [why].
  void _close(CoreFailure why) {
    if (isClosed) return;
    _closed.complete(why);
    final inFlight = _inFlight.values.toList();
    _inFlight.clear();
    for (final slot in inFlight) {
      slot.timer.cancel();
      slot.completer.completeError(why);
    }
    unawaited(_subscription.cancel());
    unawaited(_events.close());
  }
}
