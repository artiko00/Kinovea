use crate::circle_fit::fit_circle;
use crate::point::PointF;
use crate::shapes::Circle;

fn on_circle(cx: f32, cy: f32, r: f32, n: usize) -> Vec<PointF> {
    (0..n)
        .map(|i| {
            let t = i as f64 * std::f64::consts::TAU / n as f64;
            PointF::new(cx + r * t.cos() as f32, cy + r * t.sin() as f32)
        })
        .collect()
}

#[test]
fn exact_fit_of_twelve_points() {
    let c = fit_circle(&on_circle(3.0, -2.0, 5.0, 12));
    assert!((c.center.x - 3.0).abs() < 1e-4, "{c:?}");
    assert!((c.center.y + 2.0).abs() < 1e-4, "{c:?}");
    assert!((c.radius - 5.0).abs() < 1e-4, "{c:?}");
}

#[test]
fn three_points_define_the_circle() {
    let c = fit_circle(&[
        PointF::new(1.0, 0.0),
        PointF::new(0.0, 1.0),
        PointF::new(-1.0, 0.0),
    ]);
    assert!(c.center.x.abs() < 1e-5 && c.center.y.abs() < 1e-5 && (c.radius - 1.0).abs() < 1e-5);
}

#[test]
fn degenerate_inputs_return_the_empty_circle() {
    assert_eq!(fit_circle(&[]), Circle::EMPTY);
    assert_eq!(
        fit_circle(&[PointF::new(0.0, 0.0), PointF::new(1.0, 1.0)]),
        Circle::EMPTY
    );
    let collinear = [
        PointF::new(0.0, 0.0),
        PointF::new(1.0, 1.0),
        PointF::new(2.0, 2.0),
    ];
    assert_eq!(fit_circle(&collinear), Circle::EMPTY);
    let same = [PointF::new(1.0, 1.0); 5];
    assert_eq!(fit_circle(&same), Circle::EMPTY);
}
