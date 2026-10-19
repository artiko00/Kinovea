//! Tests for `synthetic.rs`: frame painting and the timestamp it encodes.

use crate::synthetic::{decode_timestamp, SyntheticReader};
use kinovea_geometry::Size;

#[test]
fn paint_round_trips_the_timestamp() {
    let image = SyntheticReader::paint(1234, Size::new(4, 2));
    assert_eq!(decode_timestamp(&image), 1234);
}

#[test]
fn paint_fills_the_requested_size() {
    let image = SyntheticReader::paint(7, Size::new(4, 2));
    assert_eq!((image.width(), image.height(), image.stride()), (4, 2, 16));
    assert_eq!(image.pixel(3, 1), Some([0, 0, 0, 0]));
    assert_eq!(image.pixel(4, 0), None);
}
