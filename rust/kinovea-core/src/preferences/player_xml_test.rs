use super::helpers::{Color, XmlField};
use super::player::PlayerPreferences;

#[test]
fn write_order_and_optional_colors() {
    let mut p = PlayerPreferences::default();
    let e = p.write("Player").unwrap();
    let names: Vec<&str> = e.children.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names[0], "DecimalPlaces");
    assert_eq!(names[names.len() - 1], "EnableHardwareScaling");
    assert!(!names.contains(&"RecentColors"));
    assert!(!names.contains(&"EnableHighSpeedDerivativesSmoothing"));
    let fading = names.iter().position(|n| *n == "InfoFading").unwrap();
    assert_eq!(names[fading - 1], "VideoFormat");

    p.recent_colors.0 = vec![Color::rgb(255, 0, 0)];
    let e = p.write("Player").unwrap();
    let names: Vec<&str> = e.children.iter().map(|c| c.name.as_str()).collect();
    let colors = names.iter().position(|n| *n == "RecentColors").unwrap();
    assert_eq!(names[colors + 1], "MaxRecentColors");
    assert_eq!(e.child_text("TimelineJumpLargeSize"), Some("10"));
}
