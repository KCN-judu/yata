//! The `yata-daemon` binary. Its one-shot subcommands run the same code as the protocol session,
//! with no UI, for CI and for a developer (`architecture/overview.md`, "One binary, two front
//! ends"). Output is for developers and is English; user-facing text lives in the application.

use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use yata_core::scheme::RawSchemePayload;
use yata_core::scheme::inspect::diff;
use yata_core::scheme::layout::{SchemeKind, SchemeLayout, header_of, parse, serialize};
use yata_core::scheme::transport;
use yata_daemon::qr;
use yata_daemon::scheme::{
    format_diff, format_dump, format_plans, parse_plan_file, read_code, read_payload,
};
use yata_daemon::store::{OpenError, Store};

const USAGE: &str = "usage: yata-daemon <command> ...

commands:
  check <store.sqlite3>                   run SQLite's full integrity check on a store
  scheme dump <code>                      print a scheme code's payload as hex
  scheme diff <code-a> <code-b>           compare two payloads byte by byte and bit by bit
  scheme decode <code> <payload.bin>      write a scheme code's payload to a file
  scheme encode <payload.bin> [<qr.png>]  print the scheme text for a payload; write its QR code
  scheme plans <code>                     list a code's kind and plans; `*` marks an open bit
  scheme retarget <code> <account-code> [<qr.png>]
                                          the same code carrying the account of <account-code>
  scheme build <account-code> <plans.txt> [<qr.png>]
                                          a strengthening set from a plan file, for that account
  scheme build-discard <account-code> <plans.txt> [<qr.png>]
                                          a discard code of every plan in a plan file

A <code> is a PNG image holding one QR code, or a text file holding the Base64 text.
A plan file has one plan per line: name | souls (all, or soul bits) | solved filter bits.";

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
        ["scheme", "plans", _] => report(
            read_code(path(2))
                .map_err(|e| format!("{e:?}"))
                .and_then(|p| parse(&p).map_err(|e| format!("{e:?}")))
                .map(|l| format_plans(&l)),
        ),
        ["scheme", "retarget", _, _] => retarget(path(2), path(3), None),
        ["scheme", "retarget", _, _, _] => retarget(path(2), path(3), Some(path(4))),
        ["scheme", "build", _, _] => build(path(2), path(3), None, SchemeKind::Strengthening),
        ["scheme", "build", _, _, _] => {
            build(path(2), path(3), Some(path(4)), SchemeKind::Strengthening)
        }
        ["scheme", "build-discard", _, _] => build(path(2), path(3), None, SchemeKind::Discard),
        ["scheme", "build-discard", _, _, _] => {
            build(path(2), path(3), Some(path(4)), SchemeKind::Discard)
        }
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
    match read_payload(payload_path) {
        Ok(p) => emit(&p, png),
        Err(e) => fail(format!("{e:?}")),
    }
}

/// `<code>` with the account of `<account-code>`: the plans byte for byte, the header's account
/// replaced.
fn retarget(code: &Path, account_code: &Path, png: Option<&Path>) -> ExitCode {
    let layout = read_code(code)
        .map_err(|e| format!("{e:?}"))
        .and_then(|p| parse(&p).map_err(|e| format!("{e:?}")));
    let account = account_of(account_code);
    match (layout, account) {
        (Ok(layout), Ok(account)) => write_layout(&layout.with_account(account), png),
        (Err(e), _) | (_, Err(e)) => fail(e),
    }
}

/// A scheme built from a plan file, carrying the account of `<account-code>`: a strengthening
/// set, or a discard code, of every plan in the file.
fn build(account_code: &Path, plans: &Path, png: Option<&Path>, kind: SchemeKind) -> ExitCode {
    let account = match account_of(account_code) {
        Ok(a) => a,
        Err(e) => return fail(e),
    };
    let records = std::fs::read_to_string(plans)
        .map_err(|e| format!("{}: {e}", plans.display()))
        .and_then(|text| parse_plan_file(&text).map_err(|e| format!("{e:?}")));
    let layout = records.and_then(|records| match kind {
        SchemeKind::Strengthening => Ok(SchemeLayout::strengthening(account, records)),
        SchemeKind::Discard => {
            SchemeLayout::discard(account, records).map_err(|e| format!("{e:?}"))
        }
    });
    match layout {
        Ok(layout) => write_layout(&layout, png),
        Err(e) => fail(e),
    }
}

fn account_of(code: &Path) -> Result<yata_core::scheme::layout::AccountSegment, String> {
    read_code(code)
        .map_err(|e| format!("{e:?}"))
        .and_then(|p| header_of(&p).map_err(|e| format!("{e:?}")))
        .map(|h| h.account)
}

fn write_layout(layout: &SchemeLayout, png: Option<&Path>) -> ExitCode {
    match serialize(layout) {
        Ok(p) => emit(&p, png),
        Err(e) => fail(format!("{e:?}")),
    }
}

fn fail(e: String) -> ExitCode {
    eprintln!("scheme.failed: {e}");
    ExitCode::FAILURE
}

/// Print the scheme text for a payload, and write its QR code when a path is given.
fn emit(payload: &RawSchemePayload, png: Option<&Path>) -> ExitCode {
    let text = match transport::encode_text(payload) {
        Ok(t) => t,
        Err(e) => return fail(format!("{e:?}")),
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
