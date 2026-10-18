use crate::lib_test::Rng;
use crate::point::PointF;
use crate::projective::{DegenerateQuadrilateral, ProjectiveMapper};
use crate::quadrilateral::Quadrilateral;
use crate::vector3::Vector3;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn near(a: PointF, b: PointF, tol: f32) -> bool {
    (a.x - b.x).abs() < tol && (a.y - b.y).abs() < tol
}

fn plane() -> Quadrilateral {
    Quadrilateral::new(p(0.0, 0.0), p(4.0, 0.0), p(4.0, 3.0), p(0.0, 3.0))
}

fn image() -> Quadrilateral {
    Quadrilateral::new(
        p(100.0, 80.0),
        p(500.0, 60.0),
        p(520.0, 400.0),
        p(90.0, 380.0),
    )
}

#[test]
fn corners_map_to_corners_both_ways() {
    let m = ProjectiveMapper::new(&plane(), &image()).unwrap();
    for (pl, im) in plane().vertices.iter().zip(image().vertices) {
        assert!(
            near(m.forward(*pl), im, 1e-3),
            "{pl:?} -> {:?}",
            m.forward(*pl)
        );
        assert!(
            near(m.backward(im), *pl, 1e-3),
            "{im:?} -> {:?}",
            m.backward(im)
        );
    }
    assert_eq!(m.forward_quad(&plane()).vertices.len(), 4);
    assert!(near(m.backward_quad(&image()).c(), p(4.0, 3.0), 1e-3));
}

#[test]
fn seeded_round_trips_return_to_the_origin_point() {
    let m = ProjectiveMapper::new(&plane(), &image()).unwrap();
    let mut rng = Rng::new(99);
    for _ in 0..100 {
        let q = p(rng.range(0.0, 4.0), rng.range(0.0, 3.0));
        let back = m.backward(m.forward(q));
        assert!(near(back, q, 1e-3), "{q:?} -> {back:?}");
    }
}

#[test]
fn identity_default_and_affine_parallelogram() {
    let id = ProjectiveMapper::default();
    assert_eq!(id.forward(p(3.0, 4.0)), p(3.0, 4.0));
    let para = Quadrilateral::new(p(10.0, 10.0), p(30.0, 10.0), p(40.0, 25.0), p(20.0, 25.0));
    let m = ProjectiveMapper::new(&Quadrilateral::unit_square(), &para).unwrap();
    assert!(near(m.forward(p(0.5, 0.5)), p(25.0, 17.5), 1e-4));
}

#[test]
fn degenerate_quad_leaves_mapping_unchanged() {
    let mut m = ProjectiveMapper::new(&plane(), &image()).unwrap();
    let before = m.clone();
    let flat = Quadrilateral::new(p(0.0, 0.0), p(1.0, 0.0), p(2.0, 0.0), p(3.0, 0.0));
    assert_eq!(m.update(&plane(), &flat), Err(DegenerateQuadrilateral));
    assert_eq!(m, before);
    assert_eq!(m.update(&flat, &image()), Err(DegenerateQuadrilateral));
    assert_eq!(m, before);
    assert!(ProjectiveMapper::new(&plane(), &flat).is_err());
    assert_eq!(
        DegenerateQuadrilateral.to_string(),
        "degenerate quadrilateral: no projective mapping"
    );
}

#[test]
fn homogeneous_forward_matches_point_forward() {
    let m = ProjectiveMapper::new(&plane(), &image()).unwrap();
    let v = m.forward_vector3(Vector3::new(1.0, 2.0, 1.0));
    assert!(near(p(v.x / v.z, v.y / v.z), m.forward(p(1.0, 2.0)), 1e-2));
}

#[test]
fn backward_vector3_reproduces_original_typo() {
    let m = ProjectiveMapper::new(&plane(), &image()).unwrap();
    let v = m.backward_vector3(Vector3::new(300.0, 200.0, 1.0));
    let exact = m.backward(p(300.0, 200.0));
    // x and z use the inverse matrix; y mixes in the forward matrix's [1][2] coefficient.
    assert!((v.x / v.z - exact.x).abs() < 1e-3, "{v:?} vs {exact:?}");
    assert!(
        (v.y / v.z - exact.y).abs() > 1e-3,
        "typo not reproduced: {v:?} vs {exact:?}"
    );
}
