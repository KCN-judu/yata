/// 方案: the imported and the generated scheme codes on the left, the chosen one on the right.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/core.dart';
import '../../state/schemes.dart';
import '../common/error_text.dart';
import '../common/labels.dart';
import 'scheme_detail.dart';

class SchemesPage extends ConsumerWidget {
  const SchemesPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final library = ref.watch(schemeLibraryProvider);
    final generated = ref.watch(generatedSchemesProvider);
    final notifier = ref.read(schemeLibraryProvider.notifier);
    return Column(
      children: [
        SizedBox(
          height: 44,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16),
            child: Row(children: [Text(l.schemesTitle, style: theme.textTheme.titleSmall)]),
          ),
        ),
        const Divider(height: 1),
        Expanded(
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              SizedBox(
                width: 320,
                child: ListView(
                  padding: const EdgeInsets.all(16),
                  children: [
                    const SchemeImportBox(),
                    const SizedBox(height: 16),
                    Text(l.schemesImported, style: theme.textTheme.titleSmall),
                    Text(l.schemesSessionOnly, style: theme.textTheme.bodySmall),
                    const SizedBox(height: 4),
                    if (library.imported.isEmpty)
                      Padding(
                        padding: const EdgeInsets.symmetric(vertical: 8),
                        child: Text(l.schemesImportedEmpty, style: theme.textTheme.bodyMedium),
                      ),
                    for (final s in library.imported)
                      ListTile(
                        dense: true,
                        selected: s.key == library.selectedKey,
                        title: Text(schemeKindName(l, s.decoded.kind)),
                        subtitle: Text(
                          [
                            l.schemeEntryCount(s.decoded.entries.length),
                            ...s.decoded.entries.take(2).map((e) => e.name),
                          ].join(' · '),
                          overflow: TextOverflow.ellipsis,
                        ),
                        onTap: () => notifier.select(s.key),
                        trailing: IconButton(
                          icon: const Icon(Icons.close, size: 16),
                          tooltip: l.actionRemove,
                          onPressed: () => notifier.remove(s.key),
                        ),
                      ),
                    const SizedBox(height: 16),
                    Text(l.schemesGenerated, style: theme.textTheme.titleSmall),
                    if (!generated.available)
                      Padding(
                        padding: const EdgeInsets.symmetric(vertical: 8),
                        child: Text(
                          l.schemesGeneratedUnavailable,
                          style: theme.textTheme.bodyMedium,
                        ),
                      ),
                  ],
                ),
              ),
              const VerticalDivider(width: 1),
              Expanded(
                child: switch (library.selected) {
                  final s? => SchemeDetail(decoded: s.decoded),
                  null => Center(
                    child: Text(l.schemeDetailEmpty, style: theme.textTheme.bodyMedium),
                  ),
                },
              ),
            ],
          ),
        ),
      ],
    );
  }
}

/// Paste the text of a scheme code, or pick its QR image. The daemon decodes both.
class SchemeImportBox extends ConsumerStatefulWidget {
  const SchemeImportBox({super.key});

  @override
  ConsumerState<SchemeImportBox> createState() => _SchemeImportBoxState();
}

class _SchemeImportBoxState extends ConsumerState<SchemeImportBox> {
  final _text = TextEditingController();

  @override
  void dispose() {
    _text.dispose();
    super.dispose();
  }

  Future<void> _importText() async {
    final text = _text.text.trim();
    if (text.isEmpty) return;
    await ref.read(schemeLibraryProvider.notifier).importText(text);
    if (mounted && ref.read(schemeLibraryProvider).importError == null) _text.clear();
  }

  Future<void> _importImage() async {
    final png = await ref.read(platformServicesProvider).pickPngImage();
    if (png != null) await ref.read(schemeLibraryProvider.notifier).importPng(png);
  }

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final library = ref.watch(schemeLibraryProvider);
    final error = library.importError;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        TextField(
          controller: _text,
          minLines: 2,
          maxLines: 4,
          enabled: !library.importing,
          decoration: InputDecoration(
            hintText: l.importTextHint,
            border: const OutlineInputBorder(),
          ),
        ),
        const SizedBox(height: 8),
        Row(
          children: [
            FilledButton(
              onPressed: library.importing ? null : _importText,
              child: Text(l.actionImportText),
            ),
            const SizedBox(width: 8),
            OutlinedButton(
              onPressed: library.importing ? null : _importImage,
              child: Text(l.actionImportImage),
            ),
          ],
        ),
        if (library.importing)
          Padding(padding: const EdgeInsets.only(top: 8), child: Text(l.importing)),
        if (error != null)
          Padding(
            padding: const EdgeInsets.only(top: 8),
            child: Text(
              mappedErrorText(l, error.code) ?? '${l.errorUnknown} ${l.labelErrorCode(error.code)}',
              style: theme.textTheme.bodySmall?.copyWith(color: theme.colorScheme.error),
            ),
          ),
      ],
    );
  }
}
