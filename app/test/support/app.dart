/// The whole application over a fake daemon client, for the widget tests.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yata/gen/l10n/app_localizations.dart';
import 'package:yata/ui/app.dart';

import 'fake_daemon.dart';

/// The only locale's text, to find what the views show.
final l = lookupAppLocalizations(const Locale('zh'));

/// The application at a desktop size, behind [client], settled.
Future<void> pumpApp(WidgetTester tester, FakeDaemonClient client) async {
  tester.view.physicalSize = const Size(1440, 900);
  tester.view.devicePixelRatio = 1;
  addTearDown(tester.view.reset);
  await tester.pumpWidget(
    ProviderScope(overrides: fakeOverrides(client), retry: (_, _) => null, child: const YataApp()),
  );
  await tester.pumpAndSettle();
}
