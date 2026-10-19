//! Image to viewport transform (`ImageToViewportTransformer`, design D3).
//!
//! Integer conversions follow the original casts: scaled points and sizes truncate toward zero,
//! integer distances use `Math.Round` (ties to even) and untransformed distances truncate.

mod extents;
#[cfg(test)]
mod extents_test;
mod pixel;
#[cfg(test)]
mod pixel_test;
mod untransform;
#[cfg(test)]
mod untransform_test;

use kinovea_geometry::PointF;

pub use pixel::{Point, Rectangle};

/// Where the frame is rendered in viewport pixels (`location`) and at which scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageToViewport {
    pub location: Point,
    pub scale: f64,
}

impl ImageToViewport {
    pub fn new(location: Point, scale: f64) -> Self {
        ImageToViewport { location, scale }
    }

    /// `Transform(Point)`: `(int)(x * scale)` then translated by the location.
    pub fn transform_point(&self, p: Point) -> Point {
        Point::new(
            (f64::from(p.x) * self.scale) as i32 + self.location.x,
            (f64::from(p.y) * self.scale) as i32 + self.location.y,
        )
    }

    /// `Transform(PointF)`: single precision scale and translate, then truncated (`ToPoint`).
    pub fn transform_point_f(&self, p: PointF) -> Point {
        let s = self.scale as f32;
        Point::new(
            (p.x * s + self.location.x as f32) as i32,
            (p.y * s + self.location.y as f32) as i32,
        )
    }

    /// `Transform(IEnumerable<PointF>)`.
    pub fn transform_points_f(&self, points: &[PointF]) -> Vec<Point> {
        points.iter().map(|p| self.transform_point_f(*p)).collect()
    }

    /// `Transform(int)`: `(int)Math.Round(distance * scale)`.
    pub fn transform_distance(&self, distance: i32) -> i32 {
        self.transform_distance_f(distance as f32)
    }

    /// `Transform(float)`: `(int)Math.Round(distance * scale)`, ties to even.
    pub fn transform_distance_f(&self, distance: f32) -> i32 {
        (f64::from(distance) * self.scale).round_ties_even() as i32
    }
}
