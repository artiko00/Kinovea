//! `HitTest` of the closed shapes: circle, cross mark and rectangle.

use super::common::{box_around, line_size, nearly_coincide, opacity, trackable_opacity};
use super::context::{Hit, HitContext};
use super::label::hit_mini_label;
use super::{hit_path, hit_point, GraphicsPath};
use crate::paint::circle::label_text as circle_text;
use crate::paint::crossmark::label_text as crossmark_text;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Circle, CrossMark, Rectangle};
use kinovea_metadata::measure_label::MeasureLabelType;

/// `DrawingCircle.HitTest` (DrawingCircle.cs:319-338): the measure label is handle 2, handle 1 is the
/// outline (`IsPointOnHandler`, `GetHitPath` with the ellipse box), body the filled disc.
pub fn hit_circle(d: &Circle, p: PointF, ctx: &HitContext) -> Hit {
    if opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    if d.measure_label_type != MeasureLabelType::None {
        let text = circle_text(d, ctx.metadata, ctx.timestamp);
        if hit_mini_label(&d.label, &d.header, &text, p, ctx) {
            return Hit::Handle(2);
        }
    }
    let mut outline = GraphicsPath::new();
    outline.add_ellipse(box_around(d.center, d.radius));
    if d.radius >= 0 && hit_path(p, &outline, line_size(&d.header), false, ctx.viewport) {
        return Hit::Handle(1);
    }
    if hit_path(p, &outline, 0, true, ctx.viewport) {
        return Hit::Body;
    }
    Hit::Miss
}

/// `DrawingCrossMark.HitTest` (DrawingCrossMark.cs:208-221): the measure label is handle 1, the point body.
pub fn hit_crossmark(d: &CrossMark, p: PointF, ctx: &HitContext) -> Hit {
    if trackable_opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    if d.measure_label_type != MeasureLabelType::None {
        let text = crossmark_text(d, ctx.metadata, ctx.timestamp);
        if hit_mini_label(&d.label, &d.header, &text, p, ctx) {
            return Hit::Handle(1);
        }
    }
    if hit_point(p, d.center, ctx.viewport) {
        return Hit::Body;
    }
    Hit::Miss
}

/// `DrawingRectangle.HitTest` (DrawingRectangle.cs:219-246): corners 1-4, edges 5-8 (edge i joins
/// corner i and i+1), then the inside as the body.
pub fn hit_rectangle(d: &Rectangle, p: PointF, ctx: &HitContext) -> Hit {
    if opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    let corners = d.quad.to_array();
    if let Some(i) = corners.iter().position(|c| hit_point(p, *c, ctx.viewport)) {
        return Hit::Handle(i as u32 + 1);
    }
    let width = line_size(&d.header);
    for i in 0..4 {
        let (a, b) = (corners[i], corners[(i + 1) % 4]);
        if nearly_coincide(a, b) {
            continue;
        }
        let mut edge = GraphicsPath::new();
        edge.add_line(a, b);
        if hit_path(p, &edge, width, false, ctx.viewport) {
            return Hit::Handle(i as u32 + 5);
        }
    }
    if d.quad.contains(p) {
        return Hit::Body;
    }
    Hit::Miss
}
