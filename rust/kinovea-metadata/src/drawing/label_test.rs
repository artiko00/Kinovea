use crate::drawing::line_test::body;
use crate::drawing::{element, KvaDrawing, Label};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<Label id=\"00000000-0000-4000-8000-000000000001\" name=\"Label 1\">\n  <Text>Hello &amp; welcome</Text>\n  <Position>100;200</Position>\n  <BackgroundVisible>true</BackgroundVisible>\n  <ArrowVisible>true</ArrowVisible>\n  <CircleVisible>false</CircleVisible>\n  <ArrowEnd>150;260</ArrowEnd>\n  <DrawingStyle>\n    <Color Key=\"back color\">\n      <Value>255;0;0;0</Value>\n    </Color>\n    <FontSize Key=\"font size\">\n      <Value>12</Value>\n    </FontSize>\n    <LineSize Key=\"LineSize\">\n      <Value>2</Value>\n    </LineSize>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n</Label>";

#[test]
fn fragment_round_trips_and_position_scales() {
    let l = Label::read_kva(&Element::parse(FRAGMENT).unwrap(), Scale::IDENTITY, &identity);
    assert_eq!(l.text, "Hello & welcome");
    assert!(l.arrow_visible && !l.circle_visible);
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    assert_eq!(body(element(&l, Filter::KVA, &refs).to_document()), FRAGMENT);
    let half = Label::read_kva(&Element::parse(FRAGMENT).unwrap(), Scale { x: 0.5, y: 0.5 }, &identity);
    assert_eq!(half.position, PointF::new(50.0, 100.0));
    assert_eq!(half.arrow_end, PointF::new(150.0, 260.0), "the arrow end is not scaled by the original");
    assert_ne!(half.content_hash(), l.content_hash());
}
