use crate::posture::model::*;
use crate::posture::parser::parse_posture;

fn handles(xml: &str) -> Vec<Handle> {
    parse_posture(&format!("<KinoveaPostureTool><Points><Point value=\"0;0\"/><Point value=\"1;1\"/></Points><Handles>{xml}</Handles></KinoveaPostureTool>"), false).unwrap().handles
}

#[test]
fn every_constraint_kind_is_read() {
    let h = handles("<Handle type=\"Point\" reference=\"1\" trackable=\"true\" color=\"#010203\"><Constraint type=\"DistanceToPoint\" optionGroup=\"g\"><DistanceToPoint point=\"0\"/></Constraint></Handle><Handle type=\"Segment\" reference=\"0\"><Constraint type=\"VerticalSlide\"/></Handle><Handle type=\"Circle\" reference=\"0\"><Constraint type=\"PerpendicularSlide\"><PerpendicularSlide origin=\"0\" leg1=\"1\"/></Constraint></Handle><Handle type=\"Point\" reference=\"0\"><Constraint type=\"ParallelSlide\"><ParallelSlide a=\"0\" b=\"1\" c=\"1\"/></Constraint></Handle><Handle type=\"Point\" reference=\"0\"><Constraint type=\"Bogus\"/></Handle><Handle type=\"Point\" reference=\"0\"><Constraint type=\"LockedInPlace\"/></Handle>");
    assert_eq!(h.len(), 6);
    assert!(h[0].trackable && h[0].kind == HandleType::Point);
    assert_eq!(h[0].color, crate::color::Color::rgb(1, 2, 3));
    assert!(matches!(&h[0].constraint, Some(Constraint { kind: ConstraintKind::DistanceToPoint { point: 0 }, option_group }) if option_group == "g"));
    assert!(matches!(h[1].constraint.as_ref().map(|c| &c.kind), Some(ConstraintKind::VerticalSlide)));
    assert!(matches!(h[2].constraint.as_ref().map(|c| &c.kind), Some(ConstraintKind::PerpendicularSlide { origin: 0, leg1: 1 })));
    assert!(matches!(h[3].constraint.as_ref().map(|c| &c.kind), Some(ConstraintKind::ParallelSlide { a: 0, b: 1, c: 1 })));
    assert!(h[4].constraint.is_none());
    assert!(matches!(h[5].constraint.as_ref().map(|c| &c.kind), Some(ConstraintKind::LockedInPlace)));
}

#[test]
fn every_impact_kind_is_read_and_unknown_ones_skipped() {
    let h = handles("<Handle type=\"Point\" reference=\"0\"><Impacts><Align pointToAlign=\"1\" point1=\"0\" point2=\"1\"/><VerticalAlign point=\"1\"/><HorizontalAlign point=\"1\"/><Pivot pivot=\"0\" impacted=\"1;1\"/><KeepAngle origin=\"0\" leg1=\"1\" leg2=\"1\"/><HorizontalSymmetry impacting=\"0\" impacted=\"1\" axis=\"0\"/><SegmentCenter pointToMove=\"1\" point1=\"0\" point2=\"1\"/><PerdpendicularAlign pointToMove=\"1\" origin=\"0\" leg1=\"1\"/><ParallelAlign pointToMove=\"1\" a=\"0\" b=\"1\" c=\"1\"/><Nope/></Impacts></Handle>");
    assert_eq!(h[0].impacts.len(), 9);
    assert!(matches!(h[0].impacts[3], Impact::Pivot { pivot: 0, ref impacted } if *impacted == vec![1, 1]));
    assert!(matches!(h[0].impacts[8], Impact::ParallelAlign { point_to_move: 1, a: 0, b: 1, c: 1 }));
}
