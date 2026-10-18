//! Parity of clipping and planar helpers with `tests/fixtures/geometry/{clipping,helpers}`.

mod parity_support;
use parity_support::*;

use kinovea_geometry::clipping::clip_line;
use kinovea_geometry::helpers;
use kinovea_geometry::shapes::PointLinePosition;
use kinovea_geometry::{PointF, RectangleF};

#[test]
fn clipping_matches_within_1e_3() {
    let rows = rows("clipping/lines.csv");
    assert!(rows.len() >= 240, "expected 60 lines × 4 modes");
    for r in &rows {
        let (a, b) = (pt(r, 0), pt(r, 2));
        let (t0, t1) = match r[4].as_str() {
            "segment" => (0.0, 1.0),
            "line" => (f64::MIN, f64::MAX),
            "ray_forward" => (0.0, f64::MAX),
            "ray_backward" => (f64::MIN, 1.0),
            other => panic!("unknown mode {other}"),
        };
        let got = clip_line(RectangleF::new(500.0, 500.0, 1000.0, 1000.0), a, b, t0, t1);
        assert_eq!(got.visible, r[5] == "1", "{a:?}-{b:?} {}", r[4]);
        if got.visible {
            assert_pt(
                got.a,
                pt(r, 6),
                1e-3,
                &format!("entry {a:?}-{b:?} {}", r[4]),
            );
            assert_pt(got.b, pt(r, 8), 1e-3, &format!("exit {a:?}-{b:?} {}", r[4]));
        }
    }
}

#[test]
fn helper_angles_match_within_1e_4() {
    let rows = rows("helpers/angles.csv");
    assert_eq!(rows.len(), 30);
    for r in &rows {
        let (a, b, c, d) = (pt(r, 0), pt(r, 2), pt(r, 4), pt(r, 6));
        assert_f(
            f64::from(helpers::angle(a, b, c, d)),
            f(&r[8]),
            1e-4,
            "angle4",
        );
        assert_f(
            f64::from(helpers::angle_at(a, b, c)),
            f(&r[9]),
            1e-4,
            "angle3",
        );
    }
}

#[test]
fn helper_points_match_within_1e_4() {
    let rows = rows("helpers/points.csv");
    assert_eq!(rows.len(), 30 * 12);
    for r in &rows {
        let (a, b, c, d) = (pt(r, 1), pt(r, 3), pt(r, 5), pt(r, 7));
        let (p1, p2) = (f(&r[9]) as f32, f(&r[10]) as f32);
        let got = match r[0].as_str() {
            "distance" => PointF::new(helpers::distance(a, b), 0.0),
            "middle" => helpers::middle_point(a, b),
            "at_distance" => helpers::point_at_distance(a, b, p1),
            "rotate" => helpers::rotate(a, b, p1),
            "closest_step" => helpers::point_at_closest_rotation_step(a, b, c, p1 as i32),
            "closest_step_cardinal" => {
                helpers::point_at_closest_rotation_step_cardinal(a, c, p1 as i32)
            }
            "at_angle" => helpers::point_at_angle(a, b, c, p1),
            "at_angle_distance" => helpers::point_at_angle_and_distance(a, b, p1, p2),
            "on_parallel" => helpers::point_on_parallel(a, b, c, d),
            "mix" => helpers::mix_points(a, b, p1),
            "closest_point" => helpers::closest_point(a, b, c, position(p1 as i32), p2 as i32),
            "intersection" => {
                let rect = RectangleF::new(a.x, a.y, b.x, b.y);
                helpers::intersection_rectangle_center(rect, c)
            }
            other => panic!("unknown op {other}"),
        };
        assert_pt(
            got,
            pt(r, 11),
            1e-4 * scale(&r[11..13]),
            &format!("{} {a:?} {b:?} {c:?} {d:?} {p1} {p2}", r[0]),
        );
    }
}

fn position(index: i32) -> PointLinePosition {
    [
        PointLinePosition::BeforeSegment,
        PointLinePosition::BeforeAndOnSegment,
        PointLinePosition::OnSegment,
        PointLinePosition::AfterAndOnSegment,
        PointLinePosition::AfterSegment,
        PointLinePosition::Anywhere,
    ][index as usize]
}
