use crate::drawing::{GenericPostureDrawing, KvaDrawing};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const GONIOMETER: &str = "02ea93f9-ba00-424b-8c87-07e156e0252f";

fn fragment() -> String {
    format!("<GenericPosture name=\"G\"><ToolId>{GONIOMETER}</ToolId><Positions><Point>10;10</Point><Point>20;10</Point><Point>30;10</Point><Point>28;7</Point></Positions><DrawingStyle><Color Key=\"line color\"><Value>255;1;2;3</Value></Color></DrawingStyle></GenericPosture>")
}

#[test]
fn reads_a_shipped_tool_and_writes_it_back() {
    let d = GenericPostureDrawing::read_kva(
        &Element::parse(&fragment()).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    let posture = d.posture.as_ref().unwrap();
    assert_eq!(posture.point_list[3], PointF::new(28.0, 7.0));
    assert_eq!(d.tool_name(), "Goniometer");
    assert_eq!(
        d.trackable_points().len(),
        3,
        "handles 0, 1 and 3 are trackable"
    );
    let out = d.write_kva(Filter::KVA, &|_, _| PointF::EMPTY);
    assert_eq!(out[0].text, GONIOMETER);
    assert_eq!(out[1].children.len(), 4);
    assert_eq!(out[1].children[3].text, "28;7");
}

#[test]
fn unknown_tool_serializes_to_nothing_and_ignores_positions() {
    let xml = "<GenericPosture><ToolId>11111111-2222-3333-4444-555555555555</ToolId><Positions><Point>1;1</Point></Positions></GenericPosture>";
    let d =
        GenericPostureDrawing::read_kva(&Element::parse(xml).unwrap(), Scale::IDENTITY, &identity);
    assert!(d.posture.is_none());
    assert!(d.write_kva(Filter::KVA, &|_, _| PointF::EMPTY).is_empty());
    assert!(d.trackable_points().is_empty());
    assert_eq!(d.tool_name(), "");
}

#[test]
fn moving_a_trackable_point_goes_through_the_engine() {
    let mut d = GenericPostureDrawing::read_kva(
        &Element::parse(&fragment()).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    let before = d.content_hash();
    d.set_trackable_point("3", PointF::new(40.0, 40.0));
    assert_eq!(
        d.posture.as_ref().unwrap().point_list[3],
        PointF::new(40.0, 40.0)
    );
    assert_ne!(d.content_hash(), before);
    d.set_trackable_point("x", PointF::EMPTY);
    let empty = GenericPostureDrawing::new(Uuid::nil(), None);
    assert_eq!(
        empty.content_hash(),
        empty.header.style.content_hash() ^ empty.header.fading.content_hash()
    );
}
