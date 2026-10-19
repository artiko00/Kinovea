use crate::ncc::match_template;
use crate::ncc_reference_test::{crop, lcg_bytes, plane, reference_score};

const TOLERANCE: f64 = 1e-4;

/// Mask bytes mixing 0 and several nonzero values: any nonzero byte counts as 1 (as OpenCV does).
fn mask_bytes(seed: u64, len: usize) -> Vec<u8> {
    lcg_bytes(seed, len)
        .iter()
        .map(|&b| [0, 255, 7][usize::from(b) % 3])
        .collect()
}

#[test]
fn masked_map_matches_brute_force_reference_and_has_window_size() {
    let image = lcg_bytes(1, 23 * 17);
    let template = lcg_bytes(2, 5 * 4);
    let mask = mask_bytes(3, 5 * 4);
    let map = match_template(
        plane(&image, 23, 17, 1),
        plane(&template, 5, 4, 1),
        Some(&mask),
    );
    assert_eq!((map.width, map.height), (19, 14));
    for y in 0..map.height {
        for x in 0..map.width {
            let included = |p: usize| mask[p] != 0;
            let expected = reference_score(
                plane(&image, 23, 17, 1),
                plane(&template, 5, 4, 1),
                (x, y),
                &included,
            );
            let actual = f64::from(map.at(x, y));
            assert!(
                (actual - expected).abs() < TOLERANCE,
                "at ({x}, {y}): {actual} vs {expected}"
            );
        }
    }
}

#[test]
fn excluded_template_pixels_are_ignored() {
    let image = lcg_bytes(1, 23 * 17);
    let template = lcg_bytes(2, 5 * 4);
    let mask = mask_bytes(3, 5 * 4);
    let mut changed = template.clone();
    for (value, &m) in changed.iter_mut().zip(&mask) {
        if m == 0 {
            *value = value.wrapping_add(101);
        }
    }
    let before = match_template(
        plane(&image, 23, 17, 1),
        plane(&template, 5, 4, 1),
        Some(&mask),
    );
    let after = match_template(
        plane(&image, 23, 17, 1),
        plane(&changed, 5, 4, 1),
        Some(&mask),
    );
    assert_eq!(before.values, after.values);
}

#[test]
fn exact_crop_scores_one_at_its_location_with_a_mask() {
    let image = lcg_bytes(1, 23 * 17);
    let template = crop(&image, 23, (5, 4), (5, 4), 1);
    let mask = vec![255u8; 5 * 4];
    let map = match_template(
        plane(&image, 23, 17, 1),
        plane(&template, 5, 4, 1),
        Some(&mask),
    );
    assert!((f64::from(map.at(5, 4)) - 1.0).abs() < 1e-5);
    assert_eq!(map.min_max_loc().max_loc, (5, 4));
}

#[test]
fn full_mask_equals_the_unmasked_map() {
    let image = lcg_bytes(1, 23 * 17);
    let template = lcg_bytes(2, 5 * 4);
    let mask = vec![1u8; 5 * 4];
    let masked = match_template(
        plane(&image, 23, 17, 1),
        plane(&template, 5, 4, 1),
        Some(&mask),
    );
    let unmasked = match_template(plane(&image, 23, 17, 1), plane(&template, 5, 4, 1), None);
    for (a, b) in masked.values.iter().zip(&unmasked.values) {
        assert!((a - b).abs() < 1e-5);
    }
}

#[test]
fn template_flat_under_the_mask_scores_one_everywhere() {
    let image = lcg_bytes(1, 23 * 17);
    let template: Vec<u8> = (0..5 * 4)
        .map(|p| if p % 2 == 0 { 40 } else { (p * 37 % 255) as u8 })
        .collect();
    let mask: Vec<u8> = (0..5 * 4).map(|p| u8::from(p % 2 == 0) * 255).collect();
    let map = match_template(
        plane(&image, 23, 17, 1),
        plane(&template, 5, 4, 1),
        Some(&mask),
    );
    assert!(map.values.iter().all(|&v| v == 1.0));
}
