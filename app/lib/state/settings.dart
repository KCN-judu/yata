/// Application preferences. View state only: nothing here reaches the daemon.
///
/// Held for the running session; persisting them belongs to the data directory (ADR-0010),
/// which the application does not write yet.
library;

import 'package:flutter/material.dart' show ThemeMode;
import 'package:flutter_riverpod/flutter_riverpod.dart';

final class AppSettings {
  const AppSettings({this.themeMode = ThemeMode.system});

  /// System by default, with a light or dark override (PRP-0002, "Theme mode").
  final ThemeMode themeMode;
}

class Settings extends Notifier<AppSettings> {
  @override
  AppSettings build() => const AppSettings();

  void setThemeMode(ThemeMode mode) => state = AppSettings(themeMode: mode);
}

final settingsProvider = NotifierProvider<Settings, AppSettings>(Settings.new);
