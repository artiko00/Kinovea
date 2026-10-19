use crate::backend::recording::Call;
use crate::paint::helpers::support_test::{surface, Scene};
use crate::paint::multi::paint_numbers;
use crate::renderer::{Brush, Font, FontFamily, LineCap, LineJoin, Pen, Transform};
use kinovea_geometry::PointF;
use kinovea_metadata::multi::{AutoNumber, NumberSequence};
use kinovea_metadata::{Color, Fading};
use uuid::Uuid;

/// Fading of 20 frames of 10 timestamps: opacity falls to 0 at 200 after the reference.
fn fading() -> Fading {
    Fading {
        average_timestamps_per_frame: 10.0,
        ..Fading::default()
    }
}

fn number(timestamp: i64, value: i32) -> AutoNumber {
    AutoNumber {
        id: Uuid::nil(),
        timestamp,
        location: PointF::new(10.0, 20.0),
        value,
    }
}

fn numbers(items: Vec<AutoNumber>) -> NumberSequence {
    let mut d = NumberSequence::default();
    d.header.fading = fading();
    d.items = items;
    d
}

#[test]
fn number_is_a_contoured_box_with_the_value_in_white_on_black() {
    // Default style: black background, 18 pt = 24 px. "3": 0.55 * 24 = 13.2 -> 13 wide, 28 high.
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_numbers(&numbers(vec![number(0, 3)]), &scene.ctx(0), &mut r);
    let font = Font {
        family: FontFamily::Arial,
        em_px: 24.0,
        bold: false,
    };
    let contour = Pen {
        line_join: LineJoin::Round,
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        ..Pen::new(Color::argb(255, 255, 255, 255), 2.0)
    };
    assert!(matches!(&r.calls[0], Call::FillPath(_, b, _) if b.color == Color::argb(255, 0, 0, 0)));
    assert!(matches!(&r.calls[1], Call::StrokePath(_, p, _) if *p == contour));
    assert_eq!(
        r.calls[2],
        Call::DrawText(
            "3".into(),
            font,
            Brush::solid(Color::argb(255, 255, 255, 255)),
            PointF::new(10.0, 20.0),
            Transform::identity()
        )
    );
}

#[test]
fn each_number_fades_from_its_own_timestamp() {
    // At 200, the number at 0 is 200 after its own reference: opacity 1 - 200 / 200 = 0, so it is
    // left out. The number at 200 is at its reference: opacity 1, so only "2" is drawn.
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_numbers(
        &numbers(vec![number(0, 1), number(200, 2)]),
        &scene.ctx(200),
        &mut r,
    );
    assert_eq!(r.calls.len(), 3);
    assert!(matches!(&r.calls[2], Call::DrawText(t, ..) if t == "2"));
}
