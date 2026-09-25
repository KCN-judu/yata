/// 式神录: the Shikigami the account owns, behind the Shikigami collection capability. The protocol
/// carries no Shikigami rows yet, so an available collection says that the list is still to come
/// rather than showing an empty table. Which souls a Shikigami wears is never imported (ADR-0031,
/// rule 9), and this page does not promise it.
library;

import 'package:flutter/material.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/profiles.dart';
import '../common/capability_views.dart';
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
          child: CapabilityGate(
            capability: Capability.shikigamiCollection,
            feature: l.navShikigami,
            content: (completeness) => Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                EmptyView(
                  key: const ValueKey('shikigami-pending'),
                  icon: Icons.people_outline,
                  title: l.shikigamiPendingTitle,
                  body: l.shikigamiPendingBody,
                ),
                const SizedBox(height: 8),
                CompletenessNote(completeness: completeness),
              ],
            ),
          ),
        ),
      ],
    );
  }
}
