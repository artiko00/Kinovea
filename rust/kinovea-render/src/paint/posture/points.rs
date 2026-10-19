//! Port of the point parts of `DrawingGenericPosture.Draw`: `DrawPoints` (handles on points),
//! `DrawComputedPoints` and `DrawSimpleText` for the symbols.

use super::frame::{colored, edge_pen, PostureFrame};
use crate::paint::helpers::frame::{point_box_f, to_viewport};
use crate::renderer::{Brush, Renderer};
use kinovea_geometry::PointF;
use kinovea_metadata::posture::HandleType;
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::Color;

/// Radius of the points and computed points.
const POINT_RADIUS: i32 = 3;
const COMPUTED_WIDTH: f32 = 2.0;

/// `DrawPoints`: a point is drawn only where a handle sits on it.
pub fn paint_points(f: &PostureFrame, r: &mut dyn Renderer) {
    for handle in &f.posture.handles {
        if !f.is_active(&handle.option_group) || !matches!(handle.kind, HandleType::Point) {
            continue;
        }
        let Ok(index) = usize::try_from(handle.reference) else {
            continue;
        };
        if index >= f.points.len()
            || index >= f.posture.points.len()
            || is_arrow_end(f, handle.reference)
        {
            continue;
        }
        let base = f.edge_base();
        let color = colored(
            f.posture.points[index].color,
            f.alpha,
            colored(handle.color, f.alpha, base),
        );
        r.fill_ellipse(
            point_box_f(f.points[index], POINT_RADIUS),
            &Brush::solid(color),
        );
    }
}

/// `DrawComputedPoints`: symbols are text, the other computed points are small rings.
pub fn paint_computed_points(f: &PostureFrame, r: &mut dyn Renderer) {
    for computed in &f.posture.computed_points {
        if !f.is_active(&computed.option_group) {
            continue;
        }
        let location = to_viewport(f.ctx, computed.location(&f.posture.point_list));
        let color = colored(computed.color, f.alpha, f.edge_base());
        if computed.symbol.is_empty() {
            let pen = edge_pen(color, COMPUTED_WIDTH, DashStyle::Solid);
            r.stroke_ellipse(point_box_f(location, POINT_RADIUS), &pen);
        } else {
            draw_simple_text(f, r, location, &computed.symbol, color);
        }
    }
}

/// `DrawSimpleText`: a label centred on `location`, without background.
fn draw_simple_text(
    f: &PostureFrame,
    r: &mut dyn Renderer,
    location: PointF,
    label: &str,
    color: Color,
) {
    let font = f.text_font();
    let size = r.measure_text(label, &font);
    let origin = PointF::new(
        location.x - size.width / 2.0,
        location.y - size.height / 2.0,
    );
    r.draw_text(label, &font, &Brush::solid(color), origin);
}

/// An arrow at the end of a segment is the handle itself.
fn is_arrow_end(f: &PostureFrame, reference: i32) -> bool {
    f.posture.segments.iter().any(|s| {
        (s.start == reference && s.stroke.arrow_begin) || (s.end == reference && s.stroke.arrow_end)
    })
}
