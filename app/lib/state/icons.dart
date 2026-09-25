/// Icon resolution (ADR-0017): a project SVG, else a local raster, else the designed text mark.
///
/// Icons are presentation, keyed by the game's identifier, never by a name. The daemon supplies
/// only the identifiers; this file decides which source draws them.
library;

import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../platform/platform_services.dart';
import 'core.dart';

export '../platform/platform_services.dart' show IconKind, IconRole;

/// What an icon shows, with the game identifier of that kind: the id's meaning is its case.
sealed class IconSubject {
  const IconSubject();

  IconKind get kind;
  int get id;
}

final class SoulSetIcon extends IconSubject {
  const SoulSetIcon(this.suitCode);

  final int suitCode;

  @override
  IconKind get kind => IconKind.soulSet;

  @override
  int get id => suitCode;

  @override
  bool operator ==(Object other) => other is SoulSetIcon && other.suitCode == suitCode;

  @override
  int get hashCode => suitCode.hashCode;
}

final class ShikigamiIcon extends IconSubject {
  const ShikigamiIcon(this.shikigamiId);

  final int shikigamiId;

  @override
  IconKind get kind => IconKind.shikigami;

  @override
  int get id => shikigamiId;

  @override
  bool operator ==(Object other) => other is ShikigamiIcon && other.shikigamiId == shikigamiId;

  @override
  int get hashCode => Object.hash(IconKind.shikigami, shikigamiId);
}

final class IconKey {
  const IconKey(this.subject, this.role);

  final IconSubject subject;
  final IconRole role;

  String get assetPath => 'assets/icons/${subject.kind.folder}/${role.folder}/${subject.id}.svg';

  @override
  bool operator ==(Object other) =>
      other is IconKey && other.subject == subject && other.role == role;

  @override
  int get hashCode => Object.hash(subject, role);
}

sealed class IconSource {
  const IconSource();
}

final class ProjectSvgIcon extends IconSource {
  const ProjectSvgIcon(this.assetPath);

  final String assetPath;
}

final class LocalRasterIcon extends IconSource {
  const LocalRasterIcon(this.filePath);

  final String filePath;
}

/// No file for the key: the designed text mark (a normal state, not an error).
final class TextMarkIcon extends IconSource {
  const TextMarkIcon();
}

/// The project icons bundled with the application.
final projectIconsProvider = FutureProvider<Set<String>>((ref) async {
  final manifest = await AssetManifest.loadFromAssetBundle(rootBundle);
  return manifest.listAssets().where((a) => a.startsWith('assets/icons/')).toSet();
});

final iconSourceProvider = Provider.family<IconSource, IconKey>((ref, key) {
  final bundled = ref.watch(projectIconsProvider).value ?? const {};
  if (bundled.contains(key.assetPath)) return ProjectSvgIcon(key.assetPath);
  final local = ref
      .watch(platformServicesProvider)
      .localIconPath(key.subject.kind, key.role, key.subject.id);
  if (local != null) return LocalRasterIcon(local);
  return const TextMarkIcon();
});
