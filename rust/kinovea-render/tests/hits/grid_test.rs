use super::*;
use std::collections::HashMap;

/// The oracle's answers of a disc of radius `r` centred on (200, 150): code 0 inside, misses outside.
fn disc(r: f32) -> Expected {
    let inside = |x: i32, y: i32| {
        let (dx, dy) = ((x - 200) as f32, (y - 150) as f32);
        dx * dx + dy * dy <= r * r
    };
    let hits: HashMap<(i32, i32), i32> = probes(400, 300, 4)
        .into_iter()
        .filter(|&(x, y)| inside(x, y))
        .map(|p| (p, 0))
        .collect();
    Expected {
        hits,
        exception: false,
    }
}

/// Rust's disc of radius `r`, answering code 0 inside.
fn rust_disc(r: f32) -> impl Fn(PointF) -> i32 {
    move |p: PointF| {
        let (dx, dy) = (p.x - 200.0, p.y - 150.0);
        if dx * dx + dy * dy <= r * r {
            0
        } else {
            -1
        }
    }
}

fn whole_image_percent(probes: &[Probe]) -> f64 {
    let agree = probes.iter().filter(|p| p.expected == p.got).count();
    100.0 * agree as f64 / probes.len() as f64
}

// A uniform 1 px error on a circle is inside the 2 px boundary tolerance, so only the box rule can
// see it: over the box of the disc it is about 2 percent, while over the whole image it passes.
#[test]
fn a_one_pixel_error_on_a_circle_fails_the_box_rule_but_not_the_whole_image() {
    let probes = grid(&disc(40.0), 400, 300, 4, rust_disc(41.0));
    let boxed = verdict(&probes, 400, 300, &[]);
    assert!(boxed.percent < MIN_PERCENT, "{:.2}%", boxed.percent);
    assert_eq!(boxed.off_boundary, 0);
    assert!(whole_image_percent(&probes) >= MIN_PERCENT);
}

#[test]
fn the_same_circle_on_both_sides_agrees_everywhere() {
    let probes = grid(&disc(40.0), 400, 300, 4, rust_disc(40.0));
    assert_eq!(verdict(&probes, 400, 300, &[]).percent, 100.0);
}

#[test]
fn exempt_codes_leave_their_probes_out_of_the_verdict() {
    let probes = grid(&disc(40.0), 400, 300, 4, rust_disc(41.0));
    let exempt_zero = verdict(&probes, 400, 300, &[0]);
    assert_eq!(exempt_zero.percent, 100.0);
    assert_eq!(exempt_zero.off_boundary, 0);
}

#[test]
fn an_image_with_no_answers_agrees_by_default() {
    let probes = grid(&Expected::default(), 400, 300, 4, |_| -1);
    assert!(probe_box(&probes, 400, 300).is_none());
    assert_eq!(verdict(&probes, 400, 300, &[]).percent, 100.0);
}

// The disc of radius 10 has probes at x 192..=208 and y 140..=160 (multiples of 4): the box adds 16.
#[test]
fn the_box_is_the_active_probes_grown_by_the_margin() {
    let probes = grid(&disc(10.0), 400, 300, 4, rust_disc(10.0));
    assert_eq!(probe_box(&probes, 400, 300), Some((176, 124, 224, 176)));
}

#[test]
fn probes_follow_the_oracle_loop() {
    assert_eq!(
        probes(5, 3, 2),
        vec![(0, 0), (2, 0), (4, 0), (0, 2), (2, 2), (4, 2)]
    );
}
