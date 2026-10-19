//! Undo of video filter changes (`HistoryMementoModifyVideoFilter`). Only the element of the filter
//! that changed is stored and restored; the other filters keep their current parameters.

use super::memento::{parse, Change, Memento};
use crate::metadata::Metadata;

impl Memento {
    /// Captures the element of the filter `filter_name` (a child of `VideoFilters`) before a change.
    pub fn modify_video_filter(m: &Metadata, filter_name: &str) -> Memento {
        Memento { name: format!("Modify {filter_name}"), change: Change::ModifyVideoFilter { name: filter_name.to_owned(), data: filter_of(m, filter_name) } }
    }
}

/// The filter element as a document; empty when `VideoFilters` has no such filter.
fn filter_of(m: &Metadata, name: &str) -> String {
    m.video_filters.child(name).map(|e| e.to_document()).unwrap_or_default()
}

/// Puts the stored filter element in place of the current one. A filter that the captured state did
/// not have is removed again.
pub(super) fn undo_modify(m: &mut Metadata, name: &str, data: &str) -> Change {
    let redo = filter_of(m, name);
    let i = m.video_filters.children.iter().position(|c| c.name == name);
    match (parse(data), i) {
        (Some(stored), Some(i)) => m.video_filters.children[i] = stored,
        (Some(stored), None) => {
            m.video_filters.children.push(stored);
        }
        (None, Some(i)) => {
            m.video_filters.children.remove(i);
        }
        (None, None) => {}
    }
    Change::ModifyVideoFilter { name: name.to_owned(), data: redo }
}
