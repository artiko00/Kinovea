use crate::timestamp_mapper::{identity, TimestampMapper};

#[test]
fn identity_maps_to_itself() {
    assert_eq!(identity(42), 42);
    let mapper: TimestampMapper = Box::new(identity);
    assert_eq!(mapper(-7), -7);
}
