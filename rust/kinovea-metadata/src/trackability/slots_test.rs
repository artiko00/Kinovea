use crate::trackability::{DrawingTracker, TrackerSlots};
use uuid::Uuid;

fn tracker(name: &str) -> DrawingTracker {
    DrawingTracker {
        drawing_id: Uuid::new_v4(),
        name: name.to_owned(),
        points: Vec::new(),
    }
}

fn names(s: &TrackerSlots) -> Vec<&str> {
    s.iter().map(|t| t.name.as_str()).collect()
}

fn key_of(s: &TrackerSlots, name: &str) -> Uuid {
    s.keys().find(|k| s.get(*k).unwrap().name == name).unwrap()
}

fn put(s: &mut TrackerSlots, name: &str) {
    let t = tracker(name);
    s.insert(t.drawing_id, t);
}

fn filled(list: &[&str]) -> TrackerSlots {
    let mut s = TrackerSlots::default();
    for name in list {
        put(&mut s, name);
    }
    s
}

// `TrackabilityManager.trackers` is a `Dictionary<Guid, DrawingTracker>` in the original. .NET Framework
// enumerates the entries array in slot order; `Remove` frees the slot onto a free list and `Add` takes the
// last slot freed. Replacing the value of a known key keeps its slot.

#[test]
fn without_removals_the_order_is_the_insertion_order() {
    assert_eq!(names(&filled(&["a", "b", "c"])), ["a", "b", "c"]);
}

#[test]
fn an_add_after_a_remove_takes_the_freed_slot() {
    let mut s = filled(&["a", "b", "c"]);
    s.remove(key_of(&s, "b"));
    put(&mut s, "d");
    assert_eq!(names(&s), ["a", "d", "c"]);
}

#[test]
fn freed_slots_are_reused_last_freed_first() {
    let mut s = filled(&["a", "b", "c"]);
    s.remove(key_of(&s, "a"));
    s.remove(key_of(&s, "c"));
    put(&mut s, "x");
    assert_eq!(names(&s), ["b", "x"]);
    put(&mut s, "y");
    assert_eq!(names(&s), ["y", "b", "x"]);
}

#[test]
fn replacing_a_known_key_keeps_its_slot() {
    let mut s = filled(&["a", "b", "c"]);
    let key = key_of(&s, "b");
    s.insert(
        key,
        DrawingTracker {
            drawing_id: key,
            name: "b2".into(),
            points: Vec::new(),
        },
    );
    assert_eq!(names(&s), ["a", "b2", "c"]);
}

#[test]
fn the_key_is_not_the_stored_drawing_id() {
    // `UpdateId` adds the old tracker under the new key (TrackabilityManager.cs:132). Its `drawingId` is
    // not changed (DrawingTracker.cs sets it only in the constructors and `Reassign`), so the key and the
    // drawing id differ.
    let old = tracker("stale");
    let (old_id, key) = (old.drawing_id, Uuid::new_v4());
    let mut s = TrackerSlots::default();
    s.insert(key, old);
    assert!(s.contains(key));
    assert!(!s.contains(old_id));
    assert_eq!(s.get(key).unwrap().drawing_id, old_id);
}

#[test]
fn removing_an_unknown_id_frees_nothing() {
    let mut s = filled(&["a", "b"]);
    s.remove(Uuid::new_v4());
    put(&mut s, "c");
    assert_eq!(names(&s), ["a", "b", "c"]);
}

#[test]
fn retain_drops_in_slot_order_and_frees_the_slots() {
    // `CleanUnassigned` removes the rejected entries in enumeration order.
    let mut s = filled(&["a", "b", "c", "d"]);
    s.retain(|_, t| t.name == "b" || t.name == "d");
    assert_eq!(names(&s), ["b", "d"]);
    put(&mut s, "x");
    assert_eq!(names(&s), ["b", "x", "d"]);
}
