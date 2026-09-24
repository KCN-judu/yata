/// The selected soul, as the daemon described it. Scores join this pane when pass 1 exists; the
/// pane shows no number the daemon did not send.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/icons.dart';
import '../../state/inventory.dart';
import '../common/labels.dart';
import '../icons/soul_set_mark.dart';

class SoulDetail extends ConsumerWidget {
  const SoulDetail({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final soul = ref.watch(selectedSoulProvider);
    if (soul == null) {
      return Center(child: Text(l.detailEmpty, style: theme.textTheme.bodyMedium));
    }
    Widget row(String label, String value) => Padding(
      padding: const EdgeInsets.symmetric(vertical: 3),
      child: Row(
        children: [
          SizedBox(width: 72, child: Text(label, style: theme.textTheme.labelMedium)),
          Expanded(child: Text(value, style: theme.textTheme.bodyMedium)),
        ],
      ),
    );
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        Row(
          children: [
            SoulSetMark(suitCode: soul.suitCode, role: IconRole.portrait, size: 56),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(setName(l, soul.suitCode), style: theme.textTheme.titleMedium),
                  Text(
                    '${slotName(l, soul.slot)} · ${l.soulStar(soul.star)} · ${l.soulLevel(soul.level)}',
                    style: theme.textTheme.bodySmall,
                  ),
                ],
              ),
            ),
          ],
        ),
        const SizedBox(height: 16),
        row(
          l.labelMain,
          '${attributeName(l, soul.main)} ${attributeValue(soul.main, soul.mainValue)}',
        ),
        row(l.labelInnate, innateText(l, soul)),
        const SizedBox(height: 8),
        Text(l.labelSubs, style: theme.textTheme.labelMedium),
        if (soul.subs.isEmpty) row('', l.labelNone),
        for (final s in soul.subs)
          row(
            attributeName(l, s.attribute),
            [
              attributeValue(s.attribute, s.value),
              if (s.hasEnhancementCount()) l.soulEnhancements(s.enhancementCount),
            ].join('  '),
          ),
      ],
    );
  }
}
