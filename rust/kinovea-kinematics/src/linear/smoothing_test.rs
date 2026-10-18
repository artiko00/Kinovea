use crate::component::Kinematics;
use crate::linear::smoothing::{smooth, VELOCITY_SPAN_MS};
use crate::time_series::{pad, TimeSeriesCollection};

fn noisy() -> TimeSeriesCollection {
    let mut tsc = TimeSeriesCollection::new(30);
    let mut v: Vec<f64> = (0..30).map(|i| 10.0 + if i % 2 == 0 { 1.0 } else { -1.0 }).collect();
    pad(&mut v, 1);
    tsc.add_component(Kinematics::LinearSpeed, v);
    tsc
}

#[test]
fn smoothing_changes_interior_velocities_at_high_frame_rate() {
    let mut tsc = noisy();
    let before = tsc[Kinematics::LinearSpeed].clone();
    smooth(&mut tsc, &[Kinematics::LinearSpeed], 200.0, VELOCITY_SPAN_MS, 1);
    let after = &tsc[Kinematics::LinearSpeed];
    assert_eq!(after.len(), 30);
    assert_ne!(after[10], before[10]);
    assert!((after[10] - 10.0).abs() < 0.3, "{}", after[10]);
}

#[test]
fn smoothing_leaves_a_25_fps_series_unchanged() {
    let mut tsc = noisy();
    let before = tsc[Kinematics::LinearSpeed].clone();
    smooth(&mut tsc, &[Kinematics::LinearSpeed], 25.0, VELOCITY_SPAN_MS, 1);
    let after = &tsc[Kinematics::LinearSpeed];
    assert!(after.iter().zip(&before).all(|(a, b)| a.to_bits() == b.to_bits()));
}
