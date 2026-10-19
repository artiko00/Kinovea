use crate::container::{CacheAddResult, CacheTimestampRelation, TryAcquireResult};

#[test]
fn add_results_use_csharp_names() {
    assert_eq!(CacheAddResult::Added.name(), "Added");
    assert_eq!(CacheAddResult::Duplicate.name(), "Duplicate");
    assert_eq!(CacheAddResult::Interrupted.name(), "Interrupted");
}

#[test]
fn timestamp_relations_use_csharp_names() {
    assert_eq!(CacheTimestampRelation::ALL.len(), 6);
    assert_eq!(
        CacheTimestampRelation::InBoundsNoMatch.name(),
        "InBoundsNoMatch"
    );
    assert_eq!(CacheTimestampRelation::FarAhead.name(), "FarAhead");
}

#[test]
fn empty_try_acquire_result_matches_make_empty() {
    assert_eq!(
        TryAcquireResult::empty(),
        TryAcquireResult::new(false, -1, false)
    );
}
