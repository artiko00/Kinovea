use crate::backend::recording::Call;
use crate::paint::helpers::support_test::{surface, Scene};
use crate::paint::multi::paint_spotlights;
use crate::renderer::{Brush, Path, Pen, Transform};
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::multi::{Spotlight, Spotlights};
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::Color;
use uuid::Uuid;

/// A scene of a video with 10 timestamps per frame: items fade over 20 frames (200 timestamps) from
/// their reference, with the player's default fading.
fn scene_at(origin: (i32, i32), scale: f64) -> Scene {
    let mut scene = Scene::new(origin, scale);
    scene.metadata.video.average_timestamps_per_frame = 10.0;
    scene
}

fn spot(timestamp: i64, x: f32, radius: i32) -> Spotlight {
    Spotlight {
        id: Uuid::nil(),
        timestamp,
        center: PointF::new(x, 100.0),
        radius,
    }
}

fn spotlights(items: Vec<Spotlight>) -> Spotlights {
    Spotlights {
        items,
        ..Spotlights::default()
    }
}

/// Ring of `DrawingSpotlightItem.Draw`: white, 2 px, dashed.
fn ring_pen(alpha: u8) -> Pen {
    Pen {
        dash: DashStyle::Dash,
        ..Pen::new(Color::argb(alpha, 255, 255, 255), 2.0)
    }
}

#[test]
fn spots_are_holes_in_a_dim_layer_outlined_by_dashed_rings() {
    // Spot at (100, 100) radius 20 at full opacity: layer alpha 160 over the clip (0, 0, 800, 600).
    let scene = scene_at((0, 0), 1.0);
    let mut r = surface();
    paint_spotlights(&spotlights(vec![spot(0, 100.0, 20)]), &scene.ctx(0), &mut r);
    let mut mask = Path::default();
    mask.rectangle(RectangleF::new(0.0, 0.0, 800.0, 600.0));
    mask.ellipse(RectangleF::new(80.0, 80.0, 40.0, 40.0));
    let black = Brush::solid(Color::argb(160, 0, 0, 0));
    assert_eq!(
        r.calls,
        vec![
            Call::FillPath(mask, black, Transform::identity()),
            Call::StrokeEllipse(
                RectangleF::new(80.0, 80.0, 40.0, 40.0),
                ring_pen(255),
                Transform::identity()
            ),
        ]
    );
}

#[test]
fn the_layer_follows_the_strongest_spot_and_faded_spots_are_left_out() {
    // Spot at 1000 is 1000 after the reference: opacity 0, so it is neither a hole nor a ring.
    let scene = scene_at((0, 0), 1.0);
    let mut r = surface();
    paint_spotlights(
        &spotlights(vec![spot(0, 100.0, 20), spot(1000, 300.0, 20)]),
        &scene.ctx(0),
        &mut r,
    );
    assert_eq!(r.calls.len(), 2);
    assert!(matches!(&r.calls[0], Call::FillPath(_, b, _) if b.color == Color::argb(160, 0, 0, 0)));
}

#[test]
fn a_half_faded_spot_halves_the_layer_and_the_ring() {
    // 100 after the reference: opacity 1 - 100 / 200 = 0.5. Layer alpha int(80) = 80, ring int(127.5) = 127.
    let scene = scene_at((0, 0), 1.0);
    let mut r = surface();
    paint_spotlights(
        &spotlights(vec![spot(0, 100.0, 20)]),
        &scene.ctx(100),
        &mut r,
    );
    assert!(matches!(&r.calls[0], Call::FillPath(_, b, _) if b.color == Color::argb(80, 0, 0, 0)));
    assert!(
        matches!(&r.calls[1], Call::StrokeEllipse(_, p, _) if p.color == Color::argb(127, 255, 255, 255))
    );
}

#[test]
fn spots_are_scaled_by_the_viewport_and_the_ring_keeps_its_width() {
    // Origin (10, 10), scale 2: centre (210, 210), radius 40, so the ellipse is (170, 170, 80, 80).
    let scene = scene_at((10, 10), 2.0);
    let mut r = surface();
    paint_spotlights(&spotlights(vec![spot(0, 100.0, 20)]), &scene.ctx(0), &mut r);
    assert!(matches!(&r.calls[1], Call::StrokeEllipse(rect, p, _)
        if *rect == RectangleF::new(170.0, 170.0, 80.0, 80.0) && p.width == 2.0));
}

#[test]
fn no_spots_paint_nothing() {
    let scene = scene_at((0, 0), 1.0);
    let mut r = surface();
    paint_spotlights(&spotlights(vec![]), &scene.ctx(0), &mut r);
    assert!(r.calls.is_empty());
}
