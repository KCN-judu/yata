//! The text the `probe` research commands print: deterministic, English, for a developer. The
//! analyses are `yata_core::import::evidence`; this module only lays their results out.

use std::collections::BTreeMap;
use std::fmt::Write;

use yata_core::import::evidence::{BitStatus, GroupKey, Source, SuitLedger, Survey, Unjoined};
use yata_core::import::observation::{Coverage, SoulField, SoulObservation, SoulReading};

use super::input::{Carrier, Loaded};

/// The provenance, each result's coverage and evidence, and each failure.
pub fn summary(loaded: &Loaded, readings: &[SoulReading]) -> String {
    let p = &loaded.provenance;
    let mut out = String::new();
    let carrier = match loaded.carrier {
        Carrier::Recording => "recording",
        Carrier::Export => "export",
    };
    let version = p
        .protocol_version
        .map_or_else(|| "none".to_owned(), |v| format!("{}.{}", v.major, v.minor));
    let _ = writeln!(out, "carrier       {carrier}");
    let _ = writeln!(out, "protocol      {version}");
    let _ = writeln!(out, "probe build   {}", or_none(&p.probe_build_id));
    let _ = writeln!(out, "engine        {}", or_none(&p.engine));
    let _ = writeln!(out, "channel       {}", p.channel.as_str_name());
    if let Some(t) = &p.target {
        let _ = writeln!(
            out,
            "target        {} (pid {}, {}-bit)",
            t.image_name, t.pid, t.pointer_bits
        );
    }
    if let Some(c) = &p.captured_at {
        let _ = writeln!(out, "captured at   {c}");
    }
    for f in &loaded.failures {
        let e = f.error.clone().unwrap_or_default();
        let _ = writeln!(
            out,
            "failed        request {}: {} {}",
            f.request_id, e.code, e.message
        );
    }
    for (i, r) in readings.iter().enumerate() {
        let coverage = match r.coverage {
            Coverage::Complete => "complete",
            Coverage::Partial => "partial",
            Coverage::Unstated => "unstated",
        };
        let observed = r.souls.iter().filter(|s| s.observed.is_some()).count();
        let _ = writeln!(
            out,
            "reading {i}     {} souls, {observed} observed records, coverage {coverage}",
            r.souls.len()
        );
        let recognition = r
            .recognition
            .map_or_else(|| "not stated".to_owned(), |e| format!("{e:?}"));
        let _ = writeln!(out, "  {:<24}{recognition}", "recognition rule");
        for f in SoulField::ALL {
            let e = r
                .evidence_of(f)
                .map_or_else(|| "not mapped".to_owned(), |e| format!("{e:?}"));
            let _ = writeln!(out, "  {:<24}{e}", f.schema_name());
        }
    }
    out
}

fn or_none(s: &str) -> &str {
    if s.is_empty() { "(none)" } else { s }
}

pub fn survey(s: &Survey) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "souls {}, observed records {}", s.souls, s.observed);
    let _ = writeln!(out, "record types  {}", counts(&s.type_names));
    let _ = writeln!(out, "container keys {}", counts(&s.container_kinds));
    for k in &s.keys {
        let range = k
            .integer_range
            .map_or_else(String::new, |(lo, hi)| format!("  range {lo}..={hi}"));
        let _ = writeln!(
            out,
            "{:<20} in {:>6}  kinds {}  distinct {}{}{range}  e.g. {}",
            k.key,
            k.records,
            counts(&k.kinds),
            k.distinct,
            if k.distinct >= yata_core::import::evidence::DISTINCT_CAP {
                "+"
            } else {
                ""
            },
            k.examples.join(" | ")
        );
    }
    out
}

fn counts<K: std::fmt::Debug>(m: &BTreeMap<K, u64>) -> String {
    if m.is_empty() {
        return "-".to_owned();
    }
    m.iter()
        .map(|(k, n)| format!("{k:?}×{n}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Each group's size and up to `samples` of its souls in full, for finding them in the game.
pub fn groups(
    groups: &BTreeMap<GroupKey, Vec<&SoulObservation>>,
    by: &Source,
    samples: usize,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{} groups by {by:?}", groups.len());
    for (key, souls) in groups {
        let _ = writeln!(out, "{:<24} {} souls", key.render(), souls.len());
        for s in souls.iter().take(samples) {
            let _ = writeln!(out, "    {}", soul_line(s));
        }
    }
    out
}

/// One soul on one line: its typed fields, its container key, and every observed entry.
pub fn soul_line(s: &SoulObservation) -> String {
    let mut parts = Vec::new();
    if let Some(id) = &s.soul_id {
        parts.push(format!("id={id}"));
    }
    for (name, v) in [
        ("suit", s.suit_code),
        ("star", s.star),
        ("slot", s.slot),
        ("level", s.level),
    ] {
        if let Some(v) = v {
            parts.push(format!("{name}={v}"));
        }
    }
    if let Some(o) = &s.observed {
        if let Some(k) = &o.container_key {
            parts.push(format!("@{}", k.render()));
        }
        let entries: Vec<String> = o
            .entries
            .iter()
            .map(|(k, v)| format!("{}: {}", k.render(), v.render()))
            .collect();
        parts.push(format!("{{{}}}", entries.join(", ")));
    }
    parts.join(" ")
}

pub fn crosstab(table: &BTreeMap<(GroupKey, GroupKey), u64>) -> String {
    let mut out = String::new();
    let mut last: Option<&GroupKey> = None;
    for ((row, column), n) in table {
        if last != Some(row) {
            let _ = writeln!(out, "{}", row.render());
            last = Some(row);
        }
        let _ = writeln!(out, "    {:<24} {n}", column.render());
    }
    out
}

pub fn ledger(l: &SuitLedger) -> String {
    let mut out = String::new();
    let count = |s: &BitStatus| l.rows.iter().filter(|r| &r.status == s).count();
    let _ = writeln!(
        out,
        "bits {}: reestablished {}, contradicted {}, unattested {}; unjoined attestations {}",
        l.rows.len(),
        count(&BitStatus::Reestablished),
        count(&BitStatus::Contradicted),
        count(&BitStatus::Unattested),
        l.unjoined.len()
    );
    let _ = writeln!(
        out,
        "inherited suit codes retired: {}",
        if l.retires_inheritance() { "yes" } else { "no" }
    );
    for r in &l.rows {
        let observed = r
            .observed
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let _ = writeln!(
            out,
            "bit {:>2}  inherited {:>2}  observed {:<8} attestations {}  {:?}",
            r.bit, r.inherited, observed, r.attestations, r.status
        );
    }
    for u in &l.unjoined {
        let _ = writeln!(
            out,
            "unjoined {}",
            match u {
                Unjoined::NoSoul { identity } => format!("{identity}: no soul has it"),
                Unjoined::Ambiguous { identity, souls } =>
                    format!("{identity}: {souls} souls have it"),
                Unjoined::NoSuitValue { identity } =>
                    format!("{identity}: no integer at the suit source"),
                Unjoined::NoSuchBit { identity, bit } =>
                    format!("{identity}: bit {bit} is not a mapped soul bit"),
            }
        );
    }
    out
}
