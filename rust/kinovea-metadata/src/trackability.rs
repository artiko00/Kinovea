//! Reference positions of trackable drawings and their link to tracks
//! (`TrackabilityManager`, `DrawingTracker`, `TrackablePoint`).

use crate::ids::{id_text, parse_id};
use crate::kva_io::*;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

mod state;
#[cfg(test)]
mod state_test;

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
    /// `isObjectTrackingInitialized`. Not in KVA: set when tracks are bound or assigned, cleared when
    /// the last bound track is forgotten.
    pub object_tracking_initialized: bool,
}

impl DrawingTracker {
    pub fn read_kva(e: &Element) -> DrawingTracker {
        let mut t = DrawingTracker { drawing_id: e.attribute("id").and_then(parse_id).unwrap_or_default(), name: e.attribute("name").unwrap_or("").to_owned(), points: Vec::new(), object_tracking_initialized: false };
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

/// Registered trackers in registration order.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TrackabilityManager {
    pub trackers: Vec<DrawingTracker>,
}

impl TrackabilityManager {
    /// Registers (or re-registers in place) a drawing with its current points as references.
    pub fn add(&mut self, drawing_id: Uuid, name: &str, timestamp: i64, points: &[(String, PointF)]) {
        let tracker = DrawingTracker {
            drawing_id,
            name: name.to_owned(),
            points: points.iter().map(|(k, p)| (k.clone(), TrackablePoint { reference_timestamp: timestamp, reference_value: *p, track_id: None })).collect(),
            object_tracking_initialized: false,
        };
        match self.trackers.iter().position(|t| t.drawing_id == drawing_id) {
            Some(i) => {
                // Re-assignment keeps the track bindings.
                let old = std::mem::replace(&mut self.trackers[i], tracker);
                self.trackers[i].object_tracking_initialized = old.object_tracking_initialized;
                for (key, p) in &mut self.trackers[i].points {
                    p.track_id = old.points.iter().find(|(k, _)| k == key).and_then(|(_, o)| o.track_id);
                }
            }
            None => self.trackers.push(tracker),
        }
    }

    pub fn remove(&mut self, drawing_id: Uuid) {
        self.trackers.retain(|t| t.drawing_id != drawing_id);
    }

    /// `ForgetTrack` (`DrawingTracker.cs:352-380`): the points driven by the deleted track are unbound.
    /// A drawing left without any bound track leaves object tracking. Ignored while object tracking
    /// is not initialized.
    pub fn forget_track(&mut self, track_id: Uuid) {
        for t in &mut self.trackers {
            if !t.object_tracking_initialized {
                continue;
            }
            let mut bound = false;
            for (_, p) in &mut t.points {
                if p.track_id == Some(track_id) {
                    p.track_id = None;
                    bound = true;
                }
            }
            if bound && t.points.iter().all(|(_, p)| p.track_id.is_none()) {
                t.object_tracking_initialized = false;
            }
        }
    }

    pub fn tracker(&self, drawing_id: Uuid) -> Option<&DrawingTracker> {
        self.trackers.iter().find(|t| t.drawing_id == drawing_id)
    }

    /// `GetReferenceValue`: `PointF.Empty` for unknown drawings or keys.
    pub fn reference_value(&self, drawing_id: Uuid, key: &str) -> PointF {
        self.tracker(drawing_id).and_then(|t| t.points.iter().find(|(k, _)| k == key)).map_or(PointF::EMPTY, |(_, p)| p.reference_value)
    }

    /// Reads a `Trackability` block: known ids are replaced in place, new ones appended.
    pub fn read_kva(&mut self, e: &Element) {
        for c in e.children.iter().filter(|c| c.name == "TrackableDrawing") {
            let tracker = DrawingTracker::read_kva(c);
            match self.trackers.iter().position(|t| t.drawing_id == tracker.drawing_id) {
                Some(i) => self.trackers[i] = tracker,
                None => self.trackers.push(tracker),
            }
        }
    }

    pub fn write_kva(&self) -> Vec<Element> {
        self.trackers.iter().map(DrawingTracker::to_element).collect()
    }
}
