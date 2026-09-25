/// The window's bottom line: which profile, whether the core is running, where the data is.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/core.dart';
import '../../state/profiles.dart';

class StatusBar extends ConsumerWidget {
  const StatusBar({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final status = ref.watch(coreStatusProvider);
    final fixture = ref.watch(fixtureModeProvider);
    final profiles = ref.watch(profilesProvider).value ?? const [];
    final selected = ref.watch(selectedProfileProvider);
    final (icon, text) = switch (status) {
      CoreStopped() => (Icons.stop_circle_outlined, l.statusCoreStopped),
      CoreStarting() => (Icons.hourglass_empty, l.statusCoreStarting),
      CoreRestarting() => (Icons.sync, l.statusCoreRestarting),
      CoreReady() => (Icons.check_circle_outline, l.statusCoreReady),
      CoreFailed() => (Icons.error_outline, l.statusCoreFailed),
    };
    final style = theme.textTheme.bodySmall;
    return Material(
      color: theme.colorScheme.surfaceContainer,
      child: SizedBox(
        height: 28,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          child: Row(
            children: [
              if (profiles.isEmpty)
                Text(l.statusNoProfile, style: style)
              else
                DropdownButton<ProfileId>(
                  value: selected,
                  isDense: true,
                  underline: const SizedBox.shrink(),
                  style: style,
                  hint: Text(l.profileLabel, style: style),
                  items: [
                    for (final p in profiles)
                      DropdownMenuItem(value: ProfileId(p.id), child: Text(p.name)),
                  ],
                  onChanged: (id) {
                    if (id != null) ref.read(selectedProfileProvider.notifier).select(id);
                  },
                ),
              const SizedBox(width: 16),
              Icon(icon, size: 14),
              const SizedBox(width: 4),
              Text(text, style: style),
              if (fixture) ...[
                const SizedBox(width: 16),
                Text(l.statusFixture, style: style?.copyWith(color: theme.colorScheme.tertiary)),
              ],
              const Spacer(),
              Text(l.statusLocalOnly, style: style),
            ],
          ),
        ),
      ),
    );
  }
}
