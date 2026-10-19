use super::colors::*;
use kinovea_metadata::Color;

#[test]
fn alpha_of_truncates_opacity_to_a_byte_like_the_original_cast() {
    assert_eq!(alpha_of(0.5), 127);
    assert_eq!(alpha_of(1.0), 255);
    assert_eq!(alpha_of(0.0), 0);
}

#[test]
fn with_alpha_replaces_alpha_only_when_in_range() {
    assert_eq!(
        with_alpha(Color::rgb(1, 2, 3), 128),
        Color::argb(128, 1, 2, 3)
    );
    assert_eq!(
        with_alpha(Color::argb(40, 1, 2, 3), -1),
        Color::argb(40, 1, 2, 3)
    );
    assert_eq!(
        with_alpha(Color::argb(40, 1, 2, 3), 256),
        Color::argb(40, 1, 2, 3)
    );
}

#[test]
fn foreground_is_black_on_bright_backgrounds() {
    assert_eq!(foreground_for(Color::rgb(128, 128, 128)), Color::BLACK);
    assert_eq!(foreground_for(Color::rgb(255, 255, 255)), Color::BLACK);
}

#[test]
fn foreground_is_white_on_dark_backgrounds() {
    assert_eq!(
        foreground_for(Color::rgb(127, 127, 127)),
        Color::rgb(255, 255, 255)
    );
    assert_eq!(foreground_for(Color::BLACK), Color::rgb(255, 255, 255));
}
