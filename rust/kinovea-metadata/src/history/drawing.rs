//! Undo of drawing additions, deletions and modifications (`HistoryMementoAddDrawing`,
//! `HistoryMementoDeleteDrawing`, `HistoryMementoModifyDrawing`).

use super::memento::{named, parse, trackers_of, Change, Memento};
use crate::drawing::Drawing;
use crate::kva_io::{identity, Filter, Scale};
use crate::metadata::{Manager, Metadata};
use kinovea_core::preferences::dom::Element;
use uuid::Uuid;

const ADD: &str = "Add drawing";
const DELETE: &str = "Delete drawing";
const MODIFY: &str = "Modify drawing";

impl Memento {
    /// Undo removes the drawing.
    pub fn add_drawing(manager: Manager, drawing: &Drawing) -> Memento {
        Memento { name: named(ADD, &drawing.header().name), change: Change::AddDrawing { manager, id: drawing.id() } }
    }

    /// Captures the drawing with its tracker; call it before the drawing is deleted.
    pub fn delete_drawing(m: &Metadata, manager: Manager, drawing: &Drawing) -> Memento {
        let data = data_of(m, manager, drawing.id(), true);
        Memento { name: named(DELETE, &drawing.header().name), change: Change::DeleteDrawing { manager, id: drawing.id(), data } }
    }

    /// Captures the drawing state; call it before the drawing is modified.
    pub fn modify_drawing(m: &Metadata, manager: Manager, drawing: &Drawing) -> Memento {
        let data = data_of(m, manager, drawing.id(), false);
        Memento { name: named(MODIFY, &drawing.header().name), change: Change::ModifyDrawing { manager, id: drawing.id(), data } }
    }
}

/// `DrawingMemento`: the drawing element, and its tracker block when `trackers` is set.
/// Empty when the drawing is not in its container.
fn data_of(m: &Metadata, manager: Manager, id: Uuid, trackers: bool) -> String {
    let Some(d) = m.drawing(manager, id) else {
        return String::new();
    };
    let refs = |u: Uuid, key: &str| m.trackability.reference_value(u, key);
    let mut root = Element::new("DrawingMemento");
    root.push(d.to_element(Filter::KVA, &refs));
    if trackers {
        root.push(trackers_of(m, [id]));
    }
    root.to_document()
}

fn drawing_of(root: &Element) -> Option<Drawing> {
    let e = root.children.iter().find(|c| c.name != "Trackability")?;
    Drawing::read_kva(e, Scale::IDENTITY, &identity)
}

/// Captures the drawing and its tracker, then removes it.
pub(super) fn undo_add(m: &mut Metadata, manager: Manager, id: Uuid) -> Change {
    let data = data_of(m, manager, id, true);
    m.delete_drawing(manager, id);
    Change::DeleteDrawing { manager, id, data }
}

/// Re-adds the stored drawing and restores its tracker.
pub(super) fn undo_delete(m: &mut Metadata, manager: Manager, id: Uuid, data: &str) -> Change {
    let root = parse(data);
    if let Some(d) = root.as_ref().and_then(drawing_of) {
        m.add_drawing(manager, d);
    }
    if let Some(t) = root.as_ref().and_then(|r| r.child("Trackability")) {
        m.trackability.read_kva(t);
    }
    Change::AddDrawing { manager, id }
}

/// Replaces the drawing with the stored state. The reference times are not part of the drawing
/// element: they keep the values of the current drawing, as the keyframe sets them.
pub(super) fn undo_modify(m: &mut Metadata, manager: Manager, id: Uuid, data: &str) -> Change {
    let redo = data_of(m, manager, id, false);
    if let Some(mut d) = parse(data).as_ref().and_then(drawing_of) {
        if let Some(current) = m.drawing(manager, id) {
            d.header_mut().reference_timestamp = current.header().reference_timestamp;
            d.header_mut().fading.reference_timestamp = current.header().fading.reference_timestamp;
        }
        if let Some(slot) = m.drawing_mut(manager, id) {
            *slot = d;
        }
    }
    Change::ModifyDrawing { manager, id, data: redo }
}
