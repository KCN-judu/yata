// A profile's capabilities as the application parses them from the wire: what the daemon's
// fixture reports, and every way a capability list can break the protocol.

import 'package:flutter_test/flutter_test.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;
import 'package:yata/state/core.dart';
import 'package:yata/state/profiles.dart';

import '../support/fake_daemon.dart';
import '../support/recorded.dart';

Matcher get protocolError => isA<RaisedFailure>().having(
  (e) => e.failure.whichKind(),
  'kind',
  pb.ClientFailure_Kind.clientProtocolError,
);

pb.Profile profileWith(List<pb.ProfileCapability> capabilities) =>
    pb.Profile(id: fixtureProfile.hex, name: 'p', capabilities: capabilities);

void main() {
  test('the recorded fixture offers the inventory and names what the rest lack', () {
    for (final wire in recordedProfiles().profiles) {
      final p = profileOf(wire);
      expect(
        p.availability(Capability.inventory),
        isA<Available>().having((a) => a.completeness, 'completeness', Completeness.complete),
      );
      final lacking = {
        Capability.shikigamiCollection: SectionKind.shikigami,
        Capability.gamePresets: SectionKind.presets,
        Capability.assets: SectionKind.assets,
        Capability.guildView: SectionKind.guild,
      };
      for (final MapEntry(key: c, value: section) in lacking.entries) {
        expect(
          p.availability(c),
          isA<Unavailable>().having((a) => a.missing, 'missing', [section]),
          reason: '$c',
        );
      }
    }
  });

  test('every wire value but the unspecified one has a member', () {
    expect(
      Capability.values.map((c) => c.wire).toSet(),
      pb.Capability.values.where((v) => v.value != 0).toSet(),
    );
    expect(
      SectionKind.values.map((k) => k.wire).toSet(),
      pb.SectionKind.values.where((v) => v.value != 0).toSet(),
    );
    expect(
      Completeness.values.map((c) => c.wire).toSet(),
      pb.Completeness.values.where((v) => v.value != 0).toSet(),
    );
  });

  test('a capability the daemon did not send is not reported', () {
    final p = profileOf(profileWith([availableAs(Capability.inventory, Completeness.complete)]));
    expect(p.availability(Capability.guildView), isA<NotReported>());
  });

  test('a capability from a newer schema is skipped', () {
    // capability = 99, available with COMPLETENESS_COMPLETE.
    final newer = pb.ProfileCapability.fromBuffer([0x08, 99, 0x1a, 0x02, 0x08, 0x03]);
    final p = profileOf(
      profileWith([newer, availableAs(Capability.inventory, Completeness.partial)]),
    );
    expect(
      p.availability(Capability.inventory),
      isA<Available>().having((a) => a.completeness, 'completeness', Completeness.partial),
    );
  });

  test('anything unstated or stated twice breaks the protocol', () {
    final inventory = availableAs(Capability.inventory, Completeness.complete);
    final broken = <String, List<pb.ProfileCapability>>{
      'unspecified capability': [
        pb.ProfileCapability(
          capability: pb.Capability.CAPABILITY_UNSPECIFIED,
          available: pb.CapabilityAvailable(completeness: Completeness.complete.wire),
        ),
      ],
      'unspecified completeness': [
        pb.ProfileCapability(
          capability: Capability.inventory.wire,
          available: pb.CapabilityAvailable(),
        ),
      ],
      'no availability': [pb.ProfileCapability(capability: Capability.inventory.wire)],
      'nothing missing': [
        pb.ProfileCapability(
          capability: Capability.gamePresets.wire,
          unavailable: pb.CapabilityUnavailable(),
        ),
      ],
      'unspecified section': [
        pb.ProfileCapability(
          capability: Capability.gamePresets.wire,
          unavailable: pb.CapabilityUnavailable(missing: [pb.SectionKind.SECTION_KIND_UNSPECIFIED]),
        ),
      ],
      'a section twice': [
        unavailableFor(Capability.gamePresets, [SectionKind.souls, SectionKind.souls]),
      ],
      'a capability twice': [inventory, inventory],
      // missing = [99]: a section this build does not know, under a capability it does.
      'unknown section': [
        pb.ProfileCapability(capability: Capability.gamePresets.wire)
          ..mergeFromBuffer([0x12, 0x03, 0x0a, 0x01, 99]),
      ],
    };
    for (final MapEntry(key: what, value: capabilities) in broken.entries) {
      expect(() => profileOf(profileWith(capabilities)), throwsA(protocolError), reason: what);
    }
  });
}
