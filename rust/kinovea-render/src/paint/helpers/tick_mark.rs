//! Port of `TickMark` (`Metadata/Drawings/CoordinateSystem/TickMark.cs`), shared by the coordinate
//! system and the calibration plane: a graduation value written on a rounded label beside its point.

use crate::paint::helpers::decimal::format_rounded;
use crate::paint::helpers::frame::to_viewport;
use crate::paint::helpers::rounded_rect::draw_rounded_rectangle;
use crate::paint::PaintContext;
use crate::renderer::{Brush, Font, Pen, Renderer};
use crate::style::colors::with_alpha;
use crate::style::StyleData;
use kinovea_geometry::{PointF, RectangleF, SizeF};
use kinovea_metadata::Color;

/// `TextAlignment`: where the label sits relative to its point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlignment {
    Top,
    Bottom,
    Left,
    Right,
    BottomRight,
}

/// A graduation: its value in world units and its location in image space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickMark {
    pub value: f32,
    pub image_location: PointF,
    pub alignment: TextAlignment,
}

/// Colours, font and precision of the labels.
pub struct TickLabel<'a> {
    pub fill: &'a Brush,
    pub text: &'a Brush,
    pub font: &'a Font,
    pub decimal_places: i32,
}

/// `TickMark.Draw`: the value rounded to `decimal_places` on a box `margin` pixels away from the tick.
/// The tick goes through `distort` (the lens or calibration distortion) and then the viewport of `ctx`.
pub fn draw_tick_mark(
    r: &mut dyn Renderer,
    tick: &TickMark,
    label: &TickLabel,
    ctx: &PaintContext,
    distort: &dyn Fn(PointF) -> PointF,
    margin: f32,
) {
    let anchor = to_viewport(ctx, distort(tick.image_location));
    let text = format_rounded(tick.value, label.decimal_places);
    let size = r.measure_text(&text, label.font);
    let origin = label_origin(tick.alignment, anchor, size, margin);
    let back = RectangleF::from_location_size(origin, size);
    // The original uses the font height over 4; the measured height of the label stands in.
    let radius = size.height as i32 / 4;
    draw_rounded_rectangle(r, back, label.fill, radius, false, false, None);
    r.draw_text(&text, label.font, label.text, origin);
}

/// `GetTextPosition`: the top-left corner of the label for its alignment.
fn label_origin(alignment: TextAlignment, p: PointF, size: SizeF, margin: f32) -> PointF {
    match alignment {
        TextAlignment::Top => PointF::new(p.x - size.width / 2.0, p.y - size.height - margin),
        TextAlignment::Bottom => PointF::new(p.x - size.width / 2.0, p.y + margin),
        TextAlignment::Left => PointF::new(p.x - size.width - margin, p.y - size.height / 2.0),
        TextAlignment::Right => PointF::new(p.x + margin, p.y - size.height / 2.0),
        TextAlignment::BottomRight => PointF::new(p.x + margin, p.y + margin),
    }
}

/// `styleData.GetBrush(defaultBackgroundAlpha)`: opacity of the tick label boxes.
const TICK_BOX_ALPHA: i32 = 92;
/// Font of the tick labels: Arial 8 points, bold, as the plane constructor sets it.
const TICK_FONT_POINTS: i32 = 8;

/// `DrawSlidingLines` tick drawing: boxes in the pen's colour, text black on bright colours and white
/// otherwise (`Color.GetBrightness() > 0.6`). Locations go through the calibration distortion.
pub fn draw_ticks(
    r: &mut dyn Renderer,
    pen: &Pen,
    ticks: &[TickMark],
    decimal_places: i32,
    margin: f32,
    ctx: &PaintContext,
) {
    let fill = Brush::solid(with_alpha(pen.color, TICK_BOX_ALPHA));
    let text_color = if brightness(pen.color) > 0.6 {
        Color::BLACK
    } else {
        Color::rgb(255, 255, 255)
    };
    let text = Brush::solid(text_color);
    let font = StyleData {
        font_size: TICK_FONT_POINTS,
        font_bold: true,
        ..StyleData::default()
    }
    .font(1.0);
    let label = TickLabel {
        fill: &fill,
        text: &text,
        font: &font,
        decimal_places,
    };
    let distortion = ctx.metadata.calibration.distortion();
    let distort = |p: PointF| {
        if distortion.is_initialized() {
            distortion.distort(p)
        } else {
            p
        }
    };
    for tick in ticks {
        draw_tick_mark(r, tick, &label, ctx, &distort, margin);
    }
}

/// `Color.GetBrightness`: `(max + min) / 2` over the channels, in 0..=1. Alpha is ignored.
fn brightness(c: Color) -> f32 {
    let max = c.r.max(c.g).max(c.b) as f32 / 255.0;
    let min = c.r.min(c.g).min(c.b) as f32 / 255.0;
    (max + min) / 2.0
}
