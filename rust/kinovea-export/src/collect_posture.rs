//! Positions, distances and angles of a generic posture (`DrawingGenericPosture.CollectMeasuredData*`).
//! A part takes the drawing name, followed by the part name when the part has one.

use crate::collect_drawings::KeyframeMeasures;
use crate::measure::{AngleFlags, Scope};
use crate::measured_data::{Position, Scalar};
use kinovea_geometry::helpers::distance;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::GenericPostureDrawing;

/// Appends the measures of `drawing` to `out`, stamped with the keyframe `time`. A drawing whose
/// tool is not shipped has no posture and contributes nothing.
pub fn collect_posture(
    scope: &Scope,
    drawing: &GenericPostureDrawing,
    time: f32,
    out: &mut KeyframeMeasures,
) {
    let Some(posture) = &drawing.posture else {
        return;
    };
    let points = &posture.point_list;
    let at = |i: i32| {
        usize::try_from(i)
            .ok()
            .and_then(|i| points.get(i))
            .copied()
            .unwrap_or(PointF::EMPTY)
    };
    let name = |part: &str| {
        if part.is_empty() {
            drawing.header.name.clone()
        } else {
            format!("{} - {part}", drawing.header.name)
        }
    };
    for p in posture.positions.iter().filter(|p| p.point1 >= 0) {
        out.positions.push(Position {
            name: name(&p.name),
            time,
            point: scope.point(at(p.point1)),
        });
    }
    for c in &posture.computed_points {
        out.positions.push(Position {
            name: name(&c.name),
            time,
            point: scope.point(c.location(points)),
        });
    }
    for d in &posture.distances {
        let value = distance(scope.point(at(d.point1)), scope.point(at(d.point2)));
        out.distances.push(Scalar {
            name: name(&d.name),
            time,
            value,
        });
    }
    for a in &posture.angles {
        let flags = AngleFlags {
            signed: a.signed,
            counter_clockwise: a.ccw,
            supplementary: a.supplementary,
        };
        let value = scope.angle(at(a.origin), at(a.leg1), at(a.leg2), flags);
        out.angles.push(Scalar {
            name: name(&a.name),
            time,
            value,
        });
    }
}
