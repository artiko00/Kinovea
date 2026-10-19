use crate::template::capture::capture;
use crate::template::matching::match_block;
use crate::template_test::{disc, square};
use kinovea_geometry::Size;
use kinovea_metadata::drawing::tracking_parameters::TrackingParameters;

pub(crate) fn parameters(use_mask: bool, block: Size) -> TrackingParameters {
    let mut parameters = TrackingParameters::default();
    parameters.set_block_window(block);
    parameters.use_mask = use_mask;
    parameters
}

#[test]
fn a_translated_square_is_found_within_half_a_pixel() {
    let scene = square(10);
    let reference = scene.render((160, 160));
    let block = Size::new(20, 20);
    let patch = capture(&reference, (40.0, 40.0), block);
    let moved = scene.render((172, 168));
    let found = match_block(
        &parameters(false, block),
        &moved,
        &patch,
        (40.0, 40.0),
        &mut None,
    );
    assert!((found.x - 43.0).abs() <= 0.5, "x = {}", found.x);
    assert!((found.y - 42.0).abs() <= 0.5, "y = {}", found.y);
    assert!(found.similarity > 0.9, "similarity = {}", found.similarity);
}

#[test]
fn a_masked_disc_is_found_with_the_elliptic_mask() {
    let scene = disc(8);
    let block = Size::new(21, 17);
    let patch = capture(&scene.render((160, 160)), (40.0, 40.0), block);
    let moved = scene.render((176, 168));
    let found = match_block(
        &parameters(true, block),
        &moved,
        &patch,
        (40.0, 40.0),
        &mut None,
    );
    assert!((found.x - 44.0).abs() <= 0.5, "x = {}", found.x);
    assert!((found.y - 42.0).abs() <= 0.5, "y = {}", found.y);
}

#[test]
fn a_search_window_outside_the_image_gives_no_similarity() {
    let scene = square(10);
    let block = Size::new(20, 20);
    let patch = capture(&scene.render((160, 160)), (40.0, 40.0), block);
    let frame = scene.render((160, 160));
    let found = match_block(
        &parameters(false, block),
        &frame,
        &patch,
        (-500.0, -500.0),
        &mut None,
    );
    assert_eq!(found.similarity, 0.0);
    assert_eq!((found.x, found.y), (-500.0, -500.0));
}
