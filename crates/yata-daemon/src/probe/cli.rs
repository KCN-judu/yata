//! The `yata-daemon probe …` commands: a live read, and the research commands over recordings and
//! export files. Output is for developers and is English.

use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use yata_core::import::evidence::{self, Attestation, Source};
use yata_core::import::observation::{SoulObservation, SoulReading};
use yata_protocol::export;
use yata_protocol::probe::read_result::Records;

use super::convert;
use super::input::{self, InputError, Loaded};
use super::report;
use super::session::{self, Inbound};

pub const USAGE: &str =
    "  probe read <reader> [--pid <n>] [--record <out.frames>] [--export <out.json>]
             [--reader-sha256 <hex>]
                                          read the souls through a reader (Windows)
  probe show <input>                      provenance, coverage, and field evidence
  probe decode <recording>                every message of a recording, checked
  probe survey <input>                    what each observed key holds
  probe group <input> <source> [<n>]      souls grouped by a value, n samples each
  probe crosstab <input> <rows> <columns> soul counts for each pair of values
  probe suit-evidence <input> <identity> <suit> <offset> <attestations>
                                          the inherited suit codes against attested souls
  probe to-export <recording> <out.json>  a recording's readings as an export file

An <input> is a recording (.frames) or an export file (proto3 JSON).
A <source> is @soul_id, @suit_code, @star, @slot, @level, @main, @subs, @innate, @locked,
@discarded, @container, or the key of an observed entry.
An attestation file has one line per soul: identity <TAB> scheme soul bit.";

pub fn run(args: &[OsString]) -> ExitCode {
    match dispatch(args) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn dispatch(args: &[OsString]) -> Result<String, String> {
    let words: Vec<&str> = args.iter().map(|a| a.to_str().unwrap_or("")).collect();
    let path = |i: usize| Path::new(&args[i]);
    match words.as_slice() {
        ["read", _, rest @ ..] => read(path(1), rest),
        ["show", _] => with_readings(path(1), |loaded, readings| {
            Ok(report::summary(loaded, readings))
        }),
        ["decode", _] => decode(path(1)),
        ["survey", _] => with_readings(path(1), |_, readings| {
            Ok(report::survey(&evidence::survey(all(readings))))
        }),
        ["group", _, source] => group(path(1), source, 3),
        ["group", _, source, n] => match n.parse() {
            Ok(n) => group(path(1), source, n),
            Err(_) => Err(format!("probe.usage: not a count: {n}")),
        },
        ["crosstab", _, rows, columns] => {
            let (rows, columns) = (parse_source(rows)?, parse_source(columns)?);
            with_readings(path(1), |_, readings| {
                Ok(report::crosstab(&evidence::crosstab(
                    all(readings),
                    &rows,
                    &columns,
                )))
            })
        }
        ["suit-evidence", _, identity, suit, offset, _] => {
            suit_evidence(path(1), identity, suit, offset, path(5))
        }
        ["to-export", _, _] => to_export(path(1), path(2)),
        _ => Err(format!("usage:\n{USAGE}")),
    }
}

fn parse_source(spec: &str) -> Result<Source, String> {
    Source::parse(spec).ok_or_else(|| format!("probe.usage: not a source: {spec}"))
}

fn all(readings: &[SoulReading]) -> impl Iterator<Item = &SoulObservation> {
    readings.iter().flat_map(|r| r.souls.iter())
}

fn load(path: &Path) -> Result<Loaded, String> {
    input::load(path).map_err(|e| match e {
        InputError::Export(e) => format!("import.malformed_export: {e:?}"),
        other => format!("probe.input: {other:?}"),
    })
}

fn readings(loaded: &Loaded) -> Result<Vec<SoulReading>, String> {
    loaded
        .results
        .iter()
        .filter(|r| matches!(r.records, Some(Records::Souls(_)) | None))
        .map(|r| convert::soul_reading(r).map_err(|e| format!("probe.convert: {e:?}")))
        .collect()
}

fn with_readings(
    path: &Path,
    f: impl FnOnce(&Loaded, &[SoulReading]) -> Result<String, String>,
) -> Result<String, String> {
    let loaded = load(path)?;
    let readings = readings(&loaded)?;
    f(&loaded, &readings)
}

fn group(path: &Path, source: &str, samples: usize) -> Result<String, String> {
    let source = parse_source(source)?;
    with_readings(path, |_, readings| {
        Ok(report::groups(
            &evidence::group(all(readings), &source),
            &source,
            samples,
        ))
    })
}

fn decode(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("probe.input: {}: {e}", path.display()))?;
    let replay = session::replay(&bytes).map_err(|e| format!("probe.recording: {e:?}"))?;
    let mut out = String::new();
    for m in &replay.messages {
        out.push_str(&match m {
            Inbound::Ack(a) => {
                let v = a.version.unwrap_or_default();
                format!(
                    "ack       version {}.{} engine {} build {} channel {}\n",
                    v.major,
                    v.minor,
                    a.engine,
                    a.probe_build_id,
                    a.channel().as_str_name()
                )
            }
            Inbound::Progress(p) => {
                format!(
                    "progress  request {} {}/{}\n",
                    p.request_id, p.done, p.total
                )
            }
            Inbound::Result(r) => {
                let n = match &r.records {
                    Some(Records::Souls(s)) => s.souls.len(),
                    None => 0,
                };
                format!(
                    "result    request {} {} {} records {n}\n",
                    r.request_id,
                    r.scope().as_str_name(),
                    r.coverage().as_str_name()
                )
            }
            Inbound::Failed(f) => {
                let e = f.error.clone().unwrap_or_default();
                format!(
                    "failed    request {} {} {}\n",
                    f.request_id, e.code, e.message
                )
            }
            Inbound::Log(l) => format!(
                "log       {} {} {}\n",
                l.level().as_str_name(),
                l.code,
                l.message
            ),
        });
    }
    out.push_str("whole capture: every request answered, the stream ends on a frame boundary\n");
    Ok(out)
}

fn suit_evidence(
    path: &Path,
    identity: &str,
    suit: &str,
    offset: &str,
    attestations: &Path,
) -> Result<String, String> {
    let identity = parse_source(identity)?;
    let suit = parse_source(suit)?;
    let offset: i64 = offset
        .parse()
        .map_err(|_| format!("probe.usage: not an offset: {offset}"))?;
    let text = std::fs::read_to_string(attestations)
        .map_err(|e| format!("probe.input: {}: {e}", attestations.display()))?;
    let attestations = parse_attestations(&text)?;
    with_readings(path, |_, readings| {
        let souls: Vec<SoulObservation> = all(readings).cloned().collect();
        Ok(report::ledger(&evidence::suit_ledger(
            &souls,
            &identity,
            &suit,
            offset,
            &attestations,
        )))
    })
}

/// `identity <TAB> bit` per line; blank lines and lines starting with `#` are skipped.
pub fn parse_attestations(text: &str) -> Result<Vec<Attestation>, String> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t');
        let (Some(identity), Some(bit)) = (fields.next(), fields.next()) else {
            return Err(format!(
                "probe.attestation: line {}: expected identity <TAB> bit",
                i + 1
            ));
        };
        let bit = bit
            .trim()
            .parse()
            .map_err(|_| format!("probe.attestation: line {}: not a bit: {bit}", i + 1))?;
        out.push(Attestation {
            identity: identity.trim().to_owned(),
            bit,
        });
    }
    Ok(out)
}

fn to_export(recording: &Path, out: &Path) -> Result<String, String> {
    let loaded = load(recording)?;
    // A recording does not say when it was taken, so the export does not either.
    let text = export::to_json(&input::to_export(&loaded, ""));
    std::fs::write(out, &text).map_err(|e| format!("probe.output: {}: {e}", out.display()))?;
    Ok(format!(
        "wrote {} results, {} bytes\n",
        loaded.results.len(),
        text.len()
    ))
}

#[cfg(windows)]
fn read(reader: &Path, rest: &[&str]) -> Result<String, String> {
    use super::launch::{self, LaunchError, ReadOptions};
    use super::session::Step;

    let mut options = ReadOptions {
        reader,
        target_pid: 0,
        record: None,
        expected_sha256: None,
    };
    let mut export_to = None;
    let mut rest = rest.iter();
    while let Some(flag) = rest.next() {
        let value = rest
            .next()
            .ok_or_else(|| format!("probe.usage: {flag} needs a value"))?;
        match *flag {
            "--pid" => {
                options.target_pid = value
                    .parse()
                    .map_err(|_| format!("probe.usage: not a pid: {value}"))?;
            }
            "--record" => options.record = Some(Path::new(value)),
            "--export" => export_to = Some(Path::new(value)),
            "--reader-sha256" => {
                options.expected_sha256 = Some(
                    launch::parse_sha256(value)
                        .ok_or_else(|| format!("probe.usage: not a SHA-256: {value}"))?,
                );
            }
            other => return Err(format!("probe.usage: unknown flag {other}")),
        }
    }
    let outcome = launch::read_souls(&options, &mut |p| {
        eprintln!("progress {}/{}", p.done, p.total);
        Step::Continue
    })
    .map_err(|e| match e {
        LaunchError::ElevationDeclined => "import.elevation_declined: run the reader's export \
                                           mode as administrator and import the file"
            .to_owned(),
        LaunchError::NoExpectedHash => "import.elevation_unverified: the game needs an \
                                        elevated reader; give --reader-sha256 to check it first"
            .to_owned(),
        LaunchError::Session(session::SessionError::Failed(e)) => {
            let mut text = format!("import.probe_failed: {} {}", e.code, e.message);
            for c in &e.candidates {
                text.push_str(&format!("\n  candidate pid {} {}", c.pid, c.image_name));
            }
            text
        }
        other => format!("import.probe_failed: {other:?}"),
    })?;
    let mut text = format!(
        "engine {} build {}{}\n",
        outcome.ack.engine,
        outcome.ack.probe_build_id,
        if outcome.elevated { " (elevated)" } else { "" }
    );
    for l in &outcome.logs {
        text.push_str(&format!("log {} {}\n", l.code, l.message));
    }
    let loaded = Loaded {
        carrier: input::Carrier::Recording,
        provenance: input::Provenance {
            protocol_version: outcome.ack.version,
            probe_build_id: outcome.ack.probe_build_id.clone(),
            engine: outcome.ack.engine.clone(),
            channel: outcome.ack.channel(),
            target: outcome.ack.target.clone(),
            captured_at: None,
        },
        results: vec![outcome.result],
        failures: Vec::new(),
    };
    let readings = readings(&loaded)?;
    text.push_str(&report::summary(&loaded, &readings));
    if let Some(out) = export_to {
        let json = export::to_json(&input::to_export(&loaded, ""));
        std::fs::write(out, json).map_err(|e| format!("probe.output: {}: {e}", out.display()))?;
        text.push_str(&format!("export written to {}\n", out.display()));
    }
    Ok(text)
}

#[cfg(not(windows))]
fn read(_reader: &Path, _rest: &[&str]) -> Result<String, String> {
    // ADR-0008: no read channel exists here; the export file is the way in.
    Err(
        "import.channel_unavailable: reading the game needs Windows; import an export file"
            .to_owned(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attestations_are_tab_separated_with_comments() {
        let text = "# soul\tbit\nabc\t3\r\n\n  \nxyz\t 69 \n";
        assert_eq!(
            parse_attestations(text),
            Ok(vec![
                Attestation {
                    identity: "abc".into(),
                    bit: 3
                },
                Attestation {
                    identity: "xyz".into(),
                    bit: 69
                },
            ])
        );
        assert!(parse_attestations("abc").is_err());
        assert!(parse_attestations("abc\tx").is_err());
    }
}
