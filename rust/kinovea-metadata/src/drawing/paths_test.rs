use crate::drawing::line_test::body;
use crate::drawing::{element, KvaDrawing, Pencil, Polyline, TimePolyline};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

fn refs(_: Uuid, _: &str) -> PointF {
    PointF::EMPTY
}

#[test]
fn pencil_point_list_round_trips() {
    let frag = "<Pencil id=\"00000000-0000-4000-8000-000000000006\" name=\"Pencil 2\">\n  <PointList Count=\"2\">\n    <Point>452;215</Point>\n    <Point>449;211</Point>\n  </PointList>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;255;102;255</Value>\n    </Color>\n    <PenSize Key=\"pen size\">\n      <Value>24</Value>\n    </PenSize>\n    <PenShape Key=\"pen shape\">\n      <Value>Solid</Value>\n    </PenShape>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>false</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>80</Frames>\n  </InfosFading>\n</Pencil>";
    let p = Pencil::read_kva(&Element::parse(frag).unwrap(), Scale::IDENTITY, &identity);
    assert_eq!(p.points.len(), 2);
    assert_eq!(body(element(&p, Filter::KVA, &refs).to_document()), frag);
    let half = Pencil::read_kva(&Element::parse(frag).unwrap(), Scale { x: 0.5, y: 0.5 }, &identity);
    assert_eq!(half.points[0], PointF::new(226.0, 107.5));
    let mut longer = p.clone();
    longer.points.push(PointF::new(7.0, 11.0));
    assert_ne!(longer.points.len(), p.points.len());
}

#[test]
fn polyline_keeps_keys_and_drops_duplicates() {
    let frag = "<Polyline id=\"00000000-0000-4000-8000-000000000001\" name=\"Polyline 1\">\n  <PointList Count=\"2\">\n    <Point key=\"0\">1;2</Point>\n    <Point key=\"1\">3;4</Point>\n  </PointList>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;0;0;0</Value>\n    </Color>\n    <LineSize Key=\"line size\">\n      <Value>2</Value>\n    </LineSize>\n    <LineShape Key=\"line shape\">\n      <Value>Solid</Value>\n    </LineShape>\n    <Arrows Key=\"arrows\">\n      <Value>None</Value>\n    </Arrows>\n    <Toggle Key=\"curved\">\n      <Value>true</Value>\n      <Variant>Curved</Variant>\n    </Toggle>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n</Polyline>";
    let mut p = Polyline::read_kva(&Element::parse(frag).unwrap(), Scale::IDENTITY, &identity);
    assert_eq!(body(element(&p, Filter::KVA, &refs).to_document()), frag);
    assert_eq!(p.trackable_points()[1], ("1".to_string(), PointF::new(3.0, 4.0)));
    p.set_trackable_point("1", PointF::new(9.0, 9.0));
    p.set_trackable_point("zz", PointF::new(9.0, 9.0));
    assert_eq!(p.points.0[1].1, PointF::new(9.0, 9.0));
    let dup = Element::parse("<Polyline><PointList Count=\"2\"><Point key=\"0\">1;2</Point><Point key=\"0\">5;6</Point></PointList></Polyline>").unwrap();
    assert_eq!(Polyline::read_kva(&dup, Scale::IDENTITY, &identity).points.0.len(), 1);
    let t = TimePolyline::new(vec![("0".into(), PointF::new(1.0, 1.0))]);
    assert_eq!(TimePolyline::KVA_NAME, "TimePolyline");
    assert_eq!(t.trackable_points().len(), 1);
}
