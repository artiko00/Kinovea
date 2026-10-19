//! Port of `CoordinateSystemGridFinder.FindForPlaneCalibration`. The original caches the result in the
//! calibration; here it is computed on each paint, from the same inputs. The debug helpers of the
//! original (`ExportImage`, random quads and strings) are not ported.

use super::grid::CoordinateSystemGrid;
use super::grid_lines::{create_lines, PlaneFrame, Walk};
use super::range::find_usable_step_size;
use kinovea_geometry::helpers::distance;
use kinovea_geometry::{PointF, Quadrilateral, RectangleF, SizeF, Vector3};
use kinovea_measurement::{CalibrationHelper, CalibratorPlane};

/// Number of divisions the finder aims for in each direction.
const TARGET_STEPS: f32 = 15.0;
/// Size of the extended plane of a perspective calibration, in plane sizes around the origin.
const PERSPECTIVE_EXTENT: f32 = 4.0;

/// The extended plane and the steps of one branch of the finder.
struct Extent {
    quad: Quadrilateral,
    orthogonal: bool,
    step_h: f32,
    step_v: f32,
}

/// `Find`: axes, grid lines and graduations of the calibrated plane.
pub fn find(calibration: &CalibrationHelper) -> CoordinateSystemGrid {
    let image = calibration.image_size();
    let image_size = SizeF::new(image.width as f32, image.height as f32);
    let clip = RectangleF::new(0.0, 0.0, image_size.width, image_size.height);
    let calibrator = calibration.calibrator();
    let extent = if calibrator.quad_image().is_axis_aligned_rectangle() {
        flat_extent(calibration, clip)
    } else {
        perspective_extent(calibrator, image_size)
    };
    let frame = PlaneFrame {
        calibrator,
        clip,
        plane: calibrator.size,
        extended: extent.quad,
        orthogonal: extent.orthogonal,
    };
    let mut grid = CoordinateSystemGrid::default();
    let vanish_y = vanishing_point(calibrator, Vector3::new(0.0, 1.0, 0.0), clip);
    create_lines(
        &mut grid,
        &frame,
        Walk::Vertical,
        0.0,
        -extent.step_h,
        vanish_y,
    );
    create_lines(
        &mut grid,
        &frame,
        Walk::Vertical,
        extent.step_h,
        extent.step_h,
        vanish_y,
    );
    let vanish_x = vanishing_point(calibrator, Vector3::new(1.0, 0.0, 0.0), clip);
    create_lines(
        &mut grid,
        &frame,
        Walk::Horizontal,
        0.0,
        -extent.step_v,
        vanish_x,
    );
    create_lines(
        &mut grid,
        &frame,
        Walk::Horizontal,
        extent.step_v,
        extent.step_v,
        vanish_x,
    );
    grid
}

/// Flat plane: the image bounds reprojected to plane space. No vanishing point can be inside the image.
fn flat_extent(calibration: &CalibrationHelper, clip: RectangleF) -> Extent {
    let [a, b, c, d] = Quadrilateral::from_rect(clip)
        .vertices
        .map(|p| calibration.point_from_rectified(p));
    let width = b.x - a.x;
    let height = a.y - d.y;
    Extent {
        quad: Quadrilateral::new(a, b, c, d),
        orthogonal: true,
        step_h: find_usable_step_size(width, TARGET_STEPS),
        step_v: find_usable_step_size(height, TARGET_STEPS),
    }
}

/// Perspective plane: the plane extended around the origin, with steps from the length of the image
/// edges that are scaled to the image size.
fn perspective_extent(calibrator: &CalibratorPlane, image_size: SizeF) -> Extent {
    let plane = calibrator.size;
    let n = PERSPECTIVE_EXTENT;
    let (w, h) = (plane.width * n, plane.height * n);
    let quad = Quadrilateral::new(
        PointF::new(-w, h),
        PointF::new(w, h),
        PointF::new(w, -h),
        PointF::new(-w, -h),
    );
    let image = calibrator.quad_image();
    let projected_width = distance(image.a(), image.b());
    let scaled_h = TARGET_STEPS / (image_size.width / projected_width);
    let projected_height = distance(image.a(), image.d());
    let scaled_v = TARGET_STEPS / (image_size.height / projected_height);
    Extent {
        quad,
        orthogonal: false,
        step_h: find_usable_step_size(plane.width, scaled_h),
        step_v: find_usable_step_size(plane.height, scaled_v),
    }
}

/// Image point of a direction at infinity, when it projects in front of the camera and inside the image.
fn vanishing_point(
    calibrator: &CalibratorPlane,
    direction: Vector3,
    clip: RectangleF,
) -> Option<PointF> {
    let v = calibrator.project_vector3(direction);
    if v.z == 0.0 {
        return None;
    }
    let p = PointF::new(v.x / v.z, v.y / v.z);
    clip.contains(p).then_some(p)
}
