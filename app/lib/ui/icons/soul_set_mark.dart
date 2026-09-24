/// A soul set's icon, from whichever source ADR-0017's order finds first.
library;

import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_svg/flutter_svg.dart';

import '../../gen/l10n/app_localizations.dart';
import '../../state/icons.dart';
import '../common/labels.dart';

class SoulSetMark extends ConsumerWidget {
  const SoulSetMark({
    super.key,
    required this.suitCode,
    this.role = IconRole.emblem,
    this.size = 28,
  });

  final int suitCode;
  final IconRole role;
  final double size;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = AppLocalizations.of(context);
    final source = ref.watch(iconSourceProvider(IconKey(IconKind.soulSet, role, suitCode)));
    final label = setName(l, suitCode);
    final child = switch (source) {
      ProjectSvgIcon(:final assetPath) => SvgPicture.asset(assetPath, width: size, height: size),
      LocalRasterIcon(:final filePath) => Image.file(
        File(filePath),
        width: size,
        height: size,
        errorBuilder: (_, _, _) => _TextMark(suitCode: suitCode, size: size),
      ),
      TextMarkIcon() => _TextMark(suitCode: suitCode, size: size),
    };
    return Semantics(
      label: label,
      image: true,
      child: ExcludeSemantics(child: child),
    );
  }
}

/// The designed fallback: a neutral tile. It carries the suit code until the core supplies set
/// names, whose first character it will then show.
class _TextMark extends StatelessWidget {
  const _TextMark({required this.suitCode, required this.size});

  final int suitCode;
  final double size;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Container(
      width: size,
      height: size,
      alignment: Alignment.center,
      decoration: BoxDecoration(
        color: theme.colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(6),
      ),
      child: Text('$suitCode', style: theme.textTheme.labelSmall?.copyWith(fontSize: size * 0.36)),
    );
  }
}
