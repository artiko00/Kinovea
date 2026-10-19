//! Owner: P1. Painters ported from `DrawingLine.Draw`.

use super::helpers::arrow::{draw_arrow, update_start_end};
use super::helpers::frame::{attach_label, opacity, stretch, to_viewport};
use super::helpers::mini_label::draw_mini_label;
use super::helpers::squiggly::draw_squiggly_line;
use super::PaintContext;
use crate::renderer::Renderer;
use crate::style::colors::alpha_of;
use crate::style::StyleData;
use kinovea_geometry::helpers::middle_point;
use kinovea_metadata::drawing::Line;
use kinovea_metadata::Metadata;
use kinovea_metadata::measure_label::MeasureLabelType;
use kinovea_metadata::style::{DashStyle, LineEnding, LineShape};

/// Port of `DrawingLine.Draw`, straight branch (`DrawStraight`). A distorted curve (`DrawDistorted`) is
/// not painted yet: it needs `DistortionHelper.Undistort`, which the `Distorter` trait does not expose,
/// so the straight segment is drawn even when a distorter is present.
pub fn paint_line(d: &Line, ctx: &PaintContext, r: &mut dyn Renderer) {
    let factor = opacity(&d.header.fading, ctx);
    if factor <= 0.0 {
        return;
    }
    let style = StyleData::from_elements(&d.header.style);
    let mut pen = style.pen(alpha_of(factor as f32), stretch(ctx));
    let mut start = to_viewport(ctx, d.a);
    let mut end = to_viewport(ctx, d.b);
    let start_arrow = matches!(
        style.line_ending,
        LineEnding::StartArrow | LineEnding::DoubleArrow
    );
    let end_arrow = matches!(
        style.line_ending,
        LineEnding::EndArrow | LineEnding::DoubleArrow
    );
    let can_arrow = update_start_end(pen.width, &mut start, &mut end, start_arrow, end_arrow);

    match style.line_shape {
        LineShape::Squiggle => draw_squiggly_line(r, &pen, start, end),
        LineShape::Dash => {
            pen.dash = DashStyle::Dash;
            r.stroke_line(start, end, &pen);
        }
        _ => r.stroke_line(start, end, &pen),
    }

    if can_arrow {
        if start_arrow {
            draw_arrow(r, &pen, start, end);
        }
        if end_arrow {
            draw_arrow(r, &pen, end, start);
        }
    }

    if d.measure_label_type != MeasureLabelType::None {
        let text = label_text(d, ctx.metadata);
        let label = attach_label(&d.label, middle_point(d.a, d.b));
        draw_mini_label(r, &label, &text, style.color, style.font_size, factor, ctx);
    }
}

/// The measure text of the mini label: the name, or the length of the segment.
pub(crate) fn label_text(d: &Line, metadata: &Metadata) -> String {
    match d.measure_label_type {
        MeasureLabelType::Name => d.header.name.clone(),
        _ => metadata.calibration.length_text(d.a, d.b, true),
    }
}

#[cfg(test)]
mod arrow_test;
#[cfg(test)]
mod fading_test;
