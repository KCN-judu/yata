/// 御魂库: the detail pane, the soul list, and the filter pane (PRP-0002, "Workspace
/// structure"), with a toolbar holding the count and the page controls.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/core.dart';
import '../../state/inventory.dart';
import '../../state/profiles.dart';
import '../common/explanation.dart';
import '../common/state_views.dart';
import 'filter_pane.dart';
import 'soul_detail.dart';
import 'soul_list.dart';

class InventoryPage extends ConsumerWidget {
  const InventoryPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final profiles = ref.watch(profilesProvider);
    final spec = ref.watch(soulQuerySpecProvider);
    final Widget body = switch (profiles) {
      AsyncValue(value: final list?) when list.isEmpty => EmptyView(
        icon: Icons.person_outline,
        title: l.inventoryNoProfileTitle,
        body: l.inventoryNoProfileBody,
      ),
      AsyncValue(:final error?) when profiles.value == null => ErrorView(
        failure: failureOf(error),
        title: l.inventoryLoadFailed,
        onRetry: () => ref.invalidate(profilesProvider),
      ),
      _ when spec == null => LoadingView(label: l.inventoryLoading),
      _ => _Workbench(spec: spec),
    };
    return body;
  }
}

class _Workbench extends ConsumerWidget {
  const _Workbench({required this.spec});

  final SoulQuerySpec spec;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final pages = ref.watch(soulPagesProvider(spec));
    final list = switch (pages) {
      AsyncValue(value: final page?) when page.souls.isEmpty => EmptyView(
        icon: Icons.inventory_2_outlined,
        title: l.inventoryEmptyTitle,
        body: l.inventoryEmptyBody,
      ),
      AsyncValue(value: final page?) => SoulList(page: page),
      AsyncValue(:final error?) => ErrorView(
        failure: failureOf(error),
        title: l.inventoryLoadFailed,
        onRetry: () => ref.invalidate(soulPagesProvider(spec)),
      ),
      _ => LoadingView(label: l.inventoryLoading),
    };
    return Column(
      children: [
        _Toolbar(spec: spec, page: pages.value),
        const Divider(height: 1),
        Expanded(
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              const SizedBox(width: 288, child: SoulDetail()),
              const VerticalDivider(width: 1),
              Expanded(child: list),
              const VerticalDivider(width: 1),
              const SizedBox(width: 320, child: FilterPane()),
            ],
          ),
        ),
      ],
    );
  }
}

class _Toolbar extends ConsumerWidget {
  const _Toolbar({required this.spec, required this.page});

  final SoulQuerySpec spec;
  final SoulPage? page;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final p = page;
    final notifier = ref.read(soulPagesProvider(spec).notifier);
    return SizedBox(
      height: 44,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16),
        child: Row(
          children: [
            Text(l.inventoryTitle, style: theme.textTheme.titleSmall),
            const SizedBox(width: 12),
            if (p != null) Text(l.inventoryCount('${p.total}'), style: theme.textTheme.bodySmall),
            const Spacer(),
            if (p != null && p.souls.isNotEmpty) ...[
              if (p.turn case TurnFailed(:final failure))
                Padding(
                  padding: const EdgeInsets.only(right: 8),
                  child: Tooltip(
                    message: explain(l, failure).cause,
                    child: Icon(Icons.error_outline, size: 16, color: theme.colorScheme.error),
                  ),
                ),
              Text(
                l.inventoryPageRange(p.firstRow + 1, p.firstRow + p.souls.length),
                style: theme.textTheme.bodySmall,
              ),
              IconButton(
                tooltip: l.actionPreviousPage,
                icon: const Icon(Icons.chevron_left),
                onPressed: p.hasPrevious && !p.turning ? notifier.previous : null,
              ),
              IconButton(
                tooltip: l.actionNextPage,
                icon: const Icon(Icons.chevron_right),
                onPressed: p.hasMore && !p.turning ? notifier.next : null,
              ),
            ],
          ],
        ),
      ),
    );
  }
}
