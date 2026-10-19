//! Owner: P1. Painters ported from `DrawingCircle.Draw` (image-plane branch).

use super::helpers::frame::{opacity, point_box_f, rect_f, stretch, to_viewport};
use super::helpers::mini_label::draw_mini_label;
use super::PaintContext;
use crate::renderer::{Pen, Renderer};
use crate::style::colors::alpha_of;
use crate::style::StyleData;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Circle;
use kinovea_metadata::measure_label::MeasureLabelType;
use kinovea_metadata::style::{DashStyle, PenShape};
use kinovea_metadata::Metadata;

/// `DrawingCircle.crossSize / 2`: arm length of the centre mark.
const CENTER_ARM: f32 = 7.5;

/// Port of `DrawingCircle.Draw` for the image-plane branch. The perspective ellipse of a plane
/// calibration (`GetEllipseFromCircle`) is not painted yet.
pub fn paint_circle(d: &Circle, ctx: &PaintContext, r: &mut dyn Renderer) {
    let factor = opacity(&d.header.fading, ctx);
    if factor <= 0.0 {
        return;
    }
    let style = StyleData::from_elements(&d.header.style);
    let alpha = alpha_of(factor as f32);
    let mut pen = style.pen(alpha, stretch(ctx));
    if style.pen_shape == PenShape::Dash {
        pen.dash = DashStyle::Dash;
    }
    let center = to_viewport(ctx, d.center);
    if d.show_center {
        draw_center(r, &pen, center);
    }

    let radius = d.radius as f32;
    let bounds = rect_f(
        ctx.viewport
            .transform_rect_f(point_box_f(d.center, d.radius)),
    );
    r.stroke_ellipse(bounds, &pen);
    if d.filled {
        r.fill_ellipse(bounds, &style.brush(alpha));
    }

    if d.measure_label_type != MeasureLabelType::None {
        let left = to_viewport(ctx, PointF::new(d.center.x - radius, d.center.y));
        let right = to_viewport(ctx, PointF::new(d.center.x + radius, d.center.y));
        match d.measure_label_type {
            MeasureLabelType::Radius => r.stroke_line(center, right, &pen),
            MeasureLabelType::Diameter => r.stroke_line(left, right, &pen),
            _ => {}
        }
        let text = label_text(d, ctx.metadata, ctx.timestamp);
        draw_mini_label(
            r,
            &d.label,
            &text,
            style.color,
            style.font_size,
            factor,
            ctx,
        );
    }
}

/// Port of `DrawingCircle.DrawCenter`: a precise one-pixel solid cross on the centre.
fn draw_center(r: &mut dyn Renderer, pen: &Pen, center: PointF) {
    let precise = Pen {
        width: 1.0,
        dash: DashStyle::Solid,
        ..pen.clone()
    };
    let (x, y) = (center.x, center.y);
    r.stroke_line(
        PointF::new(x - CENTER_ARM, y),
        PointF::new(x + CENTER_ARM, y),
        &precise,
    );
    r.stroke_line(
        PointF::new(x, y - CENTER_ARM),
        PointF::new(x, y + CENTER_ARM),
        &precise,
    );
}

/// Port of `DrawingCircle.GetMeasureLabelText` (the image-plane points).
pub(crate) fn label_text(d: &Circle, metadata: &Metadata, timestamp: i64) -> String {
    let radius = d.radius as f32;
    let left = PointF::new(d.center.x - radius, d.center.y);
    let right = PointF::new(d.center.x + radius, d.center.y);
    let calibration = &metadata.calibration;
    match d.measure_label_type {
        MeasureLabelType::Name => d.header.name.clone(),
        MeasureLabelType::Center => calibration.point_text(d.center, true, timestamp),
        MeasureLabelType::Radius => calibration.length_text(d.center, right, true),
        MeasureLabelType::Diameter => calibration.length_text(left, right, true),
        MeasureLabelType::Circumference => calibration.circumference_text(d.center, right, true),
        _ => String::new(),
    }
}
