/// 式神录: the Shikigami the account owns. The protocol carries no Shikigami yet, so the place
/// says so instead of showing an empty table.
library;

import 'package:flutter/material.dart';

import '../../gen/l10n/app_localizations.dart';
import '../common/state_views.dart';

class ShikigamiPage extends StatelessWidget {
  const ShikigamiPage({super.key});

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    return Column(
      children: [
        SizedBox(
          height: 44,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16),
            child: Row(children: [Text(l.shikigamiTitle, style: theme.textTheme.titleSmall)]),
          ),
        ),
        const Divider(height: 1),
        Expanded(
          child: EmptyView(
            icon: Icons.people_outline,
            title: l.shikigamiUnavailableTitle,
            body: l.shikigamiUnavailableBody,
          ),
        ),
      ],
    );
  }
}
