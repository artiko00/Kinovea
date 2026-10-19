use super::helpers::frame::attach_label;
use kinovea_geometry::PointF;
use kinovea_metadata::measure_label::MeasureLabel;

#[test]
fn attaching_a_label_keeps_its_offset_to_the_attach_point() {
    let label = MeasureLabel {
        position: PointF::new(10.0, 20.0),
        timestamp: 0,
        attach: PointF::new(30.0, 60.0),
    };
    let moved = attach_label(&label, PointF::new(35.0, 50.0));
    assert_eq!(moved.position, PointF::new(15.0, 10.0));
    assert_eq!(moved.attach, PointF::new(35.0, 50.0));
}
