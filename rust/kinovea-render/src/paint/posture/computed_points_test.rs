//! Computed points of `DrawComputedPoints`: a blend of two references, drawn as a flat ring of width 2,
//! or as its symbol when it has one.

use super::fixture_test::{drawing, record_frame};
use super::points::paint_computed_points;
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::COLOR;
use crate::renderer::{LineCap, LineJoin, Pen, Transform};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::posture::{ComputedPoint, GenericPosture, WeightedPoint, TRANSPARENT};

fn record(d: &kinovea_metadata::drawing::GenericPostureDrawing) -> Vec<Call> {
    record_frame(d, paint_computed_points)
}

#[test]
fn computed_point_is_a_flat_ellipse_at_the_blend_of_its_references() {
    // Weights 0.5 and 0 (the complement of 0.5): (0, 0) * 0.5 + (100, 0) * 0.5 = (50, 0).
    let posture = GenericPosture {
        point_list: vec![PointF::new(0.0, 0.0), PointF::new(100.0, 0.0)],
        computed_points: vec![ComputedPoint {
            name: "c".into(),
            weight: 1.0,
            color: TRANSPARENT,
            symbol: String::new(),
            option_group: String::new(),
            display_coordinates: false,
            children: vec![
                WeightedPoint::Reference {
                    reference: 0,
                    weight: 0.5,
                },
                WeightedPoint::Reference {
                    reference: 1,
                    weight: 0.0,
                },
            ],
        }],
        ..GenericPosture::default()
    };
    let pen = Pen {
        start_cap: LineCap::Flat,
        end_cap: LineCap::Flat,
        line_join: LineJoin::Round,
        ..Pen::new(COLOR, 2.0)
    };
    assert_eq!(
        record(&drawing(posture)),
        vec![Call::StrokeEllipse(
            RectangleF::new(47.0, -3.0, 6.0, 6.0),
            pen,
            Transform::identity(),
        )]
    );
}

#[test]
fn computed_point_with_a_symbol_is_labelled_instead() {
    let posture = GenericPosture {
        point_list: vec![PointF::new(0.0, 0.0), PointF::new(100.0, 0.0)],
        computed_points: vec![ComputedPoint {
            name: "c".into(),
            weight: 1.0,
            color: TRANSPARENT,
            symbol: "X".into(),
            option_group: String::new(),
            display_coordinates: false,
            children: vec![
                WeightedPoint::Reference {
                    reference: 0,
                    weight: 0.5,
                },
                WeightedPoint::Reference {
                    reference: 1,
                    weight: 0.5,
                },
            ],
        }],
        ..GenericPosture::default()
    };
    let calls = record(&drawing(posture));
    assert!(calls
        .iter()
        .any(|c| matches!(c, Call::DrawText(t, ..) if t == "X")));
}
