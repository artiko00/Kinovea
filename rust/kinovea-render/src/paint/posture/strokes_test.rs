//! Segments, circles and polylines of `DrawingGenericPosture.Draw`. Widths are the stroke widths
//! as they are: the viewport scale does not stretch them. Segments use round caps; the edges drawn
//! after them use flat caps (`NoAnchor`).

use super::fixture_test::{drawing, option, scene, two_points};
use super::frame::PostureFrame;
use super::strokes::{paint_circles, paint_polylines, paint_segments};
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::COLOR;
use crate::renderer::{LineCap, LineJoin, Pen, Transform};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::posture::{
    PostureCircle, PosturePolyline, SegmentLineStyle, Stroke, TRANSPARENT,
};
use kinovea_metadata::style::DashStyle;

fn round_pen(width: f32, dash: DashStyle) -> Pen {
    Pen {
        dash,
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Pen::new(COLOR, width)
    }
}

fn flat_pen(width: f32) -> Pen {
    Pen {
        start_cap: LineCap::Flat,
        end_cap: LineCap::Flat,
        line_join: LineJoin::Round,
        ..Pen::new(COLOR, width)
    }
}

fn record(d: &GenericPostureDrawing, scale: f64) -> Vec<Call> {
    let fixture = scene(scale);
    fixture.record(0, |ctx, r| {
        if let Some(frame) = PostureFrame::new(d, &ctx) {
            paint_segments(&frame, r);
            paint_circles(&frame, r);
            paint_polylines(&frame, r);
        }
    })
}

#[test]
fn segment_is_a_round_capped_line_in_the_background_colour() {
    let d = drawing(two_points());
    assert_eq!(
        record(&d, 1.0),
        vec![Call::StrokeLine(
            PointF::new(0.0, 0.0),
            PointF::new(100.0, 0.0),
            round_pen(2.0, DashStyle::Solid),
            Transform::identity(),
        )]
    );
}

#[test]
fn width_is_not_stretched_by_the_viewport() {
    // Location (0, 0) and scale 2: the end point moves to (200, 0); the width stays 2.
    let d = drawing(two_points());
    let calls = record(&d, 2.0);
    assert_eq!(
        calls,
        vec![Call::StrokeLine(
            PointF::new(0.0, 0.0),
            PointF::new(200.0, 0.0),
            round_pen(2.0, DashStyle::Solid),
            Transform::identity(),
        )]
    );
}

#[test]
fn dashed_segment_uses_the_dash_style() {
    let mut posture = two_points();
    posture.segments[0].stroke.style = SegmentLineStyle::Dash;
    let calls = record(&drawing(posture), 1.0);
    assert!(matches!(&calls[0], Call::StrokeLine(_, _, pen, _) if pen.dash == DashStyle::Dash));
}

#[test]
fn segment_with_arrow_at_its_end_adds_the_head() {
    let mut posture = two_points();
    posture.segments[0].stroke.arrow_end = true;
    let calls = record(&drawing(posture), 1.0);
    assert!(calls.len() > 1);
    assert!(matches!(calls[0], Call::StrokeLine(..)));
}

#[test]
fn segment_of_an_inactive_group_is_skipped() {
    let mut posture = two_points();
    posture.options = vec![option("opt", false)];
    posture.segments[0].option_group = "opt".into();
    assert!(record(&drawing(posture), 1.0).is_empty());
}

#[test]
fn circle_is_centred_on_its_point_with_the_radius_scaled_by_the_viewport() {
    // Centre (50, 50) at scale 2 -> (100, 100); radius 10 -> 20. Box: (100 - 20, 100 - 20, 40, 40).
    let posture = kinovea_metadata::posture::GenericPosture {
        point_list: vec![PointF::new(50.0, 50.0)],
        circles: vec![PostureCircle {
            name: "c".into(),
            center: 0,
            radius: 10,
            stroke: Stroke::default(),
            option_group: String::new(),
        }],
        ..Default::default()
    };
    let calls = record(&drawing(posture), 2.0);
    assert_eq!(
        calls,
        vec![Call::StrokeEllipse(
            RectangleF::new(80.0, 80.0, 40.0, 40.0),
            flat_pen(2.0),
            Transform::identity(),
        )]
    );
}

#[test]
fn polyline_is_a_curve_through_its_points_with_tension_one_half() {
    let posture = kinovea_metadata::posture::GenericPosture {
        point_list: vec![
            PointF::new(0.0, 0.0),
            PointF::new(100.0, 0.0),
            PointF::new(100.0, 100.0),
        ],
        polylines: vec![PosturePolyline {
            name: "p".into(),
            points: vec![0, 1, 2],
            stroke: Stroke {
                color: TRANSPARENT,
                ..Stroke::default()
            },
            option_group: String::new(),
        }],
        ..Default::default()
    };
    let calls = record(&drawing(posture), 1.0);
    assert_eq!(
        calls,
        vec![Call::StrokeCurve(
            vec![
                PointF::new(0.0, 0.0),
                PointF::new(100.0, 0.0),
                PointF::new(100.0, 100.0),
            ],
            0.5,
            flat_pen(2.0),
            Transform::identity(),
        )]
    );
}
