use crate::ncc::Map;
use crate::ncc_reference_test::lcg_bytes;
use crate::refine::{peak_offset, vertex_offset};

fn det3(m: [[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// Replaces column `col` of the Vandermonde matrix by `values` (Cramer's rule).
fn with_column(mut m: [[f64; 3]; 3], col: usize, values: [f64; 3]) -> [[f64; 3]; 3] {
    for (row, v) in m.iter_mut().zip(values) {
        row[col] = v;
    }
    m
}

/// Vertex of the degree-2 polynomial `c0 + c1 x + c2 x^2` through `(-1, a)`, `(0, b)`, `(1, c)`,
/// with the coefficients solved by Cramer's rule on the Vandermonde system. Independent of the
/// closed form in `vertex_offset`, like the MathNet fit of the original.
fn fitted_vertex(a: f64, b: f64, c: f64) -> Option<f64> {
    let vandermonde = [[1.0, -1.0, 1.0], [1.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
    let det = det3(vandermonde);
    let c1 = det3(with_column(vandermonde, 1, [a, b, c])) / det;
    let c2 = det3(with_column(vandermonde, 2, [a, b, c])) / det;
    (c2 != 0.0).then(|| -c1 / (2.0 * c2))
}

fn parabola(vertex: f64, x: f64) -> f64 {
    5.0 - (x - vertex) * (x - vertex)
}

fn map3(values: [f32; 9]) -> Map {
    Map {
        width: 3,
        height: 3,
        values: values.to_vec(),
    }
}

#[test]
fn closed_form_equals_the_three_point_polynomial_fit() {
    let bytes = lcg_bytes(9, 300);
    for triple in bytes.chunks_exact(3) {
        let [a, b, c] = [
            f64::from(triple[0]),
            f64::from(triple[1]),
            f64::from(triple[2]),
        ];
        match (vertex_offset(a, b, c), fitted_vertex(a, b, c)) {
            (Some(closed), Some(fitted)) => assert!((closed - fitted).abs() < 1e-9, "{a} {b} {c}"),
            (None, None) => {}
            other => panic!("disagree for {a} {b} {c}: {other:?}"),
        }
    }
}

#[test]
fn recovers_the_vertex_of_a_known_parabola() {
    for vertex in [-0.4, 0.0, 0.3, 0.45] {
        let found = vertex_offset(
            parabola(vertex, -1.0),
            parabola(vertex, 0.0),
            parabola(vertex, 1.0),
        );
        assert!((found.unwrap() - vertex).abs() < 1e-12, "vertex {vertex}");
    }
}

#[test]
fn collinear_samples_have_no_vertex() {
    assert_eq!(vertex_offset(1.0, 2.0, 3.0), None);
}

#[test]
fn peak_offset_refines_each_axis() {
    let value = |x: f32, y: f32| 0.5 - 0.1 * (x - 1.25).powi(2) - 0.1 * (y - 0.9).powi(2);
    let mut values = [0.0f32; 9];
    for (i, v) in values.iter_mut().enumerate() {
        *v = value((i % 3) as f32, (i / 3) as f32);
    }
    let (dx, dy) = peak_offset(&map3(values), 1, 1);
    assert!((dx - 0.25).abs() < 1e-5);
    assert!((dy + 0.1).abs() < 1e-5);
}

#[test]
fn border_peaks_are_not_refined() {
    let map = map3([0.1, 0.9, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]);
    assert_eq!(peak_offset(&map, 1, 0), (0.0, 0.0));
    assert_eq!(peak_offset(&map, 0, 1), (0.0, 0.0));
}

#[test]
fn peak_of_one_is_not_refined() {
    let map = map3([0.1, 0.2, 0.3, 0.4, 1.0, 0.5, 0.6, 0.7, 0.8]);
    assert_eq!(peak_offset(&map, 1, 1), (0.0, 0.0));
}

#[test]
fn collinear_neighbours_keep_the_integer_peak() {
    let map = map3([0.5, 0.25, 0.5, 0.25, 0.5, 0.75, 0.5, 0.25, 0.5]);
    assert_eq!(peak_offset(&map, 1, 1), (0.0, 0.0));
}
