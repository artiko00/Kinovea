use crate::capabilities::{VideoCapabilities as Caps, VideoDecodingMode};

#[test]
fn flag_values_match_csharp() {
    assert_eq!(Caps::CAN_DECODE_ON_DEMAND.bits(), 1);
    assert_eq!(Caps::CAN_PRE_BUFFER.bits(), 2);
    assert_eq!(Caps::CAN_CACHE.bits(), 4);
    assert_eq!(Caps::CAN_CHANGE_WORKING_ZONE.bits(), 8);
    assert_eq!(Caps::CAN_CHANGE_ASPECT_RATIO.bits(), 16);
    assert_eq!(Caps::CAN_CHANGE_DEINTERLACING.bits(), 32);
    assert_eq!(Caps::CAN_CHANGE_VIDEO_DURATION.bits(), 64);
    assert_eq!(Caps::CAN_CHANGE_FRAME_RATE.bits(), 128);
    assert_eq!(Caps::CAN_CHANGE_IMAGE_ROTATION.bits(), 256);
    assert_eq!(Caps::CAN_CHANGE_DEMOSAICING.bits(), 512);
    assert_eq!(Caps::CAN_STABILIZE.bits(), 1024);
}

#[test]
fn flags_combine_with_bit_or() {
    let flags = Caps::CAN_DECODE_ON_DEMAND | Caps::CAN_CACHE;
    assert!(flags.can_decode_on_demand());
    assert!(flags.can_cache());
    assert!(!flags.can_pre_buffer());
}

#[test]
fn none_declares_nothing() {
    assert_eq!(Caps::default(), Caps::NONE);
    assert!(!Caps::NONE.can_decode_on_demand());
    assert!(!Caps::NONE.can_cache());
    assert!(!Caps::NONE.can_stabilize());
}

#[test]
fn accessors_read_their_own_flag() {
    assert!(Caps::CAN_PRE_BUFFER.can_pre_buffer());
    assert!(Caps::CAN_CHANGE_WORKING_ZONE.can_change_working_zone());
    assert!(Caps::CAN_CHANGE_ASPECT_RATIO.can_change_aspect_ratio());
    assert!(Caps::CAN_CHANGE_IMAGE_ROTATION.can_change_image_rotation());
    assert!(Caps::CAN_CHANGE_DEMOSAICING.can_change_demosaicing());
    assert!(Caps::CAN_CHANGE_DEINTERLACING.can_change_deinterlacing());
    assert!(Caps::CAN_STABILIZE.can_stabilize());
}

#[test]
fn switch_matrix_follows_flags() {
    let none = Caps::NONE;
    assert!(none.can_switch_decoding_mode(VideoDecodingMode::NotInitialized));
    assert!(!none.can_switch_decoding_mode(VideoDecodingMode::OnDemand));
    assert!(!none.can_switch_decoding_mode(VideoDecodingMode::PreBuffering));
    assert!(!none.can_switch_decoding_mode(VideoDecodingMode::Caching));
    assert!(Caps::CAN_DECODE_ON_DEMAND.can_switch_decoding_mode(VideoDecodingMode::OnDemand));
    assert!(Caps::CAN_PRE_BUFFER.can_switch_decoding_mode(VideoDecodingMode::PreBuffering));
    assert!(!Caps::CAN_PRE_BUFFER.can_switch_decoding_mode(VideoDecodingMode::Caching));
    assert!(Caps::CAN_CACHE.can_switch_decoding_mode(VideoDecodingMode::Caching));
}
