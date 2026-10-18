use crate::svd::diagonalize::pythag;
use crate::svd::sign;

#[test]
fn pythag_is_a_stable_hypotenuse() {
    assert_eq!(pythag(3.0, 4.0), 5.0);
    assert_eq!(pythag(-4.0, 3.0), 5.0);
    assert_eq!(pythag(0.0, 0.0), 0.0);
    assert!((pythag(1e200, 1e200) - 2f64.sqrt() * 1e200).abs() < 1e186);
}

#[test]
fn sign_transfers_the_sign_of_the_second_argument() {
    assert_eq!(sign(2.0, 1.0), 2.0);
    assert_eq!(sign(-2.0, 1.0), 2.0);
    assert_eq!(sign(2.0, -1.0), -2.0);
    assert_eq!(sign(2.0, 0.0), 2.0);
}
