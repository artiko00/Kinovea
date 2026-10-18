//! Parity of quadrilateral operations and the SVD with `tests/fixtures/geometry/{quads,svd}`.

mod parity_support;
use parity_support::*;

use kinovea_geometry::svd::svdcmp;
use kinovea_geometry::{PointF, Quadrilateral};

#[test]
fn quadrilateral_operations_match_within_1e_4() {
    let rows = rows("quads/ops.csv");
    assert_eq!(rows.len(), 20 * 10);
    for r in &rows {
        let input = quad(r, 4);
        let (anchor, p1, p2) = (f(&r[1]) as usize, f(&r[2]) as f32, f(&r[3]) as f32);
        assert_eq!(input.is_convex(), r[20] == "1", "convex {input:?}");
        assert_eq!(
            input.is_axis_aligned_rectangle(),
            r[21] == "1",
            "axis {input:?}"
        );
        let mut q = input;
        match r[0].as_str() {
            "make_rectangle" => q.make_rectangle(anchor),
            "make_square" => q.make_square(anchor),
            "flip_horizontal" => q.flip_horizontal(),
            "flip_vertical" => q.flip_vertical(),
            "rotate90" => q.rotate90(),
            "expand" => q.expand(p1, p2),
            "translate" => q.translate(p1, p2),
            "scale" => q.scale(p1, p2),
            "bounding_box" => {
                let b = input.bounding_box();
                assert_pt(PointF::new(b.x, b.y), pt(r, 12), 1e-4, "bbox origin");
                assert_pt(PointF::new(b.width, b.height), pt(r, 14), 1e-4, "bbox size");
                continue;
            }
            "contains" => {
                assert_eq!(
                    input.contains(PointF::new(p1, p2)),
                    r[12] == "1",
                    "contains {input:?} ({p1}, {p2})"
                );
                continue;
            }
            other => panic!("unknown op {other}"),
        }
        let want = quad(r, 12);
        for (g, w) in q.vertices.iter().zip(want.vertices) {
            assert_pt(
                *g,
                w,
                1e-4 * scale(&r[12..20]),
                &format!("{} anchor {anchor} ({p1}, {p2}) {input:?}", r[0]),
            );
        }
    }
}

#[test]
fn svd_singular_values_match_within_1e_9_and_vectors_up_to_sign() {
    let rows = rows("svd/cases.csv");
    assert_eq!(rows.len(), 20);
    for r in &rows {
        let (m, n) = (f(&r[1]) as usize, f(&r[2]) as usize);
        let values = list(&r[3]);
        let a: Vec<Vec<f64>> = (0..m)
            .map(|i| values[i * n..(i + 1) * n].to_vec())
            .collect();
        let svd = svdcmp(&a).unwrap();
        let (w, u, v) = (list(&r[4]), list(&r[5]), list(&r[6]));
        for (k, want) in w.iter().enumerate() {
            assert_f(svd.w[k], *want, 1e-9, &format!("case {} w[{k}]", r[0]));
            let su = column_sign(&svd.u, &u, k, n);
            let sv = column_sign(&svd.v, &v, k, n);
            assert_eq!(
                su, sv,
                "case {} column {k}: U and V signs must flip together",
                r[0]
            );
            for i in 0..m {
                assert_f(
                    svd.u[i][k] * su,
                    u[i * n + k],
                    1e-9,
                    &format!("case {} u[{i}][{k}]", r[0]),
                );
            }
            for i in 0..n {
                assert_f(
                    svd.v[i][k] * sv,
                    v[i * n + k],
                    1e-9,
                    &format!("case {} v[{i}][{k}]", r[0]),
                );
            }
        }
    }
}

fn column_sign(got: &[Vec<f64>], want: &[f64], k: usize, n: usize) -> f64 {
    let (i, _) = got
        .iter()
        .enumerate()
        .max_by(|a, b| a.1[k].abs().total_cmp(&b.1[k].abs()))
        .unwrap();
    if got[i][k].signum() == want[i * n + k].signum() {
        1.0
    } else {
        -1.0
    }
}

fn quad(r: &[String], at: usize) -> Quadrilateral {
    Quadrilateral::new(pt(r, at), pt(r, at + 2), pt(r, at + 4), pt(r, at + 6))
}
