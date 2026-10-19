use crate::kva_io::{identity, Filter, Scale};
use crate::multi::{AutoNumber, NumberSequence, Spotlights};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;

fn body(doc: String) -> String {
    doc[doc.find('\n').unwrap() + 1..].to_owned()
}

#[test]
fn spotlights_round_trip_and_replace_known_ids() {
    let xml = "<Spotlights>\n  <Spotlight id=\"00000000-0000-4000-8000-000000000001\" name=\"Spotlight\">\n    <Time>100</Time>\n    <Center>50;60</Center>\n    <Radius>30</Radius>\n  </Spotlight>\n</Spotlights>";
    let mut s = Spotlights::default();
    s.read_kva(&Element::parse(xml).unwrap(), Scale::IDENTITY, &identity);
    assert_eq!(s.items.len(), 1);
    let mut e = Element::new(Spotlights::KVA_NAME);
    e.children = s.write_kva(Filter::KVA);
    assert_eq!(body(e.to_document()), xml);
    let again = xml.replace("<Radius>30</Radius>", "<Radius>40</Radius>");
    s.read_kva(&Element::parse(&again).unwrap(), Scale { x: 0.5, y: 0.5 }, &|t| t * 2);
    assert_eq!(s.items.len(), 1);
    assert_eq!((s.items[0].radius, s.items[0].timestamp, s.items[0].center), (20, 200, PointF::new(25.0, 30.0)));
    assert_ne!(s.content_hash(), Spotlights::default().content_hash());
}

#[test]
fn number_sequence_writes_items_then_style() {
    let xml = "<NumberSequence>\n  <AutoNumber id=\"00000000-0000-4000-8000-000000000001\" name=\"AutoNumber\">\n    <Time>5</Time>\n    <Location>10;20</Location>\n    <Value>3</Value>\n  </AutoNumber>\n  <DrawingStyle>\n    <Color Key=\"back color\">\n      <Value>255;0;0;0</Value>\n    </Color>\n    <FontSize Key=\"font size\">\n      <Value>24</Value>\n    </FontSize>\n  </DrawingStyle>\n</NumberSequence>";
    let mut n = NumberSequence::default();
    n.read_kva(&Element::parse(xml).unwrap(), Scale::IDENTITY, &identity);
    assert_eq!(n.items[0].value, 3);
    let mut e = Element::new(NumberSequence::KVA_NAME);
    e.children = n.write_kva(Filter::KVA);
    assert_eq!(body(e.to_document()), xml);
    assert_eq!(n.write_kva(Filter::CORE).len(), 1);
    n.items.push(AutoNumber::new(9, PointF::new(1.0, 1.0), 4));
    assert_ne!(n.content_hash(), NumberSequence::default().content_hash());
}
