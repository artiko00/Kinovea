use crate::timeline::Timeline;

fn sample() -> Timeline<&'static str> {
    let mut t = Timeline::new();
    t.insert(10, "a");
    t.insert(20, "b");
    t.insert(40, "c");
    t
}

#[test]
fn empty_timeline_has_no_entries() {
    let t: Timeline<u8> = Timeline::new();
    assert!(t.is_empty());
    assert_eq!(t.len(), 0);
    assert_eq!(t.last(), None);
    assert_eq!(t.closest_from(5), None);
}

#[test]
fn insert_keeps_time_order_and_replaces_existing_times() {
    let mut t = Timeline::new();
    t.insert(30, "c");
    t.insert(10, "a");
    t.insert(20, "b");
    t.insert(20, "B");
    assert_eq!(t.len(), 3);
    assert_eq!(t.iter().copied().collect::<Vec<_>>(), vec!["a", "B", "c"]);
}

#[test]
fn closest_from_picks_the_nearest_entry() {
    let t = sample();
    assert_eq!(t.closest_from(5), Some(&"a"));
    assert_eq!(t.closest_from(14), Some(&"a"));
    assert_eq!(t.closest_from(16), Some(&"b"));
    assert_eq!(t.closest_from(35), Some(&"c"));
    assert_eq!(t.closest_from(20), Some(&"b"));
}

#[test]
fn closest_from_breaks_ties_towards_the_later_entry() {
    let t = sample();
    assert_eq!(t.closest_from(15), Some(&"b"));
    assert_eq!(t.closest_from(30), Some(&"c"));
}

#[test]
fn closest_from_outside_the_range_gives_the_end_entries() {
    let t = sample();
    assert_eq!(t.closest_from(-100), Some(&"a"));
    assert_eq!(t.closest_from(1000), Some(&"c"));
}

#[test]
fn last_is_the_entry_with_the_greatest_time() {
    assert_eq!(sample().last(), Some(&"c"));
}

#[test]
fn trim_removes_entries_strictly_after_the_time() {
    let mut t = sample();
    t.trim(20);
    assert_eq!(t.iter().copied().collect::<Vec<_>>(), vec!["a", "b"]);
    t.trim(100);
    assert_eq!(t.len(), 2);
}

#[test]
fn clear_empties_the_timeline() {
    let mut t = sample();
    t.clear();
    assert!(t.is_empty());
}

#[test]
fn clear_with_passes_each_entry_to_the_disposer_in_order() {
    let mut t = sample();
    let mut disposed = Vec::new();
    t.clear_with(|v| disposed.push(v));
    assert_eq!(disposed, vec!["a", "b", "c"]);
    assert!(t.is_empty());
}
