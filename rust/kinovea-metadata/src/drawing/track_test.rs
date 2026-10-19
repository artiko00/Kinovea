use crate::drawing::line_test::body;
use crate::drawing::{element, KvaDrawing, Track};
use crate::kva_io::{identity, Filter, Scale};
use kinovea_core::preferences::dom::Element;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::PointF;
use uuid::Uuid;

const FRAGMENT: &str = "<Track id=\"00000000-0000-4000-8000-000000000029\" name=\"Trajectory 1\">\n  <TimePosition>478978</TimePosition>\n  <IsVisible>true</IsVisible>\n  <Visible>0</Visible>\n  <Invisible>-1</Invisible>\n  <ExtraData>None</ExtraData>\n  <TrackerParameters>\n    <Algorithm>Correlation</Algorithm>\n    <SearchWindow>100;100</SearchWindow>\n    <BlockWindow>20;20</BlockWindow>\n    <SimilarityThreshold>0.5</SimilarityThreshold>\n    <TemplateUpdateThreshold>0.8</TemplateUpdateThreshold>\n    <UseMask>false</UseMask>\n    <HSVRange>0;360;0;255;0;255</HSVRange>\n    <Dilate>0</Dilate>\n    <Erode>0</Erode>\n  </TrackerParameters>\n  <TrackPointList Count=\"2\">\n    <TrackPoint>243;353;0;0</TrackPoint>\n    <TrackPoint>585;361;1502;0</TrackPoint>\n  </TrackPointList>\n  <MainLabel Text=\"Trajectory 1\">\n    <SpacePosition>618;355</SpacePosition>\n    <TimePosition>0</TimePosition>\n  </MainLabel>\n  <KeyframeLabelList Count=\"1\">\n    <KeyframeLabel>\n      <SpacePosition>600;370</SpacePosition>\n      <TimePosition>1500</TimePosition>\n    </KeyframeLabel>\n  </KeyframeLabelList>\n  <SeeFuture>true</SeeFuture>\n  <ShowTrackLabel>false</ShowTrackLabel>\n  <ShowKeyframeLabels>true</ShowKeyframeLabels>\n  <UseKeyframeColors>true</UseKeyframeColors>\n  <IsInteractiveTrack>false</IsInteractiveTrack>\n  <ShowRotationCircle>false</ShowRotationCircle>\n  <DrawingStyle>\n    <Color Key=\"color\">\n      <Value>255;255;0;0</Value>\n    </Color>\n    <TrackShape Key=\"track shape\">\n      <Value>Dash;true</Value>\n    </TrackShape>\n    <LineSize Key=\"line size\">\n      <Value>6</Value>\n    </LineSize>\n    <PenSize Key=\"TrackPointSize\">\n      <Value>3</Value>\n    </PenSize>\n    <FontSize Key=\"label size\">\n      <Value>8</Value>\n    </FontSize>\n  </DrawingStyle>\n</Track>";

#[test]
fn fragment_round_trips_and_labels_attach_to_the_closest_point() {
    let t = Track::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    assert_eq!(t.positions.len(), 2);
    assert_eq!(t.keyframe_labels[0].attach_index, 1);
    assert_eq!(t.main_label.attach, PointF::new(243.0, 353.0));
    let refs = |_: Uuid, _: &str| PointF::EMPTY;
    assert_eq!(
        body(element(&t, Filter::KVA, &refs).to_document()),
        FRAGMENT
    );
}

#[test]
fn scaling_and_timestamp_mapping_apply_to_positions() {
    let t = Track::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale { x: 0.5, y: 2.0 },
        &|ts| ts + 10,
    );
    assert_eq!(
        t.positions[1],
        TimedPoint {
            x: 292.5,
            y: 722.0,
            t: 1512,
            r: 0.0
        }
    );
    assert_eq!(t.begin_timestamp, 478988);
    assert_eq!(t.visible, 10);
    assert_eq!(t.invisible, i64::MAX);
    let fresh = Track::new(vec![TimedPoint::new(1.0, 2.0, 30)]);
    assert_eq!((fresh.begin_timestamp, fresh.visible), (30, 30));
    assert_eq!(fresh.closest_point(1000), 0);
    assert_ne!(fresh.content_hash(), t.content_hash());
}

#[test]
fn keyframe_labels_follow_the_keyframes_in_range() {
    let mut t = Track::read_kva(
        &Element::parse(FRAGMENT).unwrap(),
        Scale::IDENTITY,
        &identity,
    );
    t.begin_timestamp = 0;
    t.update_keyframe_labels(&[0, 1000, 5000]);
    assert_eq!(
        t.keyframe_labels.len(),
        1,
        "only keyframes after the start and up to the last point"
    );
    assert_eq!(t.keyframe_labels[0].label.timestamp, 1502);
    assert_eq!(
        t.keyframe_labels[0].label.position,
        PointF::new(565.0, 311.0)
    );
    t.update_keyframe_labels(&[1502]);
    assert_eq!(
        t.keyframe_labels.len(),
        1,
        "an existing label at the keyframe time is kept"
    );
    t.update_keyframe_labels(&[]);
    assert!(t.keyframe_labels.is_empty());
    t.is_visible = false;
    t.update_keyframe_labels(&[1000]);
    assert!(t.keyframe_labels.is_empty());
}
