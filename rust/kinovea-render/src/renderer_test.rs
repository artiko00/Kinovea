use crate::renderer::dash_pattern;
use kinovea_metadata::style::DashStyle;

#[test]
fn only_dash_has_a_pattern_and_the_solid_styles_have_none() {
    assert_eq!(dash_pattern(DashStyle::Dash), &[3.0, 1.0]);
    assert!(dash_pattern(DashStyle::Solid).is_empty());
    assert!(dash_pattern(DashStyle::Custom).is_empty());
}
