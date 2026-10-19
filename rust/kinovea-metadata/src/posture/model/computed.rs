//! Computed points: weighted blends of reference points, possibly nested.

use crate::color::Color;
use kinovea_geometry::PointF;

#[derive(Debug, Clone, PartialEq)]
pub enum WeightedPoint {
    Reference { reference: i32, weight: f32 },
    Computed(ComputedPoint),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedPoint {
    pub name: String,
    pub weight: f32,
    pub color: Color,
    pub symbol: String,
    pub option_group: String,
    pub display_coordinates: bool,
    pub children: Vec<WeightedPoint>,
}

impl WeightedPoint {
    fn weight(&self) -> f32 {
        match self {
            WeightedPoint::Reference { weight, .. } => *weight,
            WeightedPoint::Computed(c) => c.weight,
        }
    }

    fn location(&self, points: &[PointF]) -> PointF {
        match self {
            WeightedPoint::Reference { reference, .. } => points.get(*reference as usize).copied().unwrap_or(PointF::EMPTY),
            WeightedPoint::Computed(c) => c.location(points),
        }
    }
}

impl ComputedPoint {
    /// `ComputeLocation`: a two-point blend (a zero weight is the complement of the other)
    /// or the weighted sum of every child.
    pub fn location(&self, points: &[PointF]) -> PointF {
        if let [p0, p1] = self.children.as_slice() {
            let (mut w0, mut w1) = (p0.weight(), p1.weight());
            if w0 == 0.0 {
                w0 = 1.0 - w1;
            } else if w1 == 0.0 {
                w1 = 1.0 - w0;
            }
            let (l0, l1) = (p0.location(points), p1.location(points));
            return PointF::new(l0.x * w0 + l1.x * w1, l0.y * w0 + l1.y * w1);
        }
        self.children.iter().fold(PointF::EMPTY, |acc, c| {
            let l = c.location(points).scale(c.weight(), c.weight());
            acc.translate(l.x, l.y)
        })
    }
}
