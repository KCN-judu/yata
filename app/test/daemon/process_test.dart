// The real daemon as a child process: spawn `yata-daemon serve --fixture`, open a session, read a
// page, and shut down cleanly. Skipped when the daemon has not been built
// (`cargo build -p yata-daemon`), as in the Flutter CI job, which builds no Rust.

import 'dart:io';

import 'package:fixnum/fixnum.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yata/daemon/connection.dart';
import 'package:yata/daemon/daemon_error.dart';
import 'package:yata/daemon/frame_codec.dart';
import 'package:yata/daemon/transport.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;

String? builtDaemon() {
  final name = Platform.isWindows ? 'yata-daemon.exe' : 'yata-daemon';
  for (final profile in ['debug', 'release']) {
    final f = File('../target/$profile/$name');
    if (f.existsSync()) return f.path;
  }
  return null;
}

void main() {
  final daemon = builtDaemon();
  final skip = daemon == null ? 'yata-daemon is not built' : false;

  test('a session with the real daemon, end to end', () async {
    final transport = await ProcessTransport.start(daemon!, ['serve', '--fixture']);
    final c = DaemonConnection(transport);
    final opened = await c.open();
    expect(opened.daemonVersion.major, clientProtocolVersion.major);
    final profiles = await c.listProfiles();
    expect(profiles.profiles.map((p) => p.id), contains('fixture'));
    final page = await c.query(
      pb.Query(
        profileId: 'fixture',
        collection: pb.Collection.COLLECTION_SOULS,
        page: pb.PageRequest(rowBudget: 5),
      ),
    );
    expect(page.rows, hasLength(5));
    expect(page.rows.every((r) => r.soul.soulId == r.soulId), isTrue);
    expect(page.hasNextCursor(), isTrue);
    await expectLater(
      c.query(pb.Query(profileId: 'nobody', collection: pb.Collection.COLLECTION_SOULS)),
      throwsA(isA<DaemonException>().having((e) => e.code, 'code', 'query.unknown_profile')),
    );
    await c.shutdown();
    expect(await transport.exitCode, 0);
  }, skip: skip);

  test('a newer client major is refused by the real daemon', () async {
    // Stand in for a client built against a later major: write the request by hand.
    final p = await Process.start(daemon!, ['serve']);
    final newer = pb.ClientMessage(
      id: Int64.ONE,
      protocolVersion: pb.ProtocolVersion(major: clientProtocolVersion.major + 1),
      openSession: pb.OpenSession(),
    );
    p.stdin.add(encodeFrame(newer.writeToBuffer()));
    await p.stdin.close();
    final out = await p.stdout.expand((b) => b).toList();
    expect(await p.exitCode, 0);
    final reply = pb.ServerMessage.fromBuffer(decodeAllFrames(out).single);
    expect(reply.response.error.code, 'session.protocol_unsupported');
  }, skip: skip);

  test('a missing executable is a client error, not a hang', () async {
    await expectLater(
      ProcessTransport.start('${Directory.systemTemp.path}/no-such-yata-daemon', ['serve']),
      throwsA(
        isA<DaemonException>().having((e) => e.code, 'code', ClientErrorCode.daemonStartFailed),
      ),
    );
  });
}
