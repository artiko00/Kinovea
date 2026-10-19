//! Mutations of the document used by undo and redo: the `Metadata` methods without their events,
//! since the document has no subscribers here.

use crate::drawing::Drawing;
use crate::keyframe::Keyframe;
use crate::kva_io::{identity, Filter, Scale};
use crate::metadata::{Manager, Metadata};
use crate::multi::{AutoNumber, Spotlight};
use kinovea_core::preferences::dom::Element;
use uuid::Uuid;

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

    /// `AddDrawing(managerId, drawing)`: a keyframe drawing takes the keyframe time as reference.
    pub fn add_drawing(&mut self, manager: Manager, mut drawing: Drawing) {
        match manager {
            Manager::Keyframe(kid) => {
                if let Some(kf) = self.keyframes.iter_mut().find(|k| k.id == kid) {
                    drawing.header_mut().reference_timestamp = kf.timestamp;
                    drawing.header_mut().fading.reference_timestamp = kf.timestamp;
                    kf.add_drawing(drawing);
                }
            }
            Manager::Chronos => self.chronos.push(drawing),
            Manager::Tracks => self.tracks.push(drawing),
            Manager::Singletons => {}
        }
    }

    /// `DeleteDrawing`: removes the drawing and its tracker, and returns the drawing.
    pub fn delete_drawing(&mut self, manager: Manager, id: Uuid) -> Option<Drawing> {
        let removed = match manager {
            Manager::Keyframe(kid) => self.keyframes.iter_mut().find(|k| k.id == kid)?.remove_drawing(id),
            Manager::Chronos => take_drawing(&mut self.chronos, id),
            Manager::Tracks => take_drawing(&mut self.tracks, id),
            Manager::Singletons => None,
        }?;
        self.trackability.remove(id);
        Some(removed)
    }

    /// `DeleteKeyframe`: removes the keyframe and the trackers of its drawings.
    pub fn delete_keyframe(&mut self, id: Uuid) -> Option<Keyframe> {
        let i = self.keyframes.iter().position(|k| k.id == id)?;
        let kf = self.keyframes.remove(i);
        for d in &kf.drawings {
            self.trackability.remove(d.id());
        }
        Some(kf)
    }

    /// `AddMultidrawingItem`: appends the item to its container.
    pub fn add_multi_item(&mut self, item: MultiItem) {
        match item {
            MultiItem::Spotlight(s) => self.spotlights.items.push(s),
            MultiItem::AutoNumber(a) => self.number_sequence.items.push(a),
        }
    }

    /// `DeleteMultiDrawingItem`: removes the item from the container that holds it.
    pub fn delete_multi_item(&mut self, id: Uuid) -> Option<MultiItem> {
        if let Some(i) = self.spotlights.items.iter().position(|s| s.id == id) {
            return Some(MultiItem::Spotlight(self.spotlights.items.remove(i)));
        }
        let i = self.number_sequence.items.iter().position(|a| a.id == id)?;
        Some(MultiItem::AutoNumber(self.number_sequence.items.remove(i)))
    }
}

fn take_drawing(list: &mut Vec<Drawing>, id: Uuid) -> Option<Drawing> {
    let i = list.iter().position(|d| d.id() == id)?;
    Some(list.remove(i))
}
