use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::{round_pen, style, Scene};
use crate::paint::line::paint_line;
use crate::renderer::Transform;
use crate::viewport::Point;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Line;
use kinovea_metadata::style::LineEnding;
use kinovea_metadata::StyleValue;

#[test]
fn end_arrow_shortens_the_line_so_the_tip_lands_on_the_end() {
    // Length 256, width 1: offset 12, so the stroke stops at 244 and the arrow tip is back at 256.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = Line::new(PointF::new(0.0, 0.0), PointF::new(256.0, 0.0));
    d.header.style = style(1);
    d.header
        .style
        .insert("arrows", StyleValue::Arrows(LineEnding::EndArrow));
    let calls = scene.record(0, |ctx, r| paint_line(&d, &ctx, r));
    assert_eq!(
        calls[0],
        Call::StrokeLine(
            PointF::new(0.0, 0.0),
            PointF::new(244.0, 0.0),
            round_pen(1.0),
            Transform::identity()
        )
    );
    let Call::FillPolygon(points, _, _) = &calls[1] else {
        panic!("expected the arrow head");
    };
    assert!((points[0].x - 256.0).abs() < 1e-3 && points[0].y.abs() < 1e-3);
}
