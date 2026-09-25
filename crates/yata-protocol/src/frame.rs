//! The frame codec: a 4-byte big-endian length prefix, then exactly that many payload bytes.
//!
//! A prefix is checked before anything is allocated for it: zero and anything above
//! [`MAX_FRAME_LEN`] are protocol errors, never allocation requests. A stream of frames has
//! nothing around them, so the same decoder reads the live channel and a fixture, and a stream
//! cut short is reported as truncated rather than as a clean end (`core-protocol.md`, § Frame).

/// The largest payload a frame may carry: 16 MiB.
pub const MAX_FRAME_LEN: u32 = 1 << 24;

const PREFIX: usize = 4;

/// A protocol error in the framing. After one, the stream is unusable and the session ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameError {
    /// A zero prefix, or an empty payload to encode: no message is empty.
    Empty,
    /// A prefix, or a payload to encode, longer than [`MAX_FRAME_LEN`].
    TooLong { length: u64 },
    /// The stream ended inside a frame: `available` of the `expected` bytes (prefix included)
    /// arrived.
    Truncated { expected: u64, available: u64 },
}

/// One frame for a payload.
pub fn encode(payload: &[u8]) -> Result<Vec<u8>, FrameError> {
    let length = u32::try_from(payload.len())
        .ok()
        .filter(|&n| n <= MAX_FRAME_LEN)
        .ok_or(FrameError::TooLong {
            length: payload.len() as u64,
        })?;
    if length == 0 {
        return Err(FrameError::Empty);
    }
    let mut out = Vec::with_capacity(PREFIX + payload.len());
    out.extend_from_slice(&length.to_be_bytes());
    out.extend_from_slice(payload);
    Ok(out)
}

/// The payload length a prefix announces, if it is a legal one.
fn announced(prefix: [u8; PREFIX]) -> Result<usize, FrameError> {
    match u32::from_be_bytes(prefix) {
        0 => Err(FrameError::Empty),
        n if n > MAX_FRAME_LEN => Err(FrameError::TooLong {
            length: u64::from(n),
        }),
        // At most 16 MiB, so it fits a usize on every supported target.
        n => Ok(n as usize),
    }
}

/// An incremental decoder: bytes go in as they arrive, whole payloads come out.
#[derive(Debug, Default)]
pub struct FrameDecoder {
    buffer: Vec<u8>,
}

impl FrameDecoder {
    pub fn new() -> FrameDecoder {
        FrameDecoder::default()
    }

    /// Bytes read from the stream, in order.
    pub fn push(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    /// The next whole payload, `None` if more bytes are needed, or the protocol error the next
    /// prefix is.
    pub fn next_frame(&mut self) -> Result<Option<Vec<u8>>, FrameError> {
        let Some(prefix) = self.buffer.first_chunk::<PREFIX>() else {
            return Ok(None);
        };
        let length = announced(*prefix)?;
        if self.buffer.len() < PREFIX + length {
            return Ok(None);
        }
        let payload = self.buffer[PREFIX..PREFIX + length].to_vec();
        self.buffer.drain(..PREFIX + length);
        Ok(Some(payload))
    }

    /// The stream has ended: an error if it ended inside a frame.
    pub fn finish(self) -> Result<(), FrameError> {
        if self.buffer.is_empty() {
            return Ok(());
        }
        let expected = match self.buffer.first_chunk::<PREFIX>() {
            Some(prefix) => PREFIX + announced(*prefix)?,
            None => PREFIX,
        };
        Err(FrameError::Truncated {
            expected: expected as u64,
            available: self.buffer.len() as u64,
        })
    }
}

/// Every payload of a whole stream, such as a recording read from a file.
pub fn decode_all(stream: &[u8]) -> Result<Vec<Vec<u8>>, FrameError> {
    let mut decoder = FrameDecoder::new();
    decoder.push(stream);
    let mut payloads = Vec::new();
    while let Some(p) = decoder.next_frame()? {
        payloads.push(p);
    }
    decoder.finish()?;
    Ok(payloads)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_prefix_is_the_payload_length_big_endian() {
        assert_eq!(encode(b"abc"), Ok(vec![0, 0, 0, 3, b'a', b'b', b'c']));
        let long = vec![7u8; 0x0001_0203];
        assert_eq!(encode(&long).map(|f| f[..4].to_vec()), Ok(vec![0, 1, 2, 3]));
    }

    #[test]
    fn an_empty_payload_is_not_a_message() {
        assert_eq!(encode(b""), Err(FrameError::Empty));
        assert_eq!(decode_all(&[0, 0, 0, 0]), Err(FrameError::Empty));
    }

    #[test]
    fn the_maximum_is_sixteen_mebibytes_inclusive() {
        let max = vec![0u8; MAX_FRAME_LEN as usize];
        assert!(encode(&max).is_ok());
        let over = vec![0u8; MAX_FRAME_LEN as usize + 1];
        assert_eq!(
            encode(&over),
            Err(FrameError::TooLong {
                length: u64::from(MAX_FRAME_LEN) + 1
            })
        );
    }

    #[test]
    fn an_oversized_prefix_is_rejected_before_its_bytes_arrive() {
        let mut d = FrameDecoder::new();
        d.push(&[0xff, 0xff, 0xff, 0xff]);
        assert_eq!(
            d.next_frame(),
            Err(FrameError::TooLong {
                length: 0xffff_ffff
            })
        );
    }

    #[test]
    fn frames_arriving_in_pieces_decode_whole_and_in_order() {
        let stream = [encode(b"one"), encode(b"second")]
            .into_iter()
            .map(|f| f.expect("encodable"))
            .collect::<Vec<_>>()
            .concat();
        let mut d = FrameDecoder::new();
        let mut out = Vec::new();
        for byte in &stream {
            d.push(std::slice::from_ref(byte));
            while let Some(p) = d.next_frame().expect("valid") {
                out.push(p);
            }
        }
        assert_eq!(out, vec![b"one".to_vec(), b"second".to_vec()]);
        assert_eq!(d.finish(), Ok(()));
    }

    #[test]
    fn a_recording_cut_inside_a_frame_is_truncated_not_ended() {
        let mut stream = encode(b"whole").expect("encodable");
        stream.extend_from_slice(&[0, 0, 0, 5, b'h', b'a']);
        assert_eq!(
            decode_all(&stream),
            Err(FrameError::Truncated {
                expected: 9,
                available: 6
            })
        );
        assert_eq!(
            decode_all(&[0, 0]),
            Err(FrameError::Truncated {
                expected: 4,
                available: 2
            })
        );
    }

    #[test]
    fn an_empty_recording_has_no_frames() {
        assert_eq!(decode_all(&[]), Ok(vec![]));
    }
}
