//! The frame codec against the shared fixture files in `crates/yata-protocol/fixtures/frames/`.
//!
//! The test lives here because `yata-protocol` is pure and reads no file, not even in a test
//! (ADR-0001); the daemon is where files are read.
//!
//! The Dart mirror of the codec (`app/lib/daemon/frame_codec.dart`) reads the same files and
//! asserts the same outcomes (`app/test/daemon/frame_codec_test.dart`), so the two codecs agree
//! on these bytes rather than on two descriptions of them. `YATA_BLESS=1` rewrites the files.

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    reason = "a failure here is the test failing"
)]

use std::path::PathBuf;

use yata_protocol::frame::{FrameError, MAX_FRAME_LEN, decode_all, encode};

fn fixture(name: &str, bytes: &[u8]) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../yata-protocol/fixtures/frames")
        .join(name);
    if std::env::var_os("YATA_BLESS").is_some() {
        std::fs::create_dir_all(path.parent().expect("a folder")).expect("fixture folder");
        std::fs::write(&path, bytes).expect("fixture written");
    }
    let committed = std::fs::read(&path).unwrap_or_else(|e| {
        panic!(
            "{}: {e}; run with YATA_BLESS=1 to record it",
            path.display()
        )
    });
    assert_eq!(
        committed, bytes,
        "{name}: the committed fixture is not what the codec writes"
    );
    committed
}

#[test]
fn the_shared_frame_fixtures_decode_as_recorded() {
    let two = [encode(b"one").unwrap(), encode(b"second").unwrap()].concat();
    assert_eq!(
        decode_all(&fixture("two-frames.bin", &two)),
        Ok(vec![b"one".to_vec(), b"second".to_vec()])
    );
    assert_eq!(decode_all(&fixture("empty.bin", &[])), Ok(vec![]));
    assert_eq!(
        decode_all(&fixture("zero-prefix.bin", &[0, 0, 0, 0])),
        Err(FrameError::Empty)
    );
    assert_eq!(
        decode_all(&fixture("oversize-prefix.bin", &[0xff, 0xff, 0xff, 0xff])),
        Err(FrameError::TooLong {
            length: 0xffff_ffff
        })
    );
    let over = (MAX_FRAME_LEN + 1).to_be_bytes();
    assert_eq!(
        decode_all(&fixture("max-plus-one-prefix.bin", &over)),
        Err(FrameError::TooLong {
            length: u64::from(MAX_FRAME_LEN) + 1
        })
    );
    let mut cut = encode(b"whole").unwrap();
    cut.extend_from_slice(&[0, 0, 0, 5, b'h', b'a']);
    assert_eq!(
        decode_all(&fixture("truncated-payload.bin", &cut)),
        Err(FrameError::Truncated {
            expected: 9,
            available: 6
        })
    );
    assert_eq!(
        decode_all(&fixture("truncated-prefix.bin", &[0, 0])),
        Err(FrameError::Truncated {
            expected: 4,
            available: 2
        })
    );
}
