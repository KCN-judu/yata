/// The filter pane's place in the workbench. The controls arrive with the query vocabulary in the
/// schema: each one edits `SoulQuerySpec` (`state/inventory.dart`), and the daemon filters.
library;

import 'package:flutter/material.dart';

import '../../gen/l10n/app_localizations.dart';

class FilterPane extends StatelessWidget {
  const FilterPane({super.key});

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(l.filterTitle, style: theme.textTheme.titleSmall),
          const SizedBox(height: 8),
          Text(l.filterPending, style: theme.textTheme.bodySmall),
        ],
      ),
    );
  }
}
