//! Port of `DrawAngles` and `AngleHelper.DrawText`: the pie and the arc of each angle, and its value on
//! a background placed along the middle of the sweep.

use super::angle_sweep::{compute, Angle, AngleFlags, Sweep};
use super::frame::{colored, edge_pen, PostureFrame};
use super::texts::with_symbol;
use crate::paint::helpers::decimal::format_value;
use crate::paint::helpers::frame::{rect_f, to_viewport};
use crate::paint::helpers::rounded_rect::{draw_rounded_rectangle, line_height};
use crate::renderer::{Brush, Renderer};
use kinovea_geometry::{PointF, RectangleF, SizeF};
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::Color;
use std::f64::consts::PI;

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
        let flags = AngleFlags {
            radius: angle.radius,
            signed: angle.signed,
            ccw: angle.ccw,
            supplementary: angle.supplementary,
        };
        let Some(state) = compute(o, a, b, flags, &f.ctx.metadata.calibration) else {
            continue;
        };
        let rect = rect_f(f.ctx.viewport.transform_rect_f(state.sweep.bounding));
        let edge = colored(angle.color, f.alpha, f.edge_base());
        let fill = colored(angle.color, f.alpha_background, f.fill_base());
        let pen = edge_pen(edge, ARC_WIDTH, DashStyle::Solid);
        r.fill_pie(
            rect,
            state.sweep.start,
            state.sweep.sweep,
            &Brush::solid(fill),
        );
        r.stroke_arc(rect, state.sweep.start, state.sweep.sweep, &pen);
        let origin = to_viewport(f.ctx, state.sweep.origin);
        draw_text(
            f,
            r,
            origin,
            &state,
            angle.text_distance,
            &angle.symbol,
            fill,
        );
    }
}

/// `AngleHelper.DrawText`: the value in the calibration units, scaled with the viewport.
fn draw_text(
    f: &PostureFrame,
    r: &mut dyn Renderer,
    origin: PointF,
    angle: &Angle,
    text_distance: i32,
    symbol: &str,
    fill: Color,
) {
    let calibration = &f.ctx.metadata.calibration;
    let value = format_value(calibration.convert_angle(angle.calibrated));
    let label = with_symbol(
        symbol,
        &format!("{value} {}", calibration.angle_abbreviation()),
    );
    let scale = f.ctx.viewport.scale as f32;
    let font = f.style.font(1.0);
    let size = r.measure_text(&label, &font);
    let scaled_font = f.style.font(scale);
    let scaled_size = r.measure_text(&label, &scaled_font);
    let offset = text_position(angle.sweep, text_distance, size);
    let at = PointF::new(origin.x + offset.x * scale, origin.y + offset.y * scale);
    let rect = RectangleF::new(at.x, at.y, scaled_size.width, scaled_size.height);
    let radius = line_height(r, &scaled_font) / 4;
    draw_rounded_rectangle(r, rect, &Brush::solid(fill), radius, false, false, None);
    let fore = f.style.foreground_brush(f.text_alpha());
    r.draw_text(&label, &scaled_font, &fore, at);
}

/// `GetTextPosition`: `text_distance` along the middle of the sweep, the label centred on that point.
fn text_position(sweep: Sweep, text_distance: i32, size: SizeF) -> PointF {
    // `MathHelper.Radians` returns a float.
    let degrees = sweep.start + sweep.sweep / 2.0;
    let radians = f64::from((f64::from(degrees) * PI / 180.0) as f32);
    let distance = f64::from(text_distance);
    let x = radians.cos() * distance - f64::from(size.width) / 2.0;
    let y = radians.sin() * distance - f64::from(size.height) / 2.0;
    PointF::new(x as f32, y as f32)
}
