//! Undo of video filter changes (`HistoryMementoModifyVideoFilter`). The `VideoFilters` element is
//! stored whole.

use super::memento::{parse, Change, Memento};
use crate::metadata::Metadata;

impl Memento {
    /// Captures the `VideoFilters` element before a change of the filter named `filter_name`.
    pub fn modify_video_filter(m: &Metadata, filter_name: &str) -> Memento {
        Memento { name: format!("Modify {filter_name}"), change: Change::ModifyVideoFilter { data: m.video_filters.to_document() } }
    }
}

/// Replaces the `VideoFilters` element with the stored one.
pub(super) fn undo_modify(m: &mut Metadata, data: &str) -> Change {
    let redo = m.video_filters.to_document();
    if let Some(filters) = parse(data) {
        m.video_filters = filters;
    }
    Change::ModifyVideoFilter { data: redo }
}
