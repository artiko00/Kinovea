use crate::lib_test::Rng;
use crate::matrix3x3::Matrix3x3;
use crate::vector3::Vector3;

fn assert_close(a: Matrix3x3, b: Matrix3x3, tol: f32) {
    for (ra, rb) in a.v.iter().zip(&b.v) {
        for (x, y) in ra.iter().zip(rb) {
            assert!((x - y).abs() < tol, "{a:?} vs {b:?}");
        }
    }
}

#[test]
fn identity_determinant_rows_columns_and_diagonal() {
    let m = Matrix3x3::from_rows(
        Vector3::new(1.0, 2.0, 3.0),
        Vector3::new(0.0, 1.0, 4.0),
        Vector3::new(5.0, 6.0, 0.0),
    );
    assert_eq!(Matrix3x3::IDENTITY.determinant(), 1.0);
    assert_eq!(m.determinant(), 1.0);
    assert_eq!(m.row(2), Vector3::new(5.0, 6.0, 0.0));
    assert_eq!(m.column(1), Vector3::new(2.0, 1.0, 6.0));
    assert_eq!(m.transpose().row(1), m.column(1));
    assert_eq!(
        Matrix3x3::from_columns(m.column(0), m.column(1), m.column(2)),
        m
    );
    assert_eq!(
        Matrix3x3::from_diagonal(Vector3::new(1.0, 2.0, 3.0)).to_array()[4],
        2.0
    );
    assert_eq!(m.to_array(), [1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0]);
    assert_eq!(
        m * Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(6.0, 5.0, 11.0)
    );
    assert_eq!(m * Matrix3x3::IDENTITY, m);
    assert_eq!((m + 1.0).v[1][0], 1.0);
    assert_eq!((m + m).v[0][2], 6.0);
    assert_eq!((m - m), Matrix3x3::default());
}

#[test]
fn inverse_of_a_rotation_is_its_transpose_and_singular_has_none() {
    let r = Matrix3x3::from_yaw_pitch_roll(0.3, -0.2, 0.5);
    assert_close(r * r.inverse().unwrap(), Matrix3x3::IDENTITY, 1e-5);
    assert_close(r.inverse().unwrap(), r.transpose(), 1e-5);
    assert!(Matrix3x3::default().inverse().is_none());
    let m = Matrix3x3::from_rows(
        Vector3::new(1.0, 2.0, 3.0),
        Vector3::new(0.0, 1.0, 4.0),
        Vector3::new(5.0, 6.0, 0.0),
    );
    assert_close(m * m.inverse().unwrap(), Matrix3x3::IDENTITY, 1e-5);
    assert_eq!(m.multiply_self_by_transpose(), m * m.transpose());
    assert_eq!(m.multiply_transpose_by_self(), m.transpose() * m);
}

#[test]
fn yaw_pitch_roll_round_trip() {
    let (yaw, pitch, roll) = Matrix3x3::from_yaw_pitch_roll(0.3, -0.2, 0.5).yaw_pitch_roll();
    assert!((yaw - 0.3).abs() < 1e-5);
    assert!((pitch + 0.2).abs() < 1e-5);
    assert!((roll - 0.5).abs() < 1e-5);
}

#[test]
fn seeded_rotations_times_inverse_are_identity() {
    let mut rng = Rng::new(11);
    for _ in 0..200 {
        let r = Matrix3x3::from_yaw_pitch_roll(
            rng.range(-3.0, 3.0),
            rng.range(-1.5, 1.5),
            rng.range(-3.0, 3.0),
        );
        assert_close(r * r.inverse().unwrap(), Matrix3x3::IDENTITY, 1e-5);
    }
}

#[test]
fn svd_and_pseudo_inverse_reconstruct() {
    let m = Matrix3x3::from_rows(
        Vector3::new(2.0, 0.0, 1.0),
        Vector3::new(-1.0, 3.0, 0.0),
        Vector3::new(0.5, 1.0, 4.0),
    );
    let (u, e, v) = m.svd().unwrap();
    assert_close(u * Matrix3x3::from_diagonal(e) * v.transpose(), m, 1e-4);
    assert_close(m.pseudo_inverse().unwrap(), m.inverse().unwrap(), 1e-4);
}
