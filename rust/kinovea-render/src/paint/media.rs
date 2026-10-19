//! Owner: P2. Painters ported from `DrawingBitmap.Draw` and `DrawingSVG.Draw`. Placeholders until
//! the image readers land: the original draws the picture, these draw a grey box named after the
//! file (or the sticker reference), so the output is not the original's for images.

use super::helpers::rounded_rect::to_rect_f;
use super::PaintContext;
use crate::renderer::{Brush, Font, FontFamily, Renderer};
use crate::style::colors::with_alpha;
use crate::viewport::Rectangle;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Bitmap, Svg};
use kinovea_metadata::Color;

const PLACEHOLDER_FONT: Font = Font {
    family: FontFamily::Arial,
    em_px: 16.0,
    bold: false,
};

/// Placeholder for `DrawingBitmap.Draw`: the bounding box in grey, with the file name (or the
/// sticker reference) at its top-left corner, both at the drawing's opacity.
pub fn paint_bitmap(d: &Bitmap, ctx: &PaintContext, r: &mut dyn Renderer) {
    let opacity = d
        .header
        .fading
        .opacity_factor(ctx.timestamp, ctx.default_fading);
    if opacity <= 0.0 {
        return;
    }
    let alpha = (opacity * 255.0) as i32;
    let bounds = d.bounding_box;
    let origin = ctx
        .viewport
        .transform_point_f(PointF::new(bounds.x, bounds.y));
    let width = ctx.viewport.transform_distance_f(bounds.width);
    let height = ctx.viewport.transform_distance_f(bounds.height);
    let rect = to_rect_f(Rectangle::new(origin.x, origin.y, width, height));
    r.fill_rectangle(
        rect,
        &Brush::solid(with_alpha(Color::rgb(128, 128, 128), alpha)),
    );
    let name = if d.is_sticker {
        &d.sticker_reference
    } else {
        &d.file
    };
    let text = Brush::solid(with_alpha(Color::BLACK, alpha));
    r.draw_text(name, &PLACEHOLDER_FONT, &text, origin.to_point_f());
}

/// `DrawingSVG.Draw` is not placed: the SVG model keeps no bounds, so there is no box to fill until
/// the readers and the bounds land.
pub fn paint_svg(d: &Svg, ctx: &PaintContext, r: &mut dyn Renderer) {
    let _ = (d, ctx, r);
}
