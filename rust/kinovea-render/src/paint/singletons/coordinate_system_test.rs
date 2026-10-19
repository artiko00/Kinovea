//! `DrawingCoordinateSystem.Draw` on the flat plane fixture (grid derived in `grid_finder_test`): the
//! axes in the line colour, then the grid dashed at alpha 127, then graduations. The "0" graduation sits
//! at image (20, 110) in the BottomRight position, so its label starts at (20 + 8, 110 + 8).

use super::coordinate_system::paint_coordinate_system;
use super::fixture_test::{crossed_plane, flat_plane};
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::{Scene, COLOR};
use crate::paint::{Distorter, PaintContext};
use crate::renderer::{LineCap, LineJoin, Pen, Transform};
use crate::viewport::Point;
use kinovea_geometry::PointF;
use kinovea_metadata::singletons::CoordinateSystem;
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::{Color, Header, StyleElements, StyleValue};

/// Background pen of `GetBackgroundPen(alpha)`: width 1, round caps and joins.
fn pen(alpha: u8, dash: DashStyle) -> Pen {
    Pen {
        color: Color { a: alpha, ..COLOR },
        dash,
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Pen::new(COLOR, 1.0)
    }
}

fn stroke(a: (f32, f32), b: (f32, f32), pen: Pen) -> Call {
    Call::StrokeLine(
        PointF::new(a.0, a.1),
        PointF::new(b.0, b.1),
        pen,
        Transform::identity(),
    )
}

fn visible_system() -> CoordinateSystem {
    CoordinateSystem {
        visible: true,
        header: Header::new(
            "",
            StyleElements::new(vec![("line color", StyleValue::Color(COLOR))]),
        ),
        ..CoordinateSystem::default()
    }
}

fn scene(calibration: kinovea_measurement::CalibrationHelper) -> Scene {
    let mut scene = Scene::new(Point::new(0, 0), 1.0);
    scene.metadata.calibration = calibration;
    scene
}

fn paint(scene: &Scene, d: &CoordinateSystem) -> Vec<Call> {
    scene.record(0, |ctx, r| paint_coordinate_system(d, &ctx, r))
}

#[test]
fn hidden_system_paints_nothing() {
    let d = CoordinateSystem::default();
    assert!(paint(&scene(flat_plane()), &d).is_empty());
}

#[test]
fn invalid_plane_paints_nothing() {
    assert!(paint(&scene(crossed_plane()), &visible_system()).is_empty());
}

/// The lines are the infinite lines clipped to the image (CoordinateSystemGridFinder.cs:167 and :240,
/// LiangBarsky.cs:83-86): the axes run from the bottom edge (y 120) to the top edge (y 0), the
/// horizontal one the full width. Clipped ends carry double rounding (about 1e-14 px), so the calls
/// are compared at 1e-3 px.
#[test]
fn axes_come_first_then_the_dashed_grid_at_half_alpha() {
    let mut d = visible_system();
    d.show_graduations = false;
    let expected = [
        stroke((20.0, 120.0), (20.0, 0.0), pen(255, DashStyle::Solid)),
        stroke((0.0, 110.0), (240.0, 110.0), pen(255, DashStyle::Solid)),
        stroke((0.0, 120.0), (0.0, 0.0), pen(127, DashStyle::Dash)),
        stroke((40.0, 120.0), (40.0, 0.0), pen(127, DashStyle::Dash)),
    ];
    assert_eq!(settled(paint(&scene(flat_plane()), &d))[..4], expected[..]);
}

/// Rounds the points of the recorded lines to 1e-3 px.
fn settled(calls: Vec<Call>) -> Vec<Call> {
    let snap = |p: PointF| {
        PointF::new(
            (p.x * 1000.0).round() / 1000.0,
            (p.y * 1000.0).round() / 1000.0,
        )
    };
    calls
        .into_iter()
        .map(|c| match c {
            Call::StrokeLine(a, b, pen, t) => Call::StrokeLine(snap(a), snap(b), pen, t),
            other => other,
        })
        .collect()
}

/// Moves every point one pixel of the image right of its place (`Distorter`).
struct Shift;

impl Distorter for Shift {
    fn distort(&self, p: PointF) -> PointF {
        PointF::new(p.x + 10.0, p.y)
    }
}

#[test]
fn origin_graduation_is_anchored_on_its_distorted_point() {
    // The origin graduation sits at (20, 110); distorted by +10 in x it is at (30, 110), and the
    // BottomRight label starts at (30 + 8, 110 + 8).
    let fixture = scene(flat_plane());
    let d = visible_system();
    let calls = fixture.record(0, |ctx, r| {
        let ctx = PaintContext {
            distorter: Some(&Shift),
            ..ctx
        };
        paint_coordinate_system(&d, &ctx, r)
    });
    let first_label = calls.iter().find_map(|c| match c {
        Call::DrawText(text, _, _, origin, _) => Some((text.clone(), *origin)),
        _ => None,
    });
    assert_eq!(
        first_label,
        Some(("0".to_string(), PointF::new(38.0, 118.0)))
    );
}

#[test]
fn origin_graduation_is_labelled_zero_at_the_bottom_right_of_its_point() {
    let calls = paint(&scene(flat_plane()), &visible_system());
    let first_label = calls.iter().find_map(|c| match c {
        Call::DrawText(text, _, _, origin, _) => Some((text.clone(), *origin)),
        _ => None,
    });
    assert_eq!(
        first_label,
        Some(("0".to_string(), PointF::new(28.0, 118.0)))
    );
}
