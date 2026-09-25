/// One decoded scheme code: its QR code, its text, and each plan's selection as the game's panel
/// groups. Every value is the daemon's decode; nothing here reads a bit.
library;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../gen/proto/core.pb.dart' as pb;
import '../common/labels.dart';
import 'qr_matrix_view.dart';

class SchemeDetail extends StatelessWidget {
  const SchemeDetail({super.key, required this.decoded});

  final pb.SchemeCodeDecoded decoded;

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final encoded = decoded.encoded;
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        Text(schemeKindName(l, decoded.kind), style: theme.textTheme.titleMedium),
        Text(l.schemeEntryCount(decoded.entries.length), style: theme.textTheme.bodySmall),
        const SizedBox(height: 16),
        Text(l.schemeQrTitle, style: theme.textTheme.titleSmall),
        const SizedBox(height: 8),
        Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (encoded.hasQr())
              QrMatrixView(matrix: encoded.qr, size: 200, semanticLabel: l.schemeQrLabel)
            else
              SizedBox(width: 200, child: Text(l.schemeQrUnavailable)),
            const SizedBox(width: 16),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  SelectableText(encoded.text, maxLines: 6, style: theme.textTheme.bodySmall),
                  TextButton.icon(
                    icon: const Icon(Icons.copy, size: 16),
                    label: Text(l.actionCopyText),
                    onPressed: () async {
                      await Clipboard.setData(ClipboardData(text: encoded.text));
                      if (context.mounted) {
                        ScaffoldMessenger.of(
                          context,
                        ).showSnackBar(SnackBar(content: Text(l.copied)));
                      }
                    },
                  ),
                ],
              ),
            ),
          ],
        ),
        const SizedBox(height: 16),
        for (final entry in decoded.entries) _Entry(entry: entry),
      ],
    );
  }
}

class _Entry extends StatelessWidget {
  const _Entry({required this.entry});

  final pb.SchemeEntry entry;

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final s = entry.selection;
    String list(Iterable<String> values) => values.isEmpty ? l.groupAny : values.join('、');
    final groups = [
      (
        l.groupSets,
        switch (s.whichSets()) {
          pb.SoulSelection_Sets.all => l.anySet,
          pb.SoulSelection_Sets.chosen => list(s.chosen.codes.map((c) => setName(l, c))),
          pb.SoulSelection_Sets.notSet => l.valueUnknown,
        },
      ),
      (l.groupSlots, list(s.slots.map((k) => slotName(l, k)))),
      (l.groupStars, list(s.stars.map(l.soulStar))),
      (l.groupLevels, list(s.levels.map((b) => levelBandName(l, b)))),
      (l.groupMain, list(s.mainAttributes.map((a) => attributeName(l, a)))),
      (l.groupInnate, list(s.innate.map((a) => attributeName(l, a)))),
      (
        l.groupSubs,
        list([
          for (final c in s.subAttributes)
            if (c.mode == pb.SubAttributeMode.SUB_ATTRIBUTE_MODE_INCLUDE)
              l.subInclude(attributeName(l, c.attribute))
            else if (c.mode == pb.SubAttributeMode.SUB_ATTRIBUTE_MODE_EXCLUDE)
              l.subExclude(attributeName(l, c.attribute)),
        ]),
      ),
      (l.groupCounts, list(s.subCounts.map((c) => subCountName(l, c)))),
    ];
    return Padding(
      padding: const EdgeInsets.only(bottom: 16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(entry.name, style: theme.textTheme.titleSmall),
          if (entry.hasUnknownConditions)
            Row(
              children: [
                Icon(Icons.info_outline, size: 14, color: theme.colorScheme.tertiary),
                const SizedBox(width: 4),
                Expanded(child: Text(l.schemeUnknownConditions, style: theme.textTheme.bodySmall)),
              ],
            ),
          for (final (label, value) in groups)
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 2),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  SizedBox(width: 72, child: Text(label, style: theme.textTheme.labelMedium)),
                  Expanded(child: Text(value, style: theme.textTheme.bodyMedium)),
                ],
              ),
            ),
        ],
      ),
    );
  }
}
