use crate::drawing::line_test::body;
use crate::drawing::{element, DistortionGrid, KvaDrawing, Plane};
use crate::kva_io::{identity, Filter, Scale};
use crate::style::StyleValue;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<Plane id=\"00000000-0000-4000-8000-000000000001\" name=\"Plane 1\">\n  <PointUpperLeft>10;10</PointUpperLeft>\n  <PointUpperRight>200;12</PointUpperRight>\n  <PointLowerRight>210;150</PointLowerRight>\n  <PointLowerLeft>5;140</PointLowerLeft>\n  <ReferenceTimestamp>40</ReferenceTimestamp>\n  <ShowGrid>true</ShowGrid>\n  <ShowXLine>false</ShowXLine>\n  <ShowYLine>true</ShowYLine>\n  <XLineCoord>0.5</XLineCoord>\n  <YLineCoord>0.25</YLineCoord>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;150;149;237</Value>\n    </Color>\n    <Int Key=\"cols\">\n      <Value>8</Value>\n    </Int>\n    <Int Key=\"rows\">\n      <Value>6</Value>\n    </Int>\n    <Toggle Key=\"perspective\">\n      <Value>true</Value>\n      <Variant>Perspective</Variant>\n    </Toggle>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n</Plane>";

#[test]
fn plane_round_trips_through_registered_corners() {
    let plane = Plane::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    assert!(plane.show_y_line && !plane.show_x_line);
    assert_eq!(plane.y_line_coord, 0.25);
    let refs = |_: Uuid, key: &str| plane.quad[key.parse::<usize>().unwrap()];
    assert_eq!(
        body(element(&plane, Filter::KVA, &refs).to_document()),
        FRAGMENT
    );
    assert_eq!(plane.trackable_points().len(), 4);
    let mut moved = plane.clone();
    moved.set_trackable_point("3", PointF::new(0.0, 0.0));
    assert_eq!(moved.quad.d(), PointF::new(0.0, 0.0));
    assert_ne!(moved.content_hash(), plane.content_hash());
}

#[test]
fn legacy_divisions_become_cols_and_rows() {
    let legacy = Element::parse("<Plane><DrawingStyle><Int Key=\"divisions\"><Value>12</Value></Int></DrawingStyle></Plane>").unwrap();
    let plane = Plane::read_kva(&legacy, Scale::IDENTITY, &identity);
    assert!(matches!(
        plane.header.style.get("cols"),
        Some(StyleValue::Int { value: 12, .. })
    ));
    assert!(matches!(
        plane.header.style.get("rows"),
        Some(StyleValue::Int { value: 12, .. })
    ));
    assert!(!plane.header.style.contains("divisions"));
}

#[test]
fn distortion_grid_points_round_trip() {
    let frag = "<DistortionGrid id=\"00000000-0000-4000-8000-000000000002\" name=\"Grid\">\n  <Points>\n    <Point>1;2</Point>\n    <Point>3;4</Point>\n  </Points>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;255;0;0</Value>\n    </Color>\n  </DrawingStyle>\n  <InfosFading>\n    <Enabled>true</Enabled>\n    <UseDefault>true</UseDefault>\n    <AlwaysVisible>false</AlwaysVisible>\n    <MasterFactor>1</MasterFactor>\n    <OpaqueFrames>1</OpaqueFrames>\n    <Frames>20</Frames>\n  </InfosFading>\n</DistortionGrid>";
    let mut grid =
        DistortionGrid::read_kva(&Element::parse(frag).unwrap(), Scale::IDENTITY, &identity);
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    assert_eq!(body(element(&grid, Filter::KVA, &refs).to_document()), frag);
    assert_eq!(grid.trackable_points()[1].0, "1");
    grid.set_trackable_point("1", PointF::new(9.0, 9.0));
    grid.set_trackable_point("9", PointF::new(9.0, 9.0));
    assert_eq!(grid.points[1], PointF::new(9.0, 9.0));
}
