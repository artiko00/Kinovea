use super::angle::paint_angle;
use super::helpers::frame::scene::Scene;
use crate::backend::recording::Call;
use crate::renderer::Brush;
use crate::viewport::Point;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::Angle;
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::Color;

#[test]
fn angle_painters_are_declared() {
    assert!(module_path!().contains("angle"));
}

pub(super) fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

/// Origin (100, 100), reference leg east (150, 100), measure leg north (100, 50).
pub(super) fn quarter_turn(ccw: bool) -> Angle {
    let mut d = Angle::new(
        PointF::new(100.0, 100.0),
        PointF::new(150.0, 100.0),
        PointF::new(100.0, 50.0),
    );
    d.counter_clockwise = ccw;
    d
}

pub(super) fn paint(scene: &Scene, d: &Angle, timestamp: i64) -> Vec<Call> {
    scene.record(timestamp, |ctx, r| paint_angle(d, &ctx, r))
}

pub(super) fn text_of(calls: &[Call]) -> (String, PointF) {
    calls
        .iter()
        .find_map(|c| match c {
            Call::DrawText(text, _, _, origin, _) => Some((text.clone(), *origin)),
            _ => None,
        })
        .expect("a value text")
}

#[test]
fn disk_and_arc_span_ninety_degrees_from_north_with_background_alpha_92() {
    // C#: DrawingAngle.cs:204, 209, 218.
    // Bounding box o.Box(40) = (60, 60, 80, 80); start 270, sweep 90. Disk alpha (int)(92).
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let calls = paint(&scene, &quarter_turn(true), 0);
    let Call::FillPie(rect, start, sweep, brush, _) = &calls[0] else {
        panic!("the disk is painted first");
    };
    assert_eq!(*rect, RectangleF::new(60.0, 60.0, 80.0, 80.0));
    assert!(close(*start, 270.0) && close(*sweep, 90.0));
    assert_eq!(*brush, Brush::solid(Color::argb(92, 0, 153, 153)));
    let Call::StrokeArc(arc, arc_start, arc_sweep, _, _) = &calls[1] else {
        panic!("the arc follows the disk");
    };
    assert_eq!(*arc, *rect);
    assert!(close(*arc_start, 270.0) && close(*arc_sweep, 90.0));
}

#[test]
fn reference_leg_is_dashed_and_measure_leg_is_solid() {
    // C#: DrawingAngle.cs:226-231.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let calls = paint(&scene, &quarter_turn(true), 0);
    let Call::StrokeLine(from, to, dashed, _) = &calls[2] else {
        panic!("the reference leg");
    };
    assert_eq!(
        (*from, *to),
        (PointF::new(100.0, 100.0), PointF::new(150.0, 100.0))
    );
    assert_eq!(dashed.dash, DashStyle::Dash);
    let Call::StrokeLine(_, measure, solid, _) = &calls[3] else {
        panic!("the measure leg");
    };
    assert_eq!(*measure, PointF::new(100.0, 50.0));
    assert_eq!(solid.dash, DashStyle::Solid);
}

#[test]
fn reference_leg_on_the_origin_is_put_back_upwards_by_fifty_pixels() {
    // C#: DrawingAngle.cs:547-555 (FixIfNull).
    // FixIfNull: a leg on the origin moves to (o.x, o.y - 50) for the painted copy.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = quarter_turn(true);
    d.a = PointF::new(100.0, 100.0);
    let calls = paint(&scene, &d, 0);
    let Call::StrokeLine(from, to, _, _) = &calls[2] else {
        panic!("the reference leg");
    };
    assert_eq!(*from, PointF::new(100.0, 100.0));
    assert_eq!(*to, PointF::new(100.0, 50.0));
}

#[test]
fn arrow_is_drawn_only_when_the_circle_is_wider_than_one_hundred_pixels() {
    // C#: DrawingAngle.cs:239-248.
    let arrows = |scale: f64| {
        let scene = Scene::new(Point::new(0, 0), scale);
        paint(&scene, &quarter_turn(true), 0)
            .iter()
            .filter(|c| matches!(c, Call::FillPolygon(..)))
            .count()
    };
    assert_eq!(arrows(1.0), 0);
    assert_eq!(arrows(2.0), 1);
}

#[test]
fn viewport_scale_two_doubles_the_disk_and_keeps_the_angle() {
    // C#: DrawingAngle.cs:194, 209.
    // Box (60, 60, 80, 80) * 2 = (120, 120, 160, 160); origin (200, 200).
    let scene = Scene::new(Point::new(0, 0), 2.0);
    let calls = paint(&scene, &quarter_turn(true), 0);
    let Call::FillPie(rect, start, sweep, _, _) = &calls[0] else {
        panic!("the disk");
    };
    assert_eq!(*rect, RectangleF::new(120.0, 120.0, 160.0, 160.0));
    assert!(close(*start, 270.0) && close(*sweep, 90.0));
}

#[test]
fn fully_faded_angle_paints_nothing() {
    // C#: DrawingAngle.cs:185-187.
    // Default fading: one opaque frame then 20 fading frames; 1000 frames later it is invisible.
    let mut scene = Scene::new(Point::new(0, 0), 1.0);
    scene.fading.average_timestamps_per_frame = 1.0;
    assert!(paint(&scene, &quarter_turn(true), 1000).is_empty());
}
