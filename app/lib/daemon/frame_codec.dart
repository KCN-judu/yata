/// The frame codec: a 4-byte big-endian length prefix, then exactly that many payload bytes.
///
/// A mirror of `yata-protocol::frame` (`core-protocol.md`, "Frame"), tested against the same
/// fixture files (`crates/yata-protocol/fixtures/frames/`). A prefix is checked before anything is
/// buffered for it: zero and anything above [maxFrameLength] are protocol errors, never
/// allocation requests.
library;

import 'dart:typed_data';

/// The largest payload a frame may carry: 16 MiB.
const int maxFrameLength = 1 << 24;

const int _prefix = 4;

/// A protocol error in the framing. After one, the stream is unusable and the session ends.
sealed class FrameError implements Exception {
  const FrameError();
}

/// A zero prefix, or an empty payload to encode: no message is empty.
final class FrameEmpty extends FrameError {
  const FrameEmpty();

  @override
  bool operator ==(Object other) => other is FrameEmpty;

  @override
  int get hashCode => 0;

  @override
  String toString() => 'FrameEmpty';
}

/// A prefix, or a payload to encode, longer than [maxFrameLength].
final class FrameTooLong extends FrameError {
  const FrameTooLong(this.length);

  final int length;

  @override
  bool operator ==(Object other) => other is FrameTooLong && other.length == length;

  @override
  int get hashCode => length.hashCode;

  @override
  String toString() => 'FrameTooLong($length)';
}

/// The stream ended inside a frame: [available] of the [expected] bytes (prefix included) arrived.
final class FrameTruncated extends FrameError {
  const FrameTruncated({required this.expected, required this.available});

  final int expected;
  final int available;

  @override
  bool operator ==(Object other) =>
      other is FrameTruncated && other.expected == expected && other.available == available;

  @override
  int get hashCode => Object.hash(expected, available);

  @override
  String toString() => 'FrameTruncated(expected: $expected, available: $available)';
}

/// One frame for a payload.
Uint8List encodeFrame(List<int> payload) {
  if (payload.length > maxFrameLength) throw FrameTooLong(payload.length);
  if (payload.isEmpty) throw const FrameEmpty();
  final out = Uint8List(_prefix + payload.length);
  ByteData.sublistView(out).setUint32(0, payload.length, Endian.big);
  out.setRange(_prefix, out.length, payload);
  return out;
}

/// The payload length a prefix announces, if it is a legal one.
int _announced(Uint8List prefix) {
  final n = ByteData.sublistView(prefix).getUint32(0, Endian.big);
  if (n == 0) throw const FrameEmpty();
  if (n > maxFrameLength) throw FrameTooLong(n);
  return n;
}

/// An incremental decoder: bytes go in as they arrive, whole payloads come out.
class FrameDecoder {
  final BytesBuilder _buffer = BytesBuilder(copy: false);
  Uint8List _pending = Uint8List(0);

  /// Bytes read from the stream, in order.
  void push(List<int> bytes) {
    if (bytes.isEmpty) return;
    _buffer.add(bytes);
  }

  Uint8List _bytes() {
    if (_buffer.isNotEmpty) {
      final more = _buffer.takeBytes();
      _pending = _pending.isEmpty
          ? more
          : (BytesBuilder(copy: false)
                  ..add(_pending)
                  ..add(more))
                .takeBytes();
    }
    return _pending;
  }

  /// The next whole payload, `null` if more bytes are needed; throws the [FrameError] the next
  /// prefix is.
  Uint8List? nextFrame() {
    final bytes = _bytes();
    if (bytes.length < _prefix) return null;
    final length = _announced(Uint8List.sublistView(bytes, 0, _prefix));
    if (bytes.length < _prefix + length) return null;
    final payload = Uint8List.fromList(Uint8List.sublistView(bytes, _prefix, _prefix + length));
    _pending = Uint8List.sublistView(bytes, _prefix + length);
    return payload;
  }

  /// The stream has ended: throws [FrameTruncated] if it ended inside a frame.
  void finish() {
    final bytes = _bytes();
    if (bytes.isEmpty) return;
    final expected = bytes.length >= _prefix
        ? _prefix + _announced(Uint8List.sublistView(bytes, 0, _prefix))
        : _prefix;
    throw FrameTruncated(expected: expected, available: bytes.length);
  }
}

/// Every payload of a whole stream, such as a recording read from a file.
List<Uint8List> decodeAllFrames(List<int> stream) {
  final decoder = FrameDecoder()..push(stream);
  final payloads = <Uint8List>[];
  for (var p = decoder.nextFrame(); p != null; p = decoder.nextFrame()) {
    payloads.add(p);
  }
  decoder.finish();
  return payloads;
}
