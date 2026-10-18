use crate::color::Color;

#[test]
fn parses_three_and_four_part_forms() {
    assert_eq!(Color::parse("51;152;255", Color::BLACK), Color::rgb(51, 152, 255));
    assert_eq!(Color::parse("128;255;0;0", Color::BLACK), Color::argb(128, 255, 0, 0));
    assert_eq!(Color::parse("Red", Color::rgb(1, 2, 3)), Color::rgb(1, 2, 3));
    assert_eq!(Color::parse("1;2", Color::BLACK), Color::BLACK);
    assert_eq!(Color::default(), Color::BLACK);
}

#[test]
fn writes_with_or_without_alpha_and_hashes_argb() {
    let c = Color::argb(255, 255, 127, 0);
    assert_eq!(c.write(true), "255;255;127;0");
    assert_eq!(c.write(false), "255;127;0");
    assert_eq!(Color::argb(0, 255, 255, 255).hash_code(), 0x00FF_FFFF);
    assert_eq!(Color::rgb(0, 0, 0).hash_code(), 0xFF00_0000u32 as i32);
}
