use super::canny;
use crate::hough::gradient::Gradient;

/// 5x5 gradient where column `col` has the horizontal derivative `dx` and nothing else.
fn column(col: usize, dx: &[i16]) -> Gradient {
    let mut g = Gradient {
        width: 5,
        height: 5,
        dx: vec![0; 25],
        dy: vec![0; 25],
    };
    for (y, &v) in dx.iter().enumerate() {
        g.dx[y * 5 + col] = v;
    }
    g
}

#[test]
fn strong_vertical_edge_is_kept_as_a_line() {
    let g = column(2, &[400; 5]);
    let edges = canny(&g, 100, 300);
    let expected: Vec<bool> = (0..25).map(|i| i % 5 == 2).collect();
    assert_eq!(edges, expected);
}

#[test]
fn weak_pixels_survive_only_when_connected_to_strong_ones() {
    let mut g = column(2, &[400, 200, 200, 0, 0]);
    g.dx[4 * 5 + 4] = 200;
    let edges = canny(&g, 100, 300);
    assert!(edges[2] && edges[7] && edges[12]);
    assert!(!edges[24]);
}

#[test]
fn uniform_gradient_has_no_edges() {
    let g = Gradient {
        width: 5,
        height: 5,
        dx: vec![0; 25],
        dy: vec![0; 25],
    };
    assert!(canny(&g, 1, 2).iter().all(|&e| !e));
}
