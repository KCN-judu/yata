// Each page behind its capability, in every state a capability can be in: available (complete,
// partial, unstated), unavailable with the sections it lacks, and not reported. The page's
// content appears only when the capability is available, and its souls are queried only then.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;
import 'package:yata/state/profiles.dart';

import '../support/app.dart';
import '../support/fake_daemon.dart';

FakeDaemonClient clientWith(List<pb.ProfileCapability> capabilities) =>
    FakeDaemonClient()..onListProfiles = () async => profilesWith(capabilities);

Future<void> openShikigami(WidgetTester tester) async {
  await tester.tap(find.text(l.navShikigami));
  await tester.pumpAndSettle();
}

void main() {
  testWidgets('an available, complete inventory shows its souls and no note', (tester) async {
    final client = clientWith([availableAs(Capability.inventory, Completeness.complete)]);
    await pumpApp(tester, client);
    expect(find.text(l.inventoryPageRange(1, 8)), findsOneWidget);
    expect(find.text(l.completenessPartial), findsNothing);
    expect(find.text(l.completenessUnstated), findsNothing);
  });

  for (final (completeness, note) in [
    (Completeness.partial, l.completenessPartial),
    (Completeness.unstated, l.completenessUnstated),
  ]) {
    testWidgets('a ${completeness.name} inventory shows its souls and says so', (tester) async {
      await pumpApp(tester, clientWith([availableAs(Capability.inventory, completeness)]));
      expect(find.text(l.inventoryPageRange(1, 8)), findsOneWidget);
      expect(find.text(note), findsOneWidget);
    });
  }

  testWidgets('an unavailable inventory names the missing section and queries nothing', (
    tester,
  ) async {
    final client = clientWith([
      unavailableFor(Capability.inventory, [SectionKind.souls]),
    ]);
    await pumpApp(tester, client);
    expect(find.text(l.capabilityUnavailableTitle(l.sectionSouls)), findsOneWidget);
    expect(find.text(l.capabilityUnavailableBody(l.navInventory, l.sectionSouls)), findsOneWidget);
    expect(find.text(l.inventoryEmptyTitle), findsNothing);
    expect(find.text(l.detailEmpty), findsNothing);
    expect(client.queries, isEmpty);
  });

  testWidgets('an inventory the daemon does not report is shown as such', (tester) async {
    final client = clientWith([]);
    await pumpApp(tester, client);
    expect(find.text(l.capabilityNotReportedTitle), findsOneWidget);
    expect(find.text(l.capabilityNotReportedBody(l.navInventory)), findsOneWidget);
    expect(client.queries, isEmpty);
  });

  testWidgets('the fixture\'s Shikigami collection names what it lacks, and no worn soul', (
    tester,
  ) async {
    await pumpApp(tester, FakeDaemonClient());
    await openShikigami(tester);
    expect(find.text(l.capabilityUnavailableTitle(l.sectionShikigami)), findsOneWidget);
    expect(
      find.text(l.capabilityUnavailableBody(l.navShikigami, l.sectionShikigami)),
      findsOneWidget,
    );
    expect(find.text(l.shikigamiPendingTitle), findsNothing);
  });

  testWidgets('several missing sections are all named', (tester) async {
    final sections = [SectionKind.shikigami, SectionKind.souls];
    await pumpApp(
      tester,
      clientWith([
        availableAs(Capability.inventory, Completeness.complete),
        unavailableFor(Capability.shikigamiCollection, sections),
      ]),
    );
    await openShikigami(tester);
    final named = '${l.sectionShikigami}${l.listSeparator}${l.sectionSouls}';
    expect(find.text(l.capabilityUnavailableTitle(named)), findsOneWidget);
  });

  testWidgets('an available Shikigami collection says its list is still to come', (tester) async {
    await pumpApp(
      tester,
      clientWith([
        availableAs(Capability.inventory, Completeness.complete),
        availableAs(Capability.shikigamiCollection, Completeness.partial),
      ]),
    );
    await openShikigami(tester);
    expect(find.text(l.shikigamiPendingTitle), findsOneWidget);
    expect(find.text(l.completenessPartial), findsOneWidget);
    expect(find.byKey(const ValueKey('capability-unavailable')), findsNothing);
  });

  testWidgets('a Shikigami collection the daemon does not report is shown as such', (tester) async {
    await pumpApp(tester, clientWith([availableAs(Capability.inventory, Completeness.complete)]));
    await openShikigami(tester);
    expect(find.text(l.capabilityNotReportedBody(l.navShikigami)), findsOneWidget);
  });

  testWidgets('a profile list that breaks the protocol is a failure, not a page', (tester) async {
    final twice = availableAs(Capability.inventory, Completeness.complete);
    await pumpApp(tester, clientWith([twice, twice]));
    expect(find.text(l.causeProtocolError), findsOneWidget);
    expect(find.text(l.profilesLoadFailed), findsOneWidget);
  });
}
