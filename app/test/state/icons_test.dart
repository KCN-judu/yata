// Icon resolution (ADR-0017): with no project SVG bundled, a local raster wins when the pack has
// the file, and the text mark is the designed fallback otherwise.

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:yata/platform/platform_services.dart';
import 'package:yata/state/core.dart';
import 'package:yata/state/icons.dart';

import '../support/fake_daemon.dart';

class _Pack extends FakePlatform {
  const _Pack();

  @override
  String? localIconPath(IconKind kind, IconRole role, int id) =>
      kind == IconKind.soulSet && role == IconRole.emblem && id == 30
      ? '/pack/soul-set/emblem/30.png'
      : null;
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  test('a local raster wins over the text mark; a key with no file gets the text mark', () async {
    final c = ProviderContainer.test(
      overrides: [platformServicesProvider.overrideWithValue(const _Pack() as PlatformServices)],
    );
    await c.read(projectIconsProvider.future);
    const emblem30 = IconKey(SoulSetIcon(30), IconRole.emblem);
    expect(
      c.read(iconSourceProvider(emblem30)),
      isA<LocalRasterIcon>().having((i) => i.filePath, 'path', '/pack/soul-set/emblem/30.png'),
    );
    expect(
      c.read(iconSourceProvider(const IconKey(SoulSetIcon(30), IconRole.portrait))),
      isA<TextMarkIcon>(),
    );
    expect(emblem30.assetPath, 'assets/icons/soul-set/emblem/30.svg');
  });
}
