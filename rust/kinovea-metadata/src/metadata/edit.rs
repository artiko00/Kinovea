//! Mutations of the document used by undo and redo: the `Metadata` methods without their events,
//! since the document has no subscribers here.

use crate::drawing::Drawing;
use crate::keyframe::Keyframe;
use crate::kva_io::{identity, Filter, Scale};
use crate::metadata::{Manager, Metadata};
use crate::multi::{AutoNumber, Spotlight};
use kinovea_core::preferences::dom::Element;
use uuid::Uuid;

/// Singletons are edited in place: they are never added to or removed from a container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditError {
    Singleton,
}

/// An item of the `Spotlights` or `NumberSequence` containers.
#[derive(Debug, Clone, PartialEq)]
pub enum MultiItem {
    Spotlight(Spotlight),
    AutoNumber(AutoNumber),
}

impl MultiItem {
    pub fn id(&self) -> Uuid {
        match self {
            MultiItem::Spotlight(s) => s.id,
            MultiItem::AutoNumber(a) => a.id,
        }
    }

    /// Reads a `Spotlight` or `AutoNumber` element by its name.
    pub fn read_kva(e: &Element) -> Option<MultiItem> {
        match e.name.as_str() {
            Spotlight::KVA_NAME => Some(MultiItem::Spotlight(Spotlight::read_kva(e, Scale::IDENTITY, &identity))),
            AutoNumber::KVA_NAME => Some(MultiItem::AutoNumber(AutoNumber::read_kva(e, Scale::IDENTITY, &identity))),
            _ => None,
        }
    }

    pub fn to_element(&self, filter: Filter) -> Element {
        match self {
            MultiItem::Spotlight(s) => s.to_element(filter),
            MultiItem::AutoNumber(a) => a.to_element(filter),
        }
    }
}

impl Metadata {
    /// `GetDrawing(managerId, drawingId)`.
    pub fn drawing(&self, manager: Manager, id: Uuid) -> Option<&Drawing> {
        self.find_drawing(id).filter(|(m, _)| *m == manager).map(|(_, d)| d)
    }

    /// Mutable access to a drawing in its container.
    pub fn drawing_mut(&mut self, manager: Manager, id: Uuid) -> Option<&mut Drawing> {
        match manager {
            Manager::Keyframe(kid) => self.keyframes.iter_mut().find(|k| k.id == kid)?.drawings.iter_mut().find(|d| d.id() == id),
            Manager::Chronos => self.chronos.iter_mut().find(|d| d.id() == id),
            Manager::Tracks => self.tracks.iter_mut().find(|d| d.id() == id),
            Manager::Singletons => None,
        }
    }

    /// `AddDrawing(managerId, drawing)`: a keyframe drawing takes the keyframe time as reference. A
    /// drawing whose id is already in its container is not added again.
    pub fn add_drawing(&mut self, manager: Manager, mut drawing: Drawing) -> Result<(), EditError> {
        let id = drawing.id();
        match manager {
            Manager::Keyframe(kid) => {
                if let Some(kf) = self.keyframes.iter_mut().find(|k| k.id == kid).filter(|kf| !kf.drawings.iter().any(|d| d.id() == id)) {
                    drawing.header_mut().reference_timestamp = kf.timestamp;
                    drawing.header_mut().fading.reference_timestamp = kf.timestamp;
                    kf.add_drawing(drawing);
                }
            }
            Manager::Chronos if !has_id(&self.chronos, id) => self.chronos.push(drawing),
            Manager::Tracks if !has_id(&self.tracks, id) => self.tracks.push(drawing),
            Manager::Chronos | Manager::Tracks => {}
            Manager::Singletons => return Err(EditError::Singleton),
        }
        Ok(())
    }

    /// `DeleteDrawing`: removes the drawing and its tracker, and returns the drawing. A deleted track
    /// is forgotten by the trackers bound to it.
    pub fn delete_drawing(&mut self, manager: Manager, id: Uuid) -> Result<Option<Drawing>, EditError> {
        let removed = match manager {
            Manager::Keyframe(kid) => self.keyframes.iter_mut().find(|k| k.id == kid).and_then(|k| k.remove_drawing(id)),
            Manager::Chronos => take_drawing(&mut self.chronos, id),
            Manager::Tracks => take_drawing(&mut self.tracks, id),
            Manager::Singletons => return Err(EditError::Singleton),
        };
        let Some(removed) = removed else {
            return Ok(None);
        };
        self.trackability.remove(id);
        if matches!(removed, Drawing::Track(_)) {
            self.trackability.forget_track(id);
        }
        Ok(Some(removed))
    }

    /// `DeleteKeyframe`: removes the keyframe and the trackers of its drawings, then updates the
    /// keyframe labels of the tracks.
    pub fn delete_keyframe(&mut self, id: Uuid) -> Option<Keyframe> {
        let i = self.keyframes.iter().position(|k| k.id == id)?;
        let kf = self.keyframes.remove(i);
        for d in &kf.drawings {
            self.trackability.remove(d.id());
        }
        self.update_trajectories_for_keyframes();
        Some(kf)
    }

    /// `UpdateTrajectoriesForKeyframes`: every track refreshes its labels for the keyframe times.
    pub fn update_trajectories_for_keyframes(&mut self) {
        let keyframes: Vec<i64> = self.keyframes.iter().map(|k| k.timestamp).collect();
        for track in self.tracks.iter_mut() {
            if let Drawing::Track(t) = track {
                t.update_keyframe_labels(&keyframes);
            }
        }
    }

    /// `AddMultidrawingItem`: appends the item to its container.
    pub fn add_multi_item(&mut self, item: MultiItem) {
        match item {
            MultiItem::Spotlight(s) => self.spotlights.items.push(s),
            MultiItem::AutoNumber(a) => self.number_sequence.items.push(a),
        }
    }

    /// The item with `id` in whichever container holds it, without removing it.
    pub fn multi_item(&self, id: Uuid) -> Option<MultiItem> {
        let spotlight = self.spotlights.items.iter().find(|s| s.id == id).cloned().map(MultiItem::Spotlight);
        spotlight.or_else(|| self.number_sequence.items.iter().find(|a| a.id == id).cloned().map(MultiItem::AutoNumber))
    }

    /// `DeleteMultiDrawingItem`: removes the item from the container that holds it, with its tracker.
    pub fn delete_multi_item(&mut self, id: Uuid) -> Option<MultiItem> {
        let item = match self.spotlights.items.iter().position(|s| s.id == id) {
            Some(i) => MultiItem::Spotlight(self.spotlights.items.remove(i)),
            None => {
                let i = self.number_sequence.items.iter().position(|a| a.id == id)?;
                MultiItem::AutoNumber(self.number_sequence.items.remove(i))
            }
        };
        self.trackability.remove(id);
        Some(item)
    }
}

fn has_id(list: &[Drawing], id: Uuid) -> bool {
    list.iter().any(|d| d.id() == id)
}

fn take_drawing(list: &mut Vec<Drawing>, id: Uuid) -> Option<Drawing> {
    let i = list.iter().position(|d| d.id() == id)?;
    Some(list.remove(i))
}
