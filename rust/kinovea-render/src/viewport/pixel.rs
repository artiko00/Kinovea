//! Integer `Point` and `Rectangle` as in `System.Drawing`, used for viewport pixels.
use kinovea_geometry::PointF;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Widening to `PointF`, as the implicit conversion in `System.Drawing`.
    pub fn to_point_f(self) -> PointF {
        PointF::new(self.x as f32, self.y as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}
