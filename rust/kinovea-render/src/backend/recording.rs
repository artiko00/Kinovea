//! Recording backend: stores every `Renderer` call with its arguments and the transform in force, so
//! tests can assert the painted scene without a rasterizer. Text metrics are fixed (see `measure_text`).

pub mod call;
#[cfg(test)]
mod call_test;
mod render;
#[cfg(test)]
mod render_test;
mod stack;
#[cfg(test)]
mod stack_test;

pub use call::Call;

use crate::renderer::Transform;
use kinovea_geometry::RectangleF;

#[derive(Debug)]
pub struct RecordingRenderer {
    pub calls: Vec<Call>,
    transform: Transform,
    transforms: Vec<Transform>,
    clip: RectangleF,
    clips: Vec<RectangleF>,
}

impl RecordingRenderer {
    /// A surface of `width` by `height` pixels; the clip starts as the whole surface.
    pub fn new(width: f32, height: f32) -> Self {
        RecordingRenderer {
            calls: Vec::new(),
            transform: Transform::identity(),
            transforms: Vec::new(),
            clip: RectangleF::new(0.0, 0.0, width, height),
            clips: Vec::new(),
        }
    }

    /// Records a drawing call built with the transform in force.
    fn draw(&mut self, make: impl FnOnce(Transform) -> Call) {
        let call = make(self.transform);
        self.calls.push(call);
    }
}
