//! The text the `probe` research commands print: deterministic, English, for a developer. The
//! analyses are `yata_core::import::evidence`; this module only lays their results out.

use std::collections::BTreeMap;
use std::fmt::Write;

use yata_core::fact::Channel;
use yata_core::import::evidence::{
    BitStatus, Distinct, GroupKey, Outcome, Source, SuitLedger, Survey, Unjoined,
};
use yata_core::import::observation::{
    Coverage, Field, InnateReading, Mapping, SoulField, SoulObservation, SoulReading,
};

use super::input::{Carrier, Loaded, Width};
use super::session::Failed;

fn mapping(m: Option<&Mapping>) -> String {
    match m {
        None => "not mapped".to_owned(),
        Some(Mapping::Inherited) => "inherited".to_owned(),
        Some(Mapping::Established { basis }) => format!("established ({basis})"),
    }
}

/// The provenance, each reading's coverage and mappings, and each failure.
pub fn summary(loaded: &Loaded, readings: &[SoulReading]) -> String {
    let mut out = String::new();
    let carrier = match &loaded.carrier {
        Carrier::Live { .. } => "live",
        Carrier::Recording { .. } => "recording",
        Carrier::Export { .. } => "export",
    };
    let _ = writeln!(out, "carrier       {carrier}");
    match &loaded.provenance {
        None => {
            let _ = writeln!(out, "provenance    none: the reader never acknowledged");
        }
        Some(p) => {
            let v = p.protocol_version;
            let _ = writeln!(out, "protocol      {}.{}", v.major, v.minor);
            let _ = writeln!(out, "probe build   {}", p.probe_build_id);
            let _ = writeln!(out, "engine        {}", p.engine);
            let channel = match p.channel {
                Channel::DesktopMemory => "desktop memory",
            };
            let _ = writeln!(out, "channel       {channel}");
            if let Some(t) = &p.target {
                let width = match t.width {
                    Some(Width::Bits32) => "32-bit",
                    Some(Width::Bits64) => "64-bit",
                    None => "width unknown",
                };
                let _ = writeln!(
                    out,
                    "target        {} (pid {}, {width})",
                    t.image_name, t.pid
                );
            }
        }
    }
    match &loaded.carrier {
        Carrier::Export {
            captured_at: Some(c),
        } => {
            let _ = writeln!(out, "captured at   {c}");
        }
        Carrier::Export { captured_at: None } => {}
        Carrier::Live { failures } | Carrier::Recording { failures } => {
            for f in failures {
                let subject = match f {
                    Failed::Session(_) => "session".to_owned(),
                    Failed::Request { id, .. } => format!("request {}", id.get()),
                };
                let _ = writeln!(out, "failed        {subject}: {} {}", f.name(), f.message());
            }
        }
    }
    for (i, r) in readings.iter().enumerate() {
        let coverage = match r.coverage() {
            Coverage::Complete => "complete",
            Coverage::Partial => "partial",
        };
        let observed = r.souls().iter().filter(|s| s.observed.is_some()).count();
        let _ = writeln!(
            out,
            "reading {i}     {} souls, {observed} observed records, coverage {coverage}",
            r.souls().len()
        );
        let _ = writeln!(
            out,
            "  {:<24}{}",
            "recognition rule",
            mapping(r.recognition())
        );
        for f in SoulField::ALL {
            let _ = writeln!(out, "  {:<24}{}", f.name(), mapping(r.mappings().get(f)));
        }
    }
    out
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
        let distinct = match k.distinct {
            Distinct::Exactly(n) => n.to_string(),
            Distinct::AtLeast(n) => format!("{n}+"),
        };
        let _ = writeln!(
            out,
            "{:<20} in {:>6}  kinds {}  distinct {distinct}{range}  e.g. {}",
            k.key,
            k.records,
            counts(&k.kinds),
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

fn field<T>(name: &str, f: &Field<T>, show: impl Fn(&T) -> String) -> Option<String> {
    match f {
        Field::Unmapped => None,
        Field::Mapped { value: None, .. } => Some(format!("{name}=missing")),
        Field::Mapped { value: Some(v), .. } => Some(format!("{name}={}", show(v))),
    }
}

/// One soul on one line: its mapped fields, its container key, and every observed entry.
pub fn soul_line(s: &SoulObservation) -> String {
    let parts = [
        field("id", &s.soul_id, String::clone),
        field("suit", &s.suit_code, |c| c.0.to_string()),
        field("star", &s.star, |v| v.0.to_string()),
        field("slot", &s.slot, |v| v.0.to_string()),
        field("level", &s.level, |v| v.0.to_string()),
        field("innate", &s.innate, |i| match i {
            InnateReading::None => "none".to_owned(),
            InnateReading::Present(a) => format!("({}, {})", a.code.0, a.value),
        }),
    ];
    let mut out: Vec<String> = parts.into_iter().flatten().collect();
    if let Some(o) = &s.observed {
        if let Some(k) = &o.container_key {
            out.push(format!("@{}", k.render()));
        }
        let entries: Vec<String> = o
            .entries
            .iter()
            .map(|(k, v)| format!("{}: {}", k.render(), v.render()))
            .collect();
        out.push(format!("{{{}}}", entries.join(", ")));
    }
    out.join(" ")
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
    let count = |want: Option<Outcome>| {
        l.rows
            .iter()
            .filter(|r| match &r.status {
                BitStatus::Unattested => want.is_none(),
                BitStatus::Attested { outcome, .. } => want == Some(*outcome),
            })
            .count()
    };
    let _ = writeln!(
        out,
        "bits {}: reestablished {}, contradicted {}, unattested {}; unjoined attestations {}",
        l.rows.len(),
        count(Some(Outcome::Reestablished)),
        count(Some(Outcome::Contradicted)),
        count(None),
        l.unjoined.len()
    );
    let _ = writeln!(
        out,
        "inherited suit codes retired: {}",
        if l.retires_inheritance() { "yes" } else { "no" }
    );
    for r in &l.rows {
        let head = format!(
            "bit {:>2}  inherited {:>2}",
            r.bit.index(),
            r.inherited.suit_code()
        );
        let _ = match &r.status {
            BitStatus::Unattested => writeln!(out, "{head}  unattested"),
            BitStatus::Attested {
                observed,
                attestations,
                outcome,
            } => {
                let codes: Vec<String> = observed.iter().map(i64::to_string).collect();
                writeln!(
                    out,
                    "{head}  observed {:<8} attestations {attestations}  {outcome:?}",
                    codes.join(",")
                )
            }
        };
    }
    for u in &l.unjoined {
        let text = match u {
            Unjoined::NoSoul { identity } => format!("{identity}: no soul has it"),
            Unjoined::Ambiguous { identity, souls } => format!("{identity}: {souls} souls have it"),
            Unjoined::NoSuitValue { identity } => {
                format!("{identity}: no integer at the suit source")
            }
            Unjoined::SuitOutOfRange { identity, value } => {
                format!("{identity}: {value} minus the offset is out of range")
            }
        };
        let _ = writeln!(out, "unjoined {text}");
    }
    out
}
