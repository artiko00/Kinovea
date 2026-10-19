use super::*;

#[test]
fn font_keeps_family_em_size_and_weight() {
    let font = Font {
        family: FontFamily::Consolas,
        em_px: 16.0,
        bold: true,
    };
    assert_eq!(
        (font.family, font.em_px, font.bold),
        (FontFamily::Consolas, 16.0, true)
    );
}
