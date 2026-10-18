use crate::matrix3x3::Matrix3x3;
use crate::vector3::Vector3;
use std::f32::consts::FRAC_PI_2;

fn close(a: Vector3, b: Vector3) -> bool {
    (a - b).abs().max() < 1e-6
}

#[test]
fn quarter_turns_move_axes_onto_each_other() {
    let x = Vector3::new(1.0, 0.0, 0.0);
    let y = Vector3::new(0.0, 1.0, 0.0);
    let z = Vector3::new(0.0, 0.0, 1.0);
    assert!(close(Matrix3x3::rotation_z(FRAC_PI_2) * x, y));
    assert!(close(Matrix3x3::rotation_x(FRAC_PI_2) * y, z));
    assert!(close(Matrix3x3::rotation_y(FRAC_PI_2) * z, x));
}

#[test]
fn rotations_have_unit_determinant() {
    for r in [
        Matrix3x3::rotation_x(0.7),
        Matrix3x3::rotation_y(-1.3),
        Matrix3x3::rotation_z(2.9),
        Matrix3x3::from_yaw_pitch_roll(0.3, -0.2, 0.5),
    ] {
        assert!((r.determinant() - 1.0).abs() < 1e-6);
    }
}
