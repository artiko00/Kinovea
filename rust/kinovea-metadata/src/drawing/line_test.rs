use crate::drawing::{element, KvaDrawing, Line};
use crate::kva_io::{identity, Filter, Scale};
use crate::measure_label::MeasureLabelType;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<Line id=\"00000000-0000-4000-8000-000000000009\" name=\"Line 3\">\n  <Start>267;122</Start>\n  <End>277;38</End>\n  <ReferenceTimestamp>205705</ReferenceTimestamp>\n  <ExtraData>None</ExtraData>\n  <MeasureLabel>\n    <SpacePosition>-287;-130</SpacePosition>\n    <TimePosition>0</TimePosition>\n  </MeasureLabel>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;144;238;144</Value>\n    </Color>\n    <LineSize Key=\"line size\">\n      <Value>2</Value>\n    </LineSize>\n    <LineShape Key=\"line shape\">\n      <Value>Solid</Value>\n    </LineShape>\n    <Arrows Key=\"arrows\">\n      <Value>None</Value>\n    </Arrows>\n    <FontSize Key=\"Font\">\n      <Value>10</Value>\n    </FontSize>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n</Line>";

pub(crate) fn body(doc: String) -> String {
    doc[doc.find('\n').unwrap() + 1..].to_owned()
}

#[test]
fn fragment_round_trips_with_registered_points() {
    let e = Element::parse(FRAGMENT).unwrap();
    let line = Line::read_kva(&e, Scale::IDENTITY, &identity);
    assert_eq!(line.header.name, "Line 3");
    assert_eq!((line.a, line.b), (PointF::new(267.0, 122.0), PointF::new(277.0, 38.0)));
    assert_eq!(line.header.reference_timestamp, 205705);
    assert_eq!(line.measure_label_type, MeasureLabelType::None);
    assert_eq!(line.trackable_points().len(), 2);
    let refs = |_: Uuid, key: &str| if key == "a" { line.a } else { line.b };
    assert_eq!(body(element(&line, Filter::KVA, &refs).to_document()), FRAGMENT);
    assert!((line.length() - 84.593).abs() < 1e-2);
}

#[test]
fn unregistered_points_serialize_as_empty_like_the_original() {
    let e = Element::parse(FRAGMENT).unwrap();
    let line = Line::read_kva(&e, Scale { x: 0.5, y: 0.5 }, &identity);
    assert_eq!(line.a, PointF::new(133.5, 61.0));
    let empty = |_: Uuid, _: &str| PointF::EMPTY;
    let out = element(&line, Filter::CORE, &empty);
    assert_eq!(out.child_text("Start"), Some("0;0"));
    assert!(out.child("DrawingStyle").is_none());
    let mut moved = line.clone();
    moved.set_trackable_point("a", PointF::new(0.0, 0.0));
    assert_ne!(moved.content_hash(), line.content_hash());
}
