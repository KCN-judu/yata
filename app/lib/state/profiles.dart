/// The game accounts the daemon holds, what each can do, and which one the views show.
///
/// A profile's capabilities are parsed here, once, from the wire (`core-protocol.md`, "Profiles
/// and capabilities"). Whether a page shows its content depends on them alone: nothing in the
/// application reads a file's format, and nothing recomputes which sections a capability needs.
library;

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../gen/proto/core.pb.dart' as pb;
import 'core.dart';

/// What a profile can do.
enum Capability {
  inventory(pb.Capability.CAPABILITY_INVENTORY),
  shikigamiCollection(pb.Capability.CAPABILITY_SHIKIGAMI_COLLECTION),
  gamePresets(pb.Capability.CAPABILITY_GAME_PRESETS),
  assets(pb.Capability.CAPABILITY_ASSETS),
  guildView(pb.Capability.CAPABILITY_GUILD_VIEW);

  const Capability(this.wire);

  final pb.Capability wire;
}

/// A section of an imported snapshot.
enum SectionKind {
  souls(pb.SectionKind.SECTION_KIND_SOULS),
  shikigami(pb.SectionKind.SECTION_KIND_SHIKIGAMI),
  presets(pb.SectionKind.SECTION_KIND_PRESETS),
  assets(pb.SectionKind.SECTION_KIND_ASSETS),
  guild(pb.SectionKind.SECTION_KIND_GUILD);

  const SectionKind(this.wire);

  final pb.SectionKind wire;
}

/// Whether the sections behind a capability hold everything they cover, by the file's own
/// statement. Weakest first.
enum Completeness {
  unstated(pb.Completeness.COMPLETENESS_UNSTATED),
  partial(pb.Completeness.COMPLETENESS_PARTIAL),
  complete(pb.Completeness.COMPLETENESS_COMPLETE);

  const Completeness(this.wire);

  final pb.Completeness wire;
}

/// Whether a profile can use a capability.
sealed class Availability {
  const Availability();
}

/// Every section the capability needs is held; [completeness] is the weakest of them.
final class Available extends Availability {
  const Available(this.completeness);

  final Completeness completeness;
}

/// The profile lacks [missing]: at least one section, each once. Shown as such, never as an
/// empty page.
final class Unavailable extends Availability {
  const Unavailable._(this.missing);

  final List<SectionKind> missing;
}

/// The daemon did not report the capability: it was built against an older schema than this
/// application.
final class NotReported extends Availability {
  const NotReported();
}

/// One game account (GameProfile), as the views read it.
final class Profile {
  const Profile._(this.id, this.name, this._capabilities);

  final ProfileId id;

  /// The user's own name for it.
  final String name;

  final Map<Capability, Availability> _capabilities;

  Availability availability(Capability c) => _capabilities[c] ?? const NotReported();
}

/// A wire profile as the views' [Profile]. A capability this build does not know, from a newer
/// schema, is skipped; anything left unstated or stated twice is the daemon breaking the
/// protocol, and fails with `client.protocol_error`.
Profile profileOf(pb.Profile p) {
  final capabilities = <Capability, Availability>{};
  for (final entry in p.capabilities) {
    final parsed = _capabilityOf(entry);
    if (parsed == null) continue;
    final (capability, availability) = parsed;
    if (capabilities.containsKey(capability)) {
      throw RaisedFailure.protocolError('profile ${p.id}: capability $capability twice');
    }
    capabilities[capability] = availability;
  }
  return Profile._(ProfileId(p.id), p.name, Map.unmodifiable(capabilities));
}

/// Tag of `ProfileCapability.capability`.
const _capabilityTag = 1;

/// The entry's capability and availability; `null` for a capability this build does not know.
(Capability, Availability)? _capabilityOf(pb.ProfileCapability entry) {
  final Capability capability;
  switch (_known(Capability.values, (c) => c.wire, entry.capability)) {
    case final c?:
      capability = c;
    case null when entry.unknownFields.hasField(_capabilityTag):
      // A value from a newer schema: the runtime keeps it among the unknown fields.
      return null;
    case null:
      throw RaisedFailure.protocolError('a capability left unspecified');
  }
  final availability = switch (entry.whichAvailability()) {
    pb.ProfileCapability_Availability.available => Available(
      _known(Completeness.values, (c) => c.wire, entry.available.completeness) ??
          (throw RaisedFailure.protocolError('$capability: completeness left unspecified')),
    ),
    pb.ProfileCapability_Availability.unavailable => _unavailable(capability, entry.unavailable),
    pb.ProfileCapability_Availability.notSet => throw RaisedFailure.protocolError(
      '$capability: no availability',
    ),
  };
  return (capability, availability);
}

/// Tag of `CapabilityUnavailable.missing`.
const _missingTag = 1;

Unavailable _unavailable(Capability capability, pb.CapabilityUnavailable u) {
  // A known capability names only sections of its own schema; a section it newly needed would
  // change the meaning of its tag, which a minor version never does.
  if (u.unknownFields.hasField(_missingTag)) {
    throw RaisedFailure.protocolError('$capability: a missing section this build does not know');
  }
  final missing = <SectionKind>[];
  for (final wire in u.missing) {
    final section =
        _known(SectionKind.values, (k) => k.wire, wire) ??
        (throw RaisedFailure.protocolError('$capability: a missing section left unspecified'));
    if (missing.contains(section)) {
      throw RaisedFailure.protocolError('$capability: $section missing twice');
    }
    missing.add(section);
  }
  if (missing.isEmpty) {
    throw RaisedFailure.protocolError('$capability: unavailable, nothing missing');
  }
  return Unavailable._(List.unmodifiable(missing));
}

/// The member of [values] whose wire value is [wire]; `null` for the unspecified value.
T? _known<T, W>(List<T> values, W Function(T) wireOf, W wire) {
  for (final v in values) {
    if (wireOf(v) == wire) return v;
  }
  return null;
}

final profilesProvider = FutureProvider<List<Profile>>((ref) async {
  ref.watch(sessionEpochProvider);
  final client = ref.watch(daemonClientProvider);
  final list = await coreCall(client.listProfiles);
  final profiles = await coreCall(() async => [for (final p in list.profiles) profileOf(p)]);
  if (!ref.mounted) return profiles;
  final revision = Revision(list.revision);
  ref.read(projectionRevisionProvider.notifier).observe(revision);
  ref.listen(projectionRevisionProvider, (_, held) {
    if (isStale(revision, held)) ref.invalidateSelf();
  });
  return profiles;
});

/// The profile every per-profile view reads. View state: choosing one changes nothing in the
/// daemon. It follows the profile list: the first profile until the user picks another, and the
/// first again if the chosen one disappears.
class SelectedProfile extends Notifier<ProfileId?> {
  @override
  ProfileId? build() {
    final profiles = ref.watch(profilesProvider).value ?? const [];
    final previous = stateOrNull;
    if (previous != null && profiles.any((p) => p.id == previous)) return previous;
    return profiles.isEmpty ? null : profiles.first.id;
  }

  void select(ProfileId id) => state = id;
}

final selectedProfileProvider = NotifierProvider<SelectedProfile, ProfileId?>(SelectedProfile.new);

/// The selected profile itself; `null` while there is none.
final currentProfileProvider = Provider<Profile?>((ref) {
  final id = ref.watch(selectedProfileProvider);
  final profiles = ref.watch(profilesProvider).value ?? const [];
  for (final p in profiles) {
    if (p.id == id) return p;
  }
  return null;
});
