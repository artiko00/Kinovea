use crate::distortion::DistortionParameters;
use kinovea_geometry::Size;

#[test]
fn defaults_for_full_hd() {
    let p = DistortionParameters::defaults(Size::new(1920, 1080));
    let ppmm = 1920.0 / 6.16;
    assert!((p.pixels_per_millimeter - ppmm).abs() < 1e-12);
    assert!((p.fx - 2.77 * ppmm).abs() < 1e-9);
    assert_eq!(p.fx, p.fy);
    assert_eq!((p.cx, p.cy), (960.0, 540.0));
    assert_eq!((p.k1, p.k2, p.k3, p.p1, p.p2), (0.0, 0.0, 0.0, 0.0, 0.0));
    assert_eq!(p.path, "");
}

#[test]
fn matrix_and_coefficient_layout() {
    let p = DistortionParameters::for_image(0.1, 0.2, 0.3, 0.4, 0.5, 1000.0, 1100.0, 600.0, 400.0, Size::new(1232, 700));
    assert_eq!(p.camera_matrix(), [[1000.0, 0.0, 600.0], [0.0, 1100.0, 400.0], [0.0, 0.0, 1.0]]);
    assert_eq!(p.dist_coeffs(), [0.1, 0.2, 0.4, 0.5, 0.3]);
    assert!((p.pixels_per_millimeter - 200.0).abs() < 1e-12);
}

#[test]
fn content_hash_is_stable_and_sensitive() {
    let a = DistortionParameters::defaults(Size::new(1920, 1080));
    let b = DistortionParameters::defaults(Size::new(1920, 1080));
    assert_eq!(a.content_hash(), b.content_hash());
    let mut c = b.clone();
    c.k1 = 0.01;
    assert_ne!(a.content_hash(), c.content_hash());
    c.path = "file.xml".into();
    assert_eq!(c.content_hash(), c.clone().content_hash());
}
