use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::{round_pen, style, Scene, COLOR};
use crate::paint::paths::paint_polyline;
use crate::renderer::{Brush, Transform};
use crate::viewport::Point;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::Polyline;
use kinovea_metadata::style::{LineEnding, LineShape, ToggleVariant};
use kinovea_metadata::StyleValue;

fn polyline(points: &[PointF], shape: LineShape, ending: LineEnding) -> Polyline {
    let keyed = points.iter().map(|p| (String::new(), *p)).collect();
    let mut d = Polyline::new(keyed);
    d.header.style = style(1);
    d.header
        .style
        .insert("line shape", StyleValue::LineShape(shape));
    d.header.style.insert("arrows", StyleValue::Arrows(ending));
    d
}

fn record(d: &Polyline, scene: &Scene, selected: bool) -> Vec<Call> {
    scene.record(0, |mut ctx, r| {
        ctx.selected = selected;
        paint_polyline(d, &ctx, r)
    })
}

#[test]
fn solid_polyline_is_a_straight_path_with_flat_caps() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let points = [
        PointF::new(0.0, 0.0),
        PointF::new(10.0, 0.0),
        PointF::new(10.0, 10.0),
    ];
    let d = polyline(&points, LineShape::Solid, LineEnding::None);
    let pen = round_pen(1.0);
    assert_eq!(
        record(&d, &scene, false),
        vec![Call::StrokeLines(
            points.to_vec(),
            pen,
            Transform::identity()
        )]
    );
}

#[test]
fn curved_toggle_switches_the_path_to_a_cardinal_curve() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let points = [
        PointF::new(0.0, 0.0),
        PointF::new(10.0, 0.0),
        PointF::new(10.0, 10.0),
    ];
    let mut d = polyline(&points, LineShape::Solid, LineEnding::None);
    d.header.style.insert(
        "curved",
        StyleValue::Toggle {
            value: true,
            variant: ToggleVariant::Curved,
        },
    );
    let calls = record(&d, &scene, false);
    assert!(matches!(calls[0], Call::StrokeCurve(_, tension, _, _) if tension == 0.5));
}

#[test]
fn squiggly_polyline_zigzags_each_segment() {
    // One zigzag per segment of length 20 at width 1: the curve starts at (0, 0) with tension 0.9.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let points = [PointF::new(0.0, 0.0), PointF::new(20.0, 0.0)];
    let d = polyline(&points, LineShape::Squiggle, LineEnding::None);
    let calls = record(&d, &scene, false);
    let Call::StrokeCurve(curve, tension, _, _) = &calls[0] else {
        panic!("expected a squiggle curve");
    };
    assert_eq!(*tension, 0.9);
    assert_eq!(curve[2], PointF::new(12.0, -3.0));
}

#[test]
fn start_arrow_points_its_tip_at_the_first_point() {
    // Tip (0, 0) with the arrow pointing away from (10, 0): the top sits at (-12, 0).
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let points = [PointF::new(0.0, 0.0), PointF::new(10.0, 0.0)];
    let d = polyline(&points, LineShape::Solid, LineEnding::StartArrow);
    let calls = record(&d, &scene, false);
    let Call::FillPolygon(head, brush, _) = &calls[1] else {
        panic!("expected the arrow head");
    };
    assert_eq!(head[0], PointF::new(-12.0, 0.0));
    assert_eq!(*brush, Brush::solid(COLOR));
}

#[test]
fn selected_polyline_draws_a_handle_on_every_point() {
    // Handles are Point.Box(3): squares of side 6 centred on each point.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let points = [PointF::new(10.0, 10.0), PointF::new(30.0, 10.0)];
    let d = polyline(&points, LineShape::Solid, LineEnding::None);
    let calls = record(&d, &scene, true);
    assert_eq!(
        calls[1..],
        [
            Call::FillEllipse(
                RectangleF::new(7.0, 7.0, 6.0, 6.0),
                Brush::solid(COLOR),
                Transform::identity()
            ),
            Call::FillEllipse(
                RectangleF::new(27.0, 7.0, 6.0, 6.0),
                Brush::solid(COLOR),
                Transform::identity()
            ),
        ]
    );
}
