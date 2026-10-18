use super::player_params::*;

#[test]
fn re_exports_are_reachable() {
    assert_eq!(InfosFading::default().fading_frames, 20);
    assert_eq!(KinogramParameters::default().tile_count, 18);
    assert_eq!(KeyframePresets::default().0.len(), 10);
}
