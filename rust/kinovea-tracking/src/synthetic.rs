//! Frames of the `tracking` oracle cases, by the integer formula of
//! `Kinovea.FixtureGen/Areas/TrackingArea.Frames.cs` (documented in `tests/cases/tracking/README.md`),
//! so a test feeds the trackers the same pixels the original saw.

use kinovea_video::frame::FrameBuffer;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    /// Background per channel (B, G, R) at the origin.
    pub background: [i32; 3],
    pub gx: i32,
    pub gy: i32,
    /// Object color (B, G, R).
    pub color: [i32; 3],
    pub disc: bool,
    /// Side of the square or radius of the disc, in pixels.
    pub size: i64,
}

impl Scene {
    /// The BGRA frame with the object centered at `center`, in quarter pixels.
    pub fn render(&self, center: (i64, i64)) -> FrameBuffer {
        let (cx, cy) = (2 * center.0, 2 * center.1);
        let mut bits = Vec::with_capacity((self.width * self.height * 4) as usize);
        for y in 0..i64::from(self.height) {
            for x in 0..i64::from(self.width) {
                let k = i32::from(self.coverage(8 * x, 8 * y, cx, cy));
                for c in 0..3 {
                    let bg = (self.background[c] + (x as i32 * self.gx + y as i32 * self.gy) / 16)
                        .clamp(0, 255);
                    bits.push(((bg * (16 - k) + self.color[c] * k) / 16) as u8);
                }
                bits.push(255);
            }
        }
        FrameBuffer::new(
            self.width,
            self.height,
            self.width as usize * 4,
            Arc::from(bits),
        )
    }

    /// Samples inside the object out of 16 (4x4 grid at eighth-pixel offsets 1, 3, 5, 7).
    fn coverage(&self, x8: i64, y8: i64, cx: i64, cy: i64) -> u8 {
        let mut k = 0;
        for sy in (1..8).step_by(2) {
            for sx in (1..8).step_by(2) {
                let (dx, dy) = (x8 + sx - cx, y8 + sy - cy);
                let inside = if self.disc {
                    dx * dx + dy * dy < 64 * self.size * self.size
                } else {
                    dx.abs() < 4 * self.size && dy.abs() < 4 * self.size
                };
                k += u8::from(inside);
            }
        }
        k
    }
}
