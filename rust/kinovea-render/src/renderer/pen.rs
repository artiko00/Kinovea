//! `System.Drawing.Pen` values: colour, width, dash, caps and join, as the drawings set them.

use kinovea_metadata::style::DashStyle;
use kinovea_metadata::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    /// `LineCap.NoAnchor` (GDI+ default).
    Flat,
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    Miter,
    Round,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pen {
    pub color: Color,
    pub width: f32,
    pub dash: DashStyle,
    pub start_cap: LineCap,
    pub end_cap: LineCap,
    pub line_join: LineJoin,
}

impl Pen {
    /// `new Pen(color, width)`: solid, flat caps, miter joins.
    pub fn new(color: Color, width: f32) -> Self {
        Pen {
            color,
            width,
            dash: DashStyle::Solid,
            start_cap: LineCap::Flat,
            end_cap: LineCap::Flat,
            line_join: LineJoin::Miter,
        }
    }
}

/// Dash lengths in pen-width units, as GDI+ `DashPattern`. `Custom` has no pattern in the model and draws solid.
pub fn dash_pattern(dash: DashStyle) -> &'static [f32] {
    match dash {
        DashStyle::Solid | DashStyle::Custom => &[],
        DashStyle::Dash => &[3.0, 1.0],
        DashStyle::Dot => &[1.0, 1.0],
        DashStyle::DashDot => &[3.0, 1.0, 1.0, 1.0],
        DashStyle::DashDotDot => &[3.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    }
}
