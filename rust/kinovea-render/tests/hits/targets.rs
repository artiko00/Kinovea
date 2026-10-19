//! Every drawing of a case, in the oracle's order (`WriteHits`), with what its hit test is.

use super::table::DrawingKey;
use kinovea_geometry::PointF;
use kinovea_metadata::multi::{NumberSequence, Spotlights};
use kinovea_metadata::singletons::{CoordinateSystem, TestGrid};
use kinovea_metadata::{Drawing, Metadata};
use kinovea_render::hit::context::{Hit, HitContext};
use kinovea_render::hit::drawing::{hit_test, is_ported};
use kinovea_render::hit::multi::{hit_numbers, hit_spotlights};
use kinovea_render::hit::singletons::{hit_coordinate_system, hit_test_grid};

/// What a probe is tested against: an attached or detached drawing, or one of the singletons.
pub enum Target<'m> {
    Drawing(&'m Drawing),
    Spotlights(&'m Spotlights),
    Numbers(&'m NumberSequence),
    CoordinateSystem(&'m CoordinateSystem),
    TestGrid(&'m TestGrid),
}

impl Target<'_> {
    /// False when the kind has no port: it misses everywhere.
    pub fn is_ported(&self) -> bool {
        match self {
            Target::Drawing(d) => is_ported(d),
            _ => true,
        }
    }

    pub fn hit(&self, p: PointF, ctx: &HitContext) -> Hit {
        match self {
            Target::Drawing(d) => hit_test(d, p, ctx),
            Target::Spotlights(s) => hit_spotlights(s, p, ctx),
            Target::Numbers(n) => hit_numbers(n, p, ctx),
            Target::CoordinateSystem(c) => hit_coordinate_system(c, p, ctx),
            Target::TestGrid(g) => hit_test_grid(g, p, ctx),
        }
    }
}

/// One drawing of a case: its oracle key, the time it is probed at, and its target.
pub struct Entry<'m> {
    pub key: DrawingKey,
    pub timestamp: i64,
    pub target: Target<'m>,
}

/// Keyframe drawings (`kN`) at their keyframe time, then chronos and tracks at `detached`, then the
/// singletons at `detached` in `SingletonDrawingsManager` order, whether or not they have any probe.
pub fn entries(m: &Metadata, detached: i64) -> Vec<Entry<'_>> {
    let mut out = Vec::new();
    for (k, keyframe) in m.keyframes.iter().enumerate() {
        for (i, d) in keyframe.drawings.iter().enumerate() {
            out.push(entry(&format!("k{k}"), i, d, keyframe.timestamp));
        }
    }
    for (i, d) in m.chronos.iter().enumerate() {
        out.push(entry("chronos", i, d, detached));
    }
    for (i, d) in m.tracks.iter().enumerate() {
        out.push(entry("tracks", i, d, detached));
    }
    let singletons = [
        ("Spotlights", Target::Spotlights(&m.spotlights)),
        ("AutoNumbers", Target::Numbers(&m.number_sequence)),
        (
            "CoordinateSystem",
            Target::CoordinateSystem(&m.coordinate_system),
        ),
        ("TestGrid", Target::TestGrid(&m.test_grid)),
    ];
    for (i, (element, target)) in singletons.into_iter().enumerate() {
        let key = DrawingKey {
            container: "singletons".into(),
            index: i,
            element: element.into(),
        };
        out.push(Entry {
            key,
            timestamp: detached,
            target,
        });
    }
    out
}

fn entry<'m>(container: &str, index: usize, d: &'m Drawing, timestamp: i64) -> Entry<'m> {
    Entry {
        key: DrawingKey {
            container: container.into(),
            index,
            element: element_name(d).into(),
        },
        timestamp,
        target: Target::Drawing(d),
    }
}

/// The element name of the oracle (`KvaArea.ElementName`, the class name of the drawing).
pub fn element_name(d: &Drawing) -> &'static str {
    match d {
        Drawing::Line(_) => "Line",
        Drawing::Angle(_) => "Angle",
        Drawing::CrossMark(_) => "CrossMark",
        Drawing::Circle(_) => "Circle",
        Drawing::Rectangle(_) => "Rectangle",
        Drawing::Pencil(_) => "Pencil",
        Drawing::Polyline(_) => "Polyline",
        Drawing::TimePolyline(_) => "TimePolyline",
        Drawing::Label(_) => "Label",
        Drawing::Plane(_) => "Plane",
        Drawing::DistortionGrid(_) => "DistortionGrid",
        Drawing::TimeSegment(_) => "TimeSegment",
        Drawing::Chrono(_) => "Chrono",
        Drawing::ChronoMulti(_) => "ChronoMulti",
        Drawing::Counter(_) => "Counter",
        Drawing::Track(_) => "Track",
        Drawing::Bitmap(_) => "Bitmap",
        Drawing::Svg(_) => "Svg",
        Drawing::GenericPostureDrawing(_) => "GenericPosture",
    }
}

#[cfg(test)]
#[path = "targets_test.rs"]
mod targets_test;
