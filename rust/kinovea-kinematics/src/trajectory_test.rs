use crate::context::PixelContext;
use crate::trajectory::FilteredTrajectory;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::{PointF, SizeF};

fn ctx(filtering: bool) -> PixelContext {
    PixelContext {
        filtering,
        ..PixelContext::new(SizeF::new(200.0, 100.0), 50.0)
    }
}

fn circle_samples(n: usize) -> Vec<TimedPoint> {
    (0..n)
        .map(|i| {
            let t = i as f64 * 0.1;
            TimedPoint::new((100.0 + 30.0 * t.cos()) as f32, (50.0 - 30.0 * t.sin()) as f32, i as i64)
        })
        .collect()
}

#[test]
fn samples_are_mapped_to_world_through_the_context() {
    let t = FilteredTrajectory::new(&[TimedPoint::new(110.0, 40.0, 7)], &ctx(true));
    assert_eq!(t.length(), 1);
    assert_eq!(t.times, vec![7]);
    assert_eq!(t.raw_coordinates(0), PointF::new(10.0, 10.0));
    assert_eq!(t.raw_coordinates(5), PointF::new(10.0, 10.0));
    assert_eq!(t.coordinates(3), PointF::new(10.0, 10.0));
    assert!(!t.can_filter);
}

#[test]
fn can_filter_needs_more_than_ten_samples_and_the_preference() {
    assert!(!FilteredTrajectory::new(&circle_samples(10), &ctx(true)).can_filter);
    let off = FilteredTrajectory::new(&circle_samples(40), &ctx(false));
    assert!(!off.can_filter);
    assert_eq!(off.x_cutoff_index, None);
    assert_eq!(off.xs(), &off.raw_xs[..]);
    assert_eq!(off.ys(), &off.raw_ys[..]);
    let on = FilteredTrajectory::new(&circle_samples(40), &ctx(true));
    assert!(on.can_filter);
    assert!(on.x_cutoff_index.is_some() && on.y_cutoff_index.is_some());
    assert_eq!(on.filter_result_xs.len(), 100);
    assert_ne!(on.xs(), &on.raw_xs[..]);
    assert_eq!(on.coordinates(0).x, on.xs()[0] as f32);
}

#[test]
fn best_fit_circle_recovers_a_synthetic_circle() {
    let t = FilteredTrajectory::new(&circle_samples(40), &ctx(false));
    let c = t.best_fit_circle;
    assert!(c.center.x.abs() < 1e-3 && c.center.y.abs() < 1e-3, "{c:?}");
    assert!((c.radius - 30.0).abs() < 1e-3, "{c:?}");
}
