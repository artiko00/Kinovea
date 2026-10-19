use crate::posture::model::*;
use crate::posture::parser::parse_posture;

fn tool(body: &str) -> GenericPosture {
    parse_posture(&format!("<KinoveaPostureTool><Points><Point name=\"a\" value=\"0;0\" color=\"1;2;3\"/><Point value=\"10;0\"/></Points>{body}</KinoveaPostureTool>"), false).unwrap()
}

#[test]
fn strokes_and_arrows() {
    let p = tool("<Segments><Segment name=\"s\" point1=\"0\" point2=\"1\" style=\"Dash\" width=\"4\" arrowStart=\"true\" arrowEnd=\"1\" color=\"#ff0000\" optionGroup=\"g\"/></Segments><Polylines><Polyline points=\"0;1\" arrowBegin=\"true\"/></Polylines><Circles><Ellipse center=\"$1\" radius=\"7\"/></Circles>");
    let s = &p.segments[0];
    assert_eq!((s.start, s.end, s.stroke.style, s.stroke.width), (0, 1, SegmentLineStyle::Dash, 4));
    assert!(s.stroke.arrow_begin && s.stroke.arrow_end);
    assert_eq!(s.stroke.color, crate::color::Color::rgb(255, 0, 0));
    assert_eq!(s.option_group, "g");
    assert!(p.polylines[0].stroke.arrow_begin && !p.polylines[0].stroke.arrow_end);
    assert_eq!(p.polylines[0].points, vec![0, 1]);
    assert_eq!((p.circles[0].center, p.circles[0].radius), (-2, 7), "$n references are negative");
    assert_eq!(p.points[0].color, crate::color::Color::rgb(1, 2, 3));
}

#[test]
fn angles_measures_and_computed_points() {
    let p = tool("<Angles><Angle name=\"n\" symbol=\"sym\" origin=\"0\" leg1=\"1\" leg2=\"1\" signed=\"false\" ccw=\"false\" supplementary=\"true\" radius=\"-25\" tenth=\"true\"/></Angles><Distances><Distance point1=\"0\" point2=\"1\" symbol=\"d\"/></Distances><Positions><Position point=\"1\" symbol=\"p\"/></Positions><ComputedPoints><ComputedPoint name=\"c\" weight=\"3\" displayCoordinates=\"true\"><ReferencePoint ref=\"0\" weight=\"0.5\"/><ReferencePoint ref=\"1\" weight=\"0.5\"/></ComputedPoint></ComputedPoints>");
    let a = &p.angles[0];
    assert_eq!(a.name, "sym", "the symbol attribute overrides the name, as in the original");
    assert_eq!((a.signed, a.ccw, a.supplementary, a.radius, a.text_distance, a.tenth), (false, false, true, -25, 40, true));
    assert_eq!((p.distances[0].point1, p.distances[0].point2, p.distances[0].symbol.as_str()), (0, 1, "d"));
    assert_eq!((p.positions[0].point1, p.positions[0].point2), (1, 1));
    let c = &p.computed_points[0];
    assert_eq!(c.weight, 1.0, "weights are clamped to [0, 1]");
    assert!(c.display_coordinates);
    assert_eq!(c.location(&p.point_list), kinovea_geometry::PointF::new(5.0, 0.0));
}
