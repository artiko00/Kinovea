use crate::camera_test::ShiftCamera;
use crate::trackable_point::CameraCache;
use kinovea_geometry::PointF;
use kinovea_metadata::trackability::TrackablePoint;

fn point(timestamp: i64, x: f32) -> TrackablePoint {
    TrackablePoint {
        reference_timestamp: timestamp,
        reference_value: PointF::new(x, 0.0),
        track_id: None,
    }
}

#[test]
fn camera_transform_is_computed_once_per_time() {
    let camera = ShiftCamera::on();
    let p = point(10, 1.0);
    let mut cache = CameraCache::default();
    assert_eq!(cache.camera_track(&p, 14, &camera), PointF::new(5.0, 0.0));
    assert_eq!(cache.camera_track(&p, 14, &camera), PointF::new(5.0, 0.0));
    assert_eq!(camera.calls.get(), 1);
    cache.camera_track(&p, 20, &camera);
    assert_eq!(camera.calls.get(), 2);
}

#[test]
fn changing_the_reference_clears_the_cache() {
    let camera = ShiftCamera::on();
    let mut cache = CameraCache::default();
    cache.camera_track(&point(10, 1.0), 14, &camera);
    assert_eq!(
        cache.camera_track(&point(10, 7.0), 14, &camera),
        PointF::new(11.0, 0.0)
    );
    assert_eq!(camera.calls.get(), 2);
    cache.camera_track(&point(12, 7.0), 14, &camera);
    assert_eq!(camera.calls.get(), 3);
}

#[test]
fn without_camera_motion_the_reference_value_is_returned_and_nothing_runs() {
    let camera = ShiftCamera::off();
    let mut cache = CameraCache::default();
    assert_eq!(
        cache.camera_track(&point(10, 1.0), 14, &camera),
        PointF::new(1.0, 0.0)
    );
    assert_eq!(camera.calls.get(), 0);
}
