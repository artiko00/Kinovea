//! Positions, distances and angles of the drawings of one keyframe, each list sorted by name
//! (the keyframe loop of `Metadata.CollectMeasuredData`). Postures are not collected yet.

use crate::measure::{AngleFlags, Scope};
use crate::measured_data::{Position, Scalar};
use kinovea_geometry::helpers::distance;
use kinovea_metadata::Drawing;

/// The measures of one keyframe, already stamped with the keyframe time.
#[derive(Debug, Default, PartialEq)]
pub struct KeyframeMeasures {
    pub positions: Vec<Position>,
    pub distances: Vec<Scalar>,
    pub angles: Vec<Scalar>,
}

pub fn collect_keyframe(scope: &Scope, drawings: &[Drawing], time: f32) -> KeyframeMeasures {
    let mut out = KeyframeMeasures::default();
    for drawing in drawings {
        match drawing {
            Drawing::CrossMark(c) => out.positions.push(Position {
                name: c.header.name.clone(),
                time,
                point: scope.point(c.center),
            }),
            Drawing::Line(l) => out.distances.push(Scalar {
                name: l.header.name.clone(),
                time,
                value: distance(scope.point(l.a), scope.point(l.b)),
            }),
            Drawing::Angle(a) => {
                let flags = AngleFlags {
                    signed: a.signed,
                    counter_clockwise: a.counter_clockwise,
                    supplementary: a.supplementary,
                };
                out.angles.push(Scalar {
                    name: a.header.name.clone(),
                    time,
                    value: scope.angle(a.o, a.a, a.b, flags),
                });
            }
            _ => {}
        }
    }
    out.positions.sort_by(|a, b| a.name.cmp(&b.name));
    out.distances.sort_by(|a, b| a.name.cmp(&b.name));
    out.angles.sort_by(|a, b| a.name.cmp(&b.name));
    out
}
