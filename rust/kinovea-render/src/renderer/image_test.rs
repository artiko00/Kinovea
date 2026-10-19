use super::*;

#[test]
fn image_ref_borrows_rgba_pixels() {
    let pixels = [0u8; 16];
    let image = ImageRef {
        width: 2,
        height: 2,
        rgba: &pixels,
    };
    assert_eq!(image.rgba.len(), (image.width * image.height * 4) as usize);
}
