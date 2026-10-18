use crate::clipping::{clip_infinite_line, clip_line, clip_segment};
use crate::lib_test::Rng;
use crate::point::PointF;
use crate::rect::RectangleF;

const WINDOW: RectangleF = RectangleF::new(500.0, 500.0, 1000.0, 1000.0);

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn near(a: PointF, b: PointF) -> bool {
    (a.x - b.x).abs() < 1e-3 && (a.y - b.y).abs() < 1e-3
}

#[test]
fn diagonal_through_the_window() {
    let r = clip_infinite_line(WINDOW, p(250.0, 1750.0), p(1750.0, 250.0));
    assert!(r.visible);
    assert!(
        near(r.a, p(500.0, 1500.0)) && near(r.b, p(1500.0, 500.0)),
        "{r:?}"
    );
    let s = clip_segment(WINDOW, p(250.0, 1750.0), p(1750.0, 250.0));
    assert_eq!(r, s);
}

#[test]
fn hand_crafted_lines_of_the_original_tester() {
    let cases: [(PointF, PointF, bool); 10] = [
        (p(250.0, 1750.0), p(1750.0, 250.0), true),
        (p(250.0, 1500.0), p(1750.0, 0.0), true),
        (p(500.0, 250.0), p(1750.0, 1250.0), true),
        (p(250.0, 250.0), p(500.0, 1750.0), false),
        (p(250.0, 250.0), p(250.0, 1750.0), false),
        (p(600.0, 250.0), p(600.0, 1750.0), true),
        (p(500.0, 250.0), p(500.0, 1750.0), true),
        (p(250.0, 750.0), p(1750.0, 750.0), true),
        (p(750.0, 750.0), p(1250.0, 1250.0), true),
        (p(1750.0, 250.0), p(750.0, 750.0), true),
    ];
    for (a, b, visible) in cases {
        let r = clip_infinite_line(WINDOW, a, b);
        assert_eq!(r.visible, visible, "{a:?}-{b:?}");
    }
}

#[test]
fn line_missing_the_window_is_invisible_as_segment_and_line() {
    let (a, b) = (p(250.0, 250.0), p(500.0, 1750.0));
    assert!(!clip_segment(WINDOW, a, b).visible);
    assert!(!clip_infinite_line(WINDOW, a, b).visible);
    let (c, d) = (p(250.0, 1500.0), p(1750.0, 0.0));
    assert!(clip_segment(WINDOW, c, d).visible);
    assert!(clip_infinite_line(WINDOW, c, d).visible);
}

#[test]
fn lines_parallel_to_an_edge() {
    assert!(!clip_infinite_line(WINDOW, p(250.0, 250.0), p(250.0, 1750.0)).visible);
    let on_edge = clip_infinite_line(WINDOW, p(500.0, 250.0), p(500.0, 1750.0));
    assert!(on_edge.visible);
    assert!(near(on_edge.a, p(500.0, 500.0)) && near(on_edge.b, p(500.0, 1500.0)));
}

#[test]
fn rays_extend_on_one_side_only() {
    let (a, b) = (p(750.0, 750.0), p(1250.0, 1250.0));
    let forward = clip_line(WINDOW, a, b, 0.0, f64::MAX);
    assert!(
        near(forward.a, a) && near(forward.b, p(1500.0, 1500.0)),
        "{forward:?}"
    );
    let backward = clip_line(WINDOW, a, b, f64::MIN, 1.0);
    assert!(
        near(backward.a, p(500.0, 500.0)) && near(backward.b, b),
        "{backward:?}"
    );
    let seg = clip_segment(WINDOW, a, b);
    assert!(near(seg.a, a) && near(seg.b, b));
}

#[test]
fn visible_endpoints_lie_on_the_boundary_or_inside() {
    let mut rng = Rng::new(5);
    let on_or_inside = |q: PointF| {
        q.x >= WINDOW.left() - 1e-2
            && q.x <= WINDOW.right() + 1e-2
            && q.y >= WINDOW.top() - 1e-2
            && q.y <= WINDOW.bottom() + 1e-2
    };
    for _ in 0..500 {
        let a = p(rng.range(0.0, 2000.0), rng.range(0.0, 2000.0));
        let b = p(rng.range(0.0, 2000.0), rng.range(0.0, 2000.0));
        for r in [clip_segment(WINDOW, a, b), clip_infinite_line(WINDOW, a, b)] {
            if r.visible {
                assert!(
                    on_or_inside(r.a) && on_or_inside(r.b),
                    "{a:?} {b:?} -> {r:?}"
                );
            }
        }
    }
}
