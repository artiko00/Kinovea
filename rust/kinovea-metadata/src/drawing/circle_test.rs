use crate::drawing::line_test::body;
use crate::drawing::{element, Circle, KvaDrawing};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<Circle id=\"00000000-0000-4000-8000-000000000013\" name=\"Circle 3\">\n  <Origin>347;184</Origin>\n  <Radius>169</Radius>\n  <ExtraData>None</ExtraData>\n  <MeasureLabel>\n    <SpacePosition>-20;-50</SpacePosition>\n    <TimePosition>0</TimePosition>\n  </MeasureLabel>\n  <ShowCenter>false</ShowCenter>\n  <Filled>false</Filled>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;95;158;160</Value>\n    </Color>\n    <PenSize Key=\"pen size\">\n      <Value>3</Value>\n    </PenSize>\n    <PenShape Key=\"pen shape\">\n      <Value>Solid</Value>\n    </PenShape>\n    <FontSize Key=\"Font\">\n      <Value>10</Value>\n    </FontSize>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n</Circle>";

#[test]
fn fragment_round_trips_and_radius_scales_by_the_smaller_factor() {
    let circle = Circle::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    assert_eq!(
        (circle.center, circle.radius),
        (PointF::new(347.0, 184.0), 169)
    );
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    assert_eq!(
        body(element(&circle, Filter::KVA, &refs).to_document()),
        FRAGMENT
    );
    let half = Circle::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale { x: 0.5, y: 0.25 },
        &identity,
    );
    assert_eq!(half.radius, 42);
    assert_eq!(half.center, PointF::new(173.5, 46.0));
    assert!(circle.trackable_points().is_empty());
    assert_ne!(circle.content_hash(), half.content_hash());
}
