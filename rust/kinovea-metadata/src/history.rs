//! Undo and redo history of the annotation document (`HistoryStack`). The stacks hold mementos
//! only; each step receives the document it applies to.

mod drawing;
#[cfg(test)]
mod drawing_test;
mod keyframe;
#[cfg(test)]
mod keyframe_test;
mod memento;
#[cfg(test)]
mod memento_test;
mod multi_items;
#[cfg(test)]
mod multi_items_test;
mod video_filter;
#[cfg(test)]
mod video_filter_test;

pub use memento::{Change, Memento};

use crate::metadata::Metadata;

#[derive(Debug, Default)]
pub struct HistoryStack {
    undo: Vec<Memento>,
    redo: Vec<Memento>,
}

impl HistoryStack {
    /// `PushNewCommand`: clears the redo stack. A modification identical to the top of the undo
    /// stack is dropped, since several UI parts may prepare the same change.
    pub fn push(&mut self, memento: Memento) {
        if let Change::ModifyDrawing { data, .. } = &memento.change {
            if self.undo.last().is_some_and(|top| matches!(&top.change, Change::ModifyDrawing { data: previous, .. } if previous == data)) {
                return;
            }
        }
        self.redo.clear();
        self.undo.push(memento);
    }

    /// `StepBackward`: undoes the top of the undo stack and keeps its inverse on the redo stack.
    pub fn step_backward(&mut self, m: &mut Metadata) {
        if let Some(top) = self.undo.pop() {
            self.redo.push(top.perform_undo(m));
        }
    }

    /// `StepForward`: redoes the top of the redo stack and keeps its inverse on the undo stack.
    pub fn step_forward(&mut self, m: &mut Metadata) {
        if let Some(top) = self.redo.pop() {
            self.undo.push(top.perform_undo(m));
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo_action_name(&self) -> &str {
        self.undo.last().map_or("", |m| m.command_name())
    }

    pub fn redo_action_name(&self) -> &str {
        self.redo.last().map_or("", |m| m.command_name())
    }
}
