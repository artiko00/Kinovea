use super::circle::paint_circle;
use super::helpers::frame::scene::{round_pen, style, Scene, COLOR};
use crate::backend::recording::Call;
use crate::renderer::{Brush, Transform};
use crate::viewport::Point;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::Circle;
use kinovea_metadata::Fading;

/// The outline call for `rect` at `width`, drawn with the test pen.
fn outline(rect: RectangleF, width: f32) -> Call {
    Call::StrokeEllipse(rect, round_pen(width), Transform::identity())
}

fn circle(center: PointF, radius: i32, size: i32) -> Circle {
    let mut d = Circle::new(center, radius);
    d.header.style = style(size);
    d
}

#[test]
fn outline_is_the_bounding_square_of_the_circle() {
    // Centre (100, 100), radius 20 -> square (80, 80, 40, 40) at scale 1; width 3.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let d = circle(PointF::new(100.0, 100.0), 20, 3);
    assert_eq!(
        scene.record(0, |ctx, r| paint_circle(&d, &ctx, r)),
        vec![outline(RectangleF::new(80.0, 80.0, 40.0, 40.0), 3.0)]
    );
}

#[test]
fn viewport_scale_two_doubles_the_square_and_the_width() {
    // Centre (100, 100) * 2 + (5, 7) = (205, 207); square location (80, 80) * 2 + (5, 7) = (165, 167).
    let scene = Scene::new(Point::new(5, 7), 2.0);
    let d = circle(PointF::new(100.0, 100.0), 20, 3);
    assert_eq!(
        scene.record(0, |ctx, r| paint_circle(&d, &ctx, r)),
        vec![outline(RectangleF::new(165.0, 167.0, 80.0, 80.0), 6.0)]
    );
}

#[test]
fn centre_mark_is_a_precise_cross_drawn_before_the_outline() {
    // Centre (205, 207) at scale 2 with offset (5, 7); arms of 7.5 px, width 1 and solid.
    let scene = Scene::new(Point::new(5, 7), 2.0);
    let mut d = circle(PointF::new(100.0, 100.0), 20, 3);
    d.show_center = true;
    let calls = scene.record(0, |ctx, r| paint_circle(&d, &ctx, r));
    assert_eq!(
        calls[0],
        Call::StrokeLine(
            PointF::new(197.5, 207.0),
            PointF::new(212.5, 207.0),
            round_pen(1.0),
            Transform::identity()
        )
    );
    assert_eq!(
        calls[1],
        Call::StrokeLine(
            PointF::new(205.0, 199.5),
            PointF::new(205.0, 214.5),
            round_pen(1.0),
            Transform::identity()
        )
    );
    assert!(matches!(calls[2], Call::StrokeEllipse(..)));
}

#[test]
fn filled_circle_adds_an_ellipse_in_the_brush_colour() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = circle(PointF::new(100.0, 100.0), 20, 1);
    d.filled = true;
    let calls = scene.record(0, |ctx, r| paint_circle(&d, &ctx, r));
    assert_eq!(
        calls.last(),
        Some(&Call::FillEllipse(
            RectangleF::new(80.0, 80.0, 40.0, 40.0),
            Brush::solid(COLOR),
            Transform::identity(),
        ))
    );
}

#[test]
fn opacity_zero_paints_nothing() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = circle(PointF::new(100.0, 100.0), 20, 1);
    d.header.fading = Fading {
        use_default: false,
        ..Fading::default()
    };
    assert!(scene
        .record(10, |ctx, r| paint_circle(&d, &ctx, r))
        .is_empty());
}
