/// The states a surface can be in besides showing data: loading, empty with a reason, and failed
/// with a code. Every surface uses these, so no view invents its own spinner or error text.
library;

import 'package:flutter/material.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/core.dart';
import 'error_text.dart';

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

/// A failure: the text for its code, the code itself when it has no text of its own, and the
/// English message behind a details toggle, never as the main text.
class ErrorView extends StatefulWidget {
  const ErrorView({super.key, required this.error, this.title, this.onRetry, this.log = const []});

  final CoreError error;
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
    final e = widget.error;
    final mapped = mappedErrorText(l, e.code);
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 480),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(Icons.error_outline, size: 32, color: theme.colorScheme.error),
            const SizedBox(height: 12),
            if (widget.title != null) Text(widget.title!, style: theme.textTheme.titleMedium),
            Text(mapped ?? l.errorUnknown, textAlign: TextAlign.center),
            if (mapped == null)
              SelectableText(l.labelErrorCode(e.code), style: theme.textTheme.bodySmall),
            const SizedBox(height: 12),
            Wrap(
              spacing: 8,
              children: [
                if (widget.onRetry != null)
                  FilledButton(onPressed: widget.onRetry, child: Text(l.actionRetry)),
                TextButton(
                  onPressed: () => setState(() => _details = !_details),
                  child: Text(_details ? l.actionHideDetails : l.actionShowDetails),
                ),
              ],
            ),
            if (_details) ...[
              SelectableText(l.labelErrorCode(e.code), style: theme.textTheme.bodySmall),
              SelectableText(l.labelErrorMessage(e.message), style: theme.textTheme.bodySmall),
              if (widget.log.isNotEmpty) ...[
                const SizedBox(height: 8),
                Text(l.labelCoreLog, style: theme.textTheme.labelMedium),
                SelectableText(widget.log.join('\n'), style: theme.textTheme.bodySmall),
              ],
            ],
          ],
        ),
      ),
    );
  }
}
