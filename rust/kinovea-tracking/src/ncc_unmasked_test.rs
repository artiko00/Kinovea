use crate::ncc::{match_template, Map, Plane};
use crate::ncc_reference_test::{crop, lcg_bytes, plane, reference_score};

const TOLERANCE: f64 = 1e-4;

fn assert_matches_reference(map: &Map, image: Plane<u8>, template: Plane<u8>) {
    for y in 0..map.height {
        for x in 0..map.width {
            let expected = reference_score(image, template, (x, y), &|_| true);
            let actual = f64::from(map.at(x, y));
            assert!(
                (actual - expected).abs() < TOLERANCE,
                "at ({x}, {y}): {actual} vs {expected}"
            );
        }
    }
}

#[test]
fn map_has_the_sliding_window_size() {
    let image = lcg_bytes(1, 23 * 17);
    let template = lcg_bytes(2, 5 * 4);
    let map = match_template(plane(&image, 23, 17, 1), plane(&template, 5, 4, 1), None);
    assert_eq!((map.width, map.height), (19, 14));
    assert_eq!(map.values.len(), 19 * 14);
}

#[test]
fn gray_map_matches_brute_force_reference() {
    let image = lcg_bytes(1, 23 * 17);
    let template = lcg_bytes(2, 5 * 4);
    let map = match_template(plane(&image, 23, 17, 1), plane(&template, 5, 4, 1), None);
    assert_matches_reference(&map, plane(&image, 23, 17, 1), plane(&template, 5, 4, 1));
}

#[test]
fn bgr_map_accumulates_the_channels_like_opencv() {
    let image = lcg_bytes(3, 12 * 9 * 3);
    let template = lcg_bytes(4, 4 * 3 * 3);
    let map = match_template(plane(&image, 12, 9, 3), plane(&template, 4, 3, 3), None);
    assert_matches_reference(&map, plane(&image, 12, 9, 3), plane(&template, 4, 3, 3));
}

#[test]
fn exact_crop_scores_one_at_its_location() {
    let image = lcg_bytes(1, 23 * 17);
    let template = crop(&image, 23, (5, 4), (5, 4), 1);
    let map = match_template(plane(&image, 23, 17, 1), plane(&template, 5, 4, 1), None);
    assert!((f64::from(map.at(5, 4)) - 1.0).abs() < 1e-5);
    assert_eq!(map.min_max_loc().max_loc, (5, 4));
}

#[test]
fn inverted_crop_scores_minus_one() {
    let image = lcg_bytes(1, 23 * 17);
    let template: Vec<u8> = crop(&image, 23, (5, 4), (5, 4), 1)
        .iter()
        .map(|v| 255 - v)
        .collect();
    let map = match_template(plane(&image, 23, 17, 1), plane(&template, 5, 4, 1), None);
    assert!((f64::from(map.at(5, 4)) + 1.0).abs() < 1e-5);
}

#[test]
fn flat_template_scores_one_everywhere() {
    let image = lcg_bytes(1, 23 * 17);
    let template = vec![7u8; 5 * 4];
    let map = match_template(plane(&image, 23, 17, 1), plane(&template, 5, 4, 1), None);
    assert!(map.values.iter().all(|&v| v == 1.0));
}

#[test]
fn flat_window_scores_zero_against_a_textured_template() {
    let image = vec![9u8; 23 * 17];
    let template = lcg_bytes(2, 5 * 4);
    let map = match_template(plane(&image, 23, 17, 1), plane(&template, 5, 4, 1), None);
    assert!(map.values.iter().all(|&v| v == 0.0));
}
