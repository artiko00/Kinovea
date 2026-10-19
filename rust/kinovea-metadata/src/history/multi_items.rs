//! Undo of spotlight and auto number additions and deletions (`HistoryMementoAddMultiDrawingItem`,
//! `HistoryMementoDeleteMultiDrawingItem`). The memento root holds the item and its tracker block.

use super::memento::{named, parse, restore_trackers, stored_element, trackers_of, Change, Memento};
use crate::kva_io::Filter;
use crate::metadata::edit::MultiItem;
use crate::metadata::Metadata;
use kinovea_core::preferences::dom::Element;
use uuid::Uuid;

const ADD: &str = "Add drawing";
const DELETE: &str = "Delete drawing";

/// `ToolDisplayName` of the item's container.
fn tool_name(item: &MultiItem) -> &'static str {
    match item {
        MultiItem::Spotlight(_) => "Spotlight",
        MultiItem::AutoNumber(_) => "Auto numbers",
    }
}

/// `DrawingMemento`: the item element and its tracker block.
fn data_of(m: &Metadata, item: &MultiItem) -> String {
    let mut root = Element::new("DrawingMemento");
    root.push(item.to_element(Filter::KVA));
    root.push(trackers_of(m, [item.id()]));
    root.to_document()
}

impl Memento {
    /// Undo removes the item.
    pub fn add_multi_item(item: &MultiItem) -> Memento {
        Memento { name: named(ADD, tool_name(item)), change: Change::AddMultiItem { id: item.id() } }
    }

    /// Captures the item and its tracker; call it before the item is deleted.
    pub fn delete_multi_item(m: &Metadata, item: &MultiItem) -> Memento {
        Memento { name: named(DELETE, tool_name(item)), change: Change::DeleteMultiItem { id: item.id(), data: data_of(m, item) } }
    }
}

/// Removes the item and its tracker, keeping their serialized state for redo.
pub(super) fn undo_add(m: &mut Metadata, id: Uuid) -> Change {
    let data = match m.multi_item(id) {
        Some(item) => data_of(m, &item),
        None => String::new(),
    };
    m.delete_multi_item(id);
    Change::DeleteMultiItem { id, data }
}

/// Appends the stored item back to its container and restores its tracker.
pub(super) fn undo_delete(m: &mut Metadata, id: Uuid, data: &str) -> Change {
    let root = parse(data);
    if let Some(item) = root.as_ref().and_then(stored_element).and_then(MultiItem::read_kva) {
        m.add_multi_item(item);
    }
    restore_trackers(m, root.as_ref());
    Change::AddMultiItem { id }
}
