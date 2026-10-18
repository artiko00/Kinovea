//! `RectangleF` mirroring `System.Drawing.RectangleF`.

use crate::point::{PointF, SizeF};

/// Axis-aligned rectangle stored as location and size, like `System.Drawing.RectangleF`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RectangleF {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl RectangleF {
    /// `RectangleF.Empty`.
    pub const EMPTY: RectangleF = RectangleF {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };

    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        RectangleF {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_location_size(location: PointF, size: SizeF) -> Self {
        RectangleF::new(location.x, location.y, size.width, size.height)
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn top(&self) -> f32 {
        self.y
    }

    /// `Left + Width`.
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// `Top + Height`.
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn location(&self) -> PointF {
        PointF::new(self.x, self.y)
    }

    pub fn size(&self) -> SizeF {
        SizeF::new(self.width, self.height)
    }

    /// `RectangleF.Center()` extension.
    pub fn center(&self) -> PointF {
        PointF::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// `RectangleF.IsEmpty`: zero or negative width or height.
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    /// `RectangleF.Contains(PointF)`: left/top inclusive, right/bottom exclusive.
    pub fn contains(&self, p: PointF) -> bool {
        self.x <= p.x && p.x < self.right() && self.y <= p.y && p.y < self.bottom()
    }
}
