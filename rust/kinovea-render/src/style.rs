//! Drawing style to paint (`Metadata/Styling/StyleData.cs`): pens, brushes and fonts from the style
//! elements, the fading opacity (as alpha) and the viewport stretch factor.

pub mod colors;
#[cfg(test)]
mod colors_test;
pub mod dash;
#[cfg(test)]
mod dash_test;

use crate::renderer::{Brush, Font, FontFamily, LineCap, LineJoin, Pen};
use colors::{foreground_for, with_alpha};
use dash::dash_for_track_shape;
use kinovea_metadata::style::{LineEnding, LineShape, PenShape, TrackShape};
use kinovea_metadata::{Color, StyleElements, StyleValue};

/// `StyleData.minFontSize`: floor in points, applied after the stretch.
const MIN_FONT_POINTS: f32 = 8.0;
/// Points to pixels at the 96 DPI the oracle renders with.
const PX_PER_POINT: f32 = 96.0 / 72.0;

/// Drawing-ready values of a style: the fields `StyleData` exposes to the painters.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleData {
    pub color: Color,
    pub background_color: Color,
    /// Points, as `System.Drawing.Font.Size`.
    pub font_size: i32,
    pub font_family: FontFamily,
    pub font_bold: bool,
    pub line_size: i32,
    pub line_shape: LineShape,
    pub line_ending: LineEnding,
    pub track_shape: TrackShape,
    pub pen_shape: PenShape,
}

impl Default for StyleData {
    fn default() -> Self {
        StyleData {
            color: Color::BLACK,
            background_color: Color::BLACK,
            font_size: 12,
            font_family: FontFamily::Arial,
            font_bold: false,
            line_size: 1,
            line_shape: LineShape::Solid,
            line_ending: LineEnding::None,
            track_shape: TrackShape::SOLID,
            pen_shape: PenShape::Solid,
        }
    }
}

impl StyleData {
    /// Resolves the values from a drawing's style elements. Colours are told apart by key (`back color`
    /// is the background); the other elements by kind. Missing elements keep the defaults.
    pub fn from_elements(elements: &StyleElements) -> Self {
        let mut style = StyleData::default();
        for e in &elements.elements {
            match &e.value {
                StyleValue::Color(c) if e.key.contains("back") => style.background_color = *c,
                StyleValue::Color(c) => style.color = *c,
                StyleValue::FontSize(v) => style.font_size = *v,
                StyleValue::LineSize(v) | StyleValue::PenSize(v) => style.line_size = *v,
                StyleValue::LineShape(v) => style.line_shape = *v,
                StyleValue::Arrows(v) => style.line_ending = *v,
                StyleValue::TrackShape(v) => style.track_shape = *v,
                StyleValue::PenShape(v) => style.pen_shape = *v,
                _ => {}
            }
        }
        style
    }

    /// `GetPen(alpha, stretch)`: width `max(lineSize * stretch, 1)`, dash from the track shape, round joins.
    pub fn pen(&self, alpha: i32, stretch: f32) -> Pen {
        let width = (self.line_size as f32 * stretch).max(1.0);
        Pen {
            dash: dash_for_track_shape(self.track_shape),
            line_join: LineJoin::Round,
            ..Pen::new(with_alpha(self.color, alpha), width)
        }
    }

    /// `GetPen(alpha)`: one pixel with round caps and joins.
    pub fn marker_pen(&self, alpha: i32) -> Pen {
        plain_pen(self.color, alpha)
    }

    pub fn background_pen(&self, alpha: i32) -> Pen {
        plain_pen(self.background_color, alpha)
    }

    /// Black or white, whichever contrasts with the background.
    pub fn foreground_pen(&self, alpha: i32) -> Pen {
        plain_pen(foreground_for(self.background_color), alpha)
    }

    pub fn brush(&self, alpha: i32) -> Brush {
        solid(self.color, alpha)
    }

    pub fn background_brush(&self, alpha: i32) -> Brush {
        solid(self.background_color, alpha)
    }

    pub fn foreground_brush(&self, alpha: i32) -> Brush {
        solid(foreground_for(self.background_color), alpha)
    }

    /// `GetFont(stretch)`: points scaled by the stretch, floored at 8 points, then converted to pixels.
    pub fn font(&self, stretch: f32) -> Font {
        let points = (self.font_size as f32 * stretch).max(MIN_FONT_POINTS);
        Font {
            family: self.font_family,
            em_px: points * PX_PER_POINT,
            bold: self.font_bold,
        }
    }
}

/// `NormalPen` with `alpha`: width 1 with round caps and joins.
fn plain_pen(color: Color, alpha: i32) -> Pen {
    Pen {
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Pen::new(with_alpha(color, alpha), 1.0)
    }
}

fn solid(color: Color, alpha: i32) -> Brush {
    Brush::solid(with_alpha(color, alpha))
}
