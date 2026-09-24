// The connection's failure paths: every request ends in exactly one outcome, and a protocol
// break or an exit ends the session for every request in flight.

import 'package:fixnum/fixnum.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yata/daemon/connection.dart';
import 'package:yata/daemon/daemon_error.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;

import '../support/memory_transport.dart';

Matcher failsWith(String code) =>
    throwsA(isA<DaemonException>().having((e) => e.code, 'code', code));

void main() {
  test('a daemon error arrives as its code, message and details', () async {
    final t = MemoryTransport(
      daemonThat(answer: (r) => [errorResponse(r, 'query.unknown_profile')]),
    );
    final c = DaemonConnection(t);
    await c.open();
    await expectLater(c.listProfiles(), failsWith('query.unknown_profile'));
    expect(c.isClosed, isFalse);
  });

  test('responses are matched by id, in whatever order they arrive', () async {
    final held = <pb.ClientMessage>[];
    late MemoryTransport t;
    t = MemoryTransport(
      daemonThat(
        answer: (r) {
          held.add(r);
          if (held.length < 2) return const [];
          // Answer the second request first.
          return [
            response(held[1], pb.Response(profileList: pb.ProfileList(revision: Int64(2)))),
            response(held[0], pb.Response(profileList: pb.ProfileList(revision: Int64(1)))),
          ];
        },
      ),
    );
    final c = DaemonConnection(t);
    await c.open();
    final a = c.listProfiles();
    final b = c.listProfiles();
    expect((await a).revision, Int64(1));
    expect((await b).revision, Int64(2));
  });

  test('the daemon exiting fails every request in flight with a client code', () async {
    final t = MemoryTransport(daemonThat());
    final c = DaemonConnection(t);
    await c.open();
    final pending = c.listProfiles();
    t.exit(101);
    await expectLater(pending, failsWith(ClientErrorCode.daemonExited));
    expect((await c.closed).code, ClientErrorCode.daemonExited);
    await expectLater(c.listProfiles(), failsWith(ClientErrorCode.notConnected));
  });

  test('a response to a request never sent breaks the session', () async {
    final t = MemoryTransport(daemonThat());
    final c = DaemonConnection(t);
    await c.open();
    final pending = c.listProfiles();
    t.emit(
      pb.ServerMessage(
        response: pb.Response(id: Int64(99), shutdownAccepted: pb.ShutdownAccepted()),
      ),
    );
    await expectLater(pending, failsWith(ClientErrorCode.protocolError));
    expect(t.killed, isTrue);
  });

  test('a malformed frame from the daemon breaks the session', () async {
    final t = MemoryTransport(daemonThat());
    final c = DaemonConnection(t);
    await c.open();
    t.emitBytes([0, 0, 0, 0]);
    expect((await c.closed).code, ClientErrorCode.protocolError);
    expect(t.killed, isTrue);
  });

  test('a session_failed event carries the daemon code and ends the session', () async {
    final t = MemoryTransport(daemonThat());
    final c = DaemonConnection(t);
    await c.open();
    t.emit(
      pb.ServerMessage(
        event: pb.Event(
          sessionFailed: pb.Error(code: 'session.malformed_frame', message: 'x'),
        ),
      ),
    );
    expect((await c.closed).code, 'session.malformed_frame');
  });

  test('a request with no answer times out and a late answer is dropped', () async {
    final unanswered = <pb.ClientMessage>[];
    final t = MemoryTransport(
      daemonThat(
        answer: (r) {
          unanswered.add(r);
          return const [];
        },
      ),
    );
    final c = DaemonConnection(t, requestTimeout: const Duration(milliseconds: 20));
    await c.open();
    await expectLater(c.listProfiles(), failsWith(ClientErrorCode.timeout));
    t.emit(response(unanswered.single, pb.Response(profileList: pb.ProfileList())));
    await Future<void>.delayed(Duration.zero);
    expect(c.isClosed, isFalse);
  });

  test('events other than session_failed reach the stream', () async {
    final t = MemoryTransport(daemonThat());
    final c = DaemonConnection(t);
    await c.open();
    final next = c.events.first;
    t.emit(
      pb.ServerMessage(
        event: pb.Event(projectionChanged: pb.ProjectionChanged(revision: Int64(7))),
      ),
    );
    expect((await next).projectionChanged.revision, Int64(7));
  });

  test('shutdown asks, closes stdin, and waits for the exit', () async {
    final t = MemoryTransport(daemonThat());
    final c = DaemonConnection(t);
    await c.open();
    await c.shutdown();
    expect(t.received.last.hasShutdown(), isTrue);
    expect(c.isClosed, isTrue);
    expect(t.killed, isFalse);
  });
}
