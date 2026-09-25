/// 设置: appearance, and what the core is and where its data comes from.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/core.dart';
import '../../state/settings.dart';

class SettingsPage extends ConsumerWidget {
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final settings = ref.watch(settingsProvider);
    final status = ref.watch(coreStatusProvider);
    final fixture = ref.watch(fixtureModeProvider);
    return Column(
      children: [
        SizedBox(
          height: 44,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16),
            child: Row(children: [Text(l.settingsTitle, style: theme.textTheme.titleSmall)]),
          ),
        ),
        const Divider(height: 1),
        Expanded(
          child: ListView(
            padding: const EdgeInsets.all(16),
            children: [
              Text(l.settingsAppearance, style: theme.textTheme.titleSmall),
              const SizedBox(height: 8),
              SegmentedButton<ThemeMode>(
                segments: [
                  ButtonSegment(value: ThemeMode.system, label: Text(l.themeSystem)),
                  ButtonSegment(value: ThemeMode.light, label: Text(l.themeLight)),
                  ButtonSegment(value: ThemeMode.dark, label: Text(l.themeDark)),
                ],
                selected: {settings.themeMode},
                onSelectionChanged: (s) =>
                    ref.read(settingsProvider.notifier).setThemeMode(s.first),
              ),
              const SizedBox(height: 24),
              Text(l.settingsCore, style: theme.textTheme.titleSmall),
              const SizedBox(height: 8),
              Text(switch (status) {
                CoreReady(:final version) => '${l.statusCoreReady} · ${l.coreProtocol(version)}',
                CoreStarting() => l.statusCoreStarting,
                CoreRestarting() => l.statusCoreRestarting,
                CoreFailed() => l.statusCoreFailed,
                CoreStopped() => l.statusCoreStopped,
              }),
              Text('${l.coreDataSource}：${fixture ? l.coreDataFixture : l.coreDataStore}'),
              const SizedBox(height: 8),
              Align(
                alignment: Alignment.centerLeft,
                child: OutlinedButton(
                  onPressed: ref.read(coreStatusProvider.notifier).restart,
                  child: Text(l.actionRestartCore),
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}
