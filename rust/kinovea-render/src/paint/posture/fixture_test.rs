//! Postures and drawings built by hand for the posture tests. Every element is visible unless a test
//! says otherwise, and the line colour is the background colour (`BindStyle`).

use super::frame::PostureFrame;
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::{Scene, COLOR};
use crate::renderer::Renderer;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::posture::{GenericPosture, PostureOption, Segment, Stroke};
use kinovea_metadata::{Fading, StyleElements, StyleValue};
use uuid::Uuid;

/// Opacity 1 at any timestamp, independent of the player defaults.
pub fn visible() -> Fading {
    Fading {
        use_default: false,
        always_visible: true,
        ..Fading::default()
    }
}

/// Two points at (0, 0) and (100, 0) joined by segment 0.
pub fn two_points() -> GenericPosture {
    GenericPosture {
        point_list: vec![PointF::new(0.0, 0.0), PointF::new(100.0, 0.0)],
        segments: vec![Segment {
            name: "s".into(),
            start: 0,
            end: 1,
            stroke: Stroke::default(),
            option_group: String::new(),
        }],
        ..GenericPosture::default()
    }
}

/// Drawing of `posture` with the background colour set to `COLOR` and visible fading.
pub fn drawing(posture: GenericPosture) -> GenericPostureDrawing {
    let mut d = GenericPostureDrawing::new(Uuid::nil(), Some(posture));
    d.header.style = StyleElements::new(vec![("line color", StyleValue::Color(COLOR))]);
    d.header.fading = visible();
    d
}

pub fn option(key: &str, value: bool) -> PostureOption {
    PostureOption {
        key: key.into(),
        label: key.into(),
        default_value: value,
        value,
        hidden: false,
    }
}

/// Calls of `paint` for `d` at scale 1. Nothing is recorded when the drawing is fully faded.
pub fn record_frame(
    d: &GenericPostureDrawing,
    paint: impl Fn(&PostureFrame, &mut dyn Renderer),
) -> Vec<Call> {
    scene(1.0).record(0, |ctx, r| {
        if let Some(frame) = PostureFrame::new(d, &ctx) {
            paint(&frame, r);
        }
    })
}

pub fn scene(scale: f64) -> Scene {
    Scene::new(crate::viewport::Point::new(0, 0), scale)
}

#[test]
fn two_points_fixture_has_one_segment_between_them() {
    let p = two_points();
    assert_eq!(p.point_list.len(), 2);
    assert_eq!((p.segments[0].start, p.segments[0].end), (0, 1));
}
