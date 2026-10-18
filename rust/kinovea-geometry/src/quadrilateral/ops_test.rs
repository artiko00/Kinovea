use crate::point::PointF;
use crate::quadrilateral::Quadrilateral;

fn p(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

fn rect42() -> Quadrilateral {
    Quadrilateral::from_size(4.0, 2.0)
}

#[test]
fn translate_scale_and_expand() {
    let mut q = Quadrilateral::unit_square();
    q.translate(1.0, 2.0);
    assert_eq!(q.a(), p(1.0, 2.0));
    q.scale(2.0, 3.0);
    assert_eq!(q.c(), p(4.0, 9.0));
    let mut e = Quadrilateral::unit_square();
    e.expand(1.0, 2.0);
    assert_eq!(
        e.vertices,
        [p(-1.0, -2.0), p(2.0, -2.0), p(2.0, 3.0), p(-1.0, 3.0)]
    );
}

#[test]
fn make_rectangle_aligns_on_each_anchor() {
    let skewed = Quadrilateral::new(p(0.0, 0.0), p(4.0, 1.0), p(5.0, 3.0), p(-1.0, 2.0));
    let expect = [
        [p(0.0, 0.0), p(4.0, 0.0), p(4.0, 2.0), p(0.0, 2.0)],
        [p(0.0, 1.0), p(4.0, 1.0), p(4.0, 3.0), p(0.0, 3.0)],
        [p(-1.0, 1.0), p(5.0, 1.0), p(5.0, 3.0), p(-1.0, 3.0)],
        [p(-1.0, 0.0), p(5.0, 0.0), p(5.0, 2.0), p(-1.0, 2.0)],
    ];
    for (anchor, want) in expect.iter().enumerate() {
        let mut q = skewed;
        q.make_rectangle(anchor);
        assert_eq!(&q.vertices, want, "anchor {anchor}");
        assert!(q.is_axis_aligned_rectangle());
    }
}

#[test]
fn make_square_uses_the_smallest_side_and_keeps_the_opposite_vertex() {
    let mut q0 = rect42();
    q0.make_square(0);
    assert_eq!(
        q0.vertices,
        [p(2.0, 0.0), p(4.0, 0.0), p(4.0, 2.0), p(2.0, 2.0)]
    );
    let mut q1 = rect42();
    q1.make_square(1);
    assert_eq!(
        q1.vertices,
        [p(0.0, 0.0), p(2.0, 0.0), p(2.0, 2.0), p(0.0, 2.0)]
    );
    let mut q2 = rect42();
    q2.make_square(2);
    assert_eq!(
        q2.vertices,
        [p(0.0, 0.0), p(2.0, 0.0), p(2.0, 2.0), p(0.0, 2.0)]
    );
    let mut q3 = rect42();
    q3.make_square(3);
    assert_eq!(
        q3.vertices,
        [p(2.0, 0.0), p(4.0, 0.0), p(4.0, 2.0), p(2.0, 2.0)]
    );
    let mut q4 = rect42();
    q4.make_square(4);
    assert_eq!(q4, rect42());
}

#[test]
fn flips_and_rotation_permute_vertices() {
    let base = Quadrilateral::new(p(0.0, 0.0), p(1.0, 0.0), p(1.0, 1.0), p(0.0, 1.0));
    let mut h = base;
    h.flip_horizontal();
    assert_eq!(h.vertices, [base.b(), base.a(), base.d(), base.c()]);
    let mut v = base;
    v.flip_vertical();
    assert_eq!(v.vertices, [base.d(), base.c(), base.b(), base.a()]);
    let mut r = base;
    r.rotate90();
    assert_eq!(r.vertices, [base.b(), base.c(), base.d(), base.a()]);
}
