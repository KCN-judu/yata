//! Reading scheme codes from files, with synthetic codes only, so it runs everywhere: a text
//! file with a trailing line ending, a PNG QR code, and files too large to be codes.

use std::path::PathBuf;

use yata_core::scheme::RawSchemePayload;
use yata_core::scheme::transport::encode_text;
use yata_daemon::qr;
use yata_daemon::scheme::{InputError, read_code, read_payload};

struct Scratch(PathBuf);

impl Scratch {
    #[allow(
        clippy::expect_used,
        reason = "test helper: a failure here is the test failing"
    )]
    fn new(name: &str) -> Scratch {
        let dir = std::env::temp_dir().join(format!("yata-scheme-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        Scratch(dir)
    }

    #[allow(
        clippy::expect_used,
        reason = "test helper: a failure here is the test failing"
    )]
    fn write(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, bytes).expect("write");
        path
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[allow(
    clippy::expect_used,
    reason = "test helper: a failure here is the test failing"
)]
fn synthetic() -> (RawSchemePayload, String) {
    let payload =
        RawSchemePayload::new(b"a synthetic payload, not a scheme".to_vec()).expect("valid");
    let text = encode_text(&payload).expect("encodable").into_string();
    (payload, text)
}

#[test]
fn a_text_file_with_a_trailing_line_ending_reads_as_its_code() {
    let dir = Scratch::new("text");
    let (payload, text) = synthetic();
    let path = dir.write("code.txt", format!("{text}\r\n").as_bytes());
    assert_eq!(read_code(&path), Ok(payload));
}

#[test]
#[allow(clippy::expect_used, reason = "a failure here is the test failing")]
fn a_png_qr_code_reads_as_its_code() {
    let dir = Scratch::new("png");
    let (payload, text) = synthetic();
    let png = qr::render_png(&qr::encode(&text).expect("fits"), 4).expect("renderable");
    let path = dir.write("code.png", &png);
    assert_eq!(read_code(&path), Ok(payload));
}

#[test]
fn files_too_large_to_be_codes_are_refused() {
    let dir = Scratch::new("large");
    let text = dir.write("big.txt", &vec![b'A'; 8192]);
    assert!(matches!(read_code(&text), Err(InputError::TooLarge { .. })));
    let payload = dir.write("big.bin", &vec![0; 64 * 1024 + 1]);
    assert!(matches!(
        read_payload(&payload),
        Err(InputError::TooLarge { .. })
    ));
}

#[test]
fn a_missing_file_is_an_io_error() {
    let dir = Scratch::new("missing");
    assert!(matches!(
        read_code(&dir.0.join("none.txt")),
        Err(InputError::Io { .. })
    ));
}
