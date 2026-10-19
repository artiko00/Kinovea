use super::*;
use crate::grid::Verdict;
use crate::run::{Judged, ScaleResult};

fn verdict(percent: f64, off_boundary: usize) -> Verdict {
    Verdict {
        percent,
        off_boundary,
    }
}

fn judged(case: &'static str, id: &str, element: &str, scales: Vec<ScaleResult>) -> Judged {
    Judged {
        case,
        id: id.into(),
        element: element.into(),
        ported: true,
        scales,
        exceptions: Vec::new(),
    }
}

fn scale(scale: f64, plain: Verdict, text: Verdict) -> ScaleResult {
    ScaleResult { scale, plain, text }
}

#[test]
fn a_drawing_with_no_compared_scale_is_a_violation() {
    let d = judged("geometry", "k0#0", "Line", Vec::new());
    assert!(violations(&[d])
        .iter()
        .any(|v| v.contains("no scale compared")));
}

#[test]
fn a_drawing_below_the_percentage_is_a_violation() {
    let d = judged(
        "geometry",
        "k0#0",
        "Line",
        vec![scale(1.0, verdict(98.0, 0), verdict(98.0, 0))],
    );
    assert!(violations(&[d]).iter().any(|v| v.contains("98.00% agree")));
}

#[test]
fn a_listed_drawing_that_meets_the_strict_rule_is_stale() {
    let d = judged(
        "all-drawings",
        "k0#10",
        "CrossMark",
        vec![scale(1.0, verdict(100.0, 0), verdict(100.0, 0))],
    );
    assert!(violations(&[d]).iter().any(|v| v.contains("stale entry")));
}

#[test]
fn a_listed_drawing_is_judged_by_its_text_zone() {
    let d = judged(
        "all-drawings",
        "k0#10",
        "CrossMark",
        vec![scale(1.0, verdict(90.0, 0), verdict(100.0, 0))],
    );
    let found = violations(&[d]);
    assert!(
        !found.iter().any(|v| v.contains("agree in the box")),
        "{found:?}"
    );
}
