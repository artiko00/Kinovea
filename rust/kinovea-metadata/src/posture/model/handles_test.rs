use crate::posture::model::*;
use kinovea_geometry::{PointF, PointLinePosition};

#[test]
fn constraint_and_impact_variants_carry_their_parameters() {
    let c = Constraint { kind: ConstraintKind::LineSlide { start: 0, end: 1, allowed: PointLinePosition::OnSegment, margin: 10 }, option_group: "a|b".into() };
    assert_eq!(c.option_group.split('|').count(), 2);
    let h = Handle { name: "h".into(), kind: HandleType::Point, color: TRANSPARENT, reference: 3, trackable: true, option_group: String::new(), grab_point: PointF::EMPTY, constraint: Some(c.clone()), impacts: vec![Impact::Pivot { pivot: 0, impacted: vec![1, 2] }] };
    assert_eq!(h.constraint, Some(c));
    assert!(matches!(h.impacts[0], Impact::Pivot { pivot: 0, .. }));
}
