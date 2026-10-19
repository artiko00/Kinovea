//! Replays every `tracking` oracle case on the Rust trackers, with the tolerances of design D6:
//! template positions within 0.5 px and similarity within 0.02, circle center and radius within 1 px,
//! matched flags exact.

mod tracking_cases;

use kinovea_tracking::template::TemplateTracker;

/// Design D6: positions within 0.5 px, similarity within 0.02, matched flags exact.
const POSITION_TOLERANCE: f64 = 0.5;
const SCORE_TOLERANCE: f64 = 0.02;

#[test]
fn template_cases_match_the_original() {
    for name in tracking_cases::names() {
        let case = tracking_cases::load(&name);
        if case.circle {
            continue;
        }
        let mut tracker = TemplateTracker::new(case.parameters.clone());
        let rows = tracking_cases::run(&case, &mut tracker);
        let expected = tracking_cases::expected(&name);
        assert_eq!(rows.len(), expected.len(), "{name}: row count");
        let (mut dx, mut dy, mut ds) = (0.0f64, 0.0f64, 0.0f64);
        for (got, want) in rows.iter().zip(&expected) {
            let frame = got.frame;
            dx = dx.max((got.x - want.x).abs());
            dy = dy.max((got.y - want.y).abs());
            ds = ds.max((got.score - want.score).abs());
            assert!(
                (got.x - want.x).abs() <= POSITION_TOLERANCE,
                "{name} frame {frame}: x {} vs {}",
                got.x,
                want.x
            );
            assert!(
                (got.y - want.y).abs() <= POSITION_TOLERANCE,
                "{name} frame {frame}: y {} vs {}",
                got.y,
                want.y
            );
            assert!(
                (got.score - want.score).abs() <= SCORE_TOLERANCE,
                "{name} frame {frame}: score {} vs {}",
                got.score,
                want.score
            );
            assert_eq!(got.matched, want.matched, "{name} frame {frame}: matched");
        }
        println!(
            "{name}: worst |dx| {dx:.4} px, |dy| {dy:.4} px, |dscore| {ds:.4} over {} rows",
            rows.len()
        );
    }
}
