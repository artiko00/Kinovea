use crate::drawing::{Drawing, KvaDrawing, Line};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

#[test]
fn enum_dispatches_by_kva_name() {
    let e = Element::parse("<CrossMark id=\"11111111-2222-3333-4444-555555555555\" name=\"M\"><CenterPoint>1;2</CenterPoint></CrossMark>").unwrap();
    let d = Drawing::read_kva(&e, Scale::IDENTITY, &identity).unwrap();
    assert_eq!(d.kva_name(), "CrossMark");
    assert_eq!(d.id().to_string(), "11111111-2222-3333-4444-555555555555");
    assert_eq!(d.header().name, "M");
    assert_eq!(d.trackable_points(), vec![("0".to_string(), PointF::new(1.0, 2.0))]);
    assert!(Drawing::read_kva(&Element::parse("<Hologram/>").unwrap(), Scale::IDENTITY, &identity).is_none());
    assert!(!d.is_plane());
}

#[test]
fn enum_writes_the_full_element_and_moves_points() {
    let mut d = Drawing::Line(Line::new(PointF::new(0.0, 0.0), PointF::new(3.0, 4.0)));
    d.header_mut().name = "L".into();
    let refs = |_: Uuid, k: &str| if k == "a" { PointF::new(0.0, 0.0) } else { PointF::new(3.0, 4.0) };
    let e = d.to_element(Filter::CORE, &refs);
    assert_eq!(e.name, "Line");
    assert_eq!(e.attribute("name"), Some("L"));
    assert_eq!(e.child_text("End"), Some("3;4"));
    let before = d.content_hash();
    d.set_trackable_point("b", PointF::new(6.0, 8.0));
    assert_ne!(d.content_hash(), before);
    assert_eq!(Line::KVA_NAME, "Line");
}
