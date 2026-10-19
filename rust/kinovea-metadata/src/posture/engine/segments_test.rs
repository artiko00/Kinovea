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

/// Handle 2 is the point handle on the end of segment 0 (point 1). Its impacts are those of
/// handle 1, the handle at index 1, because the original passes the point index as a handle index.
#[test]
fn segment_end_impacts_come_from_the_handle_at_the_point_index() {
    let mut p = GenericPosture {
        point_list: vec![PointF::new(0.0, 0.0), PointF::new(10.0, 0.0), PointF::new(30.0, 40.0), PointF::new(50.0, 50.0)],
        segments: vec![Segment { name: String::new(), start: 0, end: 1, stroke: Stroke::default(), option_group: String::new() }],
        handles: vec![
            Handle { name: String::new(), kind: HandleType::Segment, color: TRANSPARENT, reference: 0, trackable: false, option_group: String::new(), grab_point: PointF::EMPTY, constraint: None, impacts: Vec::new() },
            Handle { name: String::new(), kind: HandleType::Point, color: TRANSPARENT, reference: 3, trackable: false, option_group: String::new(), grab_point: PointF::EMPTY, constraint: None, impacts: vec![Impact::HorizontalAlign { point: 2 }] },
            Handle { name: String::new(), kind: HandleType::Point, color: TRANSPARENT, reference: 1, trackable: false, option_group: String::new(), grab_point: PointF::EMPTY, constraint: None, impacts: Vec::new() },
        ],
        ..GenericPosture::default()
    };
    move_handle(&mut p, Space(None), 0, PointF::new(5.0, 0.0), false);
    assert_eq!(p.point_list[1], PointF::new(15.0, 0.0));
    assert_eq!(p.point_list[2], PointF::new(50.0, 40.0), "aligned on handle 1, the handle of point 3");
}

/// The point handle on point 5 is past the handles: the original throws, `MoveHandle` catches it and
/// the rest of the move is abandoned, so the handle on point 1 is never processed.
#[test]
fn out_of_range_point_index_aborts_the_rest_of_the_segment_move() {
    let handle = |kind, reference, impacts| Handle { name: String::new(), kind, color: TRANSPARENT, reference, trackable: false, option_group: String::new(), grab_point: PointF::EMPTY, constraint: None, impacts };
    let mut p = GenericPosture {
        point_list: vec![PointF::new(0.0, 0.0), PointF::new(10.0, 0.0), PointF::new(20.0, 20.0), PointF::new(30.0, 30.0), PointF::new(40.0, 40.0), PointF::new(50.0, 50.0)],
        segments: vec![Segment { name: String::new(), start: 1, end: 5, stroke: Stroke::default(), option_group: String::new() }],
        handles: vec![
            handle(HandleType::Segment, 0, Vec::new()),
            handle(HandleType::Point, 5, Vec::new()),
            handle(HandleType::Point, 1, vec![Impact::HorizontalAlign { point: 2 }]),
        ],
        ..GenericPosture::default()
    };
    move_handle(&mut p, Space(None), 0, PointF::new(5.0, 0.0), false);
    assert_eq!((p.point_list[1], p.point_list[5]), (PointF::new(15.0, 0.0), PointF::new(55.0, 50.0)), "the segment still moves");
    assert_eq!(p.point_list[2], PointF::new(20.0, 20.0), "no impact after the abort");
}
