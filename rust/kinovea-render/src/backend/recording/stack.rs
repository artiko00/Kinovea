//! Transform and clip stacks of the recording backend (`push_*`, `pop_*`, `reset_transform`).

use super::{Call, RecordingRenderer};
use crate::renderer::Transform;
use kinovea_geometry::RectangleF;

impl RecordingRenderer {
    /// Applies `transform` before the current one, as GDI+ prepends it.
    pub(super) fn enter_transform(&mut self, transform: Transform) {
        self.transforms.push(self.transform);
        self.transform = self.transform.multiply(&transform);
        self.calls.push(Call::PushTransform(transform));
    }

    pub(super) fn leave_transform(&mut self) {
        if let Some(previous) = self.transforms.pop() {
            self.transform = previous;
        }
        self.calls.push(Call::PopTransform);
    }

    pub(super) fn reset_current_transform(&mut self) {
        self.transform = Transform::identity();
        self.calls.push(Call::ResetTransform);
    }

    pub(super) fn enter_clip(&mut self, rect: RectangleF) {
        self.clips.push(self.clip);
        self.clip = intersect(self.clip, rect);
        self.calls.push(Call::PushClip(rect));
    }

    pub(super) fn leave_clip(&mut self) {
        if let Some(previous) = self.clips.pop() {
            self.clip = previous;
        }
        self.calls.push(Call::PopClip);
    }

    pub(super) fn current_clip(&self) -> RectangleF {
        self.clip
    }
}

fn intersect(a: RectangleF, b: RectangleF) -> RectangleF {
    let left = a.left().max(b.left());
    let top = a.top().max(b.top());
    let right = a.right().min(b.right());
    let bottom = a.bottom().min(b.bottom());
    RectangleF::new(left, top, (right - left).max(0.0), (bottom - top).max(0.0))
}
