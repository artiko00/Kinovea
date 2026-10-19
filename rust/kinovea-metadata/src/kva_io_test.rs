use crate::fading::Fading;
use crate::kva_io::*;
use crate::style::{StyleElements, StyleValue};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;

#[test]
fn points_scale_and_write_back() {
    let e = Element::leaf("P", "10;20");
    assert_eq!(read_point(&e, Scale { x: 0.5, y: 2.0 }), PointF::new(5.0, 40.0));
    assert_eq!(read_point(&e, Scale::IDENTITY), PointF::new(10.0, 20.0));
    assert_eq!(point_leaf("P", PointF::new(1.5, -2.0)).text, "1.5;-2");
    assert_eq!(Scale { x: 0.5, y: 2.0 }.min(), 0.5);
}

#[test]
fn timestamps_and_open_values() {
    let double = |t: i64| t * 2;
    assert_eq!(parse_timestamp(" 42 "), 42);
    assert_eq!(parse_timestamp("x"), 0);
    assert_eq!(read_open_timestamp("-1", &double), i64::MAX);
    assert_eq!(read_open_timestamp("21", &double), 42);
    assert_eq!(read_open_timestamp("21", &identity), 21);
    assert_eq!(write_open_timestamp(i64::MAX), "-1");
    assert_eq!(write_open_timestamp(7), "7");
}

#[test]
fn style_and_fading_children_are_recognised() {
    let mut style = StyleElements::new(vec![("color", StyleValue::int(0))]);
    let mut fading = Fading::default();
    let s = Element::parse("<DrawingStyle><Int Key=\"color\"><Value>5</Value></Int></DrawingStyle>").unwrap();
    assert!(read_style_or_fading(&s, &mut style, &mut fading));
    assert_eq!(style.get("color"), Some(&StyleValue::int(5)));
    let f = Element::parse("<InfosFading><Frames>7</Frames></InfosFading>").unwrap();
    assert!(read_style_or_fading(&f, &mut style, &mut fading));
    assert_eq!(fading.fading_frames, 7);
    assert!(!read_style_or_fading(&Element::leaf("Other", ""), &mut style, &mut fading));
    let mut out = Vec::new();
    push_style_and_fading(&mut out, Filter::KVA, &style, &fading);
    assert_eq!(out.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["DrawingStyle", "InfosFading"]);
    out.clear();
    push_style_and_fading(&mut out, Filter::CORE, &style, &fading);
    assert!(out.is_empty());
    assert_eq!(parse_enum::<crate::style::LineShape>("Dash", crate::style::LineShape::Solid), crate::style::LineShape::Dash);
    assert_eq!(parse_enum("nope", crate::style::LineShape::Solid), crate::style::LineShape::Solid);
}
