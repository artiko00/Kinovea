//! `DrawingGenericPosture.Draw` as a whole. The embedded Goniometer (`02ea93f9-...`) at scale 1 with its
//! points (100, 100), (150, 100), (200, 100) and (180, 70):
//! - Stationary arm 0 -> 1: solid, width 4, round caps.
//! - Reference 1 -> 2: dashed, width 2.
//! - Movable arm 1 -> 3: solid, width 2, arrow at its end (the end moves back by the arrow).
//! - Angle at origin 1, legs 2 and 3, signed and CCW, radius -25. The radius counts from the
//!   smallest leg (|1 - 3| = 42.43): 42.43 - 25 = 17.43 -> 17, box (133, 83, 34, 34).
//!   The sweep runs from 315 degrees by 45 (atan2 of (30, 30) from the origin). The calibrated value
//!   is atan2(1500, 1500) = pi/4, read as "45".

use super::fixture_test::{drawing, scene, two_points};
use super::frame::edge_pen;
use super::paint_posture;
use crate::backend::recording::Call;
use crate::paint::helpers::frame::scene::COLOR;
use crate::renderer::{Brush, LineCap, LineJoin, Pen, Transform};
use crate::style::colors::with_alpha;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::posture::posture_by_id;
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::Fading;
use uuid::Uuid;

const GONIOMETER: &str = "02ea93f9-ba00-424b-8c87-07e156e0252f";

fn round_pen(width: f32, dash: DashStyle) -> Pen {
    Pen {
        dash,
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Pen::new(COLOR, width)
    }
}

fn record(d: &kinovea_metadata::drawing::GenericPostureDrawing) -> Vec<Call> {
    scene(1.0).record(0, |ctx, r| paint_posture(d, &ctx, r))
}

#[test]
fn faded_drawing_paints_nothing() {
    let mut d = drawing(two_points());
    d.header.fading = Fading {
        use_default: false,
        ..Fading::default()
    };
    // Fading is 0 at timestamp 10 with no average timestamps per frame: the reference is 0.
    assert!(scene(1.0)
        .record(10, |ctx, r| paint_posture(&d, &ctx, r))
        .is_empty());
}

#[test]
fn drawing_without_tool_description_paints_nothing() {
    let mut d = drawing(two_points());
    d.posture = None;
    assert!(record(&d).is_empty());
}

#[test]
fn goniometer_draws_its_arms_then_its_sector_and_read_out() {
    let id = Uuid::parse_str(GONIOMETER).expect("valid id");
    let posture = posture_by_id(id, false).expect("shipped tool");
    let calls = record(&drawing(posture));

    assert_eq!(
        calls[0],
        Call::StrokeLine(
            PointF::new(100.0, 100.0),
            PointF::new(150.0, 100.0),
            round_pen(4.0, DashStyle::Solid),
            Transform::identity(),
        )
    );
    assert_eq!(
        calls[1],
        Call::StrokeLine(
            PointF::new(150.0, 100.0),
            PointF::new(200.0, 100.0),
            round_pen(2.0, DashStyle::Dash),
            Transform::identity(),
        )
    );
    // The movable arm starts at (150, 100); its end is pulled back from (180, 70) by the arrow.
    assert!(matches!(
        &calls[2],
        Call::StrokeLine(a, _, pen, _) if *a == PointF::new(150.0, 100.0) && pen.width == 2.0
    ));

    let box_ = RectangleF::new(133.0, 83.0, 34.0, 34.0);
    let fill = Brush::solid(with_alpha(COLOR, 92));
    match calls.iter().find(|c| matches!(c, Call::FillPie(..))) {
        Some(Call::FillPie(rect, start, sweep, brush, _)) => {
            assert_eq!(*rect, box_);
            assert!((start - 315.0).abs() < 1e-3, "start {start}");
            assert!((sweep - 45.0).abs() < 1e-3, "sweep {sweep}");
            assert_eq!(*brush, fill);
        }
        other => panic!("expected the sector, got {other:?}"),
    }
    match calls.iter().find(|c| matches!(c, Call::StrokeArc(..))) {
        Some(Call::StrokeArc(rect, start, sweep, pen, _)) => {
            assert_eq!(*rect, box_);
            assert!((start - 315.0).abs() < 1e-3 && (sweep - 45.0).abs() < 1e-3);
            assert_eq!(*pen, edge_pen(COLOR, 2.0, DashStyle::Solid));
        }
        other => panic!("expected the arc, got {other:?}"),
    }
    assert!(calls
        .iter()
        .any(|c| matches!(c, Call::DrawText(t, ..) if t.starts_with("45 "))));
}
