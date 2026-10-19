//! `Transform` overloads for sizes, rectangles, rectangle lists and quadrilaterals.
use super::{ImageToViewport, Point, Rectangle};
use kinovea_geometry::{Quadrilateral, RectangleF, Size, SizeF};

impl ImageToViewport {
    /// `Transform(Size)`: converted to `SizeF` and scaled as in `Transform(SizeF)`.
    pub fn transform_size(&self, size: Size) -> Size {
        self.transform_size_f(SizeF::new(size.width as f32, size.height as f32))
    }

    /// `Transform(SizeF)`: `Size.Scale(float)` then `ToSize()`, both truncating.
    pub fn transform_size_f(&self, size: SizeF) -> Size {
        let s = self.scale as f32;
        Size::new((size.width * s) as i32, (size.height * s) as i32)
    }

    /// `Transform(Rectangle)`: location through `Transform(Point)`, size through `Transform(Size)`.
    pub fn transform_rect(&self, r: Rectangle) -> Rectangle {
        let location = self.transform_point(Point::new(r.x, r.y));
        let size = self.transform_size(Size::new(r.width, r.height));
        join(location, size)
    }

    /// `Transform(RectangleF)`: location through `Transform(PointF)`, size through `Transform(SizeF)`.
    pub fn transform_rect_f(&self, r: RectangleF) -> Rectangle {
        let location = self.transform_point_f(r.location());
        let size = self.transform_size_f(r.size());
        join(location, size)
    }

    /// `Transform(List<Rectangle>)`.
    pub fn transform_rects(&self, rects: &[Rectangle]) -> Vec<Rectangle> {
        rects.iter().map(|r| self.transform_rect(*r)).collect()
    }

    /// `Transform(QuadrilateralF)`: every vertex keeps the whole-pixel result of `Transform(PointF)`.
    pub fn transform_quad(&self, q: Quadrilateral) -> Quadrilateral {
        let [a, b, c, d] = q.vertices.map(|v| self.transform_point_f(v).to_point_f());
        Quadrilateral::new(a, b, c, d)
    }
}

fn join(location: Point, size: Size) -> Rectangle {
    Rectangle::new(location.x, location.y, size.width, size.height)
}
