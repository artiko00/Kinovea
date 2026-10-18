//! Linear and angular kinematics replayed through the oracle fixtures.

mod support;
use support::*;

use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::SizeF;
use kinovea_kinematics::Kinematics::*;
use kinovea_kinematics::{
    build_angular_kinematics, build_linear_kinematics, AngleOptions, FilteredTrajectory,
    PixelContext,
};

/// World coordinates re-entered as image samples: with a zero image size the pixel context maps
/// `(x, −y)` back to `(x, y)` exactly, so the fixture's raw world columns are reproduced bit for bit.
fn samples(xs: &[f64], ys: &[f64]) -> Vec<TimedPoint> {
    xs.iter()
        .zip(ys)
        .enumerate()
        .map(|(i, (x, y))| TimedPoint::new(*x as f32, -(*y as f32), i as i64))
        .collect()
}

fn context(name: &str) -> PixelContext {
    let fps: f64 = json_value(&filtering_json(name), "fps").parse().unwrap();
    PixelContext {
        filtering: case_flag(name, "filtering"),
        smoothing: case_flag(name, "smoothing"),
        ..PixelContext::new(SizeF::new(0.0, 0.0), fps)
    }
}

#[test]
fn linear_kinematics_match_every_cell() {
    for name in ["linear", "winter", "linear-unsmoothed", "short", "nofilter"] {
        let (header, rows) = series(name, "linear.csv");
        let ctx = context(name);
        let traj = FilteredTrajectory::new(
            &samples(&column(&header, &rows, XRaw), &column(&header, &rows, YRaw)),
            &ctx,
        );
        let json = filtering_json(name);
        assert_eq!(
            traj.can_filter.to_string(),
            json_value(&json, "can_filter"),
            "{name} can_filter"
        );
        let index = |o: Option<usize>| o.map_or(-1, |i| i as i64).to_string();
        assert_eq!(
            index(traj.x_cutoff_index),
            json_value(&json, "x_cutoff_index"),
            "{name} x cutoff"
        );
        assert_eq!(
            index(traj.y_cutoff_index),
            json_value(&json, "y_cutoff_index"),
            "{name} y cutoff"
        );
        let tsc = build_linear_kinematics(&traj, &ctx);
        assert_eq!(tsc.length(), rows.len(), "{name} length");
        for (i, row) in rows.iter().enumerate() {
            for (key, want) in header.iter().zip(row) {
                assert_cell(tsc[*key][i], *want, &format!("{name} {key}[{i}]"));
            }
        }
    }
}

#[test]
fn unsmoothed_velocities_differ_from_the_smoothed_fixture() {
    let (h1, r1) = series("linear", "linear.csv");
    let (h2, r2) = series("linear-unsmoothed", "linear.csv");
    assert_eq!(column(&h1, &r1, XRaw), column(&h2, &r2, XRaw));
    assert_ne!(
        column(&h1, &r1, LinearSpeed)[5],
        column(&h2, &r2, LinearSpeed)[5]
    );
}

#[test]
fn angular_kinematics_match_the_circular_fixture() {
    let name = "circular";
    let (lh, lr) = series(name, "linear.csv");
    let (header, rows) = series(name, "angular.csv");
    let ctx = context(name);
    let n = lr.len();
    let a = FilteredTrajectory::new(
        &samples(&column(&lh, &lr, XRaw), &column(&lh, &lr, YRaw)),
        &ctx,
    );
    let o = FilteredTrajectory::new(&samples(&vec![0.0; n], &vec![0.0; n]), &ctx);
    let b = FilteredTrajectory::new(&samples(&vec![200.0; n], &vec![0.0; n]), &ctx);
    let options = AngleOptions {
        signed: true,
        ccw: true,
        supplementary: false,
    };
    let tsc = build_angular_kinematics(&[&o, &a, &b], options, &ctx).unwrap();
    assert_eq!(tsc.length(), rows.len());
    for (i, row) in rows.iter().enumerate() {
        for (key, want) in header.iter().zip(row) {
            assert_cell(tsc[*key][i], *want, &format!("{name} {key}[{i}]"));
        }
    }
}
