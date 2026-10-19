//! Read-outs of distances and positions: the label sits on a background at the middle of the
//! segment (offset 15 down) or at the point (offset 20 up), centred on its measured size.

use super::fixture_test::{drawing, scene, two_points};
use super::frame::PostureFrame;
use super::texts::{paint_distances, paint_positions};
use crate::backend::recording::{Call, RecordingRenderer};
use crate::renderer::{Font, Renderer};
use crate::style::StyleData;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::posture::{GenericPosture, Measure, TRANSPARENT};

fn font() -> Font {
    StyleData::default().font(1.0)
}

fn label_origin(calls: &[Call]) -> Option<(String, PointF)> {
    calls.iter().find_map(|c| match c {
        Call::DrawText(text, _, _, origin, _) => Some((text.clone(), *origin)),
        _ => None,
    })
}

fn record_distances(d: &GenericPostureDrawing) -> Vec<Call> {
    scene(1.0).record(0, |ctx, r| {
        if let Some(frame) = PostureFrame::new(d, &ctx) {
            paint_distances(&frame, r);
        }
    })
}

fn record_positions(d: &GenericPostureDrawing) -> Vec<Call> {
    scene(1.0).record(0, |ctx, r| {
        if let Some(frame) = PostureFrame::new(d, &ctx) {
            paint_positions(&frame, r);
        }
    })
}

fn measure(text: &str) -> PointF {
    let size = RecordingRenderer::new(1.0, 1.0).measure_text(text, &font());
    PointF::new(size.width, size.height)
}

#[test]
fn distance_label_is_prefixed_by_its_symbol_and_sits_below_the_middle() {
    let mut posture: GenericPosture = two_points();
    posture.distances = vec![Measure {
        name: "d".into(),
        point1: 0,
        point2: 1,
        symbol: "d".into(),
        color: TRANSPARENT,
        option_group: String::new(),
    }];
    let calls = record_distances(&drawing(posture));
    let (text, origin) = label_origin(&calls).expect("a read-out");
    assert!(text.starts_with("d = "));
    // Middle (50, 0); offset (0, 15); the label centred on it.
    let size = measure(&text);
    assert_eq!(
        origin,
        PointF::new(50.0 - size.x / 2.0, 0.0 - size.y / 2.0 + 15.0)
    );
}

#[test]
fn position_label_sits_above_its_point() {
    let mut posture = two_points();
    posture.positions = vec![Measure {
        name: "p".into(),
        point1: 1,
        point2: 0,
        symbol: String::new(),
        color: TRANSPARENT,
        option_group: String::new(),
    }];
    let calls = record_positions(&drawing(posture));
    let (text, origin) = label_origin(&calls).expect("a read-out");
    let size = measure(&text);
    // Point (100, 0); offset (0, -20).
    assert_eq!(
        origin,
        PointF::new(100.0 - size.x / 2.0, 0.0 - size.y / 2.0 - 20.0)
    );
}
