//! `Untransform` overloads: viewport pixels back to image coordinates.
use super::{ImageToViewport, Point, Rectangle};
use kinovea_geometry::{PointF, RectangleF, SizeF};

impl ImageToViewport {
    /// `Untransform(Point)`: `(p - location) * (1 / scale)` in double, narrowed to float.
    pub fn untransform_point(&self, p: Point) -> PointF {
        let unscale = 1.0 / self.scale;
        PointF::new(
            ((p.x - self.location.x) as f64 * unscale) as f32,
            ((p.y - self.location.y) as f64 * unscale) as f32,
        )
    }

    /// `Untransform(SizeF)`.
    pub fn untransform_size_f(&self, size: SizeF) -> SizeF {
        let unscale = 1.0 / self.scale;
        SizeF::new(
            (f64::from(size.width) * unscale) as f32,
            (f64::from(size.height) * unscale) as f32,
        )
    }

    /// `Untransform(Rectangle)`: location through `Untransform(Point)`, size through `Untransform(SizeF)`.
    pub fn untransform_rect(&self, r: Rectangle) -> RectangleF {
        let location = self.untransform_point(Point::new(r.x, r.y));
        let size = self.untransform_size_f(SizeF::new(r.width as f32, r.height as f32));
        RectangleF::new(location.x, location.y, size.width, size.height)
    }

    /// `Untransform(int)`: `(int)(distance / scale)`, truncated.
    pub fn untransform_distance(&self, distance: i32) -> i32 {
        (f64::from(distance) / self.scale) as i32
    }
}
