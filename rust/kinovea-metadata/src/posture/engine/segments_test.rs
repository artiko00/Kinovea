use crate::posture::engine::{grab_segment_middle, move_handle, Space};
use crate::posture::model::*;
use kinovea_geometry::PointF;

fn posture() -> GenericPosture {
    let seg = |start, end| Segment { name: String::new(), start, end, stroke: Stroke::default(), option_group: String::new() };
    let handle = |kind, reference, constraint| Handle { name: String::new(), kind, color: TRANSPARENT, reference, trackable: false, option_group: String::new(), grab_point: PointF::EMPTY, constraint, impacts: Vec::new() };
    GenericPosture {
        point_list: vec![PointF::new(0.0, 0.0), PointF::new(10.0, 0.0), PointF::new(0.0, 20.0), PointF::new(10.0, 20.0), PointF::new(50.0, 50.0)],
        segments: vec![seg(0, 1), seg(2, 3)],
        circles: vec![PostureCircle { name: String::new(), center: 4, radius: 1, stroke: Stroke::default(), option_group: String::new() }],
        handles: vec![
            handle(HandleType::Segment, 0, None),
            handle(HandleType::Segment, 1, Some(Constraint { kind: ConstraintKind::HorizontalSlide, option_group: String::new() })),
            handle(HandleType::Circle, 0, None),
            handle(HandleType::Point, 1, None),
        ],
        ..GenericPosture::default()
    }
}

#[test]
fn segment_handles_translate_both_ends() {
    let mut p = posture();
    grab_segment_middle(&mut p, 0);
    assert_eq!(p.handles[0].grab_point, PointF::new(5.0, 0.0));
    move_handle(&mut p, Space(None), 0, PointF::new(15.0, 3.0), false);
    assert_eq!((p.point_list[0], p.point_list[1]), (PointF::new(10.0, 3.0), PointF::new(20.0, 3.0)));
    assert_eq!(p.handles[0].grab_point, PointF::new(15.0, 3.0));
    move_handle(&mut p, Space(None), 1, PointF::new(4.0, 9.0), false);
    assert_eq!((p.point_list[2], p.point_list[3]), (PointF::new(4.0, 20.0), PointF::new(14.0, 20.0)), "horizontal slide ignores dy");
}

#[test]
fn circle_handle_sets_the_truncated_radius() {
    let mut p = posture();
    move_handle(&mut p, Space(None), 2, PointF::new(53.0, 54.9), false);
    assert_eq!(p.circles[0].radius, 5, "(int) truncation of 5.74");
}
