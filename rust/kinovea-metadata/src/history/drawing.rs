//! Undo of drawing additions, deletions and modifications (`HistoryMementoAddDrawing`,
//! `HistoryMementoDeleteDrawing`, `HistoryMementoModifyDrawing`). Singletons (coordinate system and
//! test grid) can be modified, not added or deleted.

use super::memento::{named, parse, restore_trackers, stored_element, trackers_of, Change, Memento};
use crate::drawing::{Drawing, Header};
use crate::kva_io::{identity, Filter, Scale};
use crate::metadata::edit::EditError;
use crate::metadata::{Manager, Metadata};
use crate::singletons::{singleton_element, CoordinateSystem, TestGrid};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const ADD: &str = "Add drawing";
const DELETE: &str = "Delete drawing";
const MODIFY: &str = "Modify drawing";

impl Memento {
    /// Undo removes the drawing.
    pub fn add_drawing(manager: Manager, drawing: &Drawing) -> Result<Memento, EditError> {
        if matches!(manager, Manager::Singletons) {
            return Err(EditError::Singleton);
        }
        Ok(Memento { name: named(ADD, &drawing.header().name), change: Change::AddDrawing { manager, id: drawing.id() } })
    }

    /// Captures the drawing with its tracker; call it before the drawing is deleted.
    pub fn delete_drawing(m: &Metadata, manager: Manager, drawing: &Drawing) -> Result<Memento, EditError> {
        if matches!(manager, Manager::Singletons) {
            return Err(EditError::Singleton);
        }
        let data = data_of(m, manager, drawing.id(), Filter::KVA, true);
        Ok(Memento { name: named(DELETE, &drawing.header().name), change: Change::DeleteDrawing { manager, id: drawing.id(), data } })
    }

    /// Captures the state of the drawing or singleton `id`, with the parts of `filter` only; call it
    /// before the change.
    pub fn modify_drawing(m: &Metadata, manager: Manager, id: Uuid, filter: Filter) -> Memento {
        let data = data_of(m, manager, id, filter, false);
        let name = header_of(m, manager, id).map_or("", |h| h.name.as_str());
        Memento { name: named(MODIFY, name), change: Change::ModifyDrawing { manager, id, filter, data } }
    }

    /// `IsSameState()`: whether the drawing still holds the state the memento captured.
    pub fn is_same_state(&self, m: &Metadata) -> bool {
        match &self.change {
            Change::ModifyDrawing { manager, id, filter, data } => header_of(m, *manager, *id).is_some() && data_of(m, *manager, *id, *filter, false) == *data,
            _ => false,
        }
    }
}

/// Header of the drawing or singleton `id` in its container.
fn header_of(m: &Metadata, manager: Manager, id: Uuid) -> Option<&Header> {
    if matches!(manager, Manager::Singletons) {
        return [&m.coordinate_system.header, &m.test_grid.header].into_iter().find(|h| h.id == id);
    }
    m.drawing(manager, id).map(|d| d.header())
}

/// `DrawingMemento`: the element with the parts of `filter`, and its tracker block when `trackers`
/// is set. Empty when the drawing is not in its container.
fn data_of(m: &Metadata, manager: Manager, id: Uuid, filter: Filter, trackers: bool) -> String {
    let Some(e) = element_of(m, manager, id, filter) else {
        return String::new();
    };
    let mut root = Element::new("DrawingMemento");
    root.push(e);
    if trackers {
        root.push(trackers_of(m, [id]));
    }
    root.to_document()
}

/// The element of a drawing or singleton, with the parts of `filter`.
fn element_of(m: &Metadata, manager: Manager, id: Uuid, filter: Filter) -> Option<Element> {
    let refs = |u: Uuid, key: &str| m.trackability.reference_value(u, key);
    if matches!(manager, Manager::Singletons) {
        return singleton_of(m, id, filter, &refs);
    }
    m.drawing(manager, id).map(|d| d.to_element(filter, &refs))
}

fn singleton_of(m: &Metadata, id: Uuid, filter: Filter, refs: &dyn Fn(Uuid, &str) -> PointF) -> Option<Element> {
    if m.coordinate_system.header.id == id {
        let children = m.coordinate_system.write_kva(filter, refs);
        return Some(singleton_element(CoordinateSystem::KVA_NAME, &m.coordinate_system.header, children));
    }
    if m.test_grid.header.id == id {
        return Some(singleton_element(TestGrid::KVA_NAME, &m.test_grid.header, m.test_grid.write_kva(filter)));
    }
    None
}

fn drawing_of(root: &Element) -> Option<Drawing> {
    Drawing::read_kva(stored_element(root)?, Scale::IDENTITY, &identity)
}

/// Captures the drawing and its tracker, then removes it.
pub(super) fn undo_add(m: &mut Metadata, manager: Manager, id: Uuid) -> Change {
    let data = data_of(m, manager, id, Filter::KVA, true);
    // Singletons are refused when the memento is built, so the deletion cannot fail here.
    let _ = m.delete_drawing(manager, id);
    Change::DeleteDrawing { manager, id, data }
}

/// Re-adds the stored drawing and restores its tracker.
pub(super) fn undo_delete(m: &mut Metadata, manager: Manager, id: Uuid, data: &str) -> Change {
    let root = parse(data);
    if let Some(d) = root.as_ref().and_then(drawing_of) {
        let _ = m.add_drawing(manager, d);
    }
    restore_trackers(m, root.as_ref());
    Change::AddDrawing { manager, id }
}

/// Restores the parts of `filter` from the stored state. The other parts keep their current values,
/// so a style change undone does not move the points of a track.
pub(super) fn undo_modify(m: &mut Metadata, manager: Manager, id: Uuid, filter: Filter, data: &str) -> Change {
    let redo = data_of(m, manager, id, filter, false);
    if let Some(stored) = parse(data).as_ref().and_then(stored_element) {
        let merged = with_current_parts(m, manager, id, filter, stored);
        restore(m, manager, id, &merged);
    }
    Change::ModifyDrawing { manager, id, filter, data: redo }
}

/// The stored element, with the parts that `filter` leaves out taken from the current state.
fn with_current_parts(m: &Metadata, manager: Manager, id: Uuid, filter: Filter, stored: &Element) -> Element {
    let complement = Filter { core: !filter.core, style: !filter.style, fading: !filter.fading };
    let mut merged = stored.clone();
    if let Some(current) = element_of(m, manager, id, complement) {
        merged.children.extend(current.children);
    }
    merged
}

/// Reads the merged element into the drawing or singleton. The reference times of a drawing are not
/// part of its element: they keep the values of the current drawing, as the keyframe sets them.
fn restore(m: &mut Metadata, manager: Manager, id: Uuid, e: &Element) {
    if matches!(manager, Manager::Singletons) {
        if m.coordinate_system.header.id == id {
            m.coordinate_system.read_kva(e);
        } else if m.test_grid.header.id == id {
            m.test_grid.read_kva(e);
        }
        return;
    }
    let Some((reference, fading_reference)) = m.drawing(manager, id).map(|c| (c.header().reference_timestamp, c.header().fading.reference_timestamp)) else {
        return;
    };
    if let Some(mut d) = Drawing::read_kva(e, Scale::IDENTITY, &identity) {
        d.header_mut().reference_timestamp = reference;
        d.header_mut().fading.reference_timestamp = fading_reference;
        if let Some(slot) = m.drawing_mut(manager, id) {
            *slot = d;
        }
    }
}
