use super::estimate_radius;

/// Integer grid points whose distance to `(10, 10)` is within half a pixel of `r`.
fn ring(r: f32) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for y in 0..40usize {
        for x in 0..40usize {
            let d = (((x as f32 - 10.0).powi(2)) + ((y as f32 - 10.0).powi(2))).sqrt();
            if (d - r).abs() <= 0.5 {
                points.push((x, y));
            }
        }
    }
    points
}

#[test]
fn ring_gives_its_radius_with_its_support() {
    let points = ring(8.0);
    let n = points.len() as i32;
    let found = estimate_radius(&points, (10.0, 10.0), (6, 10), 1.0, n - 1);
    let (radius, support) = found.expect("ring above threshold");
    assert!((radius - 8.0).abs() <= 1.0);
    assert_eq!(support, n);
}

#[test]
fn support_must_be_strictly_above_threshold() {
    let points = ring(8.0);
    let n = points.len() as i32;
    assert!(estimate_radius(&points, (10.0, 10.0), (6, 10), 1.0, n).is_none());
}

#[test]
fn points_outside_the_annulus_do_not_count() {
    let points = ring(8.0);
    assert!(estimate_radius(&points, (10.0, 10.0), (12, 14), 1.0, 0).is_none());
}
