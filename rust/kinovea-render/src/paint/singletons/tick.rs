//! Port of `TickMark` (`Draw` and `GetTextPosition`): a graduation label on a rounded background.

use crate::paint::helpers::decimal::format_value;
use crate::paint::helpers::frame::to_viewport;
use crate::paint::helpers::rounded_rect::{draw_rounded_rectangle, line_height};
use crate::paint::PaintContext;
use crate::renderer::{Brush, Font, Renderer};
use kinovea_geometry::{PointF, RectangleF, SizeF};

/// Where the label sits relative to its graduation (`TextAlignment`, the values the finder uses).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlignment {
    Left,
    Bottom,
    BottomRight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TickMark {
    pub value: f32,
    /// Image coordinates of the graduation.
    pub location: PointF,
    pub alignment: TextAlignment,
}

impl TickMark {
    /// `TickMark.Draw(canvas, distorter, transformer, brushFill, fontBrush, font, margin, false)`.
    pub fn paint(
        &self,
        ctx: &PaintContext,
        r: &mut dyn Renderer,
        back: &Brush,
        fore: &Brush,
        font: &Font,
        margin: f32,
    ) {
        let label = format_value(self.value);
        let location = match ctx.distorter {
            Some(d) => d.distort(self.location),
            None => self.location,
        };
        let at = to_viewport(ctx, location);
        let size = r.measure_text(&label, font);
        let origin = text_position(at, self.alignment, size, margin);
        let rect = RectangleF::new(origin.x, origin.y, size.width, size.height);
        let radius = line_height(r, font) / 4;
        draw_rounded_rectangle(r, rect, back, radius, false, false, None);
        r.draw_text(&label, font, fore, origin);
    }
}

/// `GetTextPosition`: the top-left corner of the label for a viewport point `tick` and its alignment.
fn text_position(tick: PointF, alignment: TextAlignment, size: SizeF, margin: f32) -> PointF {
    match alignment {
        TextAlignment::Left => {
            PointF::new(tick.x - size.width - margin, tick.y - size.height / 2.0)
        }
        TextAlignment::Bottom => PointF::new(tick.x - size.width / 2.0, tick.y + margin),
        TextAlignment::BottomRight => PointF::new(tick.x + margin, tick.y + margin),
    }
}
