use crate::filtering_result::FilteringResult;

#[test]
fn stores_cutoff_data_and_score() {
    let r = FilteringResult::new(5.0, vec![1.0, 2.0], 0.25);
    assert_eq!(r.cutoff_frequency, 5.0);
    assert_eq!(r.data, vec![1.0, 2.0]);
    assert_eq!(r.durbin_watson, 0.25);
}

#[test]
fn normalized_score_is_zero_at_two_and_one_at_the_extremes() {
    assert_eq!(FilteringResult::normalized_score(2.0), 0.0);
    assert_eq!(FilteringResult::normalized_score(0.0), 1.0);
    assert_eq!(FilteringResult::normalized_score(4.0), 1.0);
    assert_eq!(FilteringResult::normalized_score(3.0), 0.5);
}
