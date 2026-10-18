use crate::point::PointF;
use crate::projective::ProjectiveMapper;
use crate::quadrilateral::Quadrilateral;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

#[test]
fn identity_maps_the_unit_circle_to_the_unit_ellipse() {
    let e = ProjectiveMapper::default().ellipse();
    assert!(e.center.x.abs() < 1e-6 && e.center.y.abs() < 1e-6);
    assert!((e.semi_major_axis - 1.0).abs() < 1e-6);
    assert!((e.semi_minor_axis - 1.0).abs() < 1e-6);
    // A circle has a == c and b == 0: the original assigns rotation π/2.
    assert!((e.rotation - std::f32::consts::FRAC_PI_2).abs() < 1e-6);
}

#[test]
fn affine_scaling_gives_the_expected_axes() {
    let plane = Quadrilateral::centered_unit_square();
    let image = Quadrilateral::new(p(-4.0, 1.0), p(4.0, 1.0), p(4.0, -1.0), p(-4.0, -1.0));
    let mut moved = image;
    moved.translate(10.0, 20.0);
    let e = ProjectiveMapper::new(&plane, &moved).unwrap().ellipse();
    assert!(
        (e.center.x - 10.0).abs() < 1e-3 && (e.center.y - 20.0).abs() < 1e-3,
        "{e:?}"
    );
    assert!((e.semi_major_axis - 4.0).abs() < 1e-3, "{e:?}");
    assert!((e.semi_minor_axis - 1.0).abs() < 1e-3, "{e:?}");
    assert!(e.rotation.abs() < 1e-4, "{e:?}");
}

#[test]
fn vertical_scaling_rotates_the_major_axis() {
    let plane = Quadrilateral::centered_unit_square();
    let image = Quadrilateral::new(p(-1.0, 3.0), p(1.0, 3.0), p(1.0, -3.0), p(-1.0, -3.0));
    let e = ProjectiveMapper::new(&plane, &image).unwrap().ellipse();
    assert!((e.semi_major_axis - 3.0).abs() < 1e-3, "{e:?}");
    assert!((e.semi_minor_axis - 1.0).abs() < 1e-3, "{e:?}");
    assert!(
        (e.rotation - std::f32::consts::FRAC_PI_2).abs() < 1e-4,
        "{e:?}"
    );
    assert!(e.is_vertical());
}
