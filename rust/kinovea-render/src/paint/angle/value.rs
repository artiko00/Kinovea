//! Owner: P3. Port of `AngleHelper.DrawText`: the value of the angle on a box along its bisector.

use crate::paint::helpers::angle_helper::{text_distance, AngleHelper};
use crate::paint::helpers::decimal::format_rounded;
use crate::paint::helpers::frame::stretch;
use crate::paint::helpers::rounded_rect::draw_rounded_rectangle;
use crate::paint::PaintContext;
use crate::renderer::{Brush, Renderer};
use crate::style::StyleData;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_measurement::CalibrationHelper;
use kinovea_metadata::drawing::Angle;
use kinovea_metadata::{StyleElements, StyleValue};

/// What `DrawText` needs besides the origin: the label, the helper that places it and the colours.
pub struct Value<'a> {
    pub text: String,
    pub helper: AngleHelper,
    pub style: &'a StyleData,
    pub disk: &'a Brush,
    pub factor: f64,
}

/// `DrawText`: the value on a box along the bisector, scaled with the viewport.
pub fn draw_value(r: &mut dyn Renderer, value: &Value, origin: PointF, ctx: &PaintContext) {
    let scale = stretch(ctx);
    let label_size = r.measure_text(&value.text, &value.style.font(1.0));
    let distance = text_distance(value.style.font_size);
    let offset = value
        .helper
        .text_position(distance, label_size)
        .scale(scale, scale);
    let font = value.style.font(scale);
    let origin = origin.translate(offset.x, offset.y);
    let scaled_size = r.measure_text(&value.text, &font);
    let back = RectangleF::from_location_size(origin, scaled_size);
    // The original uses the font height over 4; the measured height of the one-line text stands in.
    let radius = scaled_size.height as i32 / 4;
    draw_rounded_rectangle(r, back, value.disk, radius, false, false, None);
    let text_alpha = (value.factor * 255.0) as i32;
    r.draw_text(
        &value.text,
        &font,
        &value.style.foreground_brush(text_alpha),
        origin,
    );
}

/// `AngleHelper.DrawText`: the value rounded to `DecimalPlaces`, then the unit.
pub fn value_label(d: &Angle, helper: &AngleHelper, calibration: &CalibrationHelper) -> String {
    let value = format_rounded(
        calibration.convert_angle(helper.calibrated_angle),
        decimal_places(&d.header.style),
    );
    format!("{} {}", value, calibration.angle_abbreviation())
}

/// `DecimalPlaces` style element; the original defaults to 0.
fn decimal_places(style: &StyleElements) -> i32 {
    match style.get("DecimalPlaces") {
        Some(StyleValue::Int { value, .. }) => *value,
        _ => 0,
    }
}
