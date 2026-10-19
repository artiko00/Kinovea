//! Enumeration order of the original's `TrackabilityManager.trackers`, a `Dictionary<Guid, DrawingTracker>`.
//! Entries sit in slots. `Remove` frees the slot onto a free list and the next `Add` takes the last slot
//! freed, so the order in which the trackers are written depends on the whole history of adds and removes.
//! The key is not the tracker's own `drawing_id`: `UpdateId` can file a tracker under a new key.

use crate::trackability::DrawingTracker;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TrackerSlots {
    slots: Vec<Option<(Uuid, DrawingTracker)>>,
    free: Vec<usize>,
}

impl TrackerSlots {
    fn position(&self, key: Uuid) -> Option<usize> {
        self.slots
            .iter()
            .position(|s| s.as_ref().is_some_and(|(k, _)| *k == key))
    }

    pub fn contains(&self, key: Uuid) -> bool {
        self.position(key).is_some()
    }

    pub fn get(&self, key: Uuid) -> Option<&DrawingTracker> {
        self.position(key)
            .and_then(|i| self.slots[i].as_ref())
            .map(|(_, t)| t)
    }

    pub fn get_mut(&mut self, key: Uuid) -> Option<&mut DrawingTracker> {
        let i = self.position(key)?;
        self.slots[i].as_mut().map(|(_, t)| t)
    }

    /// `trackers[key] = tracker` for a known key, which keeps its slot. Otherwise `Add`.
    pub fn insert(&mut self, key: Uuid, tracker: DrawingTracker) {
        match self.position(key) {
            Some(i) => self.slots[i] = Some((key, tracker)),
            None => match self.free.pop() {
                Some(i) => self.slots[i] = Some((key, tracker)),
                None => self.slots.push(Some((key, tracker))),
            },
        }
    }

    /// `Remove`: the slot is freed for the next insertion.
    pub fn remove(&mut self, key: Uuid) {
        if let Some(i) = self.position(key) {
            self.free_slot(i);
        }
    }

    /// Removes the entries the predicate rejects, in enumeration order (`CleanUnassigned`).
    pub fn retain(&mut self, keep: impl Fn(Uuid, &DrawingTracker) -> bool) {
        let rejected: Vec<usize> = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, s)| s.as_ref().is_some_and(|(k, t)| !keep(*k, t)))
            .map(|(i, _)| i)
            .collect();
        for i in rejected {
            self.free_slot(i);
        }
    }

    /// Enumeration order: the occupied slots from the first one.
    pub fn iter(&self) -> impl Iterator<Item = &DrawingTracker> {
        self.slots.iter().flatten().map(|(_, t)| t)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut DrawingTracker> {
        self.slots.iter_mut().flatten().map(|(_, t)| t)
    }

    pub fn keys(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.slots.iter().flatten().map(|(k, _)| *k)
    }

    fn free_slot(&mut self, i: usize) {
        self.slots[i] = None;
        self.free.push(i);
    }
}
