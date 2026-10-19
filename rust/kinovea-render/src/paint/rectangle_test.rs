use super::helpers::frame::scene::{style, Scene, COLOR};
use super::rectangle::paint_rectangle;
use crate::backend::recording::Call;
use crate::renderer::{Brush, LineCap, LineJoin, Pen, Transform};
use crate::viewport::Point;
use kinovea_geometry::{PointF, Quadrilateral};
use kinovea_metadata::drawing::Rectangle;
use kinovea_metadata::style::{DashStyle, PenShape};
use kinovea_metadata::{Fading, StyleValue};

fn rectangle(filled: bool, pen_shape: PenShape) -> Rectangle {
    let quad = Quadrilateral::new(
        PointF::new(10.0, 10.0),
        PointF::new(50.0, 10.0),
        PointF::new(50.0, 30.0),
        PointF::new(10.0, 30.0),
    );
    let mut d = Rectangle::new(quad);
    d.filled = filled;
    d.header.style = style(1);
    d.header
        .style
        .insert("pen shape", StyleValue::PenShape(pen_shape));
    d
}

fn square_cap_pen(width: f32) -> Pen {
    Pen {
        end_cap: LineCap::Square,
        line_join: LineJoin::Round,
        ..Pen::new(COLOR, width)
    }
}

#[test]
fn outline_has_four_edges_with_square_caps_at_scale_two() {
    // Corners (10, 10), (50, 10), (50, 30), (10, 30) at scale 2 -> (20, 20), (100, 20), (100, 60), (20, 60).
    // Width 1 * 2 = 2.
    let scene = Scene::new(Point::new(0, 0), 2.0);
    let d = rectangle(false, PenShape::Solid);
    let pen = square_cap_pen(2.0);
    let edge = |a: (f32, f32), b: (f32, f32)| {
        Call::StrokeLine(
            PointF::new(a.0, a.1),
            PointF::new(b.0, b.1),
            pen.clone(),
            Transform::identity(),
        )
    };
    assert_eq!(
        scene.record(0, |ctx, r| paint_rectangle(&d, &ctx, r)),
        vec![
            edge((20.0, 20.0), (100.0, 20.0)),
            edge((100.0, 20.0), (100.0, 60.0)),
            edge((100.0, 60.0), (20.0, 60.0)),
            edge((20.0, 60.0), (20.0, 20.0)),
        ]
    );
}

#[test]
fn filled_rectangle_adds_the_quadrilateral_in_the_brush_colour() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let d = rectangle(true, PenShape::Solid);
    let calls = scene.record(0, |ctx, r| paint_rectangle(&d, &ctx, r));
    assert_eq!(
        calls.last(),
        Some(&Call::FillPolygon(
            vec![
                PointF::new(10.0, 10.0),
                PointF::new(50.0, 10.0),
                PointF::new(50.0, 30.0),
                PointF::new(10.0, 30.0),
            ],
            Brush::solid(COLOR),
            Transform::identity(),
        ))
    );
}

#[test]
fn dash_pen_shape_dashes_the_edges() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let d = rectangle(false, PenShape::Dash);
    let calls = scene.record(0, |ctx, r| paint_rectangle(&d, &ctx, r));
    let Call::StrokeLine(_, _, pen, _) = &calls[0] else {
        panic!("expected an edge");
    };
    assert_eq!(pen.dash, DashStyle::Dash);
}

#[test]
fn opacity_zero_paints_nothing() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = rectangle(true, PenShape::Solid);
    d.header.fading = Fading {
        use_default: false,
        ..Fading::default()
    };
    assert!(scene
        .record(10, |ctx, r| paint_rectangle(&d, &ctx, r))
        .is_empty());
}
