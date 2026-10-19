//! Replays every `tracking` oracle case on the Rust trackers, with the tolerances of design D6:
//! template positions within 0.5 px and similarity within 0.02, circle center and radius within 1 px,
//! matched flags exact.

mod tracking_cases;

use kinovea_tracking::circle::CircleTracker;
use kinovea_tracking::template::TemplateTracker;
use tracking_cases::{expected, load, names, run, Row};

const POSITION_TOLERANCE: f64 = 0.5;
const SCORE_TOLERANCE: f64 = 0.02;
/// Largest center or radius difference, in px, allowed on the circle cases.
const CIRCLE_TOLERANCE_PX: f64 = 1.0;
/// Cases where the original leaves the tolerances at one frame, from f32 rounding in OpenCV's
/// correlation. At (69, 61) of `template-mask-flat` the numerator error (about 8 units on 4.7e7,
/// measured with `TM_CCORR`) exceeds the snap margin (7.89), so OpenCV gives 0 where the exact NCC
/// is 0.9836. Rows before the frame meet the tolerances, the frame is the first one outside them, and
/// later rows are not compared since the trajectory has diverged.
const F32_NEAR_TIES: &[(&str, usize)] = &[("template-mask-flat", 4)];

#[test]
fn template_cases_match_the_original() {
    for name in names() {
        let case = load(&name);
        if case.circle {
            continue;
        }
        let mut tracker = TemplateTracker::new(case.parameters.clone());
        let rows = run(&case, &mut tracker);
        let expected = expected(&name);
        assert_eq!(rows.len(), expected.len(), "{name}: row count");
        let first_outside = F32_NEAR_TIES
            .iter()
            .find(|(case, _)| *case == name)
            .map(|(_, frame)| *frame);
        let (mut dx, mut dy, mut ds) = (0.0f64, 0.0f64, 0.0f64);
        for (got, want) in rows.iter().zip(&expected) {
            let frame = got.frame;
            assert_eq!(
                (got.frame, got.t, got.r),
                (want.frame, want.t, want.r),
                "{name}: frame, t and r"
            );
            if Some(frame) == first_outside {
                assert!(
                    !within_tolerance(got, want),
                    "{name} frame {frame}: within tolerance, the f32 near-tie deviation is gone"
                );
                break;
            }
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

fn within_tolerance(got: &Row, want: &Row) -> bool {
    (got.x - want.x).abs() <= POSITION_TOLERANCE
        && (got.y - want.y).abs() <= POSITION_TOLERANCE
        && (got.score - want.score).abs() <= SCORE_TOLERANCE
        && got.matched == want.matched
}

#[test]
fn circle_cases_match_the_original() {
    let cases: Vec<_> = names()
        .into_iter()
        .map(|name| (load(&name), name))
        .filter(|(case, _)| case.circle)
        .collect();
    assert!(!cases.is_empty());
    for (case, name) in &cases {
        let mut tracker = CircleTracker::new(case.parameters.clone());
        let rows = run(case, &mut tracker);
        let want = expected(name);
        assert_eq!(rows.len(), want.len(), "{name}: row count");
        let mut worst = 0.0_f64;
        for (got, want) in rows.iter().zip(&want) {
            assert_eq!(
                (got.frame, got.t),
                (want.frame, want.t),
                "{name}: frame and time"
            );
            assert_eq!(
                got.matched, want.matched,
                "{name}: frame {} matched",
                want.frame
            );
            let diff = (got.x - want.x)
                .abs()
                .max((got.y - want.y).abs())
                .max((got.r - want.r).abs());
            worst = worst.max(diff);
        }
        eprintln!("{name}: worst center or radius difference {worst:.3} px");
        assert!(worst <= CIRCLE_TOLERANCE_PX, "{name}: {worst} px");
    }
}
