//! Port of `Kinovea.ScreenManager/Metadata/Timeline.cs`: sparse values keyed by time, kept in
//! time order. `Insert` replaces an existing time, `ClosestFrom` breaks ties towards the later
//! entry, as the original does. `Dispose()` on trimmed entries is the job of `Drop` here.

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Timeline<T> {
    entries: BTreeMap<i64, T>,
}

impl<T> Default for Timeline<T> {
    fn default() -> Self {
        Timeline {
            entries: BTreeMap::new(),
        }
    }
}

impl<T> Timeline<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, time: i64, value: T) {
        self.entries.insert(time, value);
    }

    /// Entry nearest to `time`, or `None` when empty. Does not modify the timeline.
    pub fn closest_from(&self, time: i64) -> Option<&T> {
        let before = self.entries.range(..=time).next_back();
        let after = self.entries.range(time..).next();
        match (before, after) {
            (Some((t0, v0)), Some((t1, v1))) => {
                if time - t0 < t1 - time {
                    Some(v0)
                } else {
                    Some(v1)
                }
            }
            (Some((_, v)), None) | (None, Some((_, v))) => Some(v),
            (None, None) => None,
        }
    }

    /// Entry with the greatest time.
    pub fn last(&self) -> Option<&T> {
        self.entries.values().next_back()
    }

    /// Removes the entries with a time strictly greater than `time`.
    pub fn trim(&mut self, time: i64) {
        self.entries.retain(|t, _| *t <= time);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Passes every entry to `disposer` in time order, then empties the timeline.
    pub fn clear_with(&mut self, disposer: impl FnMut(T)) {
        std::mem::take(&mut self.entries)
            .into_values()
            .for_each(disposer);
    }

    /// Entries in time order.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.entries.values()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
