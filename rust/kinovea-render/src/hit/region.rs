//! Even-odd containment of a path's figures, the fill query behind `Region.IsVisible`.
use super::path::{Figure, GraphicsPath};
use kinovea_geometry::PointF;

impl GraphicsPath {
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
    fn segments(&self, close: bool) -> impl Iterator<Item = (PointF, PointF)> + '_ {
        let pts = &self.points;
        let open = pts.windows(2).map(|w| (w[0], w[1]));
        let closing = (close && pts.len() > 1).then(|| (pts[pts.len() - 1], pts[0]));
        open.chain(closing)
    }
}
