use crate::component::Kinematics;
use crate::time_series::{pad, TimeSeriesCollection};

#[test]
fn add_and_get_components() {
    let mut tsc = TimeSeriesCollection::new(3);
    assert_eq!(tsc.length(), 3);
    assert_eq!(tsc.times, vec![0, 0, 0]);
    tsc.set_times(vec![1, 2, 3]);
    tsc.add_component(Kinematics::X, vec![1.0, 2.0, 3.0]);
    tsc.initialize_components(&[Kinematics::LinearSpeed]);
    assert_eq!(tsc.get(Kinematics::X), Some(&[1.0, 2.0, 3.0][..]));
    assert_eq!(tsc[Kinematics::LinearSpeed], vec![0.0; 3]);
    tsc[Kinematics::LinearSpeed][1] = 9.0;
    assert_eq!(tsc[Kinematics::LinearSpeed][1], 9.0);
    assert!(tsc.get(Kinematics::Y).is_none());
    assert!(tsc.contains(Kinematics::X));
    assert_eq!(tsc.keys().collect::<Vec<_>>(), vec![Kinematics::X, Kinematics::LinearSpeed]);
}

#[test]
#[should_panic(expected = "component Y not present")]
fn indexing_a_missing_component_panics() {
    let tsc = TimeSeriesCollection::new(1);
    let _ = &tsc[Kinematics::Y];
}

#[test]
fn padding_short_and_long_series() {
    let mut short = vec![1.0, 2.0, 3.0];
    pad(&mut short, 2);
    assert!(short.iter().all(|v| v.is_nan()));
    let mut long: Vec<f64> = (0..10).map(f64::from).collect();
    pad(&mut long, 2);
    assert_eq!(long.iter().filter(|v| v.is_finite()).count(), 6);
    assert!(long[0].is_nan() && long[1].is_nan() && long[8].is_nan() && long[9].is_nan());
    assert_eq!(long[2], 2.0);
}
