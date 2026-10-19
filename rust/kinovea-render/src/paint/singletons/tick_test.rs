//! `TickMark.Draw` with a 10 pixel font: "50" measures 11 by 12 (2 characters at 0.55 em, line 1.2 em).

use super::tick::{TextAlignment, TickMark};
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::{Scene, COLOR};
use crate::paint::{Distorter, PaintContext};
use crate::renderer::{Brush, Font, Transform};
use crate::style::StyleData;
use crate::viewport::Point;
use kinovea_geometry::PointF;

struct Shift;

impl Distorter for Shift {
    fn distort(&self, p: PointF) -> PointF {
        PointF::new(p.x + 10.0, p.y)
    }
}

fn font() -> Font {
    Font {
        em_px: 10.0,
        ..StyleData::default().font(1.0)
    }
}

fn tick(location: PointF, alignment: TextAlignment) -> TickMark {
    TickMark {
        value: 50.0,
        location,
        alignment,
    }
}

fn paint(scene: &Scene, tick: &TickMark, distorter: Option<&dyn Distorter>) -> Vec<Call> {
    let brush = Brush::solid(COLOR);
    let font = font();
    scene.record(0, |ctx, r| {
        let ctx = PaintContext { distorter, ..ctx };
        tick.paint(&ctx, r, &brush, &brush, &font, 8.0);
    })
}

fn text(origin: PointF) -> Call {
    Call::DrawText(
        "50".into(),
        font(),
        Brush::solid(COLOR),
        origin,
        Transform::identity(),
    )
}

#[test]
fn bottom_label_is_centred_below_its_point_on_a_rounded_background() {
    // x = 50 - 11 / 2 = 44.5; y = 40 + margin 8 = 48.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let calls = paint(
        &scene,
        &tick(PointF::new(50.0, 40.0), TextAlignment::Bottom),
        None,
    );
    assert_eq!(calls.len(), 2);
    assert!(matches!(calls[0], Call::FillPath(..)));
    assert_eq!(calls[1], text(PointF::new(44.5, 48.0)));
}

#[test]
fn left_label_is_left_of_its_point_and_centred_vertically() {
    // x = 50 - 11 - margin 8 = 31; y = 40 - 12 / 2 = 34.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let calls = paint(
        &scene,
        &tick(PointF::new(50.0, 40.0), TextAlignment::Left),
        None,
    );
    assert_eq!(calls[1], text(PointF::new(31.0, 34.0)));
}

#[test]
fn label_is_anchored_on_the_distorted_point() {
    // The distorter moves the point to (60, 40) before the transform: x = 60 - 5.5 = 54.5.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let calls = paint(
        &scene,
        &tick(PointF::new(50.0, 40.0), TextAlignment::Bottom),
        Some(&Shift),
    );
    assert_eq!(calls[1], text(PointF::new(54.5, 48.0)));
}
