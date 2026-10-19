use crate::kva::{read_kva, KvaError};
use crate::metadata::Metadata;
use kinovea_geometry::Size;

const MINIMAL: &str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<KinoveaVideoAnalysis>\n  <FormatVersion>2.0</FormatVersion>\n  <Producer>Kinovea.0.8.16</Producer>\n  <OriginalFilename>test.avi</OriginalFilename>\n  <ImageSize>640;480</ImageSize>\n  <AverageTimeStampsPerFrame>1</AverageTimeStampsPerFrame>\n  <FirstTimeStamp>0</FirstTimeStamp>\n  <SelectionStart>0</SelectionStart>\n</KinoveaVideoAnalysis>";

#[test]
fn minimal_document_loads_into_the_header() {
    let mut m = Metadata::new(Size::new(640, 480));
    let ctx = read_kva(&mut m, MINIMAL).unwrap();
    assert_eq!(ctx.file_name, "test.avi");
    assert_eq!(ctx.image_size, Size::new(640, 480));
    assert!(m.keyframes.is_empty());
    assert_eq!(m.video.time_origin, 0);
}

#[test]
fn keyframes_and_chronos_are_loaded_and_trackables_registered() {
    let xml = "<KinoveaVideoAnalysis><FormatVersion>2.1</FormatVersion><ImageSize>100;100</ImageSize><GlobalTitle>T</GlobalTitle><Mirror>true</Mirror><TimeOrigin>7</TimeOrigin><Keyframes><Keyframe id=\"00000000-0000-4000-8000-000000000001\"><Timestamp>10</Timestamp><Drawings><Line id=\"00000000-0000-4000-8000-000000000002\" name=\"L\"><Start>1;2</Start><End>3;4</End></Line></Drawings></Keyframe></Keyframes><Chronos><Chrono id=\"00000000-0000-4000-8000-000000000003\" name=\"c\"><Position>1;1</Position></Chrono><Bogus/></Chronos><Trackability><TrackableDrawing id=\"00000000-0000-4000-8000-000000000099\" name=\"gone\"/></Trackability><VideoFilters active=\"Kinogram\"><Kinogram/></VideoFilters></KinoveaVideoAnalysis>";
    let mut m = Metadata::new(Size::new(100, 100));
    read_kva(&mut m, xml).unwrap();
    assert_eq!(m.global_title, "T");
    assert!(m.mirrored);
    assert_eq!(m.video.time_origin, 7);
    assert_eq!(m.keyframes.len(), 1);
    assert_eq!(m.chronos.len(), 1);
    let line = m.keyframes[0].drawings[0].id();
    assert_eq!(m.trackability.trackers.len(), 1, "the orphan tracker is dropped");
    assert_eq!(m.trackability.reference_value(line, "b"), kinovea_geometry::PointF::new(3.0, 4.0));
    assert_eq!(m.trackability.trackers[0].points[0].1.reference_timestamp, 10);
    assert_eq!(m.video_filters.attribute("active"), Some("Kinogram"));
}

#[test]
fn scaling_applies_when_the_file_has_another_image_size() {
    let xml = "<KinoveaVideoAnalysis><FormatVersion>2.1</FormatVersion><ImageSize>200;200</ImageSize><Keyframes><Keyframe><Timestamp>10</Timestamp><Drawings><Circle name=\"C\"><Origin>100;50</Origin><Radius>20</Radius></Circle></Drawings></Keyframe></Keyframes></KinoveaVideoAnalysis>";
    let mut m = Metadata::new(Size::new(100, 100));
    read_kva(&mut m, xml).unwrap();
    match &m.keyframes[0].drawings[0] {
        crate::drawing::Drawing::Circle(c) => assert_eq!((c.center, c.radius), (kinovea_geometry::PointF::new(50.0, 25.0), 10)),
        other => panic!("{other:?}"),
    }
}

#[test]
fn version_and_root_checks() {
    let mut m = Metadata::default();
    assert_eq!(read_kva(&mut m, "<Other/>"), Err(KvaError::NotKva("Other".into())));
    assert_eq!(read_kva(&mut m, "<KinoveaVideoAnalysis><FormatVersion>1.2</FormatVersion></KinoveaVideoAnalysis>"), Err(KvaError::TooOld("1.2".into())));
    assert!(read_kva(&mut m, "<KinoveaVideoAnalysis><FormatVersion>1.5</FormatVersion></KinoveaVideoAnalysis>").is_ok(), "1.3 to 1.5 are migrated on the fly");
    assert!(matches!(read_kva(&mut m, "<broken"), Err(KvaError::Xml(_))));
    assert_eq!(KvaError::TooOld("1.2".into()).to_string(), "KVA format 1.2 is too old");
}
