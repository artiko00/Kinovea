//! Camera position relative to the calibration rectangle (`CameraPoser.Compute`, after
//! Wefelscheid 2011). Single precision throughout, as the original.

use crate::distortion::DistortionParameters;
use kinovea_geometry::{PointF, ProjectiveMapper, Quadrilateral, Vector3};

/// Returns the camera position in the rectangle basis: x from the left edge, y from the bottom
/// (C–D) edge, z towards the camera.
pub fn compute_camera_position(plane: &Quadrilateral, mapping: &ProjectiveMapper, lens: &DistortionParameters) -> Vector3 {
    let [a, b, c, d] = plane.vertices;
    let mid = |p: PointF, q: PointF| PointF::new((p.x + q.x) / 2.0, (p.y + q.y) / 2.0);
    let m = PointF::new((a.x + b.x) / 2.0, (a.y + d.y) / 2.0);
    let (cx, cy, fx, fy) = (lens.cx as f32, lens.cy as f32, lens.fx as f32, lens.fy as f32);
    let ray = |p: PointF| {
        let i = mapping.forward(p);
        Vector3::new((i.x - cx) / fx, -(i.y - cy) / fy, 1.0)
    };
    let (pp12, pp23, pp34, pp14, mp) = (ray(mid(a, b)), ray(mid(b, c)), ray(mid(c, d)), ray(mid(a, d)), ray(m));
    let angle = |v: Vector3| f64::from(v.dot(mp) / (v.norm() * mp.norm())).acos() as f32;
    let (alpha, beta, delta, gamma) = (angle(pp12), angle(pp34), angle(pp23), angle(pp14));

    let mut dist = 10.0f32;
    let law = |near: f32, far: f32| ((2.0 * f64::from(dist) * f64::from(near).sin()) / f64::from(near + far).sin()) as f32;
    let dp12 = law(beta, alpha);
    let dp14 = law(delta, gamma);
    let mut p12cam = pp12 / pp12.norm() * dp12;
    let mut p14cam = pp14 / pp14.norm() * dp14;
    let mut mcam = mp / mp.norm() * dist;
    let (ucam, vcam) = (p12cam - mcam, p14cam - mcam);
    let width = b.x - a.x;
    let height = d.y - a.y;
    let scale = (ucam.norm() * 2.0 / height + vcam.norm() * 2.0 / width) / 2.0;
    dist /= scale;
    let _ = dist;
    p12cam = p12cam / scale;
    p14cam = p14cam / scale;
    mcam = mcam / scale;
    let (ucam, vcam) = (p12cam - mcam, p14cam - mcam);
    let to_cam = mcam.negate();
    let xdir = (vcam / vcam.norm()).negate();
    let ydir = ucam / ucam.norm();
    let zdir = ydir.cross(xdir);
    Vector3::new(to_cam.dot(xdir) + width / 2.0, to_cam.dot(ydir) + height / 2.0, to_cam.dot(zdir))
}
