use crate::style::default_style;
use crate::style::{standard_tools, StyleValue, Tool};

#[test]
fn all_standard_tools_load_with_kind_and_defaults() {
    let tools = standard_tools();
    assert_eq!(tools.len(), 26);
    for t in &tools {
        assert!(!t.name.is_empty() && t.drawing_class.starts_with("Drawing"), "{t:?}");
        assert!(t.name == "Sticker" || !t.default_style.elements.is_empty(), "{} has no default style", t.name);
    }
    let line = tools.iter().find(|t| t.name == "Line").unwrap();
    assert!(line.attached && line.keep_tool_after_drawing_creation);
    assert_eq!(line.drawing_class, "DrawingLine");
    assert_eq!(line.default_style.get("line size"), Some(&StyleValue::LineSize(2)));
    let chrono = tools.iter().find(|t| t.name == "Chrono").unwrap();
    assert!(!chrono.attached);
}

#[test]
fn plane_defaults_carry_int_bounds_and_perspective_toggle() {
    let plane = default_style("Plane");
    match plane.get("cols") {
        Some(StyleValue::Int { min, max, value, display_name }) => {
            assert_eq!((*min, *max, *value), (1, 50, 10));
            assert_eq!(display_name.as_deref(), Some("Columns"));
        }
        other => panic!("{other:?}"),
    }
    assert!(matches!(plane.get("perspective"), Some(StyleValue::Toggle { value: true, .. })));
    assert!(default_style("Nope").elements.is_empty());
    assert!(Tool::parse("<Other/>").is_none());
}
