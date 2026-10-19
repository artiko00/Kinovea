use crate::kva::PRODUCER;

#[test]
fn producer_prefix_is_kinovea() {
    assert_eq!(PRODUCER, "Kinovea");
}
