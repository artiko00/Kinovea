use crate::metadata::Metadata;
use kinovea_geometry::{PointF, Size};

fn metadata() -> Metadata {
    let mut m = Metadata::new(Size::new(640, 480));
    m.coordinate_system.origin = PointF::new(320.0, 240.0);
    m
}

#[test]
fn the_first_setup_names_the_coordinate_system_and_the_test_grid() {
    let mut m = metadata();
    m.post_setup_video();
    assert_eq!(m.coordinate_system.header.name, "Coordinate system 1");
    assert_eq!(m.test_grid.header.name, "Camera test grid 1");
    assert!(m.video_set_up);
}

#[test]
fn the_coordinate_system_is_registered_first_at_timestamp_zero_with_its_origin() {
    let mut m = metadata();
    m.post_setup_video();
    let first = m.trackability.trackers.iter().next().unwrap();
    assert_eq!(first.drawing_id, m.coordinate_system.header.id);
    assert_eq!(first.name, "Coordinate system 1");
    assert_eq!(first.points[0].1.reference_timestamp, 0);
    assert_eq!(first.points[0].1.reference_value, PointF::new(320.0, 240.0));
    assert_eq!(m.coordinate_system.header.reference_timestamp, 0);
}

#[test]
fn a_second_setup_keeps_the_names_and_a_single_registration() {
    let mut m = metadata();
    m.post_setup_video();
    m.post_setup_video();
    assert_eq!(m.coordinate_system.header.name, "Coordinate system 1");
    assert_eq!(m.trackability.trackers.iter().count(), 1);
}
