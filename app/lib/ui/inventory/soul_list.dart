/// The page of souls as rows, in the order the daemon returned them. Selecting a row shows the
/// soul in the detail pane; it changes nothing in the daemon.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../gen/proto/core.pb.dart' as pb;
import '../../state/inventory.dart';
import '../common/labels.dart';
import '../icons/soul_set_mark.dart';

class SoulList extends ConsumerWidget {
  const SoulList({super.key, required this.page});

  final SoulPage page;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final selected = ref.watch(selectedSoulProvider)?.soulId;
    return Column(
      children: [
        if (page.turning) const LinearProgressIndicator(minHeight: 2),
        Expanded(
          child: ListView.builder(
            itemCount: page.souls.length,
            itemExtent: 36,
            itemBuilder: (context, i) {
              final soul = page.souls[i];
              return SoulRow(
                soul: soul,
                selected: soul.soulId == selected,
                onTap: () => ref.read(selectedSoulProvider.notifier).select(soul),
              );
            },
          ),
        ),
      ],
    );
  }
}

class SoulRow extends StatelessWidget {
  const SoulRow({super.key, required this.soul, required this.selected, required this.onTap});

  final pb.Soul soul;
  final bool selected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final style = theme.textTheme.bodyMedium;
    final subs = soul.subs
        .map((s) => '${attributeName(l, s.attribute)} ${attributeValue(s.attribute, s.value)}')
        .join('  ');
    return Semantics(
      label: soulAccessibleName(l, soul),
      selected: selected,
      button: true,
      child: Material(
        color: selected ? theme.colorScheme.secondaryContainer : Colors.transparent,
        child: InkWell(
          onTap: onTap,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12),
            child: Row(
              children: [
                SoulSetMark(suitCode: soul.suitCode, size: 24),
                const SizedBox(width: 8),
                SizedBox(width: 72, child: Text(setName(l, soul.suitCode), style: style)),
                SizedBox(width: 56, child: Text(slotName(l, soul.slot), style: style)),
                SizedBox(width: 40, child: Text(l.soulStar(soul.star), style: style)),
                SizedBox(width: 40, child: Text(l.soulLevel(soul.level), style: style)),
                SizedBox(
                  width: 140,
                  child: Text(
                    '${attributeName(l, soul.main)} '
                    '${attributeValue(soul.main, soul.mainValue)}',
                    style: style,
                  ),
                ),
                Expanded(
                  child: Text(
                    subs,
                    style: theme.textTheme.bodySmall,
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
