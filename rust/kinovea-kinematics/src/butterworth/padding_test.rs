use crate::butterworth::{add_padding, remove_padding};

#[test]
fn padding_reflects_a_ramp_on_both_sides() {
    let ramp: Vec<f64> = (0..12).map(f64::from).collect();
    let padded = add_padding(&ramp, 3);
    assert_eq!(padded.len(), 18);
    assert_eq!(&padded[..3], &[-3.0, -2.0, -1.0]);
    assert_eq!(&padded[3..15], &ramp[..]);
    assert_eq!(&padded[15..], &[12.0, 13.0, 14.0]);
    assert_eq!(remove_padding(&padded, 3), ramp);
}
