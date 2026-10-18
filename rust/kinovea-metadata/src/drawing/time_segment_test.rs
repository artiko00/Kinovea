use crate::drawing::line_test::body;
use crate::drawing::{element, KvaDrawing, TimeSegment};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

#[test]
fn fragment_round_trips_and_time_point_follows_the_fraction() {
    let frag = "<TimeSegment id=\"00000000-0000-4000-8000-000000000001\" name=\"TimeSegment 1\">\n  <Start>0;0</Start>\n  <End>100;50</End>\n  <Fraction>0.25</Fraction>\n  <MeasureLabel>\n    <SpacePosition>10;10</SpacePosition>\n    <TimePosition>3</TimePosition>\n  </MeasureLabel>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;255;127;0</Value>\n    </Color>\n    <FontSize Key=\"Font\">\n      <Value>10</Value>\n    </FontSize>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n</TimeSegment>";
    let mut s = TimeSegment::read_kva(&Element::parse(frag).unwrap(), Scale::IDENTITY, &identity);
    assert_eq!(s.time_point(), PointF::new(25.0, 12.5));
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    assert_eq!(body(element(&s, Filter::KVA, &refs).to_document()), frag);
    assert_eq!(s.trackable_points().len(), 2);
    let before = s.content_hash();
    s.set_trackable_point("b", PointF::new(200.0, 50.0));
    assert_eq!(s.label.attach, PointF::new(50.0, 12.5));
    s.fraction = 0.5;
    assert_ne!(s.content_hash(), before);
}
