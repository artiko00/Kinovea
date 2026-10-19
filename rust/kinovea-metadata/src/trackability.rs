//! Reference positions of trackable drawings and their link to tracks
//! (`TrackabilityManager`, `DrawingTracker`, `TrackablePoint`).

use crate::ids::{id_text, parse_id};
use crate::kva_io::*;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

mod slots;
#[cfg(test)]
mod slots_test;
pub use slots::TrackerSlots;

#[derive(Debug, Clone, PartialEq)]
pub struct TrackablePoint {
    pub reference_timestamp: i64,
    pub reference_value: PointF,
    /// Track driving this point, when object tracking is set up.
    pub track_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawingTracker {
    pub drawing_id: Uuid,
    pub name: String,
    /// Keyed points in registration order.
    pub points: Vec<(String, TrackablePoint)>,
}

impl DrawingTracker {
    pub fn read_kva(e: &Element) -> DrawingTracker {
        let mut t = DrawingTracker { drawing_id: e.attribute("id").and_then(parse_id).unwrap_or_default(), name: e.attribute("name").unwrap_or("").to_owned(), points: Vec::new() };
        for c in e.children.iter().filter(|c| c.name == "TrackablePoint") {
            let mut p = TrackablePoint { reference_timestamp: 0, reference_value: PointF::EMPTY, track_id: None };
            for v in &c.children {
                match v.name.as_str() {
                    "TrackId" => p.track_id = parse_id(&v.text),
                    "ReferenceTimestamp" => p.reference_timestamp = parse_timestamp(&v.text),
                    "ReferenceValue" => p.reference_value = read_point(v, Scale::IDENTITY),
                    _ => {}
                }
            }
            t.points.push((c.attribute("key").unwrap_or("").to_owned(), p));
        }
        t
    }

    pub fn to_element(&self) -> Element {
        let mut e = Element::new("TrackableDrawing");
        e.attributes.push(("id".into(), id_text(self.drawing_id)));
        e.attributes.push(("name".into(), self.name.clone()));
        for (key, p) in &self.points {
            let mut tp = Element::new("TrackablePoint");
            tp.attributes.push(("key".into(), key.clone()));
            if let Some(track) = p.track_id {
                tp.push_leaf("TrackId", &id_text(track));
            }
            tp.push_leaf("ReferenceTimestamp", &p.reference_timestamp.to_string());
            tp.push(point_leaf("ReferenceValue", p.reference_value));
            e.push(tp);
        }
        e
    }
}

/// Registered trackers, enumerated in the slot order of the original's dictionary.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TrackabilityManager {
    pub trackers: TrackerSlots,
}

impl TrackabilityManager {
    /// Registers (or re-registers in place) a drawing with its current points as references.
    /// A known key is reassigned: the drawing id becomes `drawing_id` (`Reassign`, DrawingTracker.cs:261).
    pub fn add(&mut self, drawing_id: Uuid, name: &str, timestamp: i64, points: &[(String, PointF)]) {
        let tracker = self.registered(drawing_id, name, timestamp, points);
        self.trackers.insert(drawing_id, tracker);
    }

    /// `UpdateOrigin` after the import (`AddPoint`, `Assign`): the name and reference values of a known tracker
    /// follow the drawing, and the drawing id it was filed under is kept (no `Reassign`).
    pub fn follow(&mut self, key: Uuid, name: &str, timestamp: i64, points: &[(String, PointF)]) {
        let Some(filed) = self.trackers.get(key).map(|t| t.drawing_id) else {
            return;
        };
        let tracker = DrawingTracker { drawing_id: filed, ..self.registered(key, name, timestamp, points) };
        self.trackers.insert(key, tracker);
    }

    /// Tracker for these points; a known key keeps the track bindings of its points.
    fn registered(&self, drawing_id: Uuid, name: &str, timestamp: i64, points: &[(String, PointF)]) -> DrawingTracker {
        let known = self.trackers.get(drawing_id);
        let points = points
            .iter()
            .map(|(k, p)| {
                let track_id = known.and_then(|t| t.points.iter().find(|(x, _)| x == k)).and_then(|(_, o)| o.track_id);
                (k.clone(), TrackablePoint { reference_timestamp: timestamp, reference_value: *p, track_id })
            })
            .collect();
        DrawingTracker { drawing_id, name: name.to_owned(), points }
    }

    pub fn remove(&mut self, drawing_id: Uuid) {
        self.trackers.remove(drawing_id);
    }

    /// `UpdateId`: the coordinate system takes the id of the KVA it is merged from. A known new id drops the
    /// old entry. Otherwise the tracker is added under the new id before the old entry is removed. That copy
    /// keeps its `drawingId` (DrawingTracker.cs sets it only in the constructors and `Reassign`), so the
    /// writer skips it (see `write_kva`).
    pub fn update_id(&mut self, old_id: Uuid, new_id: Uuid) {
        if old_id == new_id || !self.trackers.contains(old_id) {
            return;
        }
        if !self.trackers.contains(new_id) {
            let moved = self.trackers.get(old_id).unwrap().clone();
            self.trackers.insert(new_id, moved);
        }
        self.trackers.remove(old_id);
    }

    /// `ForgetTrack`: the points driven by the deleted track `track_id` are no longer bound to it.
    pub fn forget_track(&mut self, track_id: Uuid) {
        for (_, p) in self.trackers.iter_mut().flat_map(|t| t.points.iter_mut()) {
            if p.track_id == Some(track_id) {
                p.track_id = None;
            }
        }
    }

    pub fn tracker(&self, drawing_id: Uuid) -> Option<&DrawingTracker> {
        self.trackers.get(drawing_id)
    }

    /// `GetReferenceValue`: `PointF.Empty` for unknown drawings or keys.
    pub fn reference_value(&self, drawing_id: Uuid, key: &str) -> PointF {
        self.tracker(drawing_id).and_then(|t| t.points.iter().find(|(k, _)| k == key)).map_or(PointF::EMPTY, |(_, p)| p.reference_value)
    }

    /// Reads a `Trackability` block: known ids are replaced in place, new ones are added (`ReadTracker`).
    pub fn read_kva(&mut self, e: &Element) {
        for c in e.children.iter().filter(|c| c.name == "TrackableDrawing") {
            let tracker = DrawingTracker::read_kva(c);
            self.trackers.insert(tracker.drawing_id, tracker);
        }
    }

    /// `WriteXml`: every tracker is written by its drawing id (`WriteTracker(tracker.Id)`, TrackabilityManager.cs
    /// 403-420). A tracker whose drawing id is no longer a key is skipped, as the original does.
    pub fn write_kva(&self) -> Vec<Element> {
        self.trackers.iter().filter(|t| self.trackers.contains(t.drawing_id)).map(DrawingTracker::to_element).collect()
    }
}
