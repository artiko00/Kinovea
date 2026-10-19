//! Primitive coverage over the `all-drawings` case (task 4.4): every drawing of every container
//! paints something at a time where it is visible, and the scene as a whole exercises the main
//! primitives of the `Renderer` trait. Pixel parity is checked separately by the snapshots.

use kinovea_fixtures::fixtures_root;
use kinovea_metadata::{header_metadata, read_kva, Fading, Metadata};
use kinovea_render::backend::recording::{Call, RecordingRenderer};
use kinovea_render::paint::multi::{paint_numbers, paint_spotlights};
use kinovea_render::paint::singletons::{paint_coordinate_system, paint_test_grid};
use kinovea_render::paint::{paint_drawing, PaintContext};
use kinovea_render::renderer::Renderer;
use kinovea_render::viewport::{ImageToViewport, Point};

fn load() -> Metadata {
    let path = fixtures_root().join("../cases/kva/inputs/all-drawings.kva");
    let xml = std::fs::read_to_string(path).unwrap();
    let mut m = header_metadata(&xml);
    read_kva(&mut m, &xml).unwrap();
    m
}

/// Records what `paint` draws at `timestamp`, with the default fading of the render cases.
fn record(
    m: &Metadata,
    timestamp: i64,
    paint: impl FnOnce(&PaintContext, &mut dyn Renderer),
) -> Vec<Call> {
    let viewport = ImageToViewport::new(Point { x: 0, y: 0 }, 1.0);
    let fading = Fading::default();
    let ctx = PaintContext {
        viewport: &viewport,
        timestamp,
        selected: false,
        metadata: m,
        default_fading: &fading,
        distorter: None,
        camera: None,
    };
    let mut r = RecordingRenderer::new(640.0, 480.0);
    paint(&ctx, &mut r);
    r.calls
}

#[test]
fn every_drawing_paints_at_a_visible_time() {
    let m = load();
    let mut empty = Vec::new();
    for kf in &m.keyframes {
        for d in &kf.drawings {
            if record(&m, kf.timestamp, |ctx, r| paint_drawing(d, ctx, r)).is_empty() {
                empty.push(d.kva_name());
            }
        }
    }
    // Chronos are visible from 400 and count from 800; the track runs from 400 to 520.
    for d in &m.chronos {
        if record(&m, 1000, |ctx, r| paint_drawing(d, ctx, r)).is_empty() {
            empty.push(d.kva_name());
        }
    }
    for d in &m.tracks {
        if record(&m, 440, |ctx, r| paint_drawing(d, ctx, r)).is_empty() {
            empty.push(d.kva_name());
        }
    }
    assert!(empty.is_empty(), "drawings that painted nothing: {empty:?}");
    assert_eq!(
        m.keyframes[0].drawings.len(),
        14,
        "the SVG without a file is dropped on import"
    );
}

#[test]
fn singletons_and_multi_drawings_paint() {
    let m = load();
    assert!(!record(&m, 400, |ctx, r| paint_coordinate_system(
        &m.coordinate_system,
        ctx,
        r
    ))
    .is_empty());
    assert!(!record(&m, 400, |ctx, r| paint_test_grid(&m.test_grid, ctx, r)).is_empty());
    assert!(!record(&m, 400, |ctx, r| paint_spotlights(&m.spotlights, ctx, r)).is_empty());
    assert!(!record(&m, 400, |ctx, r| paint_numbers(&m.number_sequence, ctx, r)).is_empty());
}

#[test]
fn the_scene_exercises_the_main_primitives() {
    let m = load();
    let mut calls = Vec::new();
    for kf in &m.keyframes {
        for d in &kf.drawings {
            calls.extend(record(&m, kf.timestamp, |ctx, r| paint_drawing(d, ctx, r)));
        }
    }
    for d in m.chronos.iter().chain(m.tracks.iter()) {
        calls.extend(record(&m, 440, |ctx, r| paint_drawing(d, ctx, r)));
    }
    calls.extend(record(&m, 400, |ctx, r| {
        paint_spotlights(&m.spotlights, ctx, r)
    }));
    let used = |f: fn(&Call) -> bool| calls.iter().any(f);
    assert!(used(|c| matches!(c, Call::StrokeLine(..))), "lines");
    assert!(
        used(|c| matches!(c, Call::StrokeCurve(..))),
        "cardinal curves"
    );
    assert!(
        used(|c| matches!(c, Call::StrokeEllipse(..) | Call::FillEllipse(..))),
        "ellipses"
    );
    assert!(
        used(|c| matches!(c, Call::StrokeArc(..) | Call::FillPie(..))),
        "arcs and pies"
    );
    assert!(used(|c| matches!(c, Call::FillPolygon(..))), "arrow heads");
    assert!(
        used(|c| matches!(c, Call::FillPath(..) | Call::StrokePath(..))),
        "paths"
    );
    assert!(used(|c| matches!(c, Call::DrawText(..))), "text");
}
