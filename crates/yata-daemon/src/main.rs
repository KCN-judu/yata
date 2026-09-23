//! The `yata-daemon` binary. Its one-shot subcommands run the same code as the protocol session,
//! with no UI, for CI and for a developer (`architecture/overview.md`, "One binary, two front
//! ends"). Output is for developers and is English; user-facing text lives in the application.

use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use yata_daemon::store::{OpenError, Store};

const USAGE: &str = "usage: yata-daemon check <store.sqlite3>

commands:
  check    open an existing store and run SQLite's full integrity check";

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    match args.as_slice() {
        [command, path] if command == "check" => check(Path::new(path)),
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
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
