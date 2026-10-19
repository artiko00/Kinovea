//! The verdict of the hit tables: the strict rule, with the exemptions of `rule.rs` only where listed.

use super::grid::MIN_PERCENT;
use super::rule::{
    known_max, pending_codes, text_codes, EXPECTED_EXCEPTIONS, KNOWN_DEVIATIONS,
    PENDING_TEXT_METRICS,
};
use super::run::Judged;
use super::table::Row;
use super::targets::Entry;
use std::collections::BTreeSet;

/// Every way the tables break the rule, one message each. Empty when the port matches the oracle.
pub fn violations(judged: &[Judged]) -> Vec<String> {
    let mut out = Vec::new();
    for (case, id, codes) in PENDING_TEXT_METRICS {
        let Some(d) = find(judged, case, id) else {
            out.push(format!(
                "{case} {id}: listed as text metric but not in the probe tables"
            ));
            continue;
        };
        let zone = text_codes(&d.element);
        if codes.iter().any(|c| !zone.contains(c)) {
            out.push(format!(
                "{case} {id}: codes {codes:?} are not the text zone of {}",
                d.element
            ));
        }
        if d.meets_rule() {
            out.push(format!(
                "{case} {id}: listed as text metric but meets the rule (stale entry)"
            ));
        }
    }
    for (case, id, _) in EXPECTED_EXCEPTIONS {
        match find(judged, case, id) {
            None => out.push(format!(
                "{case} {id}: listed as expected exception but not in the probe tables"
            )),
            Some(d) if d.exceptions.is_empty() => {
                out.push(format!(
                    "{case} {id}: listed as expected exception but it does not throw (stale entry)"
                ));
            }
            Some(_) => {}
        }
    }
    for (case, id, scale, _) in KNOWN_DEVIATIONS {
        let off = find(judged, case, id)
            .and_then(|d| d.scales.iter().find(|s| format!("{}", s.scale) == *scale))
            .map(|s| s.plain.off_boundary);
        match off {
            None => out.push(format!(
                "{case} {id} x{scale}: known deviation not in the probe tables"
            )),
            Some(0) => out.push(format!(
                "{case} {id} x{scale}: known deviation has no probe off a boundary (stale entry)"
            )),
            Some(_) => {}
        }
    }
    for d in judged {
        out.extend(drawing_violations(d));
    }
    out
}

fn drawing_violations(d: &Judged) -> Vec<String> {
    let label = format!("{} {} {}", d.case, d.id, d.element);
    let excepted = EXPECTED_EXCEPTIONS
        .iter()
        .any(|(c, i, _)| *c == d.case && *i == d.id);
    let listed = pending_codes(d.case, &d.id).is_some();
    if !d.ported {
        return vec![format!("{label}: not ported")];
    }
    let mut out = Vec::new();
    if d.scales.is_empty() && !excepted {
        out.push(format!(
            "{label}: no scale compared, and not in EXPECTED_EXCEPTIONS"
        ));
    }
    if !d.exceptions.is_empty() && !excepted {
        out.push(format!(
            "{label}: throws at scales {:?}, not in EXPECTED_EXCEPTIONS",
            d.exceptions
        ));
    }
    for s in &d.scales {
        let verdict = if listed { s.text } else { s.plain };
        if verdict.percent < MIN_PERCENT {
            out.push(format!(
                "{label} x{}: {:.2}% agree in the box",
                s.scale, verdict.percent
            ));
        }
        let max = known_max(d.case, &d.id, &format!("{}", s.scale));
        match (verdict.off_boundary, max) {
            (0, _) => {}
            (off, Some(max)) if off <= max => {}
            (off, Some(max)) => out.push(format!(
                "{label} x{}: {off} mismatches off the Rust boundary, above the known maximum {max}",
                s.scale
            )),
            (off, None) => out.push(format!("{label} x{}: {off} mismatches off the Rust boundary", s.scale)),
        }
    }
    out
}

fn find<'a>(judged: &'a [Judged], case: &str, id: &str) -> Option<&'a Judged> {
    judged.iter().find(|d| d.case == case && d.id == id)
}

/// The `hits.csv` rows must name a drawing of the metadata, with its element and its probe time.
pub fn rows_match(case: &str, rows: &[Row], entries: &[Entry]) -> Vec<String> {
    let mut problems = BTreeSet::new();
    for r in rows {
        let found = entries
            .iter()
            .find(|e| e.key.container == r.key.container && e.key.index == r.key.index);
        let label = format!("{case} {}#{}", r.key.container, r.key.index);
        match found {
            None => problems.insert(format!(
                "{label}: rows for a drawing the metadata does not have"
            )),
            Some(e) if e.key.element != r.key.element => problems.insert(format!(
                "{label}: the table names {}, the metadata {}",
                r.key.element, e.key.element
            )),
            Some(e) if e.timestamp != r.timestamp => problems.insert(format!(
                "{label}: the table probes at {}, the metadata at {}",
                r.timestamp, e.timestamp
            )),
            Some(_) => false,
        };
    }
    problems.into_iter().collect()
}

#[cfg(test)]
#[path = "check_test.rs"]
mod check_test;
