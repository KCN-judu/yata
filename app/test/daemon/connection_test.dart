// The connection's failure paths: every request ends in exactly one outcome, and a protocol
// break or an exit ends the session for every request in flight.

import 'package:fixnum/fixnum.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yata/daemon/connection.dart';
import 'package:yata/daemon/failure.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;

import '../support/memory_transport.dart';

Matcher failsWith(String code) => throwsA(isA<CoreFailure>().having((e) => e.code, 'code', code));

/// The client failure's case, which is its code.
Matcher raised(pb.ClientFailure_Kind kind) =>
    throwsA(isA<RaisedFailure>().having((e) => e.failure.whichKind(), 'kind', kind));

void main() {
  test('a daemon error arrives as its case, message and debug record', () async {
    final t = MemoryTransport(
      daemonThat(
        answer: (r) => [
          errorResponse(
            r,
            pb.Error(
              queryStaleRevision: pb.QueryStaleRevision(scan: Int64(3), current: Int64(4)),
            ),
          ),
        ],
      ),
    );
    final c = DaemonConnection(t);
    await c.open();
    final failure = await c.listProfiles().then<Object?>((_) => null, onError: (Object e) => e);
    expect(failure, isA<RequestFailure>());
    final f = failure! as RequestFailure;
    expect(f.error.whichKind(), pb.Error_Kind.queryStaleRevision);
    expect(f.code, 'query.stale_revision');
    expect(f.message, 'test refusal');
    expect(f.debugRecord, contains('current: 4'));
    expect(c.isClosed, isFalse);
  });

  test('a code this build does not know keeps its tag', () async {
    // Field 999 of Error: a oneof case from a newer daemon.
    final unknown = pb.Error.fromBuffer([0xba, 0x3e, 0x00]);
    final t = MemoryTransport(daemonThat(answer: (r) => [errorResponse(r, unknown)]));
    final c = DaemonConnection(t);
    await c.open();
    await expectLater(c.listProfiles(), failsWith('unknown (tag 999)'));
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
    await expectLater(pending, raised(pb.ClientFailure_Kind.clientDaemonExited));
    final closed = await c.closed;
    expect(closed, isA<RaisedFailure>());
    expect((closed as RaisedFailure).failure.clientDaemonExited.exitCode, 101);
    await expectLater(c.listProfiles(), raised(pb.ClientFailure_Kind.clientNotConnected));
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
    await expectLater(pending, raised(pb.ClientFailure_Kind.clientProtocolError));
    expect(t.killed, isTrue);
  });

  test('a malformed frame from the daemon breaks the session', () async {
    final t = MemoryTransport(daemonThat());
    final c = DaemonConnection(t);
    await c.open();
    t.emitBytes([0, 0, 0, 0]);
    expect((await c.closed).code, 'client.protocol_error');
    expect(t.killed, isTrue);
  });

  test('a session failure event carries the daemon code and ends the session', () async {
    final t = MemoryTransport(daemonThat());
    final c = DaemonConnection(t);
    await c.open();
    t.emit(
      pb.ServerMessage(
        event: pb.Event(
          sessionFailure: pb.SessionFailed(
            message: 'x',
            sessionMalformedFrame: pb.SessionMalformedFrame(),
          ),
        ),
      ),
    );
    final closed = await c.closed;
    expect(closed, isA<SessionFailure>());
    expect(closed.code, 'session.malformed_frame');
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
    await expectLater(c.listProfiles(), raised(pb.ClientFailure_Kind.clientTimeout));
    t.emit(response(unanswered.single, pb.Response(profileList: pb.ProfileList())));
    await Future<void>.delayed(Duration.zero);
    expect(c.isClosed, isFalse);
  });

  test('events other than a session failure reach the stream', () async {
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
