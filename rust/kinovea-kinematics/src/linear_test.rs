use crate::component::Kinematics::*;
use crate::context::PixelContext;
use crate::linear::build_linear_kinematics;
use crate::trajectory::FilteredTrajectory;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::SizeF;

fn ctx(smoothing: bool) -> PixelContext {
    PixelContext {
        filtering: false,
        smoothing,
        ..PixelContext::new(SizeF::new(100.0, 100.0), 100.0)
    }
}

fn line(n: usize) -> Vec<TimedPoint> {
    (0..n).map(|i| TimedPoint::new(50.0 + 3.0 * i as f32, 50.0 - 4.0 * i as f32, i as i64)).collect()
}

#[test]
fn two_samples_have_distances_but_nan_derivatives() {
    let tsc = build_linear_kinematics(&FilteredTrajectory::new(&line(2), &ctx(false)), &ctx(false));
    assert_eq!(tsc.length(), 2);
    assert_eq!(tsc[LinearDistance], vec![0.0, 5.0]);
    assert_eq!(tsc[LinearHorizontalDisplacement], vec![0.0, 3.0]);
    assert_eq!(tsc[LinearVerticalDisplacement], vec![0.0, 4.0]);
    for k in [LinearSpeed, LinearHorizontalVelocity, LinearVerticalVelocity, LinearAcceleration, LinearHorizontalAcceleration, LinearVerticalAcceleration] {
        assert!(tsc[k].iter().all(|v| v.is_nan()), "{k}");
    }
    assert_eq!(tsc[XRaw], vec![0.0, 3.0]);
    assert_eq!(tsc[X], tsc[XRaw]);
}

#[test]
fn constant_velocity_line_without_smoothing() {
    let tsc = build_linear_kinematics(&FilteredTrajectory::new(&line(8), &ctx(false)), &ctx(false));
    assert!(tsc[LinearSpeed][0].is_nan() && tsc[LinearSpeed][7].is_nan());
    for i in 1..7 {
        assert!((tsc[LinearSpeed][i] - 500.0).abs() < 1e-3, "{}", tsc[LinearSpeed][i]);
        assert!((tsc[LinearHorizontalVelocity][i] - 300.0).abs() < 1e-3);
        assert!((tsc[LinearVerticalVelocity][i] - 400.0).abs() < 1e-3);
    }
    assert!(tsc[LinearAcceleration][1].is_nan() && tsc[LinearAcceleration][6].is_nan());
    for i in 2..6 {
        assert!(tsc[LinearAcceleration][i].abs() < 1e-2, "{}", tsc[LinearAcceleration][i]);
    }
    assert_eq!(tsc[LinearDistance][7], 35.0);
}

#[test]
fn empty_trajectory_yields_an_empty_collection() {
    let tsc = build_linear_kinematics(&FilteredTrajectory::default(), &ctx(true));
    assert_eq!(tsc.length(), 0);
    assert!(tsc.get(LinearSpeed).is_none());
}
