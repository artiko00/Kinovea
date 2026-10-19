use crate::closest::{find_closest, index_of, insert_sorted, lower_bound, relate_timestamp};
use crate::container::CacheTimestampRelation as Relation;
use crate::frame::VideoFrame;

fn frames(stamps: &[i64]) -> Vec<VideoFrame> {
    stamps
        .iter()
        .map(|&t| VideoFrame::new(None, t, t - 1))
        .collect()
}

#[test]
fn lower_bound_is_first_index_not_before_target() {
    let f = frames(&[10, 20, 30]);
    assert_eq!(lower_bound(&f, 5), 0);
    assert_eq!(lower_bound(&f, 20), 1);
    assert_eq!(lower_bound(&f, 25), 2);
    assert_eq!(lower_bound(&f, 99), 3);
}

#[test]
fn closest_is_none_for_empty_list() {
    assert_eq!(find_closest(&[], 3), None);
}

#[test]
fn closest_clamps_to_both_ends() {
    let f = frames(&[10, 20, 30]);
    assert_eq!(find_closest(&f, -100), Some(0));
    assert_eq!(find_closest(&f, 100), Some(2));
}

#[test]
fn closest_resolves_ties_to_the_earlier_frame() {
    let f = frames(&[10, 20]);
    assert_eq!(find_closest(&f, 15), Some(0));
    assert_eq!(find_closest(&f, 14), Some(0));
    assert_eq!(find_closest(&f, 16), Some(1));
}

#[test]
fn index_of_finds_stored_timestamps_only() {
    let f = frames(&[10, 20]);
    assert_eq!(index_of(&f, 20), Some(1));
    assert_eq!(index_of(&f, 15), None);
}

#[test]
fn insert_sorted_keeps_order_and_rejects_duplicates() {
    let mut f = frames(&[10, 30]);
    assert!(insert_sorted(&mut f, VideoFrame::new(None, 20, 10)));
    assert!(!insert_sorted(&mut f, VideoFrame::new(None, 20, 10)));
    let stamps: Vec<i64> = f.iter().map(|x| x.timestamp).collect();
    assert_eq!(stamps, vec![10, 20, 30]);
}

#[test]
fn relate_is_empty_without_frames() {
    assert_eq!(relate_timestamp(&[], 0, 0.0, 0.0), Relation::Empty);
}

#[test]
fn relate_checks_match_before_bounds() {
    let f = frames(&[0, 10]);
    assert_eq!(relate_timestamp(&f, 10, 0.5, 50.0), Relation::InBoundsMatch);
    assert_eq!(relate_timestamp(&f, -1, 1.0, 50.0), Relation::InBoundsMatch);
}

#[test]
fn relate_reports_behind_no_match_ahead_and_far_ahead() {
    let f = frames(&[0, 10]);
    assert_eq!(relate_timestamp(&f, -5, 0.5, 50.0), Relation::Behind);
    assert_eq!(
        relate_timestamp(&f, 5, 0.5, 50.0),
        Relation::InBoundsNoMatch
    );
    assert_eq!(relate_timestamp(&f, 60, 0.5, 50.0), Relation::Ahead);
    assert_eq!(relate_timestamp(&f, 61, 0.5, 50.0), Relation::FarAhead);
}
