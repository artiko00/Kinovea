use crate::camera_poser::compute_camera_position;
use crate::distortion::DistortionParameters;
use kinovea_geometry::{PointF, ProjectiveMapper, Quadrilateral, Size, Vector3};

/// Pinhole projection of a point of the rectangle plane (grid coordinates, y down) seen from
/// `cam` looking at the rectangle centre with world z up.
fn project(cam: Vector3, target: Vector3, grid: PointF, height: f32, fx: f32, cx: f32, cy: f32) -> PointF {
    let world = Vector3::new(grid.x, height - grid.y, 0.0);
    let mut f = target - cam;
    f.normalize();
    let mut r = f.cross(Vector3::new(0.0, 0.0, 1.0));
    r.normalize();
    let u = r.cross(f);
    let d = world - cam;
    let (xc, yc, zc) = (d.dot(r), d.dot(u), d.dot(f));
    PointF::new(fx * xc / zc + cx, cy - fx * yc / zc)
}

#[test]
fn synthetic_camera_is_recovered() {
    let (w, h) = (4.0f32, 3.0f32);
    let plane = Quadrilateral::from_size(w, h);
    let cam = Vector3::new(2.0, -5.0, 3.0);
    let target = Vector3::new(w / 2.0, h / 2.0, 0.0);
    let (fx, cx, cy) = (1000.0, 960.0, 540.0);
    let image = Quadrilateral {
        vertices: plane.vertices.map(|v| project(cam, target, v, h, fx, cx, cy)),
    };
    let mapping = ProjectiveMapper::new(&plane, &image).unwrap();
    let lens = DistortionParameters::new(0.0, 0.0, 0.0, 0.0, 0.0, 1000.0, 1000.0, 960.0, 540.0, 300.0);
    let pos = compute_camera_position(&plane, &mapping, &lens);
    assert!((pos.x - 2.0).abs() < 1e-2, "{pos:?}");
    assert!((pos.y + 5.0).abs() < 1e-2, "{pos:?}");
    assert!((pos.z - 3.0).abs() < 1e-2, "{pos:?}");
}

#[test]
fn centred_camera_sits_above_the_middle() {
    let plane = Quadrilateral::from_size(4.0, 3.0);
    let cam = Vector3::new(2.0, 1.5, 6.0);
    // Looking straight down needs an up vector other than z; use a slightly offset target.
    let target = Vector3::new(2.0, 1.5001, 0.0);
    let image = Quadrilateral {
        vertices: plane.vertices.map(|v| project(cam, target, v, 3.0, 1000.0, 960.0, 540.0)),
    };
    let mapping = ProjectiveMapper::new(&plane, &image).unwrap();
    let lens = DistortionParameters::defaults(Size::new(1920, 1080));
    let lens = DistortionParameters::new(0.0, 0.0, 0.0, 0.0, 0.0, 1000.0, 1000.0, lens.cx, lens.cy, lens.pixels_per_millimeter);
    let pos = compute_camera_position(&plane, &mapping, &lens);
    assert!((pos.x - 2.0).abs() < 2e-2 && (pos.y - 1.5).abs() < 2e-2 && (pos.z - 6.0).abs() < 2e-2, "{pos:?}");
}
