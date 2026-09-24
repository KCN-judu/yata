//! Scheme codes from the game, end to end: for each code in the local corpus, the payload
//! survives decode → encode → decode byte for byte, parses into its layout and is written back
//! byte for byte, reads as a scheme code of selections that is written back byte for byte, and
//! survives the whole QR loop — text → QR matrix → PNG → text → payload.
//!
//! The corpus is game-derived and stays out of the public repository (ADR-0016). It is read from
//! the folder named by `YATA_SCHEME_CORPUS`, or `research/fixtures/scheme-codes/` in the working
//! copy. Where neither exists, as in CI, there is nothing to check and the test passes; the
//! transport itself is covered by the synthetic fixtures in `yata-core`.

use std::path::PathBuf;

use yata_core::scheme::code::{decode_code, encode_code};
use yata_core::scheme::layout::{parse, serialize};
use yata_core::scheme::transport::{decode_text, encode_text};
use yata_daemon::qr;
use yata_daemon::scheme::read_code_text;

fn corpus() -> Option<PathBuf> {
    let dir = std::env::var_os("YATA_SCHEME_CORPUS").map_or_else(
        || PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../research/fixtures/scheme-codes"),
        PathBuf::from,
    );
    dir.is_dir().then_some(dir)
}

#[test]
#[allow(clippy::expect_used, reason = "a failure here is the test failing")]
fn every_corpus_code_round_trips_through_text_and_qr() {
    let Some(dir) = corpus() else {
        eprintln!("no scheme-code corpus; nothing to check");
        return;
    };
    let mut codes: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("corpus folder")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "txt"))
        .filter(|p| !p.to_string_lossy().ends_with(".report.txt"))
        .collect();
    codes.sort();
    for path in codes {
        let text = read_code_text(&path).expect("readable code");
        let payload = decode_text(&text).expect("decodable code");
        let layout = parse(&payload).expect("the confirmed layout");
        assert_eq!(
            serialize(&layout).as_ref(),
            Ok(&payload),
            "{}: layout not written back byte for byte",
            path.display()
        );
        let code = decode_code(&layout).expect("a scheme code of selections");
        let written = encode_code(&code, layout.header.account).expect("encodable");
        assert_eq!(
            serialize(&written).as_ref(),
            Ok(&payload),
            "{}: selections not written back byte for byte",
            path.display()
        );
        let ours = encode_text(&payload).expect("encodable payload");
        assert_eq!(
            decode_text(ours.as_str()).as_ref(),
            Ok(&payload),
            "{}: payload changed through encode",
            path.display()
        );
        let png = qr::render_png(&qr::encode(ours.as_str()).expect("fits a QR code"), 4)
            .expect("renderable");
        let read_back = qr::decode_png(&png).expect("readable QR code");
        assert_eq!(read_back, ours.as_str(), "{}", path.display());
        eprintln!(
            "{}: {} bytes, {} records; our text is {} the game's",
            path.display(),
            payload.len(),
            layout.records.len(),
            if ours.as_str() == text {
                "identical to"
            } else {
                "different from"
            }
        );
    }
}
