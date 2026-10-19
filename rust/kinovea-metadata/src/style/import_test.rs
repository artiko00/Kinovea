use crate::color::Color;
use crate::style::import::import_value;
use crate::style::{LineShape, StyleValue, ToggleVariant};

#[test]
fn same_kind_copies_the_value_and_keeps_the_element_properties() {
    let mut dst = StyleValue::Int { min: 1, max: 5, value: 2, display_name: Some("Cols".into()) };
    let src = StyleValue::Int { min: 0, max: 100, value: 7, display_name: None };
    import_value(&mut dst, &src);
    assert_eq!(dst, StyleValue::Int { min: 1, max: 5, value: 7, display_name: Some("Cols".into()) });

    let mut dst = StyleValue::Toggle { value: false, variant: ToggleVariant::Clock };
    import_value(&mut dst, &StyleValue::Toggle { value: true, variant: ToggleVariant::Curved });
    assert_eq!(dst, StyleValue::Toggle { value: true, variant: ToggleVariant::Clock });

    let mut dst = StyleValue::LineShape(LineShape::Solid);
    import_value(&mut dst, &StyleValue::LineShape(LineShape::Dash));
    assert_eq!(dst, StyleValue::LineShape(LineShape::Dash));
}

#[test]
fn another_kind_resets_to_the_class_default() {
    let mut dst = StyleValue::Int { min: 1, max: 5, value: 2, display_name: None };
    import_value(&mut dst, &StyleValue::Color(Color::rgb(1, 2, 3)));
    assert_eq!(dst, StyleValue::Int { min: 1, max: 5, value: 10, display_name: None });

    let mut dst = StyleValue::Toggle { value: true, variant: ToggleVariant::Perspective };
    import_value(&mut dst, &StyleValue::int(4));
    assert_eq!(dst, StyleValue::Toggle { value: false, variant: ToggleVariant::Perspective });

    let mut dst = StyleValue::Color(Color::rgb(9, 9, 9));
    import_value(&mut dst, &StyleValue::FontSize(30));
    assert_eq!(dst, StyleValue::Color(Color::BLACK));

    let mut dst = StyleValue::Color(Color::rgb(9, 9, 9));
    import_value(&mut dst, &StyleValue::LineSize(8));
    assert_eq!(dst, StyleValue::Color(Color::BLACK));
}

#[test]
fn integer_classes_take_any_integer_value() {
    // `(value is int) ? (int)value : defaultValue` in the Int, FontSize, PenSize and LineSize setters.
    let mut dst = StyleValue::LineSize(24);
    import_value(&mut dst, &StyleValue::PenSize(8));
    assert_eq!(dst, StyleValue::LineSize(8));

    let mut dst = StyleValue::FontSize(14);
    import_value(&mut dst, &StyleValue::LineSize(24));
    assert_eq!(dst, StyleValue::FontSize(24));

    let mut dst = StyleValue::PenSize(3);
    import_value(&mut dst, &StyleValue::FontSize(6));
    assert_eq!(dst, StyleValue::PenSize(6));

    let mut dst = StyleValue::Int { min: 1, max: 5, value: 2, display_name: None };
    import_value(&mut dst, &StyleValue::FontSize(6));
    assert_eq!(dst, StyleValue::Int { min: 1, max: 5, value: 6, display_name: None });

    let mut dst = StyleValue::FontSize(14);
    import_value(&mut dst, &StyleValue::LineShape(LineShape::Dash));
    assert_eq!(dst, StyleValue::FontSize(14));
}
