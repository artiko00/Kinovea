//! Idempotence of the shape and point-list drawings.

use super::drawings_b::*;
use super::rounds::{check, drawing_round};

#[test]
fn rectangle_is_idempotent() {
    check(rectangle, &|x| drawing_round("Rectangle", x));
}

#[test]
fn plane_is_idempotent() {
    check(plane, &|x| drawing_round("Plane", x));
}

#[test]
fn pencil_is_idempotent() {
    check(pencil, &|x| drawing_round("Pencil", x));
}

#[test]
fn polyline_is_idempotent() {
    check(polyline, &|x| drawing_round("Polyline", x));
}

#[test]
fn time_polyline_is_idempotent() {
    check(time_polyline, &|x| drawing_round("TimePolyline", x));
}

#[test]
fn distortion_grid_is_idempotent() {
    check(distortion_grid, &|x| drawing_round("DistortionGrid", x));
}
