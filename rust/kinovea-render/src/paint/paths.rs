//! Owner: P1. Painters ported from `DrawingPencil.Draw`, `DrawingPolyline.Draw`, `DrawingTimePolyline.Draw`.

use super::helpers::arrow::draw_arrow;
use super::helpers::frame::{opacity, point_box_f, stretch, to_viewport_all};
use super::helpers::squiggly::draw_squiggly_lines;
use super::PaintContext;
use crate::renderer::{LineCap, Pen, Renderer};
use crate::style::colors::alpha_of;
use crate::style::StyleData;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Pencil, Polyline, TimePolyline};
use kinovea_metadata::style::{DashStyle, LineEnding, LineShape, PenShape, ToggleVariant};
use kinovea_metadata::{StyleElements, StyleValue};

/// Port of `DrawingPencil.Draw`: a round-capped cardinal curve (tension 0.5). A stroke whose bounds
/// have no width or no height is drawn as a one-pixel segment, as `RectangleF.IsEmpty` does.
pub fn paint_pencil(d: &Pencil, ctx: &PaintContext, r: &mut dyn Renderer) {
    let factor = opacity(&d.header.fading, ctx);
    if factor <= 0.0 {
        return;
    }
    let style = StyleData::from_elements(&d.header.style);
    let mut pen = style.pen(alpha_of(factor as f32), stretch(ctx));
    pen.start_cap = LineCap::Round;
    pen.end_cap = LineCap::Round;
    if style.pen_shape == PenShape::Dash {
        pen.dash = DashStyle::Dash;
    }
    let points = to_viewport_all(ctx, &d.points);
    let Some(&first) = points.first() else {
        return;
    };
    if is_flat(&points) {
        r.stroke_line(first, PointF::new(first.x + 1.0, first.y), &pen);
    } else {
        r.stroke_curve(&points, 0.5, &pen);
    }
}

/// Port of `DrawingPolyline.Draw`: the keyed points as a path, arrows and selection handles.
pub fn paint_polyline(d: &Polyline, ctx: &PaintContext, r: &mut dyn Renderer) {
    let factor = opacity(&d.header.fading, ctx);
    if factor <= 0.0 {
        return;
    }
    let values: Vec<PointF> = d.points.0.iter().map(|(_, p)| *p).collect();
    let style = StyleData::from_elements(&d.header.style);
    paint_chain(r, ctx, &style, &d.header.style, factor, &values);
}

/// Port of `DrawingTimePolyline.Draw`: a polyline drawn at full opacity; the camera transform is not
/// applied yet (`CameraTransformer` is not available).
pub fn paint_time_polyline(d: &TimePolyline, ctx: &PaintContext, r: &mut dyn Renderer) {
    let values: Vec<PointF> = d.points.0.iter().map(|(_, p)| *p).collect();
    let style = StyleData::from_elements(&d.header.style);
    paint_chain(r, ctx, &style, &d.header.style, 1.0, &values);
}

/// Shared body of `DrawingPolyline.Draw` and `DrawingTimePolyline.Draw`: path, arrows, handles.
fn paint_chain(
    r: &mut dyn Renderer,
    ctx: &PaintContext,
    style: &StyleData,
    elements: &StyleElements,
    factor: f64,
    values: &[PointF],
) {
    if values.len() < 2 {
        return;
    }
    let alpha = alpha_of(factor as f32);
    let points = to_viewport_all(ctx, values);
    let pen = style.pen(alpha, stretch(ctx));
    draw_path(r, style, is_curved(elements), &pen, &points);

    let n = points.len();
    if matches!(
        style.line_ending,
        LineEnding::StartArrow | LineEnding::DoubleArrow
    ) {
        draw_arrow(r, &pen, points[0], points[1]);
    }
    if matches!(
        style.line_ending,
        LineEnding::EndArrow | LineEnding::DoubleArrow
    ) {
        draw_arrow(r, &pen, points[n - 1], points[n - 2]);
    }
    if ctx.selected {
        let brush = style.brush(alpha);
        for p in &points {
            r.fill_ellipse(point_box_f(*p, 3), &brush);
        }
    }
}

/// Port of `DrawingPolyline.DrawPath`: squiggles, or a curve or straight segments for solid and dashed
/// shapes. Other line shapes draw nothing, as in the original switch.
fn draw_path(r: &mut dyn Renderer, style: &StyleData, curved: bool, pen: &Pen, points: &[PointF]) {
    let mut pen = pen.clone();
    pen.end_cap = LineCap::Flat;
    pen.dash = if style.line_shape == LineShape::Dash {
        DashStyle::Dash
    } else {
        DashStyle::Solid
    };
    match style.line_shape {
        LineShape::Squiggle => draw_squiggly_lines(r, &pen, points),
        LineShape::Solid | LineShape::Dash if curved => r.stroke_curve(points, 0.5, &pen),
        LineShape::Solid | LineShape::Dash => r.stroke_lines(points, &pen),
    }
}

/// `StyleData.Curved`: the `Curved` toggle of the style elements.
fn is_curved(elements: &StyleElements) -> bool {
    elements.elements.iter().any(|e| {
        matches!(
            e.value,
            StyleValue::Toggle {
                value: true,
                variant: ToggleVariant::Curved
            }
        )
    })
}

/// True when the points have no width or no height (`RectangleF.IsEmpty` of the curve bounds).
fn is_flat(points: &[PointF]) -> bool {
    let min_x = points.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
    let max_x = points.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
    let min_y = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
    let max_y = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);
    max_x - min_x <= 0.0 || max_y - min_y <= 0.0
}

#[cfg(test)]
mod polyline_test;
