use super::*;

#[test]
fn a_listed_entry_answers_only_codes_of_its_element_zone() {
    assert_eq!(text_codes("Circle"), &[2]);
    assert_eq!(text_codes("Rectangle"), &[] as &[i32]);
    for (case, id, codes) in PENDING_TEXT_METRICS {
        assert!(!codes.is_empty(), "{case} {id}");
    }
}

#[test]
fn pending_codes_are_found_by_case_and_drawing() {
    assert_eq!(pending_codes("all-drawings", "k0#10"), Some(&[1][..]));
    assert_eq!(pending_codes("all-drawings", "k0#0"), None);
    assert_eq!(pending_codes("geometry", "k0#10"), None);
}
