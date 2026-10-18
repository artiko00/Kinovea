use crate::drawing::line_test::body;
use crate::drawing::{element, Counter, KvaDrawing};
use crate::kva_io::{identity, Filter, Scale};
use crate::measure_label::MeasureLabelType;
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<Counter id=\"00000000-0000-4000-8000-000000000001\" name=\"Counter 1\">\n  <Position>10;20</Position>\n  <Values>\n    <Visible>0</Visible>\n    <Invisible>-1</Invisible>\n    <Beats>\n      <Beat>100</Beat>\n      <Beat>250</Beat>\n    </Beats>\n    <Locked>true</Locked>\n    <ZeroBased>false</ZeroBased>\n    <HalfCadence>false</HalfCadence>\n    <DoubleCadence>true</DoubleCadence>\n    <ExtraData>CadenceAverage</ExtraData>\n  </Values>\n  <Label>\n    <Show>false</Show>\n  </Label>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;0;0;0</Value>\n    </Color>\n    <FontSize Key=\"font size\">\n      <Value>18</Value>\n    </FontSize>\n  </DrawingStyle>\n</Counter>";

#[test]
fn fragment_round_trips_and_label_show_is_not_read() {
    let c = Counter::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    assert_eq!(c.beats, vec![100, 250]);
    assert!(c.locked && c.double_cadence && !c.zero_based);
    assert_eq!(c.measure_label_type, MeasureLabelType::CadenceAverage);
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    assert_eq!(
        body(element(&c, Filter::KVA, &refs).to_document()),
        FRAGMENT
    );
    let with_label = Element::parse("<Counter><Label><Show>true</Show></Label></Counter>").unwrap();
    assert!(!Counter::read_kva(&with_label, Scale::IDENTITY, &identity).show_label);
    let mut empty = Counter::new(PointF::EMPTY);
    assert!(element(&empty, Filter::CORE, &refs)
        .child("Values")
        .unwrap()
        .child("Beats")
        .is_none());
    empty.beats.push(5);
    assert_ne!(
        empty.content_hash(),
        Counter::new(PointF::EMPTY).content_hash()
    );
}
