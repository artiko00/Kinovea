use crate::drawing::Header;
use crate::style::StyleElements;

#[test]
fn header_starts_with_a_fresh_id_and_the_given_name() {
    let a = Header::new("A", StyleElements::default());
    let b = Header::new("B", StyleElements::default());
    assert_ne!(a.id, b.id);
    assert_eq!(a.name, "A");
    assert_eq!(
        a.reference_timestamp, -1,
        "unset until the drawing is placed"
    );
}
