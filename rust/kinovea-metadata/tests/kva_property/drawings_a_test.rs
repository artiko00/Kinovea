//! Idempotence of the line-like drawings.

use super::drawings_a::*;
use super::rounds::{check, drawing_round};

#[test]
fn line_is_idempotent() {
    check(line, &|x| drawing_round("Line", x));
}

#[test]
fn angle_is_idempotent() {
    check(angle, &|x| drawing_round("Angle", x));
}

#[test]
fn cross_mark_is_idempotent() {
    check(cross_mark, &|x| drawing_round("CrossMark", x));
}

#[test]
fn circle_is_idempotent() {
    check(circle, &|x| drawing_round("Circle", x));
}

#[test]
fn label_is_idempotent() {
    check(label, &|x| drawing_round("Label", x));
}

#[test]
fn time_segment_is_idempotent() {
    check(time_segment, &|x| drawing_round("TimeSegment", x));
}
