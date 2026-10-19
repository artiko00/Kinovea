//! Undo of keyframe additions, deletions and core modifications (`HistoryMementoAddKeyframe`,
//! `HistoryMementoDeleteKeyframe`, `HistoryMementoModifyKeyframe`).

use super::memento::{parse, trackers_of, Change, Memento};
use crate::keyframe::Keyframe;
use crate::kva_io::{identity, Filter, Scale};
use crate::metadata::Metadata;
use kinovea_core::preferences::dom::Element;
use uuid::Uuid;

const ADD: &str = "Add a key image";
const DELETE: &str = "Remove a key image";
const MODIFY: &str = "Modify keyframe";

impl Memento {
    /// Undo removes the keyframe.
    pub fn add_keyframe(id: Uuid) -> Memento {
        Memento { name: ADD.to_owned(), change: Change::AddKeyframe { id } }
    }

    /// Captures the keyframe with its drawings and their trackers; call it before the deletion.
    pub fn delete_keyframe(m: &Metadata, id: Uuid) -> Memento {
        Memento { name: DELETE.to_owned(), change: Change::DeleteKeyframe { id, data: data_of(m, id, Filter::KVA, true) } }
    }

    /// Captures the core data (title, color, comments); the drawings have their own mementos.
    pub fn modify_keyframe(m: &Metadata, id: Uuid) -> Memento {
        Memento { name: MODIFY.to_owned(), change: Change::ModifyKeyframe { id, data: data_of(m, id, Filter::CORE, false) } }
    }
}

/// `KeyframeMemento`: the keyframe element, and the `Trackability` block of its drawings when
/// `trackers` is set. Empty when the keyframe does not exist.
fn data_of(m: &Metadata, id: Uuid, filter: Filter, trackers: bool) -> String {
    let Some(kf) = m.keyframe(id) else {
        return String::new();
    };
    let refs = |u: Uuid, key: &str| m.trackability.reference_value(u, key);
    let mut root = Element::new("KeyframeMemento");
    root.push(kf.to_element(filter, &refs, m.timecode));
    if trackers {
        root.push(trackers_of(m, kf.drawings.iter().map(|d| d.id())));
    }
    root.to_document()
}

fn keyframe_of(data: &str) -> Option<Keyframe> {
    let root = parse(data)?;
    root.child("Keyframe").map(|e| Keyframe::read_kva(e, Scale::IDENTITY, &identity))
}

/// Captures the keyframe, then removes it.
pub(super) fn undo_add(m: &mut Metadata, id: Uuid) -> Change {
    let data = data_of(m, id, Filter::KVA, true);
    m.delete_keyframe(id);
    Change::DeleteKeyframe { id, data }
}

/// Merges the stored keyframe back and restores the trackers of its drawings.
pub(super) fn undo_delete(m: &mut Metadata, id: Uuid, data: &str) -> Change {
    if let Some(kf) = keyframe_of(data) {
        m.merge_insert_keyframe(kf);
    }
    if let Some(t) = parse(data).as_ref().and_then(|r| r.child("Trackability")) {
        m.trackability.read_kva(t);
    }
    Change::AddKeyframe { id }
}

/// Updates the core data of the keyframe in place.
pub(super) fn undo_modify(m: &mut Metadata, id: Uuid, data: &str) -> Change {
    let redo = data_of(m, id, Filter::CORE, false);
    if let Some(kf) = keyframe_of(data) {
        m.merge_insert_keyframe(kf);
    }
    Change::ModifyKeyframe { id, data: redo }
}
