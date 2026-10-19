use super::common::testing::Scene;
use super::multi::*;
use super::Hit;
use kinovea_geometry::PointF;
use kinovea_metadata::multi::{AutoNumber, NumberSequence, Spotlight, Spotlights};

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

// DrawingSpotlightItem.cs:128-141: the outline of the spot (handle 1), then its disc as the body.
#[test]
fn spot_outline_is_handle_one_and_its_disc_is_body() {
    let scene = Scene::at_scale(1.0);
    let spots = Spotlights {
        items: vec![Spotlight::new(0, p(100.0, 100.0), 40)],
        ..Spotlights::default()
    };
    let ctx = scene.ctx(0);
    assert_eq!(
        hit_spotlights(&spots, p(140.0, 100.0), &ctx),
        Hit::Handle(1)
    );
    assert_eq!(hit_spotlights(&spots, p(100.0, 100.0), &ctx), Hit::Body);
    assert_eq!(hit_spotlights(&spots, p(300.0, 300.0), &ctx), Hit::Miss);
}

// DrawingSpotlight.cs:158-174: the first spot in the list whose item answers wins.
#[test]
fn the_first_spot_that_answers_wins() {
    let scene = Scene::at_scale(1.0);
    let spots = Spotlights {
        items: vec![
            Spotlight::new(0, p(10.0, 10.0), 5),
            Spotlight::new(0, p(100.0, 100.0), 40),
        ],
        ..Spotlights::default()
    };
    assert_eq!(
        hit_spotlights(&spots, p(100.0, 100.0), &scene.ctx(0)),
        Hit::Body
    );
}

// DrawingNumberSequenceItem.cs:112-120: the box of the number is the body; there is no handle.
#[test]
fn number_box_is_body_and_nothing_else_is_hit() {
    let scene = Scene::at_scale(1.0);
    let numbers = NumberSequence {
        items: vec![AutoNumber::new(0, p(50.0, 50.0), 3)],
        ..NumberSequence::default()
    };
    let ctx = scene.ctx(0);
    assert_eq!(hit_numbers(&numbers, p(51.0, 51.0), &ctx), Hit::Body);
    assert_eq!(hit_numbers(&numbers, p(300.0, 300.0), &ctx), Hit::Miss);
}
