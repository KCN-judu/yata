/// A daemon process in memory: it decodes what the client sends and answers through a
/// responder, so connection and supervisor tests run without a process.
library;

import 'dart:async';

import 'package:fixnum/fixnum.dart';
import 'package:yata/daemon/frame_codec.dart';
import 'package:yata/daemon/transport.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;

typedef Responder = List<pb.ServerMessage> Function(pb.ClientMessage request);

class MemoryTransport implements DaemonTransport {
  MemoryTransport(this.respond);

  final Responder respond;
  final received = <pb.ClientMessage>[];
  final _out = StreamController<List<int>>();
  final _exit = Completer<int>();
  final _decoder = FrameDecoder();
  bool killed = false;

  @override
  Stream<List<int>> get output => _out.stream;

  @override
  void send(List<int> bytes) {
    _decoder.push(bytes);
    for (var p = _decoder.nextFrame(); p != null; p = _decoder.nextFrame()) {
      final m = pb.ClientMessage.fromBuffer(p);
      received.add(m);
      respond(m).forEach(emit);
    }
  }

  void emit(pb.ServerMessage m) => emitBytes(encodeFrame(m.writeToBuffer()));

  void emitBytes(List<int> bytes) {
    if (!_out.isClosed) _out.add(bytes);
  }

  /// The process ends with [code].
  void exit(int code) {
    if (_exit.isCompleted) return;
    unawaited(_out.close());
    _exit.complete(code);
  }

  @override
  Future<void> closeInput() async => exit(0);

  @override
  Future<int> get exitCode => _exit.future;

  @override
  void kill() {
    killed = true;
    exit(-1);
  }

  @override
  List<String> get recentLog => const [];
}

pb.ServerMessage response(pb.ClientMessage request, pb.Response r) =>
    pb.ServerMessage(response: r..id = request.id);

pb.ServerMessage errorResponse(pb.ClientMessage request, String code) => response(
  request,
  pb.Response(
    error: pb.Error(code: code, message: 'test: $code'),
  ),
);

pb.ServerMessage opened(pb.ClientMessage request, {int revision = 1}) => response(
  request,
  pb.Response(
    sessionOpened: pb.SessionOpened(
      daemonVersion: pb.ProtocolVersion(major: 1, minor: 0),
      revision: Int64(revision),
    ),
  ),
);

/// A daemon that opens every session and answers the other requests with [answer].
Responder daemonThat({List<pb.ServerMessage> Function(pb.ClientMessage)? answer}) => (request) {
  if (request.hasOpenSession()) return [opened(request)];
  if (request.hasSubscribe()) {
    return [
      response(
        request,
        pb.Response(subscribed: pb.Subscribed(revision: request.subscribe.revision)),
      ),
    ];
  }
  if (request.hasShutdown()) {
    return [response(request, pb.Response(shutdownAccepted: pb.ShutdownAccepted()))];
  }
  return answer?.call(request) ?? const [];
};
