use super::*;

#[test]
fn rule_thresholds_match_the_spec() {
    assert_eq!(MIN_PERCENT, 99.0);
    assert_eq!(BOUNDARY_RADIUS, 2);
}

fn judged(percent: f64, off_boundary: usize) -> Judged {
    Judged {
        case: "geometry",
        id: "k0#0".into(),
        element: "Line".into(),
        ported: true,
        scales: vec![ScaleResult {
            scale: 1.0,
            percent,
            off_boundary,
        }],
    }
}

#[test]
fn a_drawing_meets_the_rule_only_when_every_scale_passes() {
    assert!(judged(100.0, 0).meets_rule());
    assert!(!judged(98.9, 0).meets_rule());
    assert!(!judged(100.0, 1).meets_rule());
}

#[test]
fn a_drawing_without_a_port_never_meets_the_rule() {
    let unported = Judged {
        ported: false,
        ..judged(100.0, 0)
    };
    assert!(!unported.meets_rule());
}
