// The daemon's lifecycle: start, protocol mismatch, missing executable, exit and restart, and
// giving up. Health always ends somewhere definite; nothing waits forever.

import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:yata/daemon/daemon_client.dart';
import 'package:yata/daemon/daemon_error.dart';
import 'package:yata/daemon/transport.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;

import '../support/memory_transport.dart';

SupervisedDaemon supervise(DaemonLauncher launch) => SupervisedDaemon(
  launch,
  restartDelays: const [Duration.zero, Duration.zero],
  stableAfter: const Duration(hours: 1),
);

Future<T> nextHealth<T extends DaemonHealth>(DaemonClient d) =>
    d.healthChanges.firstWhere((h) => h is T).then((h) => h as T);

void main() {
  test('a daemon that answers opens a session', () async {
    final t = MemoryTransport(
      daemonThat(answer: (r) => [response(r, pb.Response(profileList: pb.ProfileList()))]),
    );
    final d = supervise(() async => t);
    await d.start();
    expect(d.health, isA<DaemonReady>().having((h) => h.restarted, 'restarted', isFalse));
    expect(t.received.first.hasOpenSession(), isTrue);
    expect(t.received[1].hasSubscribe(), isTrue);
    await d.listProfiles();
    await d.shutdown();
    expect(d.health, isA<DaemonStopped>());
    expect(t.received.last.hasShutdown(), isTrue);
  });

  test('a missing executable fails at once, and requests fail with it', () async {
    final d = supervise(
      () async => throw DaemonException(ClientErrorCode.daemonNotFound, 'nowhere'),
    );
    await d.start();
    expect(
      d.health,
      isA<DaemonFailed>().having((h) => h.error.code, 'code', ClientErrorCode.daemonNotFound),
    );
    await expectLater(
      d.listProfiles(),
      throwsA(isA<DaemonException>().having((e) => e.code, 'code', ClientErrorCode.daemonNotFound)),
    );
  });

  test('a request before start starts the daemon and fails with the real cause', () async {
    final d = supervise(
      () async => throw DaemonException(ClientErrorCode.daemonNotFound, 'nowhere'),
    );
    await expectLater(
      d.listProfiles(),
      throwsA(isA<DaemonException>().having((e) => e.code, 'code', ClientErrorCode.daemonNotFound)),
    );
    expect(d.health, isA<DaemonFailed>());
  });

  test('after shutdown, a request does not start the daemon again', () async {
    var launches = 0;
    final d = supervise(() async {
      launches++;
      return MemoryTransport(daemonThat());
    });
    await d.start();
    await d.shutdown();
    await expectLater(
      d.listProfiles(),
      throwsA(isA<DaemonException>().having((e) => e.code, 'code', ClientErrorCode.notConnected)),
    );
    expect(launches, 1);
  });

  test('a protocol mismatch fails without restarting', () async {
    var launches = 0;
    final d = supervise(() async {
      launches++;
      return MemoryTransport((r) => [errorResponse(r, 'session.protocol_unsupported')]);
    });
    await d.start();
    expect(
      d.health,
      isA<DaemonFailed>().having((h) => h.error.code, 'code', 'session.protocol_unsupported'),
    );
    expect(launches, 1);
  });

  test('an exit restarts the daemon, and the new session says so', () async {
    final transports = <MemoryTransport>[];
    final d = supervise(
      () async => transports.last = (transports..add(MemoryTransport(daemonThat()))).last,
    );
    await d.start();
    final restarted = nextHealth<DaemonReady>(d);
    transports.first.exit(3);
    final ready = await restarted;
    expect(ready.restarted, isTrue);
    expect(ready.session, 2);
    expect(transports, hasLength(2));
  });

  test('a request made while restarting waits for the new session', () async {
    final transports = <MemoryTransport>[];
    final d = supervise(() async {
      final t = MemoryTransport(
        daemonThat(answer: (r) => [response(r, pb.Response(profileList: pb.ProfileList()))]),
      );
      transports.add(t);
      return t;
    });
    await d.start();
    final starting = nextHealth<DaemonStarting>(d);
    transports.first.exit(3);
    await starting;
    final answer = d.listProfiles();
    await expectLater(answer, completes);
    expect(transports, hasLength(2));
  });

  test('a daemon that keeps exiting ends in failure, not an endless restart', () async {
    final d = supervise(() async {
      final t = MemoryTransport(daemonThat());
      scheduleMicrotask(
        () => Future<void>.delayed(const Duration(milliseconds: 5), () => t.exit(9)),
      );
      return t;
    });
    final failed = nextHealth<DaemonFailed>(d);
    await d.start();
    final h = await failed.timeout(const Duration(seconds: 5));
    expect(h.error.code, ClientErrorCode.daemonExited);
    await d.restart();
    expect(d.health, isNot(isA<DaemonStopped>()));
  });
}
