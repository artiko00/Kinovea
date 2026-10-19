use crate::drawing::line_test::body;
use crate::drawing::{element, Chrono, KvaDrawing};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<Chrono id=\"00000000-0000-4000-8000-000000000028\" name=\"countdown\">\n  <Position>181;349</Position>\n  <Values>\n    <Visible>419058</Visible>\n    <Invisible>458110</Invisible>\n    <StartCounting>419058</StartCounting>\n    <StopCounting>438584</StopCounting>\n    <ClockOrigin>-1</ClockOrigin>\n  </Values>\n  <Label>\n    <Show>true</Show>\n  </Label>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;255;0;0</Value>\n    </Color>\n    <FontSize Key=\"font size\">\n      <Value>30</Value>\n    </FontSize>\n    <Toggle Key=\"clock\">\n      <Value>false</Value>\n      <Variant>Clock</Variant>\n    </Toggle>\n  </DrawingStyle>\n</Chrono>";

#[test]
fn fragment_round_trips_with_open_end_times() {
    let c = Chrono::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    assert_eq!(c.clock_origin, i64::MAX);
    assert_eq!(c.stop_counting, 438584);
    assert!(c.show_label);
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    assert_eq!(
        body(element(&c, Filter::KVA, &refs).to_document()),
        FRAGMENT
    );
    let core = element(&c, Filter::CORE, &refs);
    assert!(core.child("Label").is_none() && core.child("DrawingStyle").is_none());
}

#[test]
fn sanity_check_resets_a_stop_before_start_and_label_text_renames() {
    let xml = "<Chrono name=\"old\"><Values><Visible>-1</Visible><StartCounting>500</StartCounting><StopCounting>100</StopCounting></Values><Label><Text>new</Text><Show>false</Show></Label></Chrono>";
    let c = Chrono::read_kva(&Element::parse(xml).unwrap(), Scale::IDENTITY, &identity);
    assert_eq!(c.visible, 0);
    assert_eq!(c.stop_counting, i64::MAX);
    assert_eq!(c.header.name, "new");
    assert!(!c.show_label);
    let doubled = Chrono::read_kva(&Element::parse(FRAGMENT).unwrap(), Scale::IDENTITY, &|t| {
        t * 2
    });
    assert_eq!(doubled.visible, 838116);
    assert_eq!(doubled.clock_origin, i64::MAX);
    assert_ne!(doubled.content_hash(), c.content_hash());
}
