//! The `yata-daemon probe …` commands: a live read, and the research commands over recordings and
//! export files. Output is for developers and is English.

use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use yata_core::import::evidence::{self, Attestation, Source};
use yata_core::import::observation::{SoulObservation, SoulReading};
use yata_core::scheme::edit::SoulBit;
use yata_protocol::export;
use yata_protocol::probe::reading::Records;

use super::convert;
use super::input::{self, InputError, Loaded};
use super::report;
use super::session::{self, Inbound};

pub const USAGE: &str =
    "  probe read <reader> [--pid <n>] [--record <out.frames>] [--export <out.json>]
             [--reader-sha256 <hex>]
                                          read the souls through a reader (Windows)
  probe show <input>                      provenance, coverage, and field mappings
  probe decode <recording>                every message of a recording, checked
  probe survey <input>                    what each observed key holds
  probe group <input> <source> [<n>]      souls grouped by a value, n samples each
  probe crosstab <input> <rows> <columns> soul counts for each pair of values
  probe suit-evidence <input> <identity> <suit> <offset> <attestations>
                                          the inherited suit codes against attested souls
  probe to-export <input> <out.json>      a recording's readings as an export file

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
    let words = args
        .iter()
        .map(|a| {
            a.to_str()
                .ok_or_else(|| format!("probe.usage: an argument is not Unicode: {a:?}"))
        })
        .collect::<Result<Vec<&str>, String>>()?;
    let path = |i: usize| Path::new(words[i]);
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
    Source::parse(spec).map_err(|e| format!("probe.usage: not a source: {spec} ({e:?})"))
}

fn all(readings: &[SoulReading]) -> impl Iterator<Item = &SoulObservation> {
    readings.iter().flat_map(|r| r.souls().iter())
}

fn load(path: &Path) -> Result<Loaded, String> {
    input::load(path).map_err(|e| match e {
        InputError::Export(e) => format!("import.malformed_export: {e:?}"),
        other => format!("probe.input: {other:?}"),
    })
}

/// Every reading, converted: one the converter refuses, a reading without records included, is
/// an error, never skipped.
fn readings(loaded: &Loaded) -> Result<Vec<SoulReading>, String> {
    loaded
        .readings
        .iter()
        .map(|r| convert::soul_reading(r).map_err(|e| format!("import.malformed_reading: {e:?}")))
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
    let lines: Vec<String> = replay
        .messages
        .iter()
        .map(|m| match m {
            Inbound::Ack(a) => {
                let v = a
                    .version
                    .map_or_else(|| "none".to_owned(), |v| format!("{}.{}", v.major, v.minor));
                format!(
                    "ack       version {v} engine {} build {} channel {}",
                    a.engine,
                    a.probe_build_id,
                    a.channel().as_str_name()
                )
            }
            Inbound::Progress { id, done, total } => {
                let total = total.map_or_else(|| "?".to_owned(), |t| t.to_string());
                format!("progress  request {} {done}/{total}", id.get())
            }
            Inbound::Result { id, reading } => {
                let n = match &reading.records {
                    Some(Records::Souls(s)) => s.souls.len(),
                    None => 0,
                };
                format!(
                    "result    request {} {} records {n}",
                    id.get(),
                    reading.coverage().as_str_name()
                )
            }
            Inbound::RequestFailed { id, failure } => format!(
                "failed    request {} {} {}",
                id.get(),
                failure.name(),
                failure.message
            ),
            Inbound::SessionFailed(failure) => {
                format!("failed    session {} {}", failure.name(), failure.message)
            }
            Inbound::Log(l) => format!("log       {:?} {}", l.level, l.message),
        })
        .collect();
    Ok(format!(
        "{}\nwhole capture: every request answered, the stream ends on a frame boundary\n",
        lines.join("\n")
    ))
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
        let ledger = evidence::suit_ledger(&souls, &identity, &suit, offset, &attestations)
            .map_err(|gap| format!("probe.internal: the scheme table has a gap: {gap:?}"))?;
        Ok(report::ledger(&ledger))
    })
}

/// `identity <TAB> bit` per line, exactly two fields; blank lines and lines starting with `#` are
/// skipped. The identity is not empty, and the bit is a mapped soul bit.
pub fn parse_attestations(text: &str) -> Result<Vec<Attestation>, String> {
    text.lines()
        .enumerate()
        .map(|(i, line)| (i + 1, line.trim_end_matches('\r')))
        .filter(|(_, line)| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|(n, line)| {
            let fields: Vec<&str> = line.split('\t').collect();
            let [identity, bit] = fields.as_slice() else {
                return Err(format!(
                    "probe.attestation: line {n}: expected identity <TAB> bit"
                ));
            };
            if identity.trim().is_empty() {
                return Err(format!("probe.attestation: line {n}: empty identity"));
            }
            let bit = bit
                .trim()
                .parse::<u16>()
                .ok()
                .and_then(SoulBit::new)
                .ok_or_else(|| {
                    format!("probe.attestation: line {n}: not a mapped soul bit: {bit}")
                })?;
            Ok(Attestation {
                identity: identity.trim().to_owned(),
                bit,
            })
        })
        .collect()
}

fn to_export(input: &Path, out: &Path) -> Result<String, String> {
    let loaded = load(input)?;
    let e = input::to_export(&loaded).map_err(|e| format!("probe.to_export: {e:?}"))?;
    let text = export::to_json(&e).map_err(|e| format!("probe.output: {e:?}"))?;
    std::fs::write(out, &text).map_err(|e| format!("probe.output: {}: {e}", out.display()))?;
    Ok(format!(
        "wrote {} readings, {} bytes\n",
        loaded.readings.len(),
        text.len()
    ))
}

#[cfg(windows)]
fn read(reader: &Path, rest: &[&str]) -> Result<String, String> {
    use std::num::NonZeroU32;

    use super::launch::{self, Elevation, LaunchError, ReadOptions, Sha256};
    use yata_protocol::failure::SessionReason;

    use super::session::{SessionError, Step, Target};

    let mut options = ReadOptions {
        reader,
        target: Target::Discover,
        record: None,
        expected_sha256: None,
    };
    let mut export_to = None;
    for pair in rest.chunks(2) {
        let [flag, value] = pair else {
            return Err(format!("probe.usage: {} needs a value", pair[0]));
        };
        match *flag {
            "--pid" => {
                options.target = Target::Pid(
                    value
                        .parse::<NonZeroU32>()
                        .map_err(|_| format!("probe.usage: not a process id: {value}"))?,
                );
            }
            "--record" => options.record = Some(Path::new(value)),
            "--export" => export_to = Some(Path::new(value)),
            "--reader-sha256" => {
                options.expected_sha256 = Some(
                    Sha256::parse(value)
                        .ok_or_else(|| format!("probe.usage: not a SHA-256: {value}"))?,
                );
            }
            other => return Err(format!("probe.usage: unknown flag {other}")),
        }
    }
    let outcome = launch::read_souls(&options, &mut |done, total| {
        match total {
            Some(t) => eprintln!("progress {done}/{t}"),
            None => eprintln!("progress {done}"),
        }
        Step::Continue
    })
    .map_err(|e| match e {
        LaunchError::ElevationDeclined => "import.elevation_declined: run the reader's export \
                                           mode as administrator and import the file"
            .to_owned(),
        LaunchError::NoExpectedHash => "import.elevation_unverified: the game needs an \
                                        elevated reader; give --reader-sha256 to check it first"
            .to_owned(),
        LaunchError::Session(SessionError::SessionFailed(f)) => {
            let mut text = format!("import.probe_failed: {} {}", f.name(), f.message);
            if let SessionReason::Ambiguous { candidates } = &f.reason {
                for c in candidates {
                    text.push_str(&format!("\n  candidate pid {} {}", c.pid, c.image_name));
                }
            }
            text
        }
        LaunchError::Session(SessionError::RequestFailed { failure, .. }) => {
            format!(
                "import.probe_failed: {} {}",
                failure.name(),
                failure.message
            )
        }
        other => format!("import.probe_failed: {other:?}"),
    })?;
    let mut text = format!(
        "engine {} build {}{}\n",
        outcome.ack.engine,
        outcome.ack.probe_build_id,
        match outcome.elevation {
            Elevation::Elevated => " (elevated)",
            Elevation::Unelevated => "",
        }
    );
    for l in &outcome.logs {
        text.push_str(&format!("log {:?} {}\n", l.level, l.message));
    }
    let loaded = Loaded {
        carrier: input::Carrier::Live {
            failures: Vec::new(),
        },
        provenance: Some(
            input::provenance_of(&outcome.ack).map_err(|e| format!("probe.input: {e:?}"))?,
        ),
        readings: vec![outcome.reading],
    };
    let readings = readings(&loaded)?;
    text.push_str(&report::summary(&loaded, &readings));
    if let Some(out) = export_to {
        let e = input::to_export(&loaded).map_err(|e| format!("probe.to_export: {e:?}"))?;
        let json = export::to_json(&e).map_err(|e| format!("probe.output: {e:?}"))?;
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

    fn bit(n: u16) -> SoulBit {
        SoulBit::new(n).expect("mapped")
    }

    #[test]
    fn attestations_are_tab_separated_with_comments() {
        let text = "# soul\tbit\nabc\t3\r\n\n  \nxyz\t 69 \n";
        assert_eq!(
            parse_attestations(text),
            Ok(vec![
                Attestation {
                    identity: "abc".into(),
                    bit: bit(3)
                },
                Attestation {
                    identity: "xyz".into(),
                    bit: bit(69)
                },
            ])
        );
        assert!(parse_attestations("abc").is_err());
        assert!(parse_attestations("abc\tx").is_err());
        assert!(parse_attestations("abc\t70").is_err());
        // An extra field or an empty identity is a mistake in the file, not something to drop.
        assert!(parse_attestations("abc\t3\tnote").is_err());
        assert!(parse_attestations(" \t3").is_err());
    }
}
