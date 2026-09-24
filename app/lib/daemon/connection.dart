/// One protocol session with one daemon process: request ids, correlation, events, and the end.
///
/// Every request gets exactly one outcome: its response, the daemon's `Error` as a
/// [DaemonException], or a client-side failure when the daemon exits, stays silent, or breaks
/// the protocol (`core-protocol.md`, "Requests and responses"). A protocol break is fatal: the
/// process is killed and every request in flight fails.
library;

import 'dart:async';

import 'package:fixnum/fixnum.dart';

import '../gen/proto/core.pb.dart' as pb;
import 'daemon_error.dart';
import 'frame_codec.dart';
import 'transport.dart';

/// The core protocol version this client was built against (`protocol-versions.md`): 1.0.
///
/// Requests set only the scalars that differ from their default. The Dart runtime writes a
/// proto3 scalar that was set, even to its default, and prost never does; leaving defaults unset
/// keeps the client's bytes identical to the Rust encoder's, which the recorded-session test
/// checks byte for byte.
final pb.ProtocolVersion clientProtocolVersion = pb.ProtocolVersion(major: 1);

class DaemonConnection {
  DaemonConnection(this._transport, {this.requestTimeout = const Duration(seconds: 30)}) {
    _subscription = _transport.output.listen(
      _receive,
      onError: (Object e) => _break('the daemon output failed: $e'),
      onDone: _outputEnded,
    );
    _transport.exitCode.then(_exited);
  }

  final DaemonTransport _transport;
  final Duration requestTimeout;
  final FrameDecoder _decoder = FrameDecoder();
  final Map<Int64, Completer<pb.Response>> _pending = {};
  // Ids whose caller gave up waiting; a late answer to one is dropped, not fatal.
  final Set<Int64> _abandoned = {};
  final StreamController<pb.Event> _events = StreamController.broadcast();
  final Completer<DaemonException> _closed = Completer();
  late final StreamSubscription<List<int>> _subscription;
  Int64 _nextId = Int64.ONE;
  bool _shuttingDown = false;

  /// Events the daemon sends unasked: projection changes and warnings.
  Stream<pb.Event> get events => _events.stream;

  /// Completes once when the session has ended, with why.
  Future<DaemonException> get closed => _closed.future;

  bool get isClosed => _closed.isCompleted;

  List<String> get recentLog => _transport.recentLog;

  /// The first request of every session.
  Future<pb.SessionOpened> open({Duration timeout = const Duration(seconds: 10)}) async {
    final r = await _request(pb.ClientMessage(openSession: pb.OpenSession()), timeout);
    return r.sessionOpened;
  }

  Future<pb.ProfileList> listProfiles() async =>
      (await _request(pb.ClientMessage(listProfiles: pb.ListProfiles()))).profileList;

  Future<pb.QueryPage> query(pb.Query query) async =>
      (await _request(pb.ClientMessage(query: query))).queryPage;

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
    } on DaemonException {
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
    if (isClosed) {
      return Future.error(
        DaemonException(ClientErrorCode.notConnected, 'the daemon session has ended'),
      );
    }
    final id = _nextId;
    _nextId += 1;
    message
      ..id = id
      ..protocolVersion = clientProtocolVersion;
    final completer = Completer<pb.Response>();
    _pending[id] = completer;
    _transport.send(encodeFrame(message.writeToBuffer()));
    final limit = timeout ?? requestTimeout;
    return completer.future
        .timeout(
          limit,
          onTimeout: () {
            _pending.remove(id);
            _abandoned.add(id);
            throw DaemonException(
              ClientErrorCode.timeout,
              'no response to request $id within $limit',
            );
          },
        )
        .then((r) => r.hasError() ? throw DaemonException.fromWire(r.error) : r);
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
        final pending = _pending.remove(id);
        if (pending != null) {
          pending.complete(m.response);
        } else if (!_abandoned.remove(id)) {
          _break('the daemon answered request $id, which was never sent');
        }
      case pb.ServerMessage_Kind.event:
        if (m.event.hasSessionFailed()) {
          _close(DaemonException.fromWire(m.event.sessionFailed));
          _transport.kill();
        } else {
          _events.add(m.event);
        }
      case pb.ServerMessage_Kind.notSet:
        _break('the daemon sent a message of no kind this client knows');
    }
  }

  void _break(String why) {
    _close(DaemonException(ClientErrorCode.protocolError, why));
    _transport.kill();
  }

  void _outputEnded() {
    try {
      _decoder.finish();
    } on FrameError catch (e) {
      _close(
        DaemonException(
          ClientErrorCode.protocolError,
          'the daemon output ended inside a frame: $e',
        ),
      );
    }
  }

  void _exited(int code) {
    final log = _transport.recentLog;
    final tail = log.isEmpty ? '' : '; last log line: ${log.last}';
    _close(
      DaemonException(
        ClientErrorCode.daemonExited,
        _shuttingDown
            ? 'the daemon shut down (exit code $code)'
            : 'the daemon exited with code $code$tail',
      ),
    );
  }

  /// End the session once: fail everything in flight with [why].
  void _close(DaemonException why) {
    if (isClosed) return;
    _closed.complete(why);
    final pending = _pending.values.toList();
    _pending.clear();
    for (final c in pending) {
      c.completeError(why);
    }
    unawaited(_subscription.cancel());
    unawaited(_events.close());
  }
}
