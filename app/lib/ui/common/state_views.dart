/// The states a surface can be in besides showing data: loading, empty with a reason, and failed
/// with a code. Every surface uses these, so no view invents its own spinner or error text.
library;

import 'package:flutter/material.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/core.dart';
import 'explanation.dart';

class LoadingView extends StatelessWidget {
  const LoadingView({super.key, required this.label});

  final String label;

  @override
  Widget build(BuildContext context) => Center(
    child: Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2)),
        const SizedBox(height: 12),
        Text(label, style: Theme.of(context).textTheme.bodyMedium),
      ],
    ),
  );
}

/// An empty surface that says why it is empty.
class EmptyView extends StatelessWidget {
  const EmptyView({super.key, required this.title, required this.body, this.icon});

  final String title;
  final String body;
  final IconData? icon;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 360),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (icon != null) Icon(icon, size: 32, color: theme.colorScheme.outline),
            const SizedBox(height: 12),
            Text(title, style: theme.textTheme.titleMedium, textAlign: TextAlign.center),
            const SizedBox(height: 4),
            Text(body, style: theme.textTheme.bodyMedium, textAlign: TextAlign.center),
          ],
        ),
      ),
    );
  }
}

/// A failure: its cause and remedy as the main text; the code, the English message, and the
/// debug record behind a details toggle. An unknown code is shown with the code visible.
class ErrorView extends StatefulWidget {
  const ErrorView({
    super.key,
    required this.failure,
    this.title,
    this.onRetry,
    this.log = const [],
  });

  final CoreFailure failure;
  final String? title;
  final VoidCallback? onRetry;
  final List<String> log;

  @override
  State<ErrorView> createState() => _ErrorViewState();
}

class _ErrorViewState extends State<ErrorView> {
  bool _details = false;

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final f = widget.failure;
    final explanation = explain(l, f);
    final small = theme.textTheme.bodySmall;
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 480),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.error_outline, size: 32, color: theme.colorScheme.error),
            const SizedBox(height: 12),
            if (widget.title case final title?) Text(title, style: theme.textTheme.titleMedium),
            Text(explanation.cause, textAlign: TextAlign.center),
            Text(explanation.remedy, textAlign: TextAlign.center, style: small),
            if (!isKnown(f)) SelectableText(l.labelErrorCode(f.code), style: small),
            const SizedBox(height: 12),
            Wrap(
              spacing: 8,
              children: [
                if (widget.onRetry case final retry?)
                  FilledButton(onPressed: retry, child: Text(l.actionRetry)),
                TextButton(
                  onPressed: () => setState(() => _details = !_details),
                  child: Text(_details ? l.actionHideDetails : l.actionShowDetails),
                ),
              ],
            ),
            if (_details) ...[
              SelectableText(l.labelErrorCode(f.code), style: small),
              SelectableText(l.labelErrorMessage(f.message), style: small),
              if (f.debugRecord.isNotEmpty) SelectableText(f.debugRecord, style: small),
              if (widget.log.isNotEmpty) ...[
                const SizedBox(height: 8),
                Text(l.labelCoreLog, style: theme.textTheme.labelMedium),
                SelectableText(widget.log.join('\n'), style: small),
              ],
            ],
          ],
        ),
      ),
    );
  }
}
