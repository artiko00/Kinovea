use crate::angular::{build_angular_kinematics, AngleOptions, AngularError};
use crate::component::Kinematics::*;
use crate::context::PixelContext;
use crate::trajectory::FilteredTrajectory;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::SizeF;

fn ctx() -> PixelContext {
    PixelContext {
        filtering: false,
        ..PixelContext::new(SizeF::new(0.0, 0.0), 100.0)
    }
}

fn fixed(x: f32, y: f32, n: usize) -> FilteredTrajectory {
    let pts: Vec<TimedPoint> = (0..n).map(|i| TimedPoint::new(x, -y, i as i64)).collect();
    FilteredTrajectory::new(&pts, &ctx())
}

fn opts(signed: bool, ccw: bool, supplementary: bool) -> AngleOptions {
    AngleOptions { signed, ccw, supplementary }
}

#[test]
fn needs_exactly_three_trajectories() {
    let o = fixed(0.0, 0.0, 3);
    let err = build_angular_kinematics(&[&o, &o], opts(true, true, false), &ctx()).unwrap_err();
    assert_eq!(err, AngularError(2));
    assert_eq!(err.to_string(), "angular kinematics need 3 trajectories (origin, a, b), got 2");
}

#[test]
fn unsigned_wrap_and_supplementary() {
    let o = fixed(0.0, 0.0, 1);
    let a = fixed(1.0, 0.0, 1);
    let down = fixed(0.0, -1.0, 1);
    let up = fixed(0.0, 1.0, 1);
    let signed = build_angular_kinematics(&[&o, &a, &down], opts(true, true, false), &ctx()).unwrap();
    assert!((signed[AngularPosition][0] + 90.0).abs() < 1e-4);
    let unsigned = build_angular_kinematics(&[&o, &a, &down], opts(false, true, false), &ctx()).unwrap();
    assert!((unsigned[AngularPosition][0] - 270.0).abs() < 1e-3);
    let cw = build_angular_kinematics(&[&o, &a, &up], opts(true, false, false), &ctx()).unwrap();
    assert!((cw[AngularPosition][0] + 90.0).abs() < 1e-4);
    let supp = build_angular_kinematics(&[&o, &a, &up], opts(true, true, true), &ctx()).unwrap();
    assert!((supp[AngularPosition][0] - 90.0).abs() < 1e-4, "{}", supp[AngularPosition][0]);
}

#[test]
fn constant_angular_speed_gives_constant_omega_and_zero_alpha() {
    let n = 12;
    let o = fixed(0.0, 0.0, n);
    let b = fixed(10.0, 0.0, n);
    let moving: Vec<TimedPoint> = (0..n)
        .map(|i| {
            let th = 0.1 * i as f64;
            TimedPoint::new((10.0 * th.cos()) as f32, -(10.0 * th.sin()) as f32, i as i64)
        })
        .collect();
    let a = FilteredTrajectory::new(&moving, &ctx());
    let tsc = build_angular_kinematics(&[&o, &a, &b], opts(true, true, false), &ctx()).unwrap();
    assert!((tsc[AngularPosition][3] + 0.3f64.to_degrees()).abs() < 1e-3);
    assert!((tsc[TotalAngularDisplacement][3] - tsc[AngularPosition][3]).abs() < 1e-9);
    assert!(tsc[AngularVelocity][0].is_nan() && tsc[AngularVelocity][n - 1].is_nan());
    for i in 1..n - 1 {
        assert!((tsc[AngularVelocity][i] + 10f64.to_degrees()).abs() < 1e-2, "{}", tsc[AngularVelocity][i]);
        assert!((tsc[TangentialVelocity][i] + 100.0).abs() < 1e-2);
    }
    assert!(tsc[AngularAcceleration][1].is_nan() && tsc[CentripetalAcceleration][n - 2].is_nan());
    for i in 2..n - 2 {
        assert!(tsc[AngularAcceleration][i].abs() < 1e-1, "{}", tsc[AngularAcceleration][i]);
        assert!((tsc[CentripetalAcceleration][i] - 1000.0).abs() < 1.0);
        assert!((tsc[ResultantLinearAcceleration][i] - tsc[CentripetalAcceleration][i]).abs() < 1.0);
    }
}
