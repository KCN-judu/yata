/// A feature's page behind its capability: the page's content only when the selected profile can
/// use it, and otherwise why not (`core-protocol.md`, "Profiles and capabilities").
///
/// The gate reads the capability and nothing else. It never asks where the profile's data came
/// from, and it never shows an empty page for a section the profile does not hold.
library;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/core.dart';
import '../../state/profiles.dart';
import 'state_views.dart';

String sectionName(AppLocalizations l, SectionKind k) => switch (k) {
  SectionKind.souls => l.sectionSouls,
  SectionKind.shikigami => l.sectionShikigami,
  SectionKind.presets => l.sectionPresets,
  SectionKind.assets => l.sectionAssets,
  SectionKind.guild => l.sectionGuild,
};

/// The missing sections, in the order the daemon named them.
String sectionList(AppLocalizations l, List<SectionKind> sections) =>
    sections.map((k) => sectionName(l, k)).join(l.listSeparator);

/// The page of [capability] for the selected profile: [content] when it is available, with the
/// completeness of the sections behind it; otherwise the profile list's state, or the reason the
/// capability cannot be used. [feature] names the page in that reason.
class CapabilityGate extends ConsumerWidget {
  const CapabilityGate({
    super.key,
    required this.capability,
    required this.feature,
    required this.content,
  });

  final Capability capability;
  final String feature;
  final Widget Function(Completeness completeness) content;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final profiles = ref.watch(profilesProvider);
    final profile = ref.watch(currentProfileProvider);
    return switch ((profiles, profile)) {
      (AsyncValue(value: final list?), _) when list.isEmpty => EmptyView(
        icon: Icons.person_outline,
        title: l.noProfileTitle,
        body: l.noProfileBody,
      ),
      (AsyncValue(:final error?), _) when profiles.value == null => ErrorView(
        failure: failureOf(error),
        title: l.profilesLoadFailed,
        onRetry: () => ref.invalidate(profilesProvider),
      ),
      (_, null) => LoadingView(label: l.profilesLoading),
      (_, final Profile p) => switch (p.availability(capability)) {
        Available(:final completeness) => content(completeness),
        Unavailable(:final missing) => EmptyView(
          key: const ValueKey('capability-unavailable'),
          icon: Icons.upload_file_outlined,
          title: l.capabilityUnavailableTitle(sectionList(l, missing)),
          body: l.capabilityUnavailableBody(feature, sectionList(l, missing)),
        ),
        NotReported() => EmptyView(
          key: const ValueKey('capability-not-reported'),
          icon: Icons.help_outline,
          title: l.capabilityNotReportedTitle,
          body: l.capabilityNotReportedBody(feature),
        ),
      },
    };
  }
}

/// What a page over sections that may not hold everything says about them; nothing for complete
/// ones.
class CompletenessNote extends StatelessWidget {
  const CompletenessNote({super.key, required this.completeness});

  final Completeness completeness;

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final theme = Theme.of(context);
    final text = switch (completeness) {
      Completeness.complete => null,
      Completeness.partial => l.completenessPartial,
      Completeness.unstated => l.completenessUnstated,
    };
    if (text == null) return const SizedBox.shrink();
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(Icons.info_outline, size: 14, color: theme.colorScheme.tertiary),
        const SizedBox(width: 4),
        Text(text, style: theme.textTheme.bodySmall?.copyWith(color: theme.colorScheme.tertiary)),
      ],
    );
  }
}
