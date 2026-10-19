use crate::paint::angle_test::{close, paint, quarter_turn, text_of};
use crate::paint::helpers::frame::scene::Scene;
use crate::viewport::Point;

#[test]
fn counter_clockwise_text_reads_ninety_degrees_with_a_four_glyph_label() {
    // C#: AngleHelper.cs:89-120 (DrawText), 91-92; CalibrationHelper.cs:188-195.
    // World value +90 (y-up). Label "90 °" is 4 glyphs of 0.55 em at 16 px: width 35.2, height 19.2.
    // Offset x = 40 cos 315 - 17.6 = 10.6843, y = 40 sin 315 - 9.6 = -37.8843.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let (text, origin) = text_of(&paint(&scene, &quarter_turn(true), 0));
    assert_eq!(text, "90 °");
    assert!(close(origin.x, 110.684_27) && close(origin.y, 62.115_73));
}

#[test]
fn clockwise_text_reads_minus_ninety_degrees_with_a_five_glyph_label() {
    // C#: AngleHelper.cs:89-120 (DrawText), 91-92.
    // World value -90. Label "-90 °" is 5 glyphs: width 44, offset x = 28.2843 - 22 = 6.2843.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let (text, origin) = text_of(&paint(&scene, &quarter_turn(false), 0));
    assert_eq!(text, "-90 °");
    assert!(close(origin.x, 106.284_27) && close(origin.y, 62.115_73));
}

#[test]
fn unsigned_clockwise_text_wraps_to_two_hundred_seventy_degrees() {
    // C#: AngleHelper.cs:137-138, 89-120.
    let scene = Scene::new(Point::new(0, 0), 1.0);
    let mut d = quarter_turn(false);
    d.signed = false;
    let (text, _) = text_of(&paint(&scene, &d, 0));
    assert_eq!(text, "270 °");
}
