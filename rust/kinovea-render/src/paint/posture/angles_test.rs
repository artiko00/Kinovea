//! `DrawAngles` on the origin (0, 0) with legs (100, 0) and (0, 100) and radius 20, at scale 1.
//! The sweep runs from 0 to 90 degrees; the calibrated value is pi/2, read as "90" in degrees.

use super::angles::paint_angles;
use super::fixture_test::{drawing, option, scene};
use super::frame::{edge_pen, PostureFrame};
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::COLOR;
use crate::renderer::{Brush, Pen, Transform};
use crate::style::colors::with_alpha;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::posture::{GenericPosture, PostureAngle, TRANSPARENT};
use kinovea_metadata::style::DashStyle;

fn angle(option_group: &str) -> PostureAngle {
    PostureAngle {
        name: "a".into(),
        origin: 0,
        leg1: 1,
        leg2: 2,
        text_distance: 40,
        radius: 20,
        tenth: false,
        color: TRANSPARENT,
        symbol: String::new(),
        signed: false,
        ccw: false,
        supplementary: false,
        option_group: option_group.into(),
    }
}

fn with_angle(angle: PostureAngle) -> GenericPosture {
    GenericPosture {
        point_list: vec![
            PointF::new(0.0, 0.0),
            PointF::new(100.0, 0.0),
            PointF::new(0.0, 100.0),
        ],
        angles: vec![angle],
        ..GenericPosture::default()
    }
}

fn record(d: &GenericPostureDrawing) -> Vec<Call> {
    let fixture = scene(1.0);
    fixture.record(0, |ctx, r| {
        if let Some(frame) = PostureFrame::new(d, &ctx) {
            paint_angles(&frame, r);
        }
    })
}

fn sector_box() -> RectangleF {
    RectangleF::new(-20.0, -20.0, 40.0, 40.0)
}

fn arc_pen() -> Pen {
    edge_pen(COLOR, 2.0, DashStyle::Solid)
}

#[test]
fn angle_draws_its_pie_then_its_arc_in_the_fill_and_edge_colours() {
    let d = drawing(with_angle(angle("")));
    let calls = record(&d);
    let fill = Brush::solid(with_alpha(COLOR, 92));
    assert_eq!(
        calls[0],
        Call::FillPie(sector_box(), 0.0, 90.0, fill, Transform::identity())
    );
    assert_eq!(
        calls[1],
        Call::StrokeArc(sector_box(), 0.0, 90.0, arc_pen(), Transform::identity())
    );
}

#[test]
fn value_is_read_in_degrees_with_the_default_precision() {
    let calls = record(&drawing(with_angle(angle(""))));
    let label = calls.iter().find_map(|c| match c {
        Call::DrawText(text, ..) => Some(text.clone()),
        _ => None,
    });
    assert!(label.expect("a read-out").starts_with("90 "));
}

#[test]
fn angle_with_coincident_legs_is_not_drawn() {
    let mut posture = with_angle(angle(""));
    posture.point_list[1] = PointF::new(0.0, 0.0);
    assert!(record(&drawing(posture)).is_empty());
}

#[test]
fn angle_of_an_inactive_group_is_skipped() {
    let mut posture = with_angle(angle("opt"));
    posture.options = vec![option("opt", false)];
    assert!(record(&drawing(posture)).is_empty());
}
