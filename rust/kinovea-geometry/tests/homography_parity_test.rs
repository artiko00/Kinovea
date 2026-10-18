//! Parity of `ProjectiveMapper` with `tests/fixtures/geometry/homography/`.

mod parity_support;
use parity_support::*;

use kinovea_geometry::{ProjectiveMapper, Quadrilateral, Vector3};

fn mappers() -> Vec<(String, ProjectiveMapper, Vec<String>)> {
    rows("homography/cases.csv")
        .into_iter()
        .map(|r| {
            let plane = Quadrilateral::new(pt(&r, 1), pt(&r, 3), pt(&r, 5), pt(&r, 7));
            let image = Quadrilateral::new(pt(&r, 9), pt(&r, 11), pt(&r, 13), pt(&r, 15));
            (
                r[0].clone(),
                ProjectiveMapper::new(&plane, &image).unwrap(),
                r,
            )
        })
        .collect()
}

#[test]
fn ellipse_matches_within_1e_3_and_rotation_1e_4() {
    let cases = mappers();
    assert_eq!(cases.len(), 20);
    for (id, m, r) in &cases {
        let e = m.ellipse();
        assert_pt(
            e.center,
            pt(r, 17),
            1e-3 * scale(&r[17..19]),
            &format!("case {id} ellipse centre"),
        );
        assert_f(
            f64::from(e.semi_major_axis),
            f(&r[19]),
            1e-3 * scale(&r[19..21]),
            &format!("case {id} a"),
        );
        assert_f(
            f64::from(e.semi_minor_axis),
            f(&r[20]),
            1e-3 * scale(&r[19..21]),
            &format!("case {id} b"),
        );
        assert_f(
            f64::from(e.rotation),
            f(&r[21]),
            1e-4,
            &format!("case {id} rotation"),
        );
    }
}

#[test]
fn point_mapping_matches_within_1e_3() {
    let cases = mappers();
    let rows = rows("homography/points.csv");
    assert_eq!(rows.len(), 200);
    for r in &rows {
        let (_, m, _) = cases.iter().find(|c| c.0 == r[0]).unwrap();
        assert_pt(
            m.forward(pt(r, 1)),
            pt(r, 3),
            1e-3,
            &format!("case {} forward", r[0]),
        );
        assert_pt(
            m.backward(pt(r, 5)),
            pt(r, 7),
            1e-3 * scale(&r[7..9]).max(1.0),
            &format!("case {} backward", r[0]),
        );
    }
}

#[test]
fn homogeneous_vectors_match_including_the_backward_typo() {
    let cases = mappers();
    for r in &rows("homography/vectors.csv") {
        let (_, m, _) = cases.iter().find(|c| c.0 == r[0]).unwrap();
        let h = Vector3::new(f(&r[1]) as f32, f(&r[2]) as f32, f(&r[3]) as f32);
        let fw = m.forward_vector3(h);
        let bw = m.backward_vector3(h);
        for (k, (got, want)) in [fw.x, fw.y, fw.z, bw.x, bw.y, bw.z]
            .iter()
            .zip(&r[4..10])
            .enumerate()
        {
            let want = f(want);
            assert_f(
                f64::from(*got),
                want,
                1e-3 * want.abs().max(1.0),
                &format!("case {} vector column {k}", r[0]),
            );
        }
    }
}
