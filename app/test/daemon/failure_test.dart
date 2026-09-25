// The failure types: every case's dotted code follows the schema's field names, as the daemon's
// generated names do, and every case has a cause and a remedy.

import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:protobuf/protobuf.dart';
import 'package:yata/gen/l10n/app_localizations.dart';
import 'package:yata/gen/proto/core.pb.dart' as pb;
import 'package:yata/state/core.dart';
import 'package:yata/ui/common/explanation.dart';

final l = lookupAppLocalizations(const Locale('zh'));

/// One failure per case of the `kind` oneof of [empty]'s message, each with an empty record.
List<(String, T)> everyCase<T extends GeneratedMessage>(T Function() empty) {
  final info = empty().info_;
  return [
    for (final field in info.byIndex)
      if (field.isGroupOrMessage && info.oneofs[field.tagNumber] == 0)
        (field.protoName, empty()..setField(field.tagNumber, field.subBuilder!())),
  ];
}

void main() {
  final requests = everyCase(pb.Error.new);
  final sessions = everyCase(pb.SessionFailed.new);
  final clients = everyCase(pb.ClientFailure.new);

  test('every oneof case is found', () {
    expect(requests.length, pb.Error_Kind.values.length - 1);
    expect(sessions.length, pb.SessionFailed_Kind.values.length - 1);
    expect(clients.length, pb.ClientFailure_Kind.values.length - 1);
  });

  test('a code is its field name with the first underscore read as a dot', () {
    String dotted(String field) => field.replaceFirst('_', '.');
    for (final (field, error) in requests) {
      expect(RequestFailure(error).code, dotted(field));
    }
    for (final (field, failed) in sessions) {
      expect(SessionFailure(failed).code, dotted(field));
    }
    for (final (field, failure) in clients) {
      expect(RaisedFailure(failure, 'test').code, dotted(field));
    }
  });

  test('every known case has a cause and a remedy, and is known', () {
    final failures = <CoreFailure>[
      for (final (_, e) in requests) RequestFailure(e),
      for (final (_, f) in sessions) SessionFailure(f),
      for (final (_, f) in clients) RaisedFailure(f, 'test'),
    ];
    for (final f in failures) {
      final e = explain(l, f);
      expect(isKnown(f), isTrue, reason: f.code);
      expect(e.cause, isNot(l.causeUnknown), reason: f.code);
      expect(e.remedy, isNotEmpty, reason: f.code);
    }
  });

  test('the debug record is the case message in text form, apart from the explanation', () {
    final f = RaisedFailure.timeout(7, const Duration(seconds: 30));
    expect(f.debugRecord, contains('requestId: 7'));
    expect(f.debugRecord, contains('limitMs: 30000'));
    expect(explain(l, f).cause, isNot(contains('30000')));
    expect(RequestFailure(pb.Error()).debugRecord, isEmpty);
    expect(RequestFailure(pb.Error()).code, 'unknown');
  });
}
