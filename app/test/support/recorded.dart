/// The shared fixtures the Rust tests write (`crates/yata-protocol/fixtures/`), read from the
/// working copy. Test data comes from here, so the Dart tests use rows the daemon encoded rather
/// than rows written by hand.
library;

import 'dart:io';
import 'dart:typed_data';

import 'package:fixnum/fixnum.dart';
import 'package:yata/daemon/frame_codec.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;

/// `flutter test` runs in `app/`.
const fixtureRoot = '../crates/yata-protocol/fixtures';

Uint8List fixtureBytes(String path) => File('$fixtureRoot/$path').readAsBytesSync();

/// The client messages of the recorded session, as the Rust test encoded them.
List<pb.ClientMessage> recordedRequests() =>
    decodeAllFrames(fixtureBytes('core/session.in')).map(pb.ClientMessage.fromBuffer).toList();

/// The daemon's messages of the recorded session, in order.
List<pb.ServerMessage> recordedServerMessages() =>
    decodeAllFrames(fixtureBytes('core/session.out')).map(pb.ServerMessage.fromBuffer).toList();

/// The recorded response to request [id].
pb.Response recordedResponse(int id) => recordedServerMessages()
    .where((m) => m.hasResponse() && m.response.id == Int64(id))
    .single
    .response;

/// Requests of the recorded session by id: 3 lists profiles, 4 and 5 are the two pages of the
/// fixture profile, 6 names an unknown profile, 7 decodes a scheme code.
pb.ProfileList recordedProfiles() => recordedResponse(3).profileList;

pb.QueryPage recordedFirstPage() => recordedResponse(4).queryPage;

pb.QueryPage recordedSecondPage() => recordedResponse(5).queryPage;

pb.Error recordedUnknownProfile() => recordedResponse(6).error;

pb.SchemeCodeDecoded recordedScheme() => recordedResponse(7).schemeCodeDecoded;
