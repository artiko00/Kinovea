use super::dom::Element;
use super::enums::{TimelineJumpUnit, VideoContainer};
use super::helpers::{Color, XmlField};
use super::player::PlayerPreferences;
use crate::timecode::TimecodeFormat;
use crate::units::{AngleUnit, SpeedUnit};

#[test]
fn defaults_match_the_original() {
    let p = PlayerPreferences::default();
    assert_eq!(p.timecode_format, TimecodeFormat::ClassicTime);
    assert_eq!(p.speed_unit, SpeedUnit::MetersPerSecond);
    assert_eq!(p.angle_unit, AngleUnit::Degree);
    assert!(p.enable_filtering && p.enable_high_speed_derivatives_smoothing);
    assert_eq!(p.video_format, VideoContainer::MKV);
    assert_eq!(
        (p.timeline_jump_small_size, p.timeline_jump_small_unit),
        (0.5, TimelineJumpUnit::Second)
    );
    assert_eq!(
        (p.timeline_jump_large_size, p.timeline_jump_large_unit),
        (10.0, TimelineJumpUnit::SnapPercent)
    );
    assert_eq!(p.keyframe_presets.0.len(), 10);
    assert_eq!(p.export_profile.name, "Default");
}

#[test]
fn reads_known_values_and_ignores_unknown() {
    let xml = "<Player><TimecodeFormat>Frames</TimecodeFormat><RecentColors><RecentColor>1;2;3</RecentColor></RecentColors><Bogus>1</Bogus><DecimalPlaces>3</DecimalPlaces></Player>";
    let mut p = PlayerPreferences::default();
    p.read(&Element::parse(xml).unwrap());
    assert_eq!(p.timecode_format, TimecodeFormat::Frames);
    assert_eq!(p.decimal_places, 3);
    assert_eq!(p.recent_colors.0, [Color::rgb(1, 2, 3)]);
}
