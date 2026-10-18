//! Point-to-line projection with allowed position and the rectangle-centre intersection.

use crate::point::PointF;
use crate::rect::RectangleF;
use crate::shapes::PointLinePosition;
use crate::vector::Vector;

/// Closest point to `c` on line (AB), restricted by `allowed` with a `margin` in pixels.
pub fn closest_point(
    a: PointF,
    b: PointF,
    c: PointF,
    allowed: PointLinePosition,
    margin: i32,
) -> PointF {
    let ac = Vector::from_points(a, c);
    let ab = Vector::from_points(a, b);
    let mut t = ac.dot(ab) / ab.squared();
    let m = margin as f32 / ab.norm();
    t = match allowed {
        PointLinePosition::BeforeSegment => t.min(-m),
        PointLinePosition::BeforeAndOnSegment => t.min(1.0 - m),
        PointLinePosition::OnSegment => t.max(m).min(1.0 - m),
        PointLinePosition::AfterSegment => t.max(1.0 + m),
        PointLinePosition::AfterAndOnSegment => t.max(m),
        PointLinePosition::Anywhere => t,
    };
    a + ab * t
}

/// Intersection of the rectangle border with the ray from its centre towards `end`.
/// `end` is expected outside the rectangle; otherwise `end` itself is returned.
pub fn intersection_rectangle_center(rect: RectangleF, end: PointF) -> PointF {
    let center = rect.center();
    let slope = (end.y - center.y) / (end.x - center.x);
    let inside_y = |y: f32| y > rect.top() && y < rect.bottom();
    let inside_x = |x: f32| x > rect.left() && x < rect.right();
    if end.x <= center.x {
        let y = center.y + slope * (rect.left() - center.x);
        if inside_y(y) {
            return PointF::new(rect.left(), y);
        }
    }
    if end.x >= center.x {
        let y = center.y + slope * (rect.right() - center.x);
        if inside_y(y) {
            return PointF::new(rect.right(), y);
        }
    }
    if end.y <= center.y {
        let x = center.x + (rect.top() - center.y) / slope;
        if inside_x(x) {
            return PointF::new(x, rect.top());
        }
    }
    if end.y >= center.y {
        let x = center.x + (rect.bottom() - center.y) / slope;
        if inside_x(x) {
            return PointF::new(x, rect.bottom());
        }
    }
    end
}
