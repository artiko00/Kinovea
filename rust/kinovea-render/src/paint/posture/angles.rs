//! Port of `DrawAngles` and `AngleHelper.DrawText`: the pie and the arc of each angle, and its value on
//! a background placed along the middle of the sweep. The geometry comes from the shared `AngleHelper`.

use super::frame::{colored, edge_pen, PostureFrame};
use super::texts::with_symbol;
use crate::paint::helpers::angle_helper::{AngleFlags, AngleHelper};
use crate::paint::helpers::decimal::format_value;
use crate::paint::helpers::frame::{rect_f, to_viewport};
use crate::paint::helpers::rounded_rect::{draw_rounded_rectangle, line_height};
use crate::renderer::{Brush, Renderer};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::Color;

/// Pen width of the angle arc (`penEdge.Width = 2`).
const ARC_WIDTH: f32 = 2.0;

/// `DrawAngles`. An angle is skipped when its legs coincide with the origin.
pub fn paint_angles(f: &PostureFrame, r: &mut dyn Renderer) {
    for angle in &f.posture.angles {
        if !f.is_active(&angle.option_group) {
            continue;
        }
        let o = f.image_point(angle.origin);
        let a = f.image_point(angle.leg1);
        let b = f.image_point(angle.leg2);
        // The shared helper leaves an empty sweep here; the posture has no earlier state to keep.
        if o == a || o == b {
            continue;
        }
        let flags = AngleFlags {
            radius: angle.radius,
            signed: angle.signed,
            ccw: angle.ccw,
            supplementary: angle.supplementary,
        };
        let helper = AngleHelper::new(o, a, b, flags, &f.ctx.metadata.calibration);
        let sweep = helper.sweep;
        let rect = rect_f(f.ctx.viewport.transform_rect_f(sweep.bounding_box));
        let edge = colored(angle.color, f.alpha, f.edge_base());
        let fill = colored(angle.color, f.alpha_background, f.fill_base());
        let pen = edge_pen(edge, ARC_WIDTH, DashStyle::Solid);
        r.fill_pie(rect, sweep.start, sweep.sweep, &Brush::solid(fill));
        r.stroke_arc(rect, sweep.start, sweep.sweep, &pen);
        let origin = to_viewport(f.ctx, o);
        let label = Label {
            helper: &helper,
            distance: angle.text_distance,
            symbol: &angle.symbol,
            fill,
        };
        draw_text(f, r, origin, &label);
    }
}

/// What `draw_text` reads of the angle besides its origin.
struct Label<'a> {
    helper: &'a AngleHelper,
    distance: i32,
    symbol: &'a str,
    fill: Color,
}

/// `AngleHelper.DrawText`: the value in the calibration units, scaled with the viewport.
fn draw_text(f: &PostureFrame, r: &mut dyn Renderer, origin: PointF, text: &Label) {
    let calibration = &f.ctx.metadata.calibration;
    let value = format_value(calibration.convert_angle(text.helper.calibrated_angle));
    let label = with_symbol(
        text.symbol,
        &format!("{value} {}", calibration.angle_abbreviation()),
    );
    let scale = f.ctx.viewport.scale as f32;
    let font = f.style.font(1.0);
    let size = r.measure_text(&label, &font);
    let scaled_font = f.style.font(scale);
    let scaled_size = r.measure_text(&label, &scaled_font);
    let offset = text.helper.text_position(text.distance, size);
    let at = PointF::new(origin.x + offset.x * scale, origin.y + offset.y * scale);
    let rect = RectangleF::new(at.x, at.y, scaled_size.width, scaled_size.height);
    let radius = line_height(r, &scaled_font) / 4;
    draw_rounded_rectangle(
        r,
        rect,
        &Brush::solid(text.fill),
        radius,
        false,
        false,
        None,
    );
    let fore = f.style.foreground_brush(f.text_alpha());
    r.draw_text(&label, &scaled_font, &fore, at);
}
