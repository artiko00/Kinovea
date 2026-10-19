use crate::info::{frame_interval_ms, Fraction, VideoInfo};
use kinovea_core::image_options::ImageRotation;
use kinovea_geometry::Size;

#[test]
fn empty_info_describes_no_source() {
    let info = VideoInfo::empty();
    assert!(info.file_path.is_empty());
    assert_eq!(info.original_size, Size::default());
    assert_eq!(info.original_rotation, ImageRotation::Rotate0);
    assert_eq!(info.pixel_aspect_ratio, 1.0);
    assert!(info.sample_aspect_ratio.is_empty());
    assert_eq!(info.duration_timestamps, 0);
    assert!(!info.is_single_frame());
}

#[test]
fn single_frame_is_one_timestamp_long() {
    let single = VideoInfo {
        duration_timestamps: 1,
        ..VideoInfo::empty()
    };
    let two = VideoInfo {
        duration_timestamps: 2,
        ..VideoInfo::empty()
    };
    assert!(single.is_single_frame());
    assert!(!two.is_single_frame());
}

#[test]
fn frame_interval_follows_frame_rate() {
    assert_eq!(frame_interval_ms(25.0), 40.0);
    assert_eq!(frame_interval_ms(0.0), 0.0);
}

#[test]
fn fraction_with_a_zero_part_is_empty() {
    assert!(Fraction::default().is_empty());
    assert!(Fraction::new(0, 1).is_empty());
    assert!(Fraction::new(1, 0).is_empty());
    assert!(!Fraction::new(4, 3).is_empty());
}
