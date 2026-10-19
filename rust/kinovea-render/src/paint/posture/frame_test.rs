//! Preamble of `DrawingGenericPosture.Draw`: alphas are `(int)(opacity * 255)` and
//! `(int)(opacity * 92)`, the style is bound to the background, and option groups need every key on.

use super::fixture_test::{drawing, option, scene, two_points};
use super::frame::{colored, PostureFrame};
use crate::paint::helpers::frame::scene::COLOR;
use crate::style::colors::with_alpha;
use kinovea_metadata::posture::TRANSPARENT;
use kinovea_metadata::{Color, Fading};

#[test]
fn half_opacity_gives_alpha_127_and_background_alpha_46() {
    // 0.5 * 255 = 127.5 -> 127; 0.5 * 92 = 46.
    let scene = scene(1.0);
    let mut d = drawing(two_points());
    d.header.fading = Fading {
        use_default: false,
        always_visible: true,
        master_factor: 0.5,
        ..Fading::default()
    };
    let ctx = scene.ctx(0);
    let frame = PostureFrame::new(&d, &ctx).expect("visible");
    assert_eq!((frame.alpha, frame.alpha_background), (127, 46));
}

#[test]
fn zero_opacity_has_no_frame() {
    let scene = scene(1.0);
    let mut d = drawing(two_points());
    d.header.fading = Fading {
        use_default: false,
        ..Fading::default()
    };
    let ctx = scene.ctx(10);
    assert!(PostureFrame::new(&d, &ctx).is_none());
}

#[test]
fn base_colour_is_the_background_at_the_frame_alpha() {
    let scene = scene(1.0);
    let d = drawing(two_points());
    let ctx = scene.ctx(0);
    let frame = PostureFrame::new(&d, &ctx).expect("visible");
    assert_eq!(frame.edge_base(), COLOR);
}

#[test]
fn transparent_colours_fall_back_to_the_base() {
    let scene = scene(1.0);
    let d = drawing(two_points());
    let ctx = scene.ctx(0);
    let frame = PostureFrame::new(&d, &ctx).expect("visible");
    let red = Color::rgb(255, 0, 0);
    let base = frame.edge_base();
    assert_eq!(colored(TRANSPARENT, 255, base), base);
    assert_eq!(colored(red, 127, base), with_alpha(red, 127));
}

#[test]
fn a_group_is_active_only_when_every_option_in_it_is_on() {
    let scene = scene(1.0);
    let mut posture = two_points();
    posture.options = vec![option("a", true), option("b", false)];
    let d = drawing(posture);
    let ctx = scene.ctx(0);
    let frame = PostureFrame::new(&d, &ctx).expect("visible");
    assert!(frame.is_active(""));
    assert!(frame.is_active("a"));
    assert!(!frame.is_active("b"));
    assert!(!frame.is_active("a|b"));
    // Unknown keys are ignored, as the original does.
    assert!(frame.is_active("a|missing"));
    assert!(frame.is_active("missing"));
}
