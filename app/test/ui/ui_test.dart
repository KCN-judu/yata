// Presentation and interaction over a fake daemon client (ADR-0012, "Tests"): navigation, the
// inventory's data, empty, and error states, the core banner, and scheme import with its QR code.
// Nothing here asserts a score; the application has none.

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yata/daemon/daemon_client.dart';
import 'package:yata/daemon/daemon_error.dart';
import 'package:yata/gen/l10n/app_localizations.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;
import 'package:yata/ui/app.dart';
import 'package:yata/ui/schemes/qr_matrix_view.dart';

import '../support/fake_daemon.dart';
import '../support/recorded.dart';

final l = lookupAppLocalizations(const Locale('zh'));

Future<void> pumpApp(WidgetTester tester, FakeDaemonClient client) async {
  tester.view.physicalSize = const Size(1440, 900);
  tester.view.devicePixelRatio = 1;
  addTearDown(tester.view.reset);
  await tester.pumpWidget(
    ProviderScope(overrides: fakeOverrides(client), retry: (_, _) => null, child: const YataApp()),
  );
  await tester.pumpAndSettle();
}

void main() {
  testWidgets('the rail reaches every place', (tester) async {
    await pumpApp(tester, FakeDaemonClient());
    expect(find.text(l.inventoryTitle), findsWidgets);
    await tester.tap(find.text(l.navSchemes));
    await tester.pumpAndSettle();
    expect(find.text(l.schemesImported), findsOneWidget);
    await tester.tap(find.text(l.navShikigami));
    await tester.pumpAndSettle();
    expect(find.text(l.shikigamiUnavailableTitle), findsOneWidget);
    await tester.tap(find.text(l.navSettings));
    await tester.pumpAndSettle();
    expect(find.text(l.settingsAppearance), findsOneWidget);
    await tester.tap(find.text(l.navInventory));
    await tester.pumpAndSettle();
    expect(find.text(l.detailEmpty), findsOneWidget);
  });

  testWidgets('the inventory shows the page the daemon sent, and a selection its detail', (
    tester,
  ) async {
    await pumpApp(tester, FakeDaemonClient());
    final soul = recordedFirstPage().rows.first.soul;
    expect(find.text(l.inventoryCount('12')), findsOneWidget);
    expect(find.text(l.inventoryPageRange(1, 8)), findsOneWidget);
    expect(find.text(l.detailEmpty), findsOneWidget);
    await tester.tap(find.text(l.soulStar(soul.star)).first);
    await tester.pumpAndSettle();
    expect(find.text(l.detailEmpty), findsNothing);
    expect(find.text(l.labelSubs), findsOneWidget);
    await tester.tap(find.byTooltip(l.actionNextPage));
    await tester.pumpAndSettle();
    expect(find.text(l.inventoryPageRange(9, 12)), findsOneWidget);
  });

  testWidgets('a profile with no souls says so', (tester) async {
    final client = FakeDaemonClient()..onQuery = (_) async => pb.QueryPage();
    await pumpApp(tester, client);
    expect(find.text(l.inventoryEmptyTitle), findsOneWidget);
  });

  testWidgets('no profile at all says so', (tester) async {
    final client = FakeDaemonClient()..onListProfiles = () async => pb.ProfileList();
    await pumpApp(tester, client);
    expect(find.text(l.inventoryNoProfileTitle), findsOneWidget);
    expect(find.text(l.statusNoProfile), findsOneWidget);
  });

  testWidgets('a failed query shows the text for its code and a retry', (tester) async {
    final client = FakeDaemonClient()
      ..onQuery = (_) async => throw daemonError('query.unknown_profile');
    await pumpApp(tester, client);
    expect(find.text(l.errorUnknownProfile), findsOneWidget);
    client.onQuery = (q) async => recordedFirstPage();
    await tester.tap(find.text(l.actionRetry));
    await tester.pumpAndSettle();
    expect(find.text(l.errorUnknownProfile), findsNothing);
    expect(find.text(l.inventoryPageRange(1, 8)), findsOneWidget);
  });

  testWidgets('an unmapped code shows the generic text with the code visible', (tester) async {
    final client = FakeDaemonClient()
      ..onQuery = (_) async => throw daemonError('query.something_new');
    await pumpApp(tester, client);
    expect(find.text(l.errorUnknown), findsOneWidget);
    expect(find.text(l.labelErrorCode('query.something_new')), findsOneWidget);
  });

  testWidgets('a failed core shows the banner, and retry restarts it', (tester) async {
    final client = FakeDaemonClient(
      health: DaemonFailed(DaemonException(ClientErrorCode.daemonNotFound, 'looked nowhere')),
    )..onListProfiles = () async => throw daemonError(ClientErrorCode.daemonNotFound);
    await pumpApp(tester, client);
    expect(find.text(l.bannerCoreFailed), findsOneWidget);
    expect(find.text(l.errorDaemonNotFound), findsWidgets);
    expect(find.text(l.statusCoreFailed), findsOneWidget);
    await tester.tap(find.text(l.actionRetry).first);
    await tester.pumpAndSettle();
    expect(client.restarts, 1);
  });

  testWidgets('a restarting core keeps the data and says why', (tester) async {
    final client = FakeDaemonClient();
    await pumpApp(tester, client);
    client.setHealth(DaemonStarting(attempt: 2, after: daemonError(ClientErrorCode.daemonExited)));
    await tester.pumpAndSettle();
    expect(find.textContaining(l.bannerCoreRestarting), findsOneWidget);
    expect(find.text(l.inventoryPageRange(1, 8)), findsOneWidget);
  });

  testWidgets('an imported scheme shows its plans and its QR code', (tester) async {
    await pumpApp(tester, FakeDaemonClient());
    await tester.tap(find.text(l.navSchemes));
    await tester.pumpAndSettle();
    expect(find.text(l.schemesImportedEmpty), findsOneWidget);
    expect(find.text(l.schemesGeneratedUnavailable), findsOneWidget);
    await tester.enterText(find.byType(TextField), recordedScheme().encoded.text);
    await tester.tap(find.text(l.actionImportText));
    await tester.pumpAndSettle();
    expect(find.text('spd'), findsWidgets);
    expect(find.text('six'), findsWidgets);
    expect(find.byType(QrMatrixView), findsOneWidget);
    expect(find.text(l.anySet), findsOneWidget);
  });

  testWidgets('choosing a scheme during an import keeps the import controls disabled', (
    tester,
  ) async {
    final client = FakeDaemonClient();
    await pumpApp(tester, client);
    await tester.tap(find.text(l.navSchemes));
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextField), 'first');
    await tester.tap(find.text(l.actionImportText));
    await tester.pumpAndSettle();
    final gate = Completer<pb.SchemeCodeDecoded>();
    var calls = 0;
    client.onDecode = (_) {
      calls++;
      return gate.future;
    };
    await tester.enterText(find.byType(TextField), 'second');
    await tester.tap(find.text(l.actionImportText));
    await tester.pump();
    // Choosing the scheme already imported must not end the running import.
    await tester.tap(find.text(l.schemeKindStrengthening).first);
    await tester.pump();
    final importButton = tester.widget<FilledButton>(
      find.widgetWithText(FilledButton, l.actionImportText),
    );
    expect(importButton.onPressed, isNull);
    expect(find.text(l.importing), findsOneWidget);
    await tester.tap(find.text(l.actionImportText), warnIfMissed: false);
    await tester.pump();
    expect(calls, 1);
    gate.complete(recordedScheme());
    await tester.pumpAndSettle();
    expect(find.text(l.importing), findsNothing);
  });

  testWidgets('a scheme that does not decode shows why', (tester) async {
    final client = FakeDaemonClient()
      ..onDecode = (_) async => throw daemonError('decode.malformed_text');
    await pumpApp(tester, client);
    await tester.tap(find.text(l.navSchemes));
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextField), 'not a code');
    await tester.tap(find.text(l.actionImportText));
    await tester.pumpAndSettle();
    expect(find.text(l.errorDecodeMalformedText), findsOneWidget);
  });
}
