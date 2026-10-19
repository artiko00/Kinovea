use crate::ncc::{match_template, Plane};
use crate::ncc_reference_test::{lcg_bytes, plane};

#[test]
fn f32_pixels_give_the_same_map_as_u8_pixels() {
    let image = lcg_bytes(1, 23 * 17);
    let template = lcg_bytes(2, 5 * 4);
    let image_f: Vec<f32> = image.iter().map(|&v| f32::from(v)).collect();
    let template_f: Vec<f32> = template.iter().map(|&v| f32::from(v)).collect();
    let as_u8 = match_template(plane(&image, 23, 17, 1), plane(&template, 5, 4, 1), None);
    let as_f32 = match_template(
        Plane {
            data: &image_f,
            width: 23,
            height: 17,
            channels: 1,
        },
        Plane {
            data: &template_f,
            width: 5,
            height: 4,
            channels: 1,
        },
        None,
    );
    assert_eq!(as_u8, as_f32);
}

#[test]
#[should_panic]
fn template_larger_than_the_image_panics() {
    let image = lcg_bytes(1, 4 * 4);
    let template = lcg_bytes(2, 5 * 4);
    match_template(plane(&image, 4, 4, 1), plane(&template, 5, 4, 1), None);
}

#[test]
#[should_panic]
fn mask_of_the_wrong_length_panics() {
    let image = lcg_bytes(1, 23 * 17);
    let template = lcg_bytes(2, 5 * 4);
    match_template(
        plane(&image, 23, 17, 1),
        plane(&template, 5, 4, 1),
        Some(&[1u8; 3]),
    );
}
