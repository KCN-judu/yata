// The recorded session (`crates/yata-daemon/tests/core_session.rs`) in both directions.
//
// Rust encode → Dart decode: every frame the daemon wrote decodes here, typed.
// Dart encode → Rust decode: the requests the Dart client sends are, byte for byte, the frames
// the Rust test sent and decoded (`session.in`), so the daemon reads them as it read its own.

import 'package:fixnum/fixnum.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yata/daemon/connection.dart';
import 'package:yata/daemon/frame_codec.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;
import 'package:yata/state/core.dart';
import 'package:yata/state/inventory.dart';

import '../support/memory_transport.dart';
import '../support/recorded.dart';

void main() {
  test('the client opens with the protocol version the daemon was recorded with', () {
    final open = recordedRequests().first;
    expect(open.hasOpenSession(), isTrue);
    expect(open.openSession.clientVersion.writeToBuffer(), clientProtocolVersion.writeToBuffer());
  });

  test('re-encoding every recorded request gives the recorded bytes', () {
    final reencoded = [for (final r in recordedRequests()) ...encodeFrame(r.writeToBuffer())];
    expect(reencoded, fixtureBytes('core/session.in'));
  });

  test('the soul query the state layer builds is the one the Rust test sent', () {
    const spec = SoulQuerySpec(profileId: fixtureProfile, rowBudget: 8);
    final first = soulQuery(spec, const FirstPosition());
    expect(first.writeToBuffer(), recordedRequests()[3].sessionQuery.writeToBuffer());
    final second = soulQuery(
      spec,
      NextPosition(recordedFirstPage().nextCursor, Revision(Int64.ONE)),
    );
    expect(second.writeToBuffer(), recordedRequests()[4].sessionQuery.writeToBuffer());
  });

  test('every daemon frame decodes, one response per request and one event', () {
    final out = recordedServerMessages();
    final responses = out.where((m) => m.hasResponse()).map((m) => m.response.id.toInt()).toList();
    expect(responses, [1, 2, 3, 4, 5, 6, 7, 8]);
    final events = out.where((m) => m.hasEvent()).toList();
    expect(events.single.event.projectionChanged.revision, Int64.ONE);
  });

  test('the recorded values arrive typed', () {
    expect(recordedProfiles().profiles.map((p) => p.id), [fixtureProfile.hex, emptyProfile.hex]);
    final first = recordedFirstPage();
    expect(first.total, Int64(12));
    expect(first.hasNextCursor(), isTrue);
    expect(first.rows, hasLength(8));
    expect(first.revision, Int64.ONE);
    final soul = first.rows.first.soul;
    expect(soul.soulId, 'fixture-01');
    expect(first.rows.first.whichVerdict(), pb.SessionRow_Verdict.exact);
    expect(soul.whichKind(), pb.Soul_Kind.ordinary);
    expect(soul.slot, pb.SoulSlot.SOUL_SLOT_2);
    expect(soul.main, pb.SoulAttribute.SOUL_ATTRIBUTE_SPD);
    expect(soul.subs.first.hasEnhancementCount(), isFalse);
    final second = recordedSecondPage();
    expect(second.hasNextCursor(), isFalse);
    expect(second.rows.first.soul.soulId, 'fixture-09');
    expect(recordedUnknownProfile().whichKind(), pb.Error_Kind.queryUnknownProfile);
    final scheme = recordedScheme();
    expect(scheme.kind, pb.SchemeKind.SCHEME_KIND_STRENGTHENING);
    expect(scheme.entries.map((e) => e.name), ['spd', 'six']);
    expect(scheme.encoded.qr.modules, hasLength(scheme.encoded.qr.size * scheme.encoded.qr.size));
  });

  test('a connection replaying the recorded daemon gets the recorded answers', () async {
    final byId = {
      for (final m in recordedServerMessages())
        if (m.hasResponse()) m.response.id: m,
    };
    final sent = <int>[];
    final transport = MemoryTransport((request) {
      sent.addAll(encodeFrame(request.writeToBuffer()));
      return [byId[request.id]!];
    });
    final connection = DaemonConnection(transport);
    final opened = await connection.open();
    expect(opened.revision, Int64.ONE);
    await connection.subscribe(Int64.ZERO);
    expect((await connection.listProfiles()).profiles, hasLength(2));
    final page = await connection.query(recordedRequests()[3].sessionQuery);
    expect(page.rows, hasLength(8));
    await connection.query(recordedRequests()[4].sessionQuery);
    await expectLater(
      connection.query(recordedRequests()[5].sessionQuery),
      throwsA(isA<RequestFailure>().having((e) => e.code, 'code', 'query.unknown_profile')),
    );
    final decoded = await connection.decodeSchemeCode(recordedRequests()[6].decodeSchemeCode);
    expect(decoded.entries, hasLength(2));
    await connection.shutdown();
    // What the client sent is, byte for byte, what the Rust test sent.
    expect(sent, fixtureBytes('core/session.in'));
  });
}
