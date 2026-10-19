//! Owner: P1. Painters ported from `DrawingCrossMark.Draw`.

use super::helpers::frame::{opacity, point_box, rect_f};
use super::helpers::mini_label::draw_mini_label;
use super::PaintContext;
use crate::renderer::Renderer;
use crate::style::colors::alpha_of;
use crate::style::StyleData;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::CrossMark;
use kinovea_metadata::measure_label::MeasureLabelType;

/// `DrawingCrossMark.defaultRadius`: arm length of the cross, in viewport pixels.
const ARM: i32 = 3;
/// `DrawingCrossMark.defaultBackgroundAlpha`: alpha of the disc behind the cross.
const BACKGROUND_ALPHA: f64 = 64.0;

/// Port of `DrawingCrossMark.Draw`: a 2x2 block when `show_as_dot`, otherwise a cross on a disc. The
/// arms and the disc keep their size at any viewport scale, as in the original.
pub fn paint_crossmark(d: &CrossMark, ctx: &PaintContext, r: &mut dyn Renderer) {
    let factor = opacity(&d.header.fading, ctx);
    if factor <= 0.0 {
        return;
    }
    let style = StyleData::from_elements(&d.header.style);
    let pen = style.marker_pen(alpha_of(factor as f32));
    let c = ctx.viewport.transform_point_f(d.center);
    let (x, y) = (c.x, c.y);

    if d.show_as_dot {
        r.stroke_rectangle(rect_f(point_box(c, 0.5)), &pen);
    } else {
        let arm = ARM;
        r.stroke_line(
            PointF::new((x - arm) as f32, y as f32),
            PointF::new((x + arm) as f32, y as f32),
            &pen,
        );
        r.stroke_line(
            PointF::new(x as f32, (y - arm) as f32),
            PointF::new(x as f32, (y + arm) as f32),
            &pen,
        );
        let disc = style.brush((factor * BACKGROUND_ALPHA) as i32);
        r.fill_ellipse(rect_f(point_box(c, (ARM + 1) as f32)), &disc);
    }

    if d.measure_label_type != MeasureLabelType::None {
        let text = label_text(d, ctx);
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

/// Port of `DrawingCrossMark.GetMeasureLabelText`.
fn label_text(d: &CrossMark, ctx: &PaintContext) -> String {
    let calibration = &ctx.metadata.calibration;
    match d.measure_label_type {
        MeasureLabelType::Name => d.header.name.clone(),
        MeasureLabelType::Distance => calibration.length_text(calibration.origin(), d.center, true),
        MeasureLabelType::Position => calibration.point_text(d.center, true, ctx.timestamp),
        _ => String::new(),
    }
}
