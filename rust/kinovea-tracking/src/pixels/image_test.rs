use crate::pixels::image::{GrayImage, Roi};
use kinovea_geometry::Size;

#[test]
fn gray_at_reads_row_major() {
    let img = GrayImage {
        width: 3,
        height: 2,
        data: vec![1, 2, 3, 4, 5, 6],
    };
    assert_eq!(img.at(0, 0), 1);
    assert_eq!(img.at(2, 0), 3);
    assert_eq!(img.at(1, 1), 5);
}

#[test]
fn roi_inside_the_image_is_unchanged() {
    assert_eq!(
        Roi::new(2, 3, 4, 5).clamp_to(10, 8),
        Roi {
            x: 2,
            y: 3,
            size: Size::new(4, 5)
        }
    );
}

#[test]
fn roi_is_cut_to_the_image_on_every_side() {
    let r = Roi::new(-3, 2, 6, 10).clamp_to(10, 8);
    assert_eq!((r.x, r.y), (0, 2));
    assert_eq!((r.size.width, r.size.height), (3, 6));
}

#[test]
fn roi_outside_the_image_is_empty() {
    let r = Roi::new(20, 20, 5, 5).clamp_to(10, 8);
    assert_eq!((r.size.width, r.size.height), (0, 0));
}
