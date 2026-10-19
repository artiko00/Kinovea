use super::helpers::frame::scene::{round_pen, style, Scene};
use super::paths::{paint_pencil, paint_time_polyline};
use crate::backend::recording::Call;
use crate::renderer::{LineCap, Pen, Transform};
use crate::viewport::Point;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Pencil, TimePolyline};
use kinovea_metadata::style::{LineEnding, LineShape};
use kinovea_metadata::{Fading, StyleValue};

/// The pencil pen: `round_pen` with round caps at both ends.
fn capped_pen(width: f32) -> Pen {
    Pen {
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        ..round_pen(width)
    }
}

#[test]
fn pencil_stroke_is_a_round_capped_curve_at_tension_half() {
    // Scale 2 with offset (5, 7): (1, 1) -> (7, 9) and (2, 3) -> (9, 13); width 2 * 2 = 4.
    let scene = Scene::new(Point::new(5, 7), 2.0);
    let mut d = Pencil::new(vec![PointF::new(1.0, 1.0), PointF::new(2.0, 3.0)]);
    d.header.style = style(2);
    assert_eq!(
        scene.record(0, |ctx, r| paint_pencil(&d, &ctx, r)),
        vec![Call::StrokeCurve(
            vec![PointF::new(7.0, 9.0), PointF::new(9.0, 13.0)],
            0.5,
            capped_pen(4.0),
            Transform::identity(),
        )]
    );
}

#[test]
fn flat_pencil_stroke_is_a_one_pixel_segment() {
    // All points on y = 0 have no height, so the stroke becomes (0, 0) to (1, 0).
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = Pencil::new(vec![PointF::new(0.0, 0.0), PointF::new(10.0, 0.0)]);
    d.header.style = style(1);
    assert_eq!(
        scene.record(0, |ctx, r| paint_pencil(&d, &ctx, r)),
        vec![Call::StrokeLine(
            PointF::new(0.0, 0.0),
            PointF::new(1.0, 0.0),
            capped_pen(1.0),
            Transform::identity(),
        )]
    );
}

#[test]
fn time_polyline_ignores_fading_and_paints_at_full_opacity() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let points = vec![
        (String::new(), PointF::new(0.0, 0.0)),
        (String::new(), PointF::new(10.0, 0.0)),
    ];
    let mut d = TimePolyline::new(points);
    d.header.style = style(1);
    d.header
        .style
        .insert("line shape", StyleValue::LineShape(LineShape::Solid));
    d.header
        .style
        .insert("arrows", StyleValue::Arrows(LineEnding::None));
    d.header.fading = Fading {
        use_default: false,
        ..Fading::default()
    };
    assert_eq!(
        scene
            .record(10, |ctx, r| paint_time_polyline(&d, &ctx, r))
            .len(),
        1
    );
}
