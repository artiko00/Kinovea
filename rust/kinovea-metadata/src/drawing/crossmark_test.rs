use crate::drawing::line_test::body;
use crate::drawing::{element, CrossMark, KvaDrawing};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<CrossMark id=\"00000000-0000-4000-8000-000000000017\" name=\"Marker 4\">\n  <CenterPoint>243;54</CenterPoint>\n  <ReferenceTimestamp>318318</ReferenceTimestamp>\n  <ExtraData>Position</ExtraData>\n  <ShowAsDot>false</ShowAsDot>\n  <MeasureLabel>\n    <SpacePosition>-263;-104</SpacePosition>\n    <TimePosition>0</TimePosition>\n  </MeasureLabel>\n  <DrawingStyle>\n    <Color Key=\"back color\">\n      <Value>255;100;149;237</Value>\n    </Color>\n    <FontSize Key=\"Font\">\n      <Value>10</Value>\n    </FontSize>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n</CrossMark>";

#[test]
fn fragment_round_trips() {
    let mark = CrossMark::read_kva(&Element::parse(FRAGMENT).unwrap(), Scale::IDENTITY, &identity);
    assert_eq!(mark.center, PointF::new(243.0, 54.0));
    let refs = |_: Uuid, _: &str| mark.center;
    assert_eq!(body(element(&mark, Filter::KVA, &refs).to_document()), FRAGMENT);
    let mut moved = mark.clone();
    moved.set_trackable_point("0", PointF::new(300.0, 54.0));
    assert_eq!(moved.label.attach, PointF::new(300.0, 54.0));
    assert_ne!(moved.content_hash(), mark.content_hash());
    assert_eq!(mark.trackable_points(), vec![("0".to_string(), mark.center)]);
}
