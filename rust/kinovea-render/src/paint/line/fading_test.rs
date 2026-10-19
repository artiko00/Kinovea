use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::{style, Scene};
use crate::paint::line::paint_line;
use crate::viewport::Point;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Line;
use kinovea_metadata::style::LineShape;
use kinovea_metadata::{Fading, StyleValue};

fn solid_line() -> Line {
    let mut d = Line::new(PointF::new(0.0, 0.0), PointF::new(10.0, 0.0));
    d.header.style = style(1);
    d.header
        .style
        .insert("line shape", StyleValue::LineShape(LineShape::Solid));
    d
}

#[test]
fn opacity_zero_paints_nothing() {
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = solid_line();
    d.header.fading = Fading {
        use_default: false,
        ..Fading::default()
    };
    // Fading is 0 at timestamp 10 with no average timestamps per frame: the reference is 0.
    assert!(scene
        .record(10, |ctx, r| paint_line(&d, &ctx, r))
        .is_empty());
}

#[test]
fn half_opacity_halves_the_pen_alpha() {
    // Fading over 2 frames of 1 timestamp each: distance 1 gives 1 - 1/2 = 0.5, alpha 127.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = solid_line();
    d.header.fading = Fading {
        use_default: false,
        fading_frames: 2,
        average_timestamps_per_frame: 1.0,
        ..Fading::default()
    };
    let calls = scene.record(1, |ctx, r| paint_line(&d, &ctx, r));
    let Call::StrokeLine(_, _, pen, _) = &calls[0] else {
        panic!("expected a line");
    };
    assert_eq!(pen.color.a, 127);
}
