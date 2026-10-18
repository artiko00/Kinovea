use crate::drawing::chrono_multi::{parse_video_section, ChronoColumn};
use crate::drawing::line_test::body;
use crate::drawing::{element, ChronoMulti, KvaDrawing};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_core::video_section::VideoSection;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<ChronoMulti id=\"00000000-0000-4000-8000-000000000001\" name=\"Multi\">\n  <Position>10;20</Position>\n  <Values>\n    <Visible>100</Visible>\n    <Invisible>-1</Invisible>\n    <Sections>\n      <Section name=\"first\">200;300</Section>\n      <Section tag=\"x\">400;-1</Section>\n    </Sections>\n    <Locked>false</Locked>\n    <VisibleColumns>Name;Duration</VisibleColumns>\n  </Values>\n  <Label>\n    <Show>true</Show>\n  </Label>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;0;0;0</Value>\n    </Color>\n    <FontSize Key=\"font size\">\n      <Value>18</Value>\n    </FontSize>\n  </DrawingStyle>\n</ChronoMulti>";

#[test]
fn fragment_round_trips_with_sections_and_columns() {
    let c = ChronoMulti::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    assert_eq!(c.sections.len(), 2);
    assert_eq!(c.sections[1].section.end, i64::MAX);
    assert_eq!(
        c.visible_columns,
        vec![ChronoColumn::Name, ChronoColumn::Duration]
    );
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    assert_eq!(
        body(element(&c, Filter::KVA, &refs).to_document()),
        FRAGMENT
    );
}

#[test]
fn sections_stay_sorted_by_start_and_parsing_has_fallbacks() {
    let mut c = ChronoMulti::new(PointF::EMPTY);
    c.insert_section(VideoSection::new(300, 400), "", "");
    c.insert_section(VideoSection::new(100, 200), "a", "");
    c.insert_section(VideoSection::new(300, 350), "", "b");
    assert_eq!(
        c.sections
            .iter()
            .map(|s| s.section.start)
            .collect::<Vec<_>>(),
        [100, 300, 300]
    );
    assert_eq!(c.sections[1].tag, "b");
    assert_eq!(parse_video_section("-1;5"), VideoSection::new(i64::MAX, 5));
    assert_eq!(
        parse_video_section("junk"),
        VideoSection::new(i64::MAX, i64::MAX)
    );
    let no_sections = element(&c, Filter::CORE, &|_: Uuid, _: &str| PointF::EMPTY);
    assert!(no_sections
        .child("Values")
        .unwrap()
        .child("Sections")
        .is_some());
    let empty = ChronoMulti::new(PointF::EMPTY);
    assert!(
        element(&empty, Filter::CORE, &|_: Uuid, _: &str| PointF::EMPTY)
            .child("Values")
            .unwrap()
            .child("Sections")
            .is_none()
    );
    assert_ne!(c.content_hash(), empty.content_hash());
}
