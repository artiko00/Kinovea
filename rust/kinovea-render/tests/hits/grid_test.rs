use super::*;

#[test]
fn probes_stop_before_the_image_edge() {
    let p = probes(5, 3, 2);
    assert_eq!(p, vec![(0, 0), (2, 0), (4, 0), (0, 2), (2, 2), (4, 2)]);
}

#[test]
fn score_counts_unlisted_probes_as_misses() {
    let mut expected = Expected::default();
    expected.hits.insert((0, 0), 0);
    let s = score(&expected, 2, 2, 1, |p| {
        if p.x == 0.0 && p.y == 0.0 {
            0
        } else {
            -1
        }
    });
    assert_eq!(s.total, 4);
    assert_eq!(s.matches, 4);
    assert!(s.mismatches.is_empty());
    assert_eq!(s.percent(), 100.0);
}

#[test]
fn score_lists_disagreements() {
    let s = score(&Expected::default(), 2, 1, 1, |_| 0);
    assert_eq!(s.matches, 0);
    assert_eq!(
        s.mismatches[0],
        Mismatch {
            x: 0,
            y: 0,
            expected: -1,
            got: 0
        }
    );
}

#[test]
fn boundary_is_found_within_the_radius() {
    let hit = |p: PointF| if p.x >= 10.0 { 0 } else { -1 };
    assert!(near_rust_boundary((9, 5), 2, hit));
    assert!(!near_rust_boundary((5, 5), 2, hit));
}
