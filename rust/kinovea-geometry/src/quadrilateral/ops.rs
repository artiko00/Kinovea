//! In-place manipulations of a [`Quadrilateral`].

use super::Quadrilateral;
use crate::point::PointF;

impl Quadrilateral {
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.vertices = self.vertices.map(|p| p.translate(dx, dy));
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.vertices = self.vertices.map(|p| p.scale(sx, sy));
    }

    /// Moves each vertex outwards by `width` horizontally and `height` vertically.
    pub fn expand(&mut self, width: f32, height: f32) {
        let signs = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
        for (v, (sx, sy)) in self.vertices.iter_mut().zip(signs) {
            *v = v.translate(sx * width, sy * height);
        }
    }

    /// Aligns the other vertices on the `anchor` vertex (0..=3) to form a rectangle.
    pub fn make_rectangle(&mut self, anchor: usize) {
        let [a, b, c, d] = self.vertices;
        let (x, y) = (|p: PointF| p.x, |p: PointF| p.y);
        self.vertices = match anchor {
            0 => [
                a,
                PointF::new(x(b), y(a)),
                PointF::new(x(b), y(d)),
                PointF::new(x(a), y(d)),
            ],
            1 => [
                PointF::new(x(a), y(b)),
                b,
                PointF::new(x(b), y(c)),
                PointF::new(x(a), y(c)),
            ],
            2 => [
                PointF::new(x(d), y(b)),
                PointF::new(x(c), y(b)),
                c,
                PointF::new(x(d), y(c)),
            ],
            3 => [
                PointF::new(x(d), y(a)),
                PointF::new(x(c), y(a)),
                PointF::new(x(c), y(d)),
                d,
            ],
            _ => self.vertices,
        };
    }

    /// Makes a square on the smallest side, keeping the vertex opposite to `anchor` fixed.
    /// Assumes the quadrilateral is already a rectangle.
    pub fn make_square(&mut self, anchor: usize) {
        let [a, b, c, d] = self.vertices;
        let side_ac = (c.x - a.x).min(c.y - a.y);
        let side_bd = (b.x - d.x).min(d.y - b.y);
        self.vertices = match anchor {
            0 => [
                PointF::new(c.x - side_ac, c.y - side_ac),
                PointF::new(c.x, c.y - side_ac),
                c,
                PointF::new(c.x - side_ac, c.y),
            ],
            1 => [
                PointF::new(d.x, d.y - side_bd),
                PointF::new(d.x + side_bd, d.y - side_bd),
                PointF::new(d.x + side_bd, d.y),
                d,
            ],
            2 => [
                a,
                PointF::new(a.x + side_ac, a.y),
                PointF::new(a.x + side_ac, a.y + side_ac),
                PointF::new(a.x, a.y + side_ac),
            ],
            3 => [
                PointF::new(b.x - side_bd, b.y),
                b,
                PointF::new(b.x, b.y + side_bd),
                PointF::new(b.x - side_bd, b.y + side_bd),
            ],
            _ => self.vertices,
        };
    }

    /// Swaps A with B and C with D.
    pub fn flip_horizontal(&mut self) {
        self.vertices.swap(0, 1);
        self.vertices.swap(2, 3);
    }

    /// Swaps A with D and B with C.
    pub fn flip_vertical(&mut self) {
        self.vertices.swap(0, 3);
        self.vertices.swap(1, 2);
    }

    /// Rotates the corner labels 90° clockwise: A←B, B←C, C←D, D←A.
    pub fn rotate90(&mut self) {
        self.vertices.rotate_left(1);
    }
}
