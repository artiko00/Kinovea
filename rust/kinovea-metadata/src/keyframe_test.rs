use crate::drawing::{Drawing, Line, Plane};
use crate::keyframe::Keyframe;
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::{PointF, Quadrilateral};
use uuid::Uuid;

const KF: &str = "<Keyframe id=\"00000000-0000-4000-8000-000000000025\">\n  <Timestamp>480480</Timestamp>\n  <Name>0:00:05:34</Name>\n  <Color>51;152;255</Color>\n</Keyframe>";

#[test]
fn empty_keyframe_round_trips_without_drawings_element() {
    let kf = Keyframe::read_kva(&Element::parse(KF).unwrap(), Scale::IDENTITY, &identity);
    assert_eq!(kf.timestamp, 480480);
    assert_eq!(kf.name, "0:00:05:34");
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    let doc = kf.to_element(Filter::KVA, &refs).to_document();
    assert_eq!(&doc[doc.find('\n').unwrap() + 1..], KF);
    let mapped = Keyframe::read_kva(&Element::parse(KF).unwrap(), Scale::IDENTITY, &|t| t + 1);
    assert_eq!(mapped.timestamp, 480481);
}

#[test]
fn drawings_are_inserted_first_planes_last_and_written_in_file_order() {
    let xml = "<Keyframe id=\"00000000-0000-4000-8000-000000000001\"><Timestamp>5</Timestamp><Comment>hi</Comment><Drawings><Line id=\"00000000-0000-4000-8000-000000000002\" name=\"A\"/><Plane id=\"00000000-0000-4000-8000-000000000003\" name=\"P\"/><Line id=\"00000000-0000-4000-8000-000000000004\" name=\"B\"/><Line id=\"00000000-0000-4000-8000-000000000004\" name=\"dup\"/><Unknown/></Drawings></Keyframe>";
    let kf = Keyframe::read_kva(&Element::parse(xml).unwrap(), Scale::IDENTITY, &identity);
    let names: Vec<&str> = kf.drawings.iter().map(|d| d.header().name.as_str()).collect();
    assert_eq!(names, ["B", "A", "P"]);
    assert_eq!(kf.drawings[0].header().reference_timestamp, 5);
    assert_eq!(kf.comments, "hi");
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    let out = kf.to_element(Filter::KVA, &refs);
    let written: Vec<&str> = out.child("Drawings").unwrap().children.iter().map(|d| d.attribute("name").unwrap()).collect();
    assert_eq!(written, ["P", "A", "B"]);
    assert_eq!(out.child_text("Comment"), Some("hi"));
    assert!(out.child("Name").is_none());
    assert!(kf.to_element(Filter::CORE, &refs).child("Drawings").is_none());
}

#[test]
fn add_remove_and_hash() {
    let mut kf = Keyframe::new(10, "k");
    let line = Drawing::Line(Line::new(PointF::EMPTY, PointF::new(1.0, 1.0)));
    let id = line.id();
    kf.add_drawing(line);
    kf.add_drawing(Drawing::Plane(Plane::new(Quadrilateral::unit_square())));
    kf.add_drawing(Drawing::Line(Line::new(PointF::EMPTY, PointF::new(2.0, 2.0))));
    assert_eq!(kf.drawings[2].kva_name(), "Plane");
    assert_eq!(kf.drawings[1].id(), id);
    let h = kf.content_hash();
    assert!(kf.drawing(id).is_some());
    assert!(kf.remove_drawing(id).is_some());
    assert!(kf.remove_drawing(id).is_none());
    assert_ne!(kf.content_hash(), h);
}
