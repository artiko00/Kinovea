//! Port of `DrawDistances`, `DrawPositions` and `DrawTextOnBackground`: the read-outs of distances and
//! positions, on a background in the fill colour, with the foreground chosen against it.

use super::frame::{colored, PostureFrame};
use crate::paint::helpers::rounded_rect::{draw_rounded_rectangle, line_height};
use crate::renderer::{Brush, Renderer};
use crate::style::colors::{foreground_for, with_alpha};
use kinovea_geometry::helpers::middle_point;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::Color;

/// `DrawDistances`: the label of the length between two points, at the middle of the segment.
pub fn paint_distances(f: &PostureFrame, r: &mut dyn Renderer) {
    let calibration = &f.ctx.metadata.calibration;
    for distance in &f.posture.distances {
        if !f.is_active(&distance.option_group) {
            continue;
        }
        let text = calibration.length_text(
            f.image_point(distance.point1),
            f.image_point(distance.point2),
            true,
        );
        let label = with_symbol(&distance.symbol, &text);
        let middle = middle_point(f.point(distance.point1), f.point(distance.point2));
        let fill = colored(distance.color, f.alpha_background, f.fill_base());
        text_on_background(f, r, middle, PointF::new(0.0, 15.0), &label, fill);
    }
}

/// `DrawPositions`: the coordinates of a point, above it.
pub fn paint_positions(f: &PostureFrame, r: &mut dyn Renderer) {
    let calibration = &f.ctx.metadata.calibration;
    for position in &f.posture.positions {
        if !f.is_active(&position.option_group) {
            continue;
        }
        let text =
            calibration.point_text(f.image_point(position.point1), true, f.reference_timestamp);
        let label = with_symbol(&position.symbol, &text);
        let fill = colored(position.color, f.alpha_background, f.fill_base());
        text_on_background(
            f,
            r,
            f.point(position.point1),
            PointF::new(0.0, -20.0),
            &label,
            fill,
        );
    }
}

/// `label = text` when the element has a symbol.
pub fn with_symbol(symbol: &str, text: &str) -> String {
    if symbol.is_empty() {
        text.to_string()
    } else {
        format!("{symbol} = {text}")
    }
}

/// `DrawTextOnBackground`: the label centred on `location` plus `offset`, on a rounded background in
/// `fill`, with the foreground contrasting it at the drawing opacity.
fn text_on_background(
    f: &PostureFrame,
    r: &mut dyn Renderer,
    location: PointF,
    offset: PointF,
    label: &str,
    fill: Color,
) {
    let font = f.text_font();
    let size = r.measure_text(label, &font);
    let origin = PointF::new(
        location.x - size.width / 2.0 + offset.x,
        location.y - size.height / 2.0 + offset.y,
    );
    let rect = RectangleF::new(origin.x, origin.y, size.width, size.height);
    let radius = line_height(r, &font) / 4;
    draw_rounded_rectangle(r, rect, &Brush::solid(fill), radius, false, false, None);
    let fore = with_alpha(foreground_for(fill), f.text_alpha());
    r.draw_text(label, &font, &Brush::solid(fore), origin);
}
