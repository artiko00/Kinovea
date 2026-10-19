//! Naming and lookup helpers of [`Metadata`].

use super::{Manager, Metadata};
use crate::drawing::Drawing;
use uuid::Uuid;

impl Metadata {
    /// `SetDrawingName`: `"<tool> <n>"` with the first `n` not used by any drawing, counting
    /// `extra` names reserved during the same import step.
    pub fn next_name(&self, tool: &str, extra: &[String]) -> String {
        let taken = |name: &str| self.all_drawings().any(|d| d.header().name == name) || extra.iter().any(|e| e == name);
        (1..).map(|i| format!("{tool} {i}")).find(|n| !taken(n)).unwrap_or_default()
    }

    /// Gives empty-named drawings their automatic name, in order (`AfterDrawingCreation`).
    pub fn name_untitled(&self, drawings: &mut [Drawing]) {
        let mut given: Vec<String> = Vec::new();
        for d in drawings.iter_mut() {
            if d.header().name.is_empty() {
                let name = self.next_name(&d.tool_display_name(), &given);
                d.header_mut().name = name.clone();
                given.push(name);
            }
        }
    }

    /// Attached drawings, chronos and tracks.
    pub fn all_drawings(&self) -> impl Iterator<Item = &Drawing> {
        self.attached_drawings().chain(self.chronos.iter()).chain(self.tracks.iter())
    }

    /// Finds a drawing anywhere, with its manager.
    pub fn find_drawing(&self, id: Uuid) -> Option<(Manager, &Drawing)> {
        for kf in &self.keyframes {
            if let Some(d) = kf.drawing(id) {
                return Some((Manager::Keyframe(kf.id), d));
            }
        }
        if let Some(d) = self.chronos.iter().find(|d| d.id() == id) {
            return Some((Manager::Chronos, d));
        }
        self.tracks.iter().find(|d| d.id() == id).map(|d| (Manager::Tracks, d))
    }
}
