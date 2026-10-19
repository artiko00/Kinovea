//! Owned 8-bit gray images and the integer region of interest used to crop frames.

use kinovea_geometry::Size;

/// Single-channel 8-bit image, row-major, without padding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl GrayImage {
    pub fn at(&self, x: u32, y: u32) -> u8 {
        self.data[(y * self.width + x) as usize]
    }
}

/// Integer rectangle in image coordinates. It may reach outside the image until clamped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Roi {
    pub x: i32,
    pub y: i32,
    pub size: Size,
}

impl Roi {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Roi {
            x,
            y,
            size: Size::new(width, height),
        }
    }

    /// Intersection with `[0, width) x [0, height)`, as `srchRect.Intersect(...)` in the original.
    /// The result is empty when nothing overlaps.
    pub fn clamp_to(self, width: u32, height: u32) -> Roi {
        let (w, h) = (i64::from(width), i64::from(height));
        let x0 = i64::from(self.x).clamp(0, w);
        let y0 = i64::from(self.y).clamp(0, h);
        let x1 = (i64::from(self.x) + i64::from(self.size.width)).clamp(x0, w);
        let y1 = (i64::from(self.y) + i64::from(self.size.height)).clamp(y0, h);
        Roi {
            x: x0 as i32,
            y: y0 as i32,
            size: Size::new((x1 - x0) as i32, (y1 - y0) as i32),
        }
    }
}
