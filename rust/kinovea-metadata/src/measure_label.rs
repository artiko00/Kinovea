//! The small label attached to measurable drawings (`MiniLabel`) and the measurement it shows.

use crate::kva_io::{point_leaf, read_point, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;

crate::text_enum!(
    /// Which quantity a drawing's label displays (`MeasureLabelType`).
    MeasureLabelType [
        None, Name, Clock, RelativeTime, Frame, Position, RelativePosition, Distance, TravelDistance,
        HorizontalTravel, VerticalTravel, AngleToHorizontal, AngleToVertical, Speed, HorizontalVelocity,
        VerticalVelocity, Acceleration, HorizontalAcceleration, VerticalAcceleration, Center, Radius,
        Diameter, Circumference, Count, CountReverse, CadenceInstant, CadenceAverage, CadenceVariation,
        PeriodInstant, PeriodAverage, PointDeviation, LineDeviation, HorizontalDeviation,
        VerticalDeviation, CircularDeviation, TotalHorizontalDisplacement, TotalVerticalDisplacement
    ]
);

/// Label position relative to its attach point, plus the time it refers to.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MeasureLabel {
    /// Top-left of the label background in image space.
    pub position: PointF,
    pub timestamp: i64,
    /// Point the label is attached to (not persisted); `write_kva_attached` moves the label by
    /// the difference between the saved reference and this point, as the original does.
    pub attach: PointF,
}

impl MeasureLabel {
    pub fn read_kva(e: &Element, scale: Scale) -> MeasureLabel {
        let mut label = MeasureLabel::default();
        for c in &e.children {
            match c.name.as_str() {
                "SpacePosition" => label.position = read_point(c, scale),
                "TimePosition" => label.timestamp = c.text.trim().parse().unwrap_or(0),
                _ => {}
            }
        }
        label
    }

    /// `SpacePosition`, `TimePosition`.
    pub fn write_kva(&self) -> Vec<Element> {
        vec![point_leaf("SpacePosition", self.position), Element::leaf("TimePosition", &self.timestamp.to_string())]
    }

    /// Writes the label as if attached to `reference` instead of the current attach point.
    pub fn write_kva_attached(&self, reference: PointF) -> Vec<Element> {
        let moved = MeasureLabel {
            position: self.position.translate(reference.x - self.attach.x, reference.y - self.attach.y),
            ..*self
        };
        moved.write_kva()
    }

    pub fn set_attach(&mut self, attach: PointF) {
        self.attach = attach;
    }

    /// Hash of the position relative to the attach point.
    pub fn content_hash(&self) -> i32 {
        let dx = self.position.x - self.attach.x;
        let dy = self.position.y - self.attach.y;
        (dx.to_bits() as i32) ^ (dy.to_bits() as i32)
    }
}

/// `<MeasureLabel>` wrapper.
pub fn measure_label_block(children: Vec<Element>) -> Element {
    let mut e = Element::new("MeasureLabel");
    e.children = children;
    e
}
