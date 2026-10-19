//! Idempotence of the media drawings and the generic posture.

use super::media_posture::*;
use super::rounds::{check, drawing_round};

#[test]
fn bitmap_is_idempotent() {
    check(bitmap, &|x| drawing_round("Bitmap", x));
}

#[test]
fn svg_is_idempotent() {
    check(svg, &|x| drawing_round("SVG", x));
}

#[test]
fn generic_posture_is_idempotent() {
    check(generic_posture, &|x| drawing_round("GenericPosture", x));
}
