/// Icon resolution (ADR-0017): a project SVG, else a local raster, else the designed text mark.
///
/// Icons are presentation, keyed by the game's identifier, never by a name. The daemon supplies
/// only the identifiers; this file decides which source draws them.
library;

import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'core.dart';

enum IconKind {
  soulSet('soul-set'),
  shikigami('shikigami');

  const IconKind(this.path);

  final String path;
}

enum IconRole {
  /// Beside text: tables, lists, filters.
  emblem('emblem'),

  /// Large: the inspector and detail views.
  portrait('portrait');

  const IconRole(this.path);

  final String path;
}

final class IconKey {
  const IconKey(this.kind, this.role, this.id);

  final IconKind kind;
  final IconRole role;

  /// The suit code of a soul set, or the game's Shikigami id.
  final int id;

  String get assetPath => 'assets/icons/${kind.path}/${role.path}/$id.svg';

  @override
  bool operator ==(Object other) =>
      other is IconKey && other.kind == kind && other.role == role && other.id == id;

  @override
  int get hashCode => Object.hash(kind, role, id);
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
      .localIconPath(key.kind.path, key.role.path, key.id);
  if (local != null) return LocalRasterIcon(local);
  return const TextMarkIcon();
});
