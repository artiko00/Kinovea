use crate::drawing::tracking_parameters::{HsvRange, TrackingAlgorithm, TrackingParameters};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::Size;

const BLOCK: &str = "<TrackerParameters>\n  <Algorithm>Correlation</Algorithm>\n  <SearchWindow>100;100</SearchWindow>\n  <BlockWindow>20;20</BlockWindow>\n  <SimilarityThreshold>0.5</SimilarityThreshold>\n  <TemplateUpdateThreshold>0.8</TemplateUpdateThreshold>\n  <UseMask>false</UseMask>\n  <HSVRange>0;360;0;255;0;255</HSVRange>\n  <Dilate>0</Dilate>\n  <Erode>0</Erode>\n</TrackerParameters>";

#[test]
fn defaults_match_the_fixture_block_and_round_trip() {
    let mut e = Element::new("TrackerParameters");
    e.children = TrackingParameters::default().write_kva();
    let doc = e.to_document();
    assert_eq!(&doc[doc.find('\n').unwrap() + 1..], BLOCK);
    let read = TrackingParameters::read_kva(&Element::parse(BLOCK).unwrap());
    assert_eq!(read, TrackingParameters::default());
}

#[test]
fn values_parse_with_fallbacks() {
    let e = Element::parse("<TrackerParameters><Algorithm>Circle</Algorithm><SearchWindow>50;40</SearchWindow><SimilarityThreshold>0.75</SimilarityThreshold><UseMask>true</UseMask><HSVRange>10;20;30;40;50;60</HSVRange><Dilate>2</Dilate><Erode>x</Erode></TrackerParameters>").unwrap();
    let p = TrackingParameters::read_kva(&e);
    assert_eq!(p.algorithm, TrackingAlgorithm::Circle);
    assert_eq!(p.search_window, Size::new(50, 40));
    assert_eq!(p.block_window, Size::new(20, 20));
    assert_eq!(p.similarity_threshold, 0.75);
    assert!(p.use_mask);
    assert_eq!(
        p.hsv_range,
        HsvRange {
            hue: (10.0, 20.0),
            saturation: (30.0, 40.0),
            value: (50.0, 60.0)
        }
    );
    assert_eq!((p.dilate, p.erode), (2, 0));
    assert_eq!(HsvRange::parse("1;2").hue, (0.0, 0.0));
    assert_ne!(
        p.content_hash(),
        TrackingParameters::default().content_hash()
    );
}
