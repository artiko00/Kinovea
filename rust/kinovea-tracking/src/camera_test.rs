use crate::camera::CameraTransform;
use kinovea_geometry::PointF;
use std::cell::Cell;

/// Camera that moves points right by one unit per millisecond, counting the transforms it runs.
pub(crate) struct ShiftCamera {
    on: bool,
    pub(crate) calls: Cell<usize>,
}

impl ShiftCamera {
    pub(crate) fn on() -> Self {
        ShiftCamera {
            on: true,
            calls: Cell::new(0),
        }
    }

    pub(crate) fn off() -> Self {
        ShiftCamera {
            on: false,
            calls: Cell::new(0),
        }
    }
}

impl CameraTransform for ShiftCamera {
    fn initialized(&self) -> bool {
        self.on
    }

    fn transform(&self, source_t: i64, target_t: i64, point: PointF) -> PointF {
        self.calls.set(self.calls.get() + 1);
        PointF::new(point.x + (target_t - source_t) as f32, point.y)
    }
}

#[test]
fn shift_camera_moves_points_by_the_elapsed_time() {
    let camera = ShiftCamera::on();
    let dyn_camera: &dyn CameraTransform = &camera;
    assert!(dyn_camera.initialized());
    assert_eq!(
        dyn_camera.transform(10, 14, PointF::new(1.0, 2.0)),
        PointF::new(5.0, 2.0)
    );
    assert_eq!(camera.calls.get(), 1);
}
