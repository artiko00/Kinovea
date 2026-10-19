//! Port of the segment, circle and polyline parts of `DrawingGenericPosture.Draw` (`DrawSegments`,
//! `DrawCircles`, `DrawPolylines`). Segments use round caps; the others come after them, with flat caps.

use super::frame::{colored, edge_pen, PostureFrame};
use crate::paint::helpers::arrow::{draw_arrow, update_start_end};
use crate::paint::helpers::frame::point_box_f;
use crate::renderer::{LineCap, LineJoin, Pen, Renderer};
use kinovea_geometry::PointF;
use kinovea_metadata::posture::{SegmentLineStyle, Stroke};
use kinovea_metadata::style::DashStyle;

/// `DrawCurve` tension of the polylines.
const POLYLINE_TENSION: f32 = 0.5;

/// `DrawSegments`.
pub fn paint_segments(f: &PostureFrame, r: &mut dyn Renderer) {
    for segment in &f.posture.segments {
        if !f.is_active(&segment.option_group) {
            continue;
        }
        let stroke = &segment.stroke;
        let pen = Pen {
            start_cap: LineCap::Round,
            end_cap: LineCap::Round,
            line_join: LineJoin::Round,
            ..pen_of(f, stroke)
        };
        let mut start = f.point(segment.start);
        let mut end = f.point(segment.end);
        let can_arrow = update_start_end(
            pen.width,
            &mut start,
            &mut end,
            stroke.arrow_begin,
            stroke.arrow_end,
        );
        r.stroke_line(start, end, &pen);
        if can_arrow {
            if stroke.arrow_begin {
                draw_arrow(r, &pen, truncated(start), truncated(end));
            }
            if stroke.arrow_end {
                draw_arrow(r, &pen, truncated(end), truncated(start));
            }
        }
    }
}

/// `DrawCircles`.
pub fn paint_circles(f: &PostureFrame, r: &mut dyn Renderer) {
    for circle in &f.posture.circles {
        if !f.is_active(&circle.option_group) {
            continue;
        }
        let pen = edge_pen_of(f, &circle.stroke);
        let center = f.point(circle.center);
        let radius = f.ctx.viewport.transform_distance(circle.radius);
        r.stroke_ellipse(point_box_f(center, radius), &pen);
    }
}

/// `DrawPolylines`.
pub fn paint_polylines(f: &PostureFrame, r: &mut dyn Renderer) {
    for polyline in &f.posture.polylines {
        if !f.is_active(&polyline.option_group) {
            continue;
        }
        let pen = edge_pen_of(f, &polyline.stroke);
        let path: Vec<PointF> = polyline.points.iter().map(|&i| f.point(i)).collect();
        r.stroke_curve(&path, POLYLINE_TENSION, &pen);
    }
}

/// Pen of a stroke with its own width and dash, in the stroke colour or the base colour.
fn pen_of(f: &PostureFrame, stroke: &Stroke) -> Pen {
    let color = colored(stroke.color, f.alpha, f.edge_base());
    Pen {
        dash: dash_of(stroke.style),
        ..Pen::new(color, stroke.width as f32)
    }
}

fn edge_pen_of(f: &PostureFrame, stroke: &Stroke) -> Pen {
    let color = colored(stroke.color, f.alpha, f.edge_base());
    edge_pen(color, stroke.width as f32, dash_of(stroke.style))
}

fn dash_of(style: SegmentLineStyle) -> DashStyle {
    match style {
        SegmentLineStyle::Solid => DashStyle::Solid,
        SegmentLineStyle::Dash => DashStyle::Dash,
    }
}

/// `ToPoint()`: the truncation of an arrow end to whole pixels.
fn truncated(p: PointF) -> PointF {
    PointF::new(p.x.trunc(), p.y.trunc())
}
