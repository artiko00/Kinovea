//! Handle points and computed points of `DrawingGenericPosture.Draw`. A point is only drawn when a
//! handle sits on it; a handle at the end of an arrow is the arrow, so its ellipse is skipped.

use super::fixture_test::{drawing, option, record_frame, two_points};
use super::points::{paint_computed_points, paint_points};
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::COLOR;
use crate::renderer::{Brush, Transform};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::GenericPostureDrawing;
use kinovea_metadata::posture::{GenericPosture, Handle, HandleType, PosturePoint, TRANSPARENT};
use kinovea_metadata::Color;

const GREEN: Color = Color::rgb(0, 255, 0);
const RED: Color = Color::rgb(255, 0, 0);

fn handle(reference: i32, kind: HandleType, color: Color) -> Handle {
    Handle {
        name: "h".into(),
        kind,
        color,
        reference,
        trackable: false,
        option_group: String::new(),
        grab_point: PointF::EMPTY,
        constraint: None,
        impacts: Vec::new(),
    }
}

fn point(color: Color) -> PosturePoint {
    PosturePoint {
        name: "p".into(),
        value: PointF::EMPTY,
        color,
    }
}

fn record(d: &GenericPostureDrawing) -> Vec<Call> {
    record_frame(d, |f, r| {
        paint_points(f, r);
        paint_computed_points(f, r);
    })
}

fn fill_at(c: (f32, f32), color: Color) -> Call {
    Call::FillEllipse(
        RectangleF::new(c.0 - 3.0, c.1 - 3.0, 6.0, 6.0),
        Brush::solid(color),
        Transform::identity(),
    )
}

fn with_handle(handle: Handle) -> GenericPosture {
    GenericPosture {
        point_list: vec![PointF::new(50.0, 50.0)],
        points: vec![point(TRANSPARENT)],
        handles: vec![handle],
        ..GenericPosture::default()
    }
}

#[test]
fn handle_point_is_a_filled_ellipse_of_radius_three_in_the_background_colour() {
    let d = drawing(with_handle(handle(0, HandleType::Point, TRANSPARENT)));
    assert_eq!(record(&d), vec![fill_at((50.0, 50.0), COLOR)]);
}

#[test]
fn point_colour_wins_over_handle_colour() {
    let mut posture = with_handle(handle(0, HandleType::Point, RED));
    posture.points[0].color = GREEN;
    assert_eq!(
        record(&drawing(posture)),
        vec![fill_at((50.0, 50.0), GREEN)]
    );
}

#[test]
fn handle_colour_is_used_when_the_point_is_transparent() {
    let posture = with_handle(handle(0, HandleType::Point, RED));
    assert_eq!(record(&drawing(posture)), vec![fill_at((50.0, 50.0), RED)]);
}

#[test]
fn point_at_the_end_of_an_arrow_is_not_drawn() {
    let mut posture = two_points();
    posture.segments[0].stroke.arrow_begin = true;
    posture.handles = vec![handle(0, HandleType::Point, TRANSPARENT)];
    posture.points = vec![point(TRANSPARENT), point(TRANSPARENT)];
    let calls = record(&drawing(posture));
    assert!(calls.iter().all(|c| !matches!(c, Call::FillEllipse(..))));
}

#[test]
fn handle_of_an_inactive_group_is_skipped() {
    let mut posture = with_handle(handle(0, HandleType::Point, TRANSPARENT));
    posture.options = vec![option("opt", false)];
    posture.handles[0].option_group = "opt".into();
    assert!(record(&drawing(posture)).is_empty());
}
