//! The `yata-daemon` binary. Its one-shot subcommands run the same code as the protocol session,
//! with no UI, for CI and for a developer (`architecture/overview.md`, "One binary, two front
//! ends"). Output is for developers and is English; user-facing text lives in the application.

use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use yata_core::scheme::inspect::diff;
use yata_core::scheme::transport;
use yata_daemon::qr;
use yata_daemon::scheme::{format_diff, format_dump, read_code, read_payload};
use yata_daemon::store::{OpenError, Store};

const USAGE: &str = "usage: yata-daemon <command> ...

commands:
  check <store.sqlite3>                   run SQLite's full integrity check on a store
  scheme dump <code>                      print a scheme code's payload as hex
  scheme diff <code-a> <code-b>           compare two payloads byte by byte and bit by bit
  scheme decode <code> <payload.bin>      write a scheme code's payload to a file
  scheme encode <payload.bin> [<qr.png>]  print the scheme text for a payload; write its QR code

A <code> is a PNG image holding one QR code, or a text file holding the Base64 text.";

/// Pixels per module of a written QR code: large enough to scan from a screen.
const QR_SCALE: u32 = 8;

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    let words: Vec<&str> = args.iter().map(|a| a.to_str().unwrap_or("")).collect();
    let path = |i: usize| Path::new(&args[i]);
    match words.as_slice() {
        ["check", _] => check(path(1)),
        ["scheme", "dump", _] => report(read_code(path(2)).map(|p| format_dump(&p))),
        ["scheme", "diff", _, _] => report(
            read_code(path(2)).and_then(|a| read_code(path(3)).map(|b| format_diff(&diff(&a, &b)))),
        ),
        ["scheme", "decode", _, _] => report(read_code(path(2)).and_then(|p| {
            std::fs::write(path(3), p.as_bytes())
                .map(|()| format!("wrote {} bytes\n", p.len()))
                .map_err(|e| yata_daemon::scheme::InputError::Io {
                    path: path(3).to_owned(),
                    reason: e.to_string(),
                })
        })),
        ["scheme", "encode", _] => encode(path(2), None),
        ["scheme", "encode", _, _] => encode(path(2), Some(path(3))),
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn report<E: std::fmt::Debug>(result: Result<String, E>) -> ExitCode {
    match result {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("scheme.failed: {e:?}");
            ExitCode::FAILURE
        }
    }
}

fn encode(payload_path: &Path, png: Option<&Path>) -> ExitCode {
    let text = read_payload(payload_path)
        .map_err(|e| format!("{e:?}"))
        .and_then(|p| transport::encode_text(&p).map_err(|e| format!("{e:?}")));
    let text = match text {
        Ok(t) => t,
        Err(e) => {
            eprintln!("scheme.failed: {e}");
            return ExitCode::FAILURE;
        }
    };
    println!("{}", text.as_str());
    let Some(png) = png else {
        return ExitCode::SUCCESS;
    };
    let written = qr::encode(text.as_str())
        .map_err(|e| format!("{e:?}"))
        .and_then(|m| qr::render_png(&m, QR_SCALE).map_err(|e| e.to_string()))
        .and_then(|bytes| std::fs::write(png, bytes).map_err(|e| e.to_string()));
    match written {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("scheme.qr_failed: {e}");
            ExitCode::FAILURE
        }
    }
}

fn check(path: &Path) -> ExitCode {
    let mut store = match Store::open(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", describe(&e));
            return ExitCode::FAILURE;
        }
    };
    match store.integrity_check() {
        Ok(problems) if problems.is_empty() => {
            println!("ok");
            ExitCode::SUCCESS
        }
        Ok(problems) => {
            for p in problems {
                println!("{p}");
            }
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("store.check_failed: {e:?}");
            ExitCode::FAILURE
        }
    }
}

/// The developer-facing text for an open failure: a stable code, then the fields.
fn describe(e: &OpenError) -> String {
    match e {
        OpenError::Missing { path } => format!("store.missing: {}", path.display()),
        OpenError::NotADatabase => "store.not_a_database".to_owned(),
        OpenError::Foreign {
            application_id,
            objects,
        } => format!(
            "store.foreign: not a Yata store (application_id {application_id}, {objects} objects)"
        ),
        OpenError::NewerFormat { found, known } => {
            format!("store.newer_format: format {found}, this build knows {known}")
        }
        OpenError::NoFormatVersion => "store.no_format_version".to_owned(),
        OpenError::Damaged { problems } => {
            format!("store.damaged: {}", problems.join("; "))
        }
        OpenError::Failure(f) => format!("store.failure: {f:?}"),
    }
}
