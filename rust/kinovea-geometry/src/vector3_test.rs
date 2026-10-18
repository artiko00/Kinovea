use crate::vector3::Vector3;

#[test]
fn dot_cross_and_norm() {
    let a = Vector3::new(1.0, 0.0, 0.0);
    let b = Vector3::new(0.0, 1.0, 0.0);
    assert_eq!(a.cross(b), Vector3::new(0.0, 0.0, 1.0));
    assert_eq!(b.cross(a), Vector3::new(0.0, 0.0, -1.0));
    assert_eq!(a.dot(b), 0.0);
    assert_eq!(Vector3::new(3.0, 4.0, 12.0).norm(), 13.0);
    assert_eq!(Vector3::new(1.0, 2.0, 3.0).square(), 14.0);
}

#[test]
fn min_max_and_indices() {
    let v = Vector3::new(3.0, -1.0, 7.0);
    assert_eq!(v.max(), 7.0);
    assert_eq!(v.min(), -1.0);
    assert_eq!(v.max_index(), 2);
    assert_eq!(v.min_index(), 1);
    assert_eq!(Vector3::splat(2.0).max_index(), 0);
    assert_eq!(Vector3::splat(2.0).min_index(), 0);
    assert_eq!(Vector3::new(1.0, 5.0, 5.0).max_index(), 1);
}

#[test]
fn component_and_scalar_operators() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(2.0, 2.0, 2.0);
    assert_eq!(a + b, Vector3::new(3.0, 4.0, 5.0));
    assert_eq!(a - b, Vector3::new(-1.0, 0.0, 1.0));
    assert_eq!(a * b, Vector3::new(2.0, 4.0, 6.0));
    assert_eq!(a / b, Vector3::new(0.5, 1.0, 1.5));
    assert_eq!(a + 1.0, Vector3::new(2.0, 3.0, 4.0));
    assert_eq!(a - 1.0, Vector3::new(0.0, 1.0, 2.0));
    assert_eq!(a * 2.0, Vector3::new(2.0, 4.0, 6.0));
    assert_eq!(a / 2.0, Vector3::new(0.5, 1.0, 1.5));
    assert_eq!(a.negate(), Vector3::new(-1.0, -2.0, -3.0));
    assert_eq!(a.negate().abs(), a);
    assert_eq!(a.to_array(), [1.0, 2.0, 3.0]);
}

#[test]
fn normalize_returns_previous_norm_and_inverse_skips_zeros() {
    let mut v = Vector3::new(0.0, 3.0, 4.0);
    assert_eq!(v.normalize(), 5.0);
    assert!((v.norm() - 1.0).abs() < 1e-6);
    assert_eq!(
        Vector3::new(0.0, 2.0, 4.0).inverse(),
        Vector3::new(0.0, 0.5, 0.25)
    );
}
