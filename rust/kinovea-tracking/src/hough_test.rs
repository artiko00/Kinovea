use super::{hough_circles, HoughCircle, HoughParams};

/// Filled disc by the integer rule `(x - cx)^2 + (y - cy)^2 <= r^2` on a dark background.
fn disc(width: usize, height: usize, cx: i32, cy: i32, r: i32) -> Vec<u8> {
    (0..width * height)
        .map(|i| {
            let (x, y) = ((i % width) as i32, (i / width) as i32);
            if (x - cx).pow(2) + (y - cy).pow(2) <= r * r {
                220
            } else {
                40
            }
        })
        .collect()
}

fn params(acc_threshold: i32, min_radius: i32, max_radius: i32) -> HoughParams {
    HoughParams {
        dp: 1.0,
        min_dist: 8.0,
        canny_threshold: 50,
        acc_threshold,
        min_radius,
        max_radius,
    }
}

fn near(c: &HoughCircle, x: f32, y: f32, radius: f32) -> bool {
    (c.x - x).abs() <= 1.0 && (c.y - y).abs() <= 1.0 && (c.radius - radius).abs() <= 1.0
}

#[test]
fn disc_is_found_at_its_center_and_radius() {
    let data = disc(60, 60, 30, 30, 10);
    let circles = hough_circles(&data, 60, 60, &params(20, 9, 11));
    assert_eq!(circles.len(), 1);
    assert!(near(&circles[0], 30.0, 30.0, 10.0));
}

#[test]
fn discs_of_different_radii_are_separated() {
    let left = disc(120, 60, 30, 30, 8);
    let right = disc(120, 60, 85, 30, 14);
    let data: Vec<u8> = left.iter().zip(&right).map(|(&a, &b)| a.max(b)).collect();
    let p = HoughParams {
        min_dist: 20.0,
        ..params(10, 6, 16)
    };
    let circles = hough_circles(&data, 120, 60, &p);
    assert_eq!(circles.len(), 2);
    assert!(circles.iter().any(|c| near(c, 30.0, 30.0, 8.0)));
    assert!(circles.iter().any(|c| near(c, 85.0, 30.0, 14.0)));
}

#[test]
fn uniform_image_has_no_circle() {
    let data = vec![120u8; 60 * 60];
    assert!(hough_circles(&data, 60, 60, &params(1, 9, 11)).is_empty());
}

#[test]
fn vote_threshold_discards_an_incomplete_arc() {
    let arc: Vec<u8> = (0..60 * 60)
        .map(|i: i32| {
            let (x, y) = (i % 60, i / 60);
            let inside = (x - 30).pow(2) + (y - 30).pow(2) <= 100;
            if inside && x >= 30 {
                220
            } else {
                40
            }
        })
        .collect();
    let full = disc(60, 60, 30, 30, 10);
    assert_eq!(hough_circles(&full, 60, 60, &params(20, 9, 11)).len(), 1);
    assert!(hough_circles(&arc, 60, 60, &params(20, 9, 11)).is_empty());
    assert_eq!(hough_circles(&arc, 60, 60, &params(5, 9, 11)).len(), 1);
}

#[test]
fn equal_radii_widen_to_the_range_min_plus_two() {
    let data = disc(60, 60, 30, 30, 10);
    let equal = hough_circles(&data, 60, 60, &params(5, 10, 10));
    let widened = hough_circles(&data, 60, 60, &params(5, 10, 12));
    assert_eq!(equal, widened);
    assert_eq!(equal.len(), 1);
}
