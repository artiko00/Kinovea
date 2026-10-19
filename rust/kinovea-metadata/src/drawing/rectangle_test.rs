use crate::drawing::line_test::body;
use crate::drawing::{element, KvaDrawing, Rectangle};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<Rectangle id=\"00000000-0000-4000-8000-000000000001\" name=\"Rectangle 1\">\n  <PointUpperLeft>10;20</PointUpperLeft>\n  <PointUpperRight>110;20</PointUpperRight>\n  <PointLowerRight>110;70</PointLowerRight>\n  <PointLowerLeft>10;70</PointLowerLeft>\n  <Filled>true</Filled>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;255;127;0</Value>\n    </Color>\n    <LineSize Key=\"line size\">\n      <Value>2</Value>\n    </LineSize>\n    <PenShape Key=\"pen shape\">\n      <Value>Solid</Value>\n    </PenShape>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n</Rectangle>";

#[test]
fn fragment_round_trips_and_scales() {
    let r = Rectangle::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    assert!(r.filled);
    assert_eq!(r.quad.c(), PointF::new(110.0, 70.0));
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    assert_eq!(
        body(element(&r, Filter::KVA, &refs).to_document()),
        FRAGMENT
    );
    let half = Rectangle::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale { x: 0.5, y: 0.5 },
        &identity,
    );
    assert_eq!(half.quad.a(), PointF::new(5.0, 10.0));
    let mut unfilled = r.clone();
    unfilled.filled = false;
    assert_ne!(unfilled.content_hash(), r.content_hash());
}
