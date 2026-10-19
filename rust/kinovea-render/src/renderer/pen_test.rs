use super::*;
use kinovea_metadata::style::DashStyle;
use kinovea_metadata::Color;

#[test]
fn new_pen_has_gdi_defaults() {
    let pen = Pen::new(Color::rgb(1, 2, 3), 2.0);
    assert_eq!(pen.color, Color::rgb(1, 2, 3));
    assert_eq!(pen.width, 2.0);
    assert_eq!(
        (pen.dash, pen.start_cap, pen.end_cap, pen.line_join),
        (
            DashStyle::Solid,
            LineCap::Flat,
            LineCap::Flat,
            LineJoin::Miter
        )
    );
}

#[test]
fn dash_patterns_are_expressed_in_pen_widths() {
    assert_eq!(dash_pattern(DashStyle::Dash), &[3.0, 1.0]);
    assert_eq!(dash_pattern(DashStyle::Dot), &[1.0, 1.0]);
    assert_eq!(dash_pattern(DashStyle::DashDot), &[3.0, 1.0, 1.0, 1.0]);
    assert_eq!(
        dash_pattern(DashStyle::DashDotDot),
        &[3.0, 1.0, 1.0, 1.0, 1.0, 1.0]
    );
}

#[test]
fn solid_and_custom_have_no_pattern() {
    assert!(dash_pattern(DashStyle::Solid).is_empty());
    assert!(dash_pattern(DashStyle::Custom).is_empty());
}
