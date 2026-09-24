/// The game accounts the daemon holds, and which one the views show.
library;

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../gen/proto/core.pb.dart' as pb;
import 'core.dart';

final profilesProvider = FutureProvider<List<pb.Profile>>((ref) async {
  ref.watch(sessionEpochProvider);
  final client = ref.watch(daemonClientProvider);
  final list = await coreCall(client.listProfiles());
  ref.read(projectionRevisionProvider.notifier).observe(list.revision);
  ref.listen(projectionRevisionProvider, (_, held) {
    if (list.revision < held) ref.invalidateSelf();
  });
  return list.profiles;
});

/// The profile every per-profile view reads. View state: choosing one changes nothing in the
/// daemon. It follows the profile list: the first profile until the user picks another, and the
/// first again if the chosen one disappears.
class SelectedProfile extends Notifier<String?> {
  @override
  String? build() {
    final profiles = ref.watch(profilesProvider).value ?? const [];
    final previous = stateOrNull;
    if (previous != null && profiles.any((p) => p.id == previous)) return previous;
    return profiles.isEmpty ? null : profiles.first.id;
  }

  void select(String id) => state = id;
}

final selectedProfileProvider = NotifierProvider<SelectedProfile, String?>(SelectedProfile.new);
