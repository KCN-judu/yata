/// The application widget: localization, theme, and the shell.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../gen/l10n/app_localizations.dart';
import '../state/settings.dart';
import 'shell/app_shell.dart';
import 'theme.dart';

class YataApp extends ConsumerWidget {
  const YataApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final settings = ref.watch(settingsProvider);
    return MaterialApp(
      onGenerateTitle: (context) => AppLocalizations.of(context).appTitle,
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
      locale: const Locale('zh'),
      theme: yataTheme(Brightness.light),
      darkTheme: yataTheme(Brightness.dark),
      themeMode: settings.themeMode,
      home: const AppShell(),
    );
  }
}
