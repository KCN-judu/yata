// The Dart frame codec against the fixture files the Rust codec is tested on
// (`crates/yata-daemon/tests/frame_fixtures.rs`): the same bytes, the same outcomes.

import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:yata/daemon/frame_codec.dart';

import '../support/recorded.dart';

List<int> bytes(String s) => s.codeUnits;

void main() {
  group('shared fixtures', () {
    test('two frames decode whole and in order', () {
      final frames = decodeAllFrames(fixtureBytes('frames/two-frames.bin'));
      expect(frames, [bytes('one'), bytes('second')]);
    });

    test('an empty recording has no frames', () {
      expect(decodeAllFrames(fixtureBytes('frames/empty.bin')), isEmpty);
    });

    test('a zero prefix is an empty frame', () {
      expect(
        () => decodeAllFrames(fixtureBytes('frames/zero-prefix.bin')),
        throwsA(const FrameEmpty()),
      );
    });

    test('an oversized prefix is refused before its bytes arrive', () {
      expect(
        () => decodeAllFrames(fixtureBytes('frames/oversize-prefix.bin')),
        throwsA(const FrameTooLong(0xffffffff)),
      );
      expect(
        () => decodeAllFrames(fixtureBytes('frames/max-plus-one-prefix.bin')),
        throwsA(const FrameTooLong(maxFrameLength + 1)),
      );
    });

    test('a recording cut inside a frame is truncated, not ended', () {
      expect(
        () => decodeAllFrames(fixtureBytes('frames/truncated-payload.bin')),
        throwsA(const FrameTruncated(expected: 9, available: 6)),
      );
      expect(
        () => decodeAllFrames(fixtureBytes('frames/truncated-prefix.bin')),
        throwsA(const FrameTruncated(expected: 4, available: 2)),
      );
    });

    test('encoding writes the fixture bytes exactly', () {
      final encoded = [...encodeFrame(bytes('one')), ...encodeFrame(bytes('second'))];
      expect(encoded, fixtureBytes('frames/two-frames.bin'));
    });
  });

  test('the prefix is the payload length, big-endian', () {
    expect(encodeFrame(bytes('abc')), [0, 0, 0, 3, 97, 98, 99]);
    expect(encodeFrame(Uint8List(0x010203)).sublist(0, 4), [0, 1, 2, 3]);
  });

  test('an empty payload is not a message', () {
    expect(() => encodeFrame(const []), throwsA(const FrameEmpty()));
  });

  test('the maximum is sixteen mebibytes inclusive', () {
    expect(encodeFrame(Uint8List(maxFrameLength)).length, maxFrameLength + 4);
    expect(
      () => encodeFrame(Uint8List(maxFrameLength + 1)),
      throwsA(const FrameTooLong(maxFrameLength + 1)),
    );
  });

  test('frames arriving one byte at a time decode whole', () {
    final stream = fixtureBytes('frames/two-frames.bin');
    final decoder = FrameDecoder();
    final out = <List<int>>[];
    for (final b in stream) {
      decoder.push([b]);
      for (var p = decoder.nextFrame(); p != null; p = decoder.nextFrame()) {
        out.add(p);
      }
    }
    expect(out, [bytes('one'), bytes('second')]);
    decoder.finish();
  });
}
