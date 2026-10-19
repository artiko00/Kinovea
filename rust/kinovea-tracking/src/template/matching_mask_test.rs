use crate::pixels::ellipse_mask;
use crate::template::capture::{capture, Patch};
use crate::template::matching::{match_block, Found};
use crate::template::matching_test::parameters;
use crate::template_test::disc;
use kinovea_geometry::Size;
use kinovea_video::frame::FrameBuffer;
use std::sync::Arc;

/// `match_block` at the reference point (40, 40) with no mask kept yet.
fn fresh_match(use_mask: bool, block: Size, frame: &FrameBuffer, patch: &Patch) -> Found {
    match_block(
        &parameters(use_mask, block),
        frame,
        patch,
        (40.0, 40.0),
        &mut None,
    )
}

/// A 160x120 frame of uniform background with the four corners of a 21x17 block set apart. Its
/// elliptic mask covers only background, so the masked template is flat.
fn corners_at(top_left: (u32, u32)) -> FrameBuffer {
    let (width, height) = (160u32, 120u32);
    let mut bits: Vec<u8> = std::iter::repeat_n([50, 50, 50, 255], (width * height) as usize)
        .flatten()
        .collect();
    for (dx, dy) in [(0, 0), (20, 0), (0, 16), (20, 16)] {
        let at = ((top_left.1 + dy) * width + top_left.0 + dx) as usize * 4;
        bits[at..at + 3].copy_from_slice(&[230, 230, 230]);
    }
    FrameBuffer::new(width, height, width as usize * 4, Arc::from(bits))
}

#[test]
fn a_template_flat_under_the_mask_falls_back_to_the_unmasked_match() {
    // TrackerTemplateMatching.cs:498: the infinite masked map is retried without the mask, which finds
    // the corners at (45, 43) with similarity 1 (the original). The masked map alone would stay at the
    // window corner.
    let block = Size::new(21, 17);
    let patch = capture(&corners_at((30, 32)), (40.0, 40.0), block);
    let moved = corners_at((35, 35));
    let found = fresh_match(true, block, &moved, &patch);
    assert!((found.x - 45.0).abs() <= 0.5, "x = {}", found.x);
    assert!((found.y - 43.0).abs() <= 0.5, "y = {}", found.y);
    assert!(found.similarity > 0.99, "similarity = {}", found.similarity);
}

#[test]
fn a_mask_built_for_masking_is_kept_after_masking_is_turned_off() {
    // TrackerTemplateMatching.cs:452-480: cvMaskGray is tracker state and is passed on every match,
    // whatever UseMask says now. The unmasked score of the same peak differs, so the test sees it.
    let scene = disc(8);
    let block = Size::new(21, 17);
    let patch = capture(&scene.render((160, 160)), (40.0, 40.0), block);
    let moved = scene.render((176, 168));
    let mut mask = None;
    let masked = match_block(
        &parameters(true, block),
        &moved,
        &patch,
        (40.0, 40.0),
        &mut mask,
    );
    let kept = match_block(
        &parameters(false, block),
        &moved,
        &patch,
        (40.0, 40.0),
        &mut mask,
    );
    let plain = fresh_match(false, block, &moved, &patch);
    assert_eq!(kept, masked);
    assert_ne!(plain.similarity, masked.similarity);
}

#[test]
fn a_mask_of_another_size_is_not_applied() {
    // OpenCV rejects a mask whose size differs from the template (templmatch.cpp:766). The original
    // throws there; the port matches without a mask.
    let scene = disc(8);
    let block = Size::new(21, 17);
    let patch = capture(&scene.render((160, 160)), (40.0, 40.0), block);
    let moved = scene.render((176, 168));
    let mut stale = Some(ellipse_mask(10, 10));
    let found = match_block(
        &parameters(false, block),
        &moved,
        &patch,
        (40.0, 40.0),
        &mut stale,
    );
    let plain = fresh_match(false, block, &moved, &patch);
    assert_eq!(found, plain);
}
