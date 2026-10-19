//! Distance to a path and even-odd containment, the two queries behind `Region.IsVisible`.
use super::path::{Figure, GraphicsPath};
use kinovea_geometry::{
    helpers::{closest_point, distance},
    PointF, PointLinePosition,
};

impl GraphicsPath {
    /// Distance from `p` to the nearest segment; closed figures include their closing edge.
    pub(super) fn distance_to(&self, p: PointF) -> f32 {
        self.figures
            .iter()
            .flat_map(|f| f.segments(f.closed))
            .map(|(a, b)| distance_to_segment(p, a, b))
            .fold(f32::INFINITY, f32::min)
    }

    /// Alternate (even-odd) fill over every figure, which are all implicitly closed.
    pub(super) fn contains_even_odd(&self, p: PointF) -> bool {
        let crossings = self
            .figures
            .iter()
            .flat_map(|f| f.segments(true))
            .filter(|(a, b)| (a.y > p.y) != (b.y > p.y))
            .filter(|(a, b)| p.x < a.x + (p.y - a.y) * (b.x - a.x) / (b.y - a.y))
            .count();
        crossings % 2 == 1
    }
}

impl Figure {
    /// Segments between consecutive points, plus the closing edge when `close` is set.
    /// A single point yields a zero-length segment so that it still has a distance.
    fn segments(&self, close: bool) -> impl Iterator<Item = (PointF, PointF)> + '_ {
        let pts = &self.points;
        let open = pts.windows(2).map(|w| (w[0], w[1]));
        let single = (pts.len() == 1).then(|| (pts[0], pts[0]));
        let closing = (close && pts.len() > 1).then(|| (pts[pts.len() - 1], pts[0]));
        open.chain(single).chain(closing)
    }
}

fn distance_to_segment(p: PointF, a: PointF, b: PointF) -> f32 {
    if a == b {
        return distance(p, a);
    }
    distance(closest_point(a, b, p, PointLinePosition::OnSegment, 0), p)
}
