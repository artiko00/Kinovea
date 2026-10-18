//! Square calibration quad built from a line and an axis mode (`CalibratorPlane.MakeQuad`).

use crate::calibration_axis::CalibrationAxis;
use kinovea_geometry::helpers::distance;
use kinovea_geometry::{PointF, Quadrilateral};

/// Vertex D is `start`; the line becomes edge D→C (+X) or D→A (+Y); `ImageAxes` keeps only the
/// length and aligns the square with the image.
pub fn make_quad(start: PointF, end: PointF, axis: CalibrationAxis) -> Quadrilateral {
    match axis {
        CalibrationAxis::LineHorizontal => square_from_dc(start, end),
        CalibrationAxis::LineVertical => {
            let (d, a) = (start, end);
            let c = PointF::new(d.x + (d.y - a.y), d.y + (a.x - d.x));
            let b = PointF::new(a.x + (c.x - d.x), a.y + (c.y - d.y));
            Quadrilateral::new(a, b, c, d)
        }
        CalibrationAxis::ImageAxes => {
            let length = distance(start, end);
            square_from_dc(start, PointF::new(start.x + length, start.y))
        }
    }
}

fn square_from_dc(d: PointF, c: PointF) -> Quadrilateral {
    let a = PointF::new(d.x + (c.y - d.y), d.y - (c.x - d.x));
    let b = PointF::new(a.x + (c.x - d.x), a.y + (c.y - d.y));
    Quadrilateral::new(a, b, c, d)
}
