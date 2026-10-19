use super::crossmark::paint_crossmark;
use super::helpers::frame::scene::{style, Scene, COLOR};
use crate::backend::recording::Call;
use crate::renderer::{Brush, LineCap, LineJoin, Pen, Transform};
use crate::viewport::Point;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::CrossMark;
use kinovea_metadata::{Color, Fading};

fn crossmark(center: PointF) -> CrossMark {
    let mut d = CrossMark::new(center);
    d.header.style = style(1);
    d
}

/// `GetPen(alpha)`: width 1 with round caps and joins, whatever the viewport scale.
fn marker() -> Pen {
    Pen {
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Pen::new(COLOR, 1.0)
    }
}

#[test]
fn cross_has_fixed_arms_and_a_disc_of_radius_four() {
    // Centre (100, 100): arms of 3 px, disc box (96, 96, 8, 8) with the background alpha 64.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let d = crossmark(PointF::new(100.0, 100.0));
    let disc = Color { a: 64, ..COLOR };
    assert_eq!(
        scene.record(0, |ctx, r| paint_crossmark(&d, &ctx, r)),
        vec![
            Call::StrokeLine(
                PointF::new(97.0, 100.0),
                PointF::new(103.0, 100.0),
                marker(),
                Transform::identity()
            ),
            Call::StrokeLine(
                PointF::new(100.0, 97.0),
                PointF::new(100.0, 103.0),
                marker(),
                Transform::identity()
            ),
            Call::FillEllipse(
                RectangleF::new(96.0, 96.0, 8.0, 8.0),
                Brush::solid(disc),
                Transform::identity()
            ),
        ]
    );
}

#[test]
fn cross_keeps_its_size_at_viewport_scale_two() {
    // Centre (10, 10) * 2 = (20, 20); arms stay 3 px and the marker pen stays 1 px.
    let scene = Scene::new(Point::new(0, 0), 2.0);
    let d = crossmark(PointF::new(10.0, 10.0));
    let calls = scene.record(0, |ctx, r| paint_crossmark(&d, &ctx, r));
    assert_eq!(
        calls[0],
        Call::StrokeLine(
            PointF::new(17.0, 20.0),
            PointF::new(23.0, 20.0),
            marker(),
            Transform::identity()
        )
    );
}

#[test]
fn dot_is_a_one_pixel_box_around_the_centre() {
    // Box of radius 0.5 around (100, 100): truncated to (99, 99, 1, 1).
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = crossmark(PointF::new(100.0, 100.0));
    d.show_as_dot = true;
    assert_eq!(
        scene.record(0, |ctx, r| paint_crossmark(&d, &ctx, r)),
        vec![Call::StrokeRectangle(
            RectangleF::new(99.0, 99.0, 1.0, 1.0),
            marker(),
            Transform::identity()
        )]
    );
}

#[test]
fn opacity_zero_paints_nothing() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = crossmark(PointF::new(100.0, 100.0));
    d.header.fading = Fading {
        use_default: false,
        ..Fading::default()
    };
    assert!(scene
        .record(10, |ctx, r| paint_crossmark(&d, &ctx, r))
        .is_empty());
}
