use crate::drawing::line_test::body;
use crate::drawing::{element, Angle, KvaDrawing};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<Angle id=\"00000000-0000-4000-8000-000000000002\" name=\"Angle 1\">\n  <PointO>10;10</PointO>\n  <PointA>20;10</PointA>\n  <PointB>10;0</PointB>\n  <ReferenceTimestamp>0</ReferenceTimestamp>\n  <Signed>true</Signed>\n  <CCW>true</CCW>\n  <Supplementary>false</Supplementary>\n  <ShowCircle>false</ShowCircle>\n  <DrawingStyle>\n    <Color Key=\"line color\">\n      <Value>255;255;0;0</Value>\n    </Color>\n    <FontSize Key=\"font size\">\n      <Value>12</Value>\n    </FontSize>\n    <Int Key=\"DecimalPlaces\">\n      <Value>0</Value>\n    </Int>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n</Angle>";

#[test]
fn fragment_round_trips_and_angle_is_measured() {
    let angle = Angle::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    let refs = |_: Uuid, key: &str| match key {
        "o" => angle.o,
        "a" => angle.a,
        _ => angle.b,
    };
    assert_eq!(
        body(element(&angle, Filter::KVA, &refs).to_document()),
        FRAGMENT
    );
    assert!(
        (angle.value() + std::f32::consts::FRAC_PI_2).abs() < 1e-5,
        "{}",
        angle.value()
    );
    let mut unsigned = angle.clone();
    unsigned.signed = false;
    assert!((unsigned.value() - 3.0 * std::f32::consts::FRAC_PI_2).abs() < 1e-4);
    let mut cw = angle.clone();
    cw.counter_clockwise = false;
    assert!((cw.value() - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
    let mut supp = angle.clone();
    supp.supplementary = true;
    assert!((supp.value() + std::f32::consts::FRAC_PI_2).abs() < 1e-5);
}

#[test]
fn trackable_points_and_hash() {
    let mut a = Angle::new(
        PointF::new(0.0, 0.0),
        PointF::new(1.0, 0.0),
        PointF::new(0.0, 1.0),
    );
    assert_eq!(
        a.trackable_points()
            .iter()
            .map(|(k, _)| k.as_str())
            .collect::<Vec<_>>(),
        ["o", "a", "b"]
    );
    let h = a.content_hash();
    a.set_trackable_point("b", PointF::new(5.0, 5.0));
    assert_ne!(a.content_hash(), h);
    assert_eq!(a.b, PointF::new(5.0, 5.0));
}
