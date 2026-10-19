use crate::backend::recording::Call;
use crate::paint::chrono::paint_chrono_multi;
use crate::paint::helpers::support_test::{surface, Scene};
use crate::renderer::{Brush, Font, FontFamily, Transform};
use kinovea_core::video_section::VideoSection;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{ChronoColumn, ChronoMulti, ChronoSection};
use kinovea_metadata::style::StyleValue;
use kinovea_metadata::{Color, StyleElements};

/// Style of the table: dark background (white text), 12 pt.
fn style() -> StyleElements {
    StyleElements::new(vec![
        (
            "back color",
            StyleValue::Color(Color::argb(255, 10, 20, 30)),
        ),
        ("font size", StyleValue::FontSize(12)),
    ])
}

const WHITE: Color = Color::argb(255, 255, 255, 255);

fn font(em_px: f32) -> Font {
    Font {
        family: FontFamily::Arial,
        em_px,
        bold: false,
    }
}

fn two_sections() -> Vec<ChronoSection> {
    vec![
        ChronoSection {
            section: VideoSection { start: 0, end: 100 },
            name: String::new(),
            tag: "A".into(),
        },
        ChronoSection {
            section: VideoSection {
                start: 100,
                end: 300,
            },
            name: "Jump".into(),
            tag: String::new(),
        },
    ]
}

fn multi() -> ChronoMulti {
    let mut d = ChronoMulti::new(PointF::new(10.0, 20.0));
    d.sections = two_sections();
    d.visible_columns = vec![
        ChronoColumn::Name,
        ChronoColumn::Duration,
        ChronoColumn::Cumul,
        ChronoColumn::Tag,
    ];
    d.header.style = style();
    d.header.fading.average_timestamps_per_frame = 10.0;
    d
}

#[test]
fn multi_paints_the_table_text_at_the_position() {
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_chrono_multi(&multi(), &scene.ctx(150), &mut r);
    assert_eq!(
        r.calls[1],
        Call::DrawText(
            "     1: 100 | 100 | A\n▶ Jump:  50 | 150 |  \n".into(),
            font(16.0),
            Brush::solid(WHITE),
            PointF::new(10.0, 20.0),
            Transform::identity()
        )
    );
}

#[test]
fn locked_multi_dims_to_three_quarters_and_prefixes_the_name() {
    // Master factor 0.75: background alpha int(0.75 * 225) = 168, text alpha int(0.75 * 255) = 191.
    let mut d = multi();
    d.locked = true;
    d.show_label = true;
    d.header.name = "Split".into();
    let scene = Scene::new((0, 0), 1.0);
    let mut r = surface();
    paint_chrono_multi(&d, &scene.ctx(150), &mut r);
    assert!(
        matches!(&r.calls[0], Call::FillPath(_, b, _) if b.color == Color::argb(168, 10, 20, 30))
    );
    assert!(
        matches!(&r.calls[3], Call::DrawText(t, _, b, _, _) if t == "■ Split" && b.color == Color::argb(191, 255, 255, 255))
    );
}
