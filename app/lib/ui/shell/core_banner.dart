/// The notice above every surface when the core is not simply running: restarting, restarted,
/// failed, or sending a warning (PRP-0002, "States": the daemon restarting).
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/core.dart';
import '../common/error_text.dart';
import '../common/state_views.dart';

class CoreBanner extends ConsumerWidget {
  const CoreBanner({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final status = ref.watch(coreStatusProvider);
    final warnings = ref.watch(coreWarningsProvider);
    final restart = ref.read(coreStatusProvider.notifier).restart;
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        switch (status) {
          CoreStarting(cause: final cause?) => _Strip(
            icon: Icons.sync,
            text: '${l.bannerCoreRestarting} ${errorText(l, cause.code)}',
          ),
          CoreReady(restarted: true) => _Strip(
            icon: Icons.info_outline,
            text: l.bannerCoreRestarted,
          ),
          CoreFailed(:final error, :final log) => SizedBox(
            height: 280,
            child: ErrorView(error: error, title: l.bannerCoreFailed, onRetry: restart, log: log),
          ),
          _ => const SizedBox.shrink(),
        },
        for (final w in warnings)
          _Strip(
            icon: Icons.warning_amber_outlined,
            text: warningText(l, w.code),
            onDismiss: () => ref.read(coreWarningsProvider.notifier).dismiss(w),
          ),
      ],
    );
  }
}

class _Strip extends StatelessWidget {
  const _Strip({required this.icon, required this.text, this.onDismiss});

  final IconData icon;
  final String text;
  final VoidCallback? onDismiss;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Material(
      color: theme.colorScheme.surfaceContainerHigh,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        child: Row(
          children: [
            Icon(icon, size: 16),
            const SizedBox(width: 8),
            Expanded(child: Text(text, style: theme.textTheme.bodyMedium)),
            if (onDismiss != null)
              IconButton(
                icon: const Icon(Icons.close, size: 16),
                tooltip: AppLocalizations.of(context).actionDismiss,
                onPressed: onDismiss,
              ),
          ],
        ),
      ),
    );
  }
}
