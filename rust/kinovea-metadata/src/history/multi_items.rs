//! Undo of spotlight and auto number additions and deletions (`HistoryMementoAddMultiDrawingItem`,
//! `HistoryMementoDeleteMultiDrawingItem`). The item element is the memento root.

use super::memento::{named, parse, Change, Memento};
use crate::kva_io::Filter;
use crate::metadata::edit::MultiItem;
use crate::metadata::Metadata;
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

fn data_of(item: &MultiItem) -> String {
    item.to_element(Filter::KVA).to_document()
}

impl Memento {
    /// Undo removes the item.
    pub fn add_multi_item(item: &MultiItem) -> Memento {
        Memento { name: named(ADD, tool_name(item)), change: Change::AddMultiItem { id: item.id() } }
    }

    /// Captures the item; call it before the item is deleted.
    pub fn delete_multi_item(item: &MultiItem) -> Memento {
        Memento { name: named(DELETE, tool_name(item)), change: Change::DeleteMultiItem { id: item.id(), data: data_of(item) } }
    }
}

/// Removes the item, keeping its serialized state for redo.
pub(super) fn undo_add(m: &mut Metadata, id: Uuid) -> Change {
    let data = m.delete_multi_item(id).map(|item| data_of(&item)).unwrap_or_default();
    Change::DeleteMultiItem { id, data }
}

/// Appends the stored item back to its container.
pub(super) fn undo_delete(m: &mut Metadata, id: Uuid, data: &str) -> Change {
    if let Some(item) = parse(data).as_ref().and_then(MultiItem::read_kva) {
        m.add_multi_item(item);
    }
    Change::AddMultiItem { id }
}
