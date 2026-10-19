use crate::kva_io::Filter;
use crate::singletons::{singleton_element, CoordinateSystem, TestGrid};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

fn body(doc: String) -> String {
    doc[doc.find('\n').unwrap() + 1..].to_owned()
}

const CS: &str = "<CoordinateSystem id=\"00000000-0000-4000-8000-000000000003\" name=\"\">\n  <Position>0;0</Position>\n  <ReferenceTimestamp>-1</ReferenceTimestamp>\n  <Visible>false</Visible>\n  <ShowGrid>true</ShowGrid>\n  <ShowGraduations>true</ShowGraduations>\n  <DrawingStyle>\n    <Color Key=\"line color\">\n      <Value>255;255;0;0</Value>\n    </Color>\n  </DrawingStyle>\n</CoordinateSystem>";

const GRID: &str = "<TestGrid id=\"00000000-0000-4000-8000-000000000004\" name=\"\">\n  <Visible>false</Visible>\n  <ShowHorizontalAxis>true</ShowHorizontalAxis>\n  <ShowVerticalAxis>true</ShowVerticalAxis>\n  <ShowFraming>true</ShowFraming>\n  <ShowThirds>false</ShowThirds>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;255;0;0</Value>\n    </Color>\n  </DrawingStyle>\n</TestGrid>";

#[test]
fn coordinate_system_round_trips_with_unregistered_position() {
    let mut cs = CoordinateSystem::default();
    let e = Element::parse(CS.replace("<Position>0;0</Position>", "<Position>12;34</Position>").as_str()).unwrap();
    cs.read_kva(&e);
    assert_eq!(cs.origin, PointF::new(12.0, 34.0));
    assert_eq!(cs.trackable_points(), vec![("0".to_string(), cs.origin)]);
    let unregistered = |_: Uuid, _: &str| PointF::EMPTY;
    assert_eq!(body(singleton_element(CoordinateSystem::KVA_NAME, &cs.header, cs.write_kva(Filter::KVA, &unregistered)).to_document()), CS);
    let registered = |_: Uuid, _: &str| cs.origin;
    let out = singleton_element(CoordinateSystem::KVA_NAME, &cs.header, cs.write_kva(Filter::KVA, &registered));
    assert_eq!(out.child_text("Position"), Some("12;34"));
    let mut visible = cs.clone();
    visible.visible = true;
    assert_ne!(visible.content_hash(), cs.content_hash());
}

#[test]
fn test_grid_round_trips() {
    let mut grid = TestGrid::default();
    grid.read_kva(&Element::parse(GRID).unwrap());
    assert_eq!(body(singleton_element(TestGrid::KVA_NAME, &grid.header, grid.write_kva(Filter::KVA)).to_document()), GRID);
    let mut thirds = grid.clone();
    thirds.show_thirds = true;
    assert_ne!(thirds.content_hash(), grid.content_hash());
    assert!(grid.write_kva(Filter::CORE).iter().all(|e| e.name != "DrawingStyle"));
}
