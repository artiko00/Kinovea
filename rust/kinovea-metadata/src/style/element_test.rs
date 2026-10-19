use crate::color::Color;
use crate::style::element::{pick_among, FONT_SIZES, LINE_SIZES};
use crate::style::{DashStyle, LineEnding, LineShape, PenShape, StyleValue, ToggleVariant, TrackShape};
use kinovea_core::preferences::dom::Element;

#[test]
fn pick_among_snaps_to_the_next_option() {
    assert_eq!(pick_among(&LINE_SIZES, 3), 3);
    assert_eq!(pick_among(&LINE_SIZES, 5), 6);
    assert_eq!(pick_among(&LINE_SIZES, 100), 36);
    assert_eq!(pick_among(&FONT_SIZES, 13), 14);
}

#[test]
fn track_shape_text_form() {
    assert_eq!(TrackShape::parse("Dash;true"), TrackShape { dash: DashStyle::Dash, steps: true });
    assert_eq!(TrackShape::parse("garbage"), TrackShape::SOLID);
    assert_eq!(TrackShape::parse("Solid;false;x"), TrackShape::SOLID);
    assert_eq!(TrackShape { dash: DashStyle::DashDot, steps: false }.write(), "DashDot;false");
}

fn round_trip(xml: &str) -> String {
    let e = Element::parse(xml).unwrap();
    let value = StyleValue::read_kva(&e);
    let mut doc = value.write_kva(e.attribute("Key").unwrap()).to_document();
    doc.drain(..doc.find('\n').unwrap() + 1);
    doc
}

#[test]
fn every_kind_round_trips_its_block() {
    let blocks = [
        "<Color Key=\"line color\">\n  <Value>255;85;107;47</Value>\n</Color>",
        "<FontSize Key=\"font size\">\n  <Value>12</Value>\n</FontSize>",
        "<PenSize Key=\"pen size\">\n  <Value>24</Value>\n</PenSize>",
        "<LineSize Key=\"line size\">\n  <Value>14</Value>\n</LineSize>",
        "<LineShape Key=\"line shape\">\n  <Value>Dash</Value>\n</LineShape>",
        "<Arrows Key=\"arrows\">\n  <Value>EndArrow</Value>\n</Arrows>",
        "<TrackShape Key=\"track shape\">\n  <Value>Dash;true</Value>\n</TrackShape>",
        "<PenShape Key=\"pen shape\">\n  <Value>Solid</Value>\n</PenShape>",
        "<Int Key=\"DecimalPlaces\">\n  <Value>0</Value>\n</Int>",
        "<Toggle Key=\"clock\">\n  <Value>false</Value>\n  <Variant>Clock</Variant>\n</Toggle>",
        "<Future Key=\"x\">\n  <Odd>1</Odd>\n</Future>",
    ];
    for b in blocks {
        assert_eq!(round_trip(b), b);
    }
}

#[test]
fn parsing_applies_fallbacks() {
    let v = StyleValue::read_kva(&Element::parse("<Toggle Key=\"k\"><Value>true</Value></Toggle>").unwrap());
    assert_eq!(v, StyleValue::Toggle { value: true, variant: ToggleVariant::Curved });
    let v = StyleValue::read_kva(&Element::parse("<LineSize Key=\"k\"><Value>5</Value></LineSize>").unwrap());
    assert_eq!(v, StyleValue::LineSize(6));
    let v = StyleValue::read_kva(&Element::parse("<Arrows Key=\"k\"><Value>Nope</Value></Arrows>").unwrap());
    assert_eq!(v, StyleValue::Arrows(LineEnding::None));
    let v = StyleValue::read_kva(&Element::parse("<LineShape Key=\"k\"/>").unwrap());
    assert_eq!(v, StyleValue::LineShape(LineShape::Solid));
    assert_eq!(StyleValue::Color(Color::rgb(1, 2, 3)).kva_name(), "Color");
    assert_eq!(StyleValue::PenShape(PenShape::Dash).content_hash(), 4);
    assert_ne!(StyleValue::int(1).content_hash(), StyleValue::int(2).content_hash());
}
