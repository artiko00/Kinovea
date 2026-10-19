//! `HitTest` of the stroked kinds: straight line, pencil and polyline.

use super::common::{
    box_around, line_size, nearly_coincide, opacity, toggle_on, trackable_opacity,
};
use super::context::{Hit, HitContext};
use super::label::hit_mini_label;
use super::{hit_path, hit_point, GraphicsPath};
use crate::paint::helpers::frame::attach_label;
use crate::paint::line::label_text;
use kinovea_geometry::helpers::middle_point;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Line, Pencil, Polyline};
use kinovea_metadata::measure_label::MeasureLabelType;
use kinovea_metadata::Header;

/// `DrawingLine.HitTest` (DrawingLine.cs:278-295): the measure label, anchored to the middle of the
/// segment, is handle 3. Handles 1 and 2 are the ends; `IsPointInObject` (DrawingLine.cs:591-611) is
/// the stroke of the segment.
pub fn hit_line(d: &Line, p: PointF, ctx: &HitContext) -> Hit {
    if trackable_opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    if d.measure_label_type != MeasureLabelType::None {
        let label = attach_label(&d.label, middle_point(d.a, d.b));
        let text = label_text(d, ctx.metadata);
        if hit_mini_label(&label, &d.header, &text, p, ctx) {
            return Hit::Handle(3);
        }
    }
    if hit_point(p, d.a, ctx.viewport) {
        return Hit::Handle(1);
    }
    if hit_point(p, d.b, ctx.viewport) {
        return Hit::Handle(2);
    }
    if segment_area(d.a, d.b, &d.header, p, ctx) {
        return Hit::Body;
    }
    Hit::Miss
}

/// `IsPointInObject` of the segments (DrawingLine.cs:591, DrawingTimeSegment.cs:379): the stroke of
/// `a`-`b`, or of a short stub when both ends coincide.
pub(super) fn segment_area(
    a: PointF,
    b: PointF,
    header: &Header,
    p: PointF,
    ctx: &HitContext,
) -> bool {
    let mut path = GraphicsPath::new();
    if nearly_coincide(a, b) {
        path.add_line(a, PointF::new(a.x + 2.0, a.y + 2.0));
    } else {
        path.add_line(a, b);
    }
    hit_path(p, &path, line_size(header), false, ctx.viewport)
}

/// `DrawingPencil.HitTest` (DrawingPencil.cs:178-186): the curve through the points, widened.
/// The original throws when there are fewer than two points; this port misses.
pub fn hit_pencil(d: &Pencil, p: PointF, ctx: &HitContext) -> Hit {
    if opacity(&d.header, ctx) <= 0.0 || d.points.len() < 2 {
        return Hit::Miss;
    }
    let mut path = GraphicsPath::new();
    path.add_curve(&d.points, 0.5);
    if hit_path(p, &path, line_size(&d.header), false, ctx.viewport) {
        return Hit::Body;
    }
    Hit::Miss
}

/// `DrawingPolyline.HitTest` (DrawingPolyline.cs:222-239): the last point in order whose box holds
/// `p` gives handle `key + 1`; otherwise the widened polyline (or curve, with the Curved toggle).
pub fn hit_polyline(d: &Polyline, p: PointF, ctx: &HitContext) -> Hit {
    if trackable_opacity(&d.header, ctx) <= 0.0 {
        return Hit::Miss;
    }
    let mut handle = None;
    for (key, point) in &d.points.0 {
        if hit_point(p, *point, ctx.viewport) {
            handle = key.parse::<u32>().ok().map(|k| k + 1);
        }
    }
    if let Some(n) = handle {
        return Hit::Handle(n);
    }
    if polyline_area(&d.points.0, &d.header, p, ctx) {
        return Hit::Body;
    }
    Hit::Miss
}

/// `DrawingPolyline.IsPointInObject` (DrawingPolyline.cs:476-505): one point is a 10 px square
/// outline; more points drop consecutive duplicates and become a line or a cardinal curve.
pub(super) fn polyline_area(
    keyed: &[(String, PointF)],
    header: &Header,
    p: PointF,
    ctx: &HitContext,
) -> bool {
    let points: Vec<PointF> = keyed.iter().map(|(_, pt)| *pt).collect();
    let mut path = GraphicsPath::new();
    match points.as_slice() {
        [] => return false,
        [only] => path.add_rectangle(box_around(*only, 5)),
        many => {
            let mut dedup: Vec<PointF> = Vec::new();
            for pt in many {
                if dedup.last() != Some(pt) {
                    dedup.push(*pt);
                }
            }
            if toggle_on(header, "curved") {
                path.add_curve(&dedup, 0.5);
            } else {
                path.add_lines(&dedup);
            }
        }
    }
    hit_path(p, &path, line_size(header), false, ctx.viewport)
}
