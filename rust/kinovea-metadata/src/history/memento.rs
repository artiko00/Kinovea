//! A reversible change (`HistoryMemento`): its command name and the serialized state that
//! undoing it restores. The state is kept as KVA text, as the original does.

use crate::kva_io::Filter;
use crate::metadata::{Manager, Metadata};
use crate::trackability::DrawingTracker;
use kinovea_core::preferences::dom::Element;
use uuid::Uuid;

use super::{drawing, keyframe, multi_items, video_filter};

/// What undoing a memento restores. Ids are kept next to the data, so that an empty data string
/// (the state is gone) still names the target.
#[derive(Debug, Clone, PartialEq)]
pub enum Change {
    AddDrawing { manager: Manager, id: Uuid },
    DeleteDrawing { manager: Manager, id: Uuid, data: String },
    ModifyDrawing { manager: Manager, id: Uuid, filter: Filter, data: String },
    AddKeyframe { id: Uuid },
    DeleteKeyframe { id: Uuid, data: String },
    ModifyKeyframe { id: Uuid, data: String },
    AddMultiItem { id: Uuid },
    DeleteMultiItem { id: Uuid, data: String },
    ModifyVideoFilter { name: String, data: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Memento {
    pub(super) name: String,
    pub(super) change: Change,
}

impl Memento {
    pub fn command_name(&self) -> &str {
        &self.name
    }

    /// Restores the state this memento holds and returns the memento that undoes that restoration.
    /// The command name is kept, so both stacks show the same name for one operation.
    pub fn perform_undo(self, m: &mut Metadata) -> Memento {
        let change = match self.change {
            Change::AddDrawing { manager, id } => drawing::undo_add(m, manager, id),
            Change::DeleteDrawing { manager, id, data } => drawing::undo_delete(m, manager, id, &data),
            Change::ModifyDrawing { manager, id, filter, data } => drawing::undo_modify(m, manager, id, filter, &data),
            Change::AddKeyframe { id } => keyframe::undo_add(m, id),
            Change::DeleteKeyframe { id, data } => keyframe::undo_delete(m, id, &data),
            Change::ModifyKeyframe { id, data } => keyframe::undo_modify(m, id, &data),
            Change::AddMultiItem { id } => multi_items::undo_add(m, id),
            Change::DeleteMultiItem { id, data } => multi_items::undo_delete(m, id, &data),
            Change::ModifyVideoFilter { name, data } => video_filter::undo_modify(m, &name, &data),
        };
        Memento { name: self.name, change }
    }
}

/// `string.Format("{0} ({1})", action, subject)` of the original.
pub(super) fn named(action: &str, subject: &str) -> String {
    format!("{action} ({subject})")
}

/// Parses a stored memento root; `None` when there is nothing to restore (empty or malformed).
pub(super) fn parse(data: &str) -> Option<Element> {
    Element::parse(data).ok()
}

/// The stored element of a memento root: the first child that is not its `Trackability` block.
pub(super) fn stored_element(root: &Element) -> Option<&Element> {
    root.children.iter().find(|c| c.name != "Trackability")
}

/// Restores the `Trackability` block of a memento root, when it has one.
pub(super) fn restore_trackers(m: &mut Metadata, root: Option<&Element>) {
    if let Some(t) = root.and_then(|r| r.child("Trackability")) {
        m.trackability.read_kva(t);
    }
}

/// `Trackability` block with the trackers of the given drawings, as the memento stores them.
pub(super) fn trackers_of(m: &Metadata, ids: impl IntoIterator<Item = Uuid>) -> Element {
    let mut e = Element::new("Trackability");
    e.children = ids.into_iter().filter_map(|id| m.trackability.tracker(id)).map(DrawingTracker::to_element).collect();
    e
}
