use crate::posture::model::*;
use kinovea_geometry::PointF;

fn computed(children: Vec<WeightedPoint>) -> ComputedPoint {
    ComputedPoint { name: "c".into(), weight: 1.0, color: TRANSPARENT, symbol: String::new(), option_group: String::new(), display_coordinates: false, children }
}

#[test]
fn two_point_blends_complete_a_missing_weight() {
    let points = [PointF::new(0.0, 0.0), PointF::new(100.0, 0.0), PointF::new(0.0, 100.0)];
    let c = computed(vec![WeightedPoint::Reference { reference: 0, weight: 0.0 }, WeightedPoint::Reference { reference: 1, weight: 0.25 }]);
    assert_eq!(c.location(&points), PointF::new(25.0, 0.0));
    let c = computed(vec![WeightedPoint::Reference { reference: 0, weight: 0.5 }, WeightedPoint::Reference { reference: 1, weight: 0.0 }]);
    assert_eq!(c.location(&points), PointF::new(50.0, 0.0));
}

#[test]
fn many_points_are_a_weighted_sum_and_bad_references_are_empty() {
    let points = [PointF::new(0.0, 0.0), PointF::new(100.0, 0.0), PointF::new(0.0, 100.0)];
    let inner = computed(vec![WeightedPoint::Reference { reference: 1, weight: 0.5 }, WeightedPoint::Reference { reference: 2, weight: 0.5 }]);
    let mut inner = inner;
    inner.weight = 0.5;
    let c = computed(vec![WeightedPoint::Reference { reference: 0, weight: 0.25 }, WeightedPoint::Computed(inner), WeightedPoint::Reference { reference: 9, weight: 0.25 }]);
    assert_eq!(c.location(&points), PointF::new(25.0, 25.0));
}
