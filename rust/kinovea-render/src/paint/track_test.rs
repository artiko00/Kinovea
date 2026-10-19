use crate::backend::recording::Call;
use crate::paint::helpers::support_test::{surface, Scene};
use crate::paint::track::paint_track;
use crate::renderer::{Brush, LineCap, LineJoin, Pen, Transform};
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::Track;
use kinovea_metadata::style::{StyleValue, TrackShape};
use kinovea_metadata::{Color, StyleElements};

const RED: Color = Color::argb(255, 255, 0, 0);

/// Four points on the x axis at t = 0, 10, 20, 30; the track is visible from its first point.
pub(crate) fn track() -> Track {
    let mut d = Track::new(vec![
        TimedPoint::new(0.0, 0.0, 0),
        TimedPoint::new(10.0, 0.0, 10),
        TimedPoint::new(20.0, 0.0, 20),
        TimedPoint::new(30.0, 0.0, 30),
    ]);
    d.header.style = StyleElements::new(vec![
        ("color", StyleValue::Color(Color::rgb(255, 0, 0))),
        ("line size", StyleValue::LineSize(1)),
        ("track shape", StyleValue::TrackShape(TrackShape::SOLID)),
        ("font size", StyleValue::FontSize(12)),
    ]);
    d.header.fading.average_timestamps_per_frame = 10.0;
    d
}

/// Trajectory pen of `DrawingTrack.DrawTrajectory`: the track colour at `alpha`, 1 px, round caps.
fn trail(alpha: u8) -> Pen {
    Pen {
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Pen::new(Color::argb(alpha, 255, 0, 0), 1.0)
    }
}

/// Cursor cross of `DrawMarker`: radius 4 around the current point, full colour.
fn cross(x: f32, y: f32) -> Vec<Call> {
    let pen = Pen::new(RED, 1.0);
    vec![
        Call::StrokeLine(
            PointF::new(x, y - 4.0),
            PointF::new(x, y + 4.0),
            pen.clone(),
            Transform::identity(),
        ),
        Call::StrokeLine(
            PointF::new(x - 4.0, y),
            PointF::new(x + 4.0, y),
            pen,
            Transform::identity(),
        ),
    ]
}

fn points(xs: &[(f32, f32)]) -> Vec<PointF> {
    xs.iter().map(|(x, y)| PointF::new(*x, *y)).collect()
}

#[test]
fn past_trail_at_90_percent_then_future_at_25_percent_then_the_cross() {
    // At t = 10 the past is points 0..=1 at alpha int(0.9 * 255) = 229, the future is points 1..=3
    // at alpha int(0.25 * 255) = 63. The cross is on point 1 (10, 0).
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_track(&track(), &scene.ctx(10), &mut r);
    let mut expected = vec![
        Call::StrokeCurve(
            points(&[(0.0, 0.0), (10.0, 0.0)]),
            0.5,
            trail(229),
            Transform::identity(),
        ),
        Call::StrokeCurve(
            points(&[(10.0, 0.0), (20.0, 0.0), (30.0, 0.0)]),
            0.5,
            trail(63),
            Transform::identity(),
        ),
    ];
    expected.extend(cross(10.0, 0.0));
    assert_eq!(r.calls, expected);
}

#[test]
fn steps_mark_each_drawn_point_with_a_disc_of_radius_three() {
    let mut d = track();
    d.header.style.elements[2] = kinovea_metadata::StyleElement {
        key: "track shape".into(),
        value: StyleValue::TrackShape(TrackShape {
            steps: true,
            ..TrackShape::SOLID
        }),
    };
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_track(&d, &scene.ctx(10), &mut r);
    let disc = |x: f32, y: f32| {
        Call::FillEllipse(
            RectangleF::new(x - 3.0, y - 3.0, 6.0, 6.0),
            Brush::solid(Color::argb(229, 255, 0, 0)),
            Transform::identity(),
        )
    };
    assert_eq!(r.calls[1], disc(0.0, 0.0));
    assert_eq!(r.calls[2], disc(10.0, 0.0));
}

#[test]
fn before_the_first_point_only_the_future_trail_is_drawn_at_25_percent() {
    // Visible from -10, at -5 the closest point is the first one: no past trail (one point), the
    // whole track is the future at alpha 63, and there is no cross (opacity is not 1).
    let mut d = track();
    d.visible = -10;
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_track(&d, &scene.ctx(-5), &mut r);
    assert_eq!(
        r.calls,
        vec![Call::StrokeCurve(
            points(&[(0.0, 0.0), (10.0, 0.0), (20.0, 0.0), (30.0, 0.0)]),
            0.5,
            trail(63),
            Transform::identity()
        )]
    );
}

#[test]
fn without_future_the_trail_stops_at_the_current_point() {
    let mut d = track();
    d.see_future = false;
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_track(&d, &scene.ctx(10), &mut r);
    assert_eq!(r.calls.len(), 3);
    assert!(matches!(&r.calls[0], Call::StrokeCurve(p, _, _, _) if p.len() == 2));
}

#[test]
fn scale_two_maps_the_trail_and_keeps_the_pen_at_one_pixel() {
    // Origin (10, 10), scale 2: point 1 is (30, 10).
    let scene = Scene::new((10, 10), 2.0);
    let mut r = surface();
    paint_track(&track(), &scene.ctx(10), &mut r);
    assert!(matches!(&r.calls[0], Call::StrokeCurve(p, _, pen, _)
        if *p == points(&[(10.0, 10.0), (30.0, 10.0)]) && pen.width == 1.0));
}

#[test]
fn nothing_is_painted_when_hidden_or_faded_out() {
    let mut hidden = track();
    hidden.is_visible = false;
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_track(&hidden, &scene.ctx(10), &mut r);
    assert!(r.calls.is_empty());

    // Invisible from 0; at 300 the fade is 1 - 300 / 200 < 0.
    let mut gone = track();
    gone.invisible = 0;
    paint_track(&gone, &scene.ctx(300), &mut r);
    assert!(r.calls.is_empty());
}
