/// The game accounts the daemon holds, and which one the views show.
library;

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../gen/proto/core.pb.dart' as pb;
import 'core.dart';

final profilesProvider = FutureProvider<List<pb.Profile>>((ref) async {
  ref.watch(sessionEpochProvider);
  final client = ref.watch(daemonClientProvider);
  final list = await coreCall(client.listProfiles);
  if (!ref.mounted) return list.profiles;
  final revision = Revision(list.revision);
  ref.read(projectionRevisionProvider.notifier).observe(revision);
  ref.listen(projectionRevisionProvider, (_, held) {
    if (isStale(revision, held)) ref.invalidateSelf();
  });
  return list.profiles;
});

/// The profile every per-profile view reads. View state: choosing one changes nothing in the
/// daemon. It follows the profile list: the first profile until the user picks another, and the
/// first again if the chosen one disappears.
class SelectedProfile extends Notifier<ProfileId?> {
  @override
  ProfileId? build() {
    final profiles = ref.watch(profilesProvider).value ?? const [];
    final previous = stateOrNull;
    if (previous != null && profiles.any((p) => p.id == previous.hex)) return previous;
    return profiles.isEmpty ? null : ProfileId(profiles.first.id);
  }

  void select(ProfileId id) => state = id;
}

final selectedProfileProvider = NotifierProvider<SelectedProfile, ProfileId?>(SelectedProfile.new);
