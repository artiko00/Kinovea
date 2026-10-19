use crate::posture::catalog::custom_postures;
use crate::posture::engine::{move_handle, Space};
use crate::posture::model::*;
use kinovea_geometry::PointF;

fn with_handle(points: &[(f32, f32)], impacts: Vec<Impact>) -> GenericPosture {
    let point_list: Vec<PointF> = points.iter().map(|(x, y)| PointF::new(*x, *y)).collect();
    let handle = Handle { name: String::new(), kind: HandleType::Point, color: TRANSPARENT, reference: 0, trackable: false, option_group: String::new(), grab_point: PointF::EMPTY, constraint: None, impacts };
    GenericPosture { point_list, handles: vec![handle], ..GenericPosture::default() }
}

#[test]
fn alignments_follow_the_moved_point() {
    let mut p = with_handle(&[(0.0, 0.0), (10.0, 10.0), (20.0, 20.0)], vec![Impact::VerticalAlign { point: 1 }, Impact::HorizontalAlign { point: 2 }]);
    move_handle(&mut p, Space(None), 0, PointF::new(5.0, 7.0), false);
    assert_eq!(p.point_list[1], PointF::new(10.0, 7.0));
    assert_eq!(p.point_list[2], PointF::new(5.0, 20.0));
}

#[test]
fn line_align_projects_and_pivot_rotates() {
    let mut p = with_handle(&[(0.0, 0.0), (100.0, 0.0), (50.0, 30.0)], vec![Impact::LineAlign { start: 0, end: 1, point_to_align: 2 }]);
    move_handle(&mut p, Space(None), 0, PointF::new(0.0, 0.0), false);
    assert_eq!(p.point_list[2], PointF::new(50.0, 0.0));
    let mut p = with_handle(&[(100.0, 0.0), (0.0, 0.0), (0.0, 50.0)], vec![Impact::Pivot { pivot: 1, impacted: vec![2] }]);
    move_handle(&mut p, Space(None), 0, PointF::new(0.0, 100.0), false);
    let r = p.point_list[2];
    assert!((r.x + 50.0).abs() < 1e-3 && r.y.abs() < 1e-3, "{r:?}");
}

#[test]
fn segment_center_and_degenerate_keep_angle() {
    let mut p = with_handle(&[(0.0, 0.0), (40.0, 0.0), (0.0, 99.0)], vec![Impact::SegmentCenter { point_to_move: 2, point1: 0, point2: 1 }]);
    move_handle(&mut p, Space(None), 0, PointF::new(0.0, 0.0), false);
    assert_eq!(p.point_list[2], PointF::new(20.0, 0.0));
    let mut p = with_handle(&[(0.0, 0.0), (0.0, 0.0), (5.0, 5.0)], vec![Impact::KeepAngle { origin: 1, leg1: 0, leg2: 2, old_angle: 0.0, old_distance: 0.0 }]);
    move_handle(&mut p, Space(None), 0, PointF::new(0.0, 0.0), false);
    assert_eq!(p.point_list[2], PointF::new(5.0, 5.0), "origin == leg1 leaves the point alone");
    assert!(custom_postures().iter().any(|c| c.handles.iter().any(|h| !h.impacts.is_empty())));
}
