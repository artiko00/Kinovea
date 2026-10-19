use crate::drawing::Drawing;
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
    assert_eq!(m.trackability.trackers.iter().count(), 1, "the orphan tracker is dropped");
    assert_eq!(m.trackability.reference_value(line, "b"), kinovea_geometry::PointF::new(3.0, 4.0));
    assert_eq!(m.trackability.trackers.iter().next().unwrap().points[0].1.reference_timestamp, 10);
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
    assert!(read_kva(&mut m, "<KinoveaVideoAnalysis><FormatVersion>1.2</FormatVersion></KinoveaVideoAnalysis>").is_ok(), "1.2 is read as is, like the original");
    assert!(read_kva(&mut m, "<KinoveaVideoAnalysis><FormatVersion>1.5</FormatVersion></KinoveaVideoAnalysis>").is_ok(), "1.3 to 1.5 are migrated on the fly");
    assert!(matches!(read_kva(&mut m, "<broken"), Err(KvaError::Xml(_))));
}

#[test]
fn format_version_is_searched_as_a_descendant() {
    // The legacy drawing only becomes a Line when the 1.5 version under the wrapper triggers the migration.
    let xml = "<KinoveaVideoAnalysis><Header><FormatVersion>1.5</FormatVersion></Header><Keyframes><Keyframe><Position>10</Position><Drawings><Drawing Type=\"DrawingLine2D\"><m_StartPoint>1;2</m_StartPoint><m_EndPoint>3;4</m_EndPoint></Drawing></Drawings></Keyframe></Keyframes></KinoveaVideoAnalysis>";
    let mut m = Metadata::default();
    read_kva(&mut m, xml).unwrap();
    assert_eq!(m.keyframes[0].drawings.len(), 1);
}

#[test]
fn legacy_track_keyframe_labels_follow_the_absolute_times() {
    // Relative times 0 and 50 from a track starting at 100 become 100 and 150; the keyframe at 130
    // is inside that span, so it gets a label on the point at 150. Before the fix the labels were
    // computed on the relative times, where 130 lies after the last point, and none was made.
    let xml = "<KinoveaVideoAnalysis><FormatVersion>1.5</FormatVersion><ImageSize>100;100</ImageSize><AverageTimeStampsPerFrame>1</AverageTimeStampsPerFrame><Keyframes><Keyframe><Position>130</Position></Keyframe></Keyframes><Tracks><Track><TimePosition>100</TimePosition><Mode>Complete</Mode><TrackPositionList Count=\"2\" UserUnitLength=\"cm\"><TrackPosition>1;2;0</TrackPosition><TrackPosition>3;4;50</TrackPosition></TrackPositionList><Label><Text>t</Text></Label><MainLabel><KeyframeLabel><SpacePosition>5;6</SpacePosition><TimePosition>0</TimePosition></KeyframeLabel></MainLabel><TrackLine><LineStyle><Size>2</Size><ColorRGB>1;2;3</ColorRGB></LineStyle></TrackLine></Track></Tracks></KinoveaVideoAnalysis>";
    let mut m = Metadata::new(Size::new(100, 100));
    read_kva(&mut m, xml).unwrap();
    let Drawing::Track(track) = &m.tracks[0] else { panic!("expected a track") };
    assert_eq!(track.positions.iter().map(|p| p.t).collect::<Vec<_>>(), vec![100, 150]);
    assert_eq!(track.keyframe_labels.len(), 1);
    assert_eq!(track.keyframe_labels[0].attach_index, 1);
}

// A merged KVA whose coordinate system brings its own id: the setup registration (a random id) is replaced.
const MERGED: &str = "<KinoveaVideoAnalysis><FormatVersion>2.1</FormatVersion><ImageSize>100;100</ImageSize><Keyframes><Keyframe id=\"00000000-0000-4000-8000-000000000001\"><Timestamp>10</Timestamp><Drawings><Line id=\"00000000-0000-4000-8000-000000000002\" name=\"L\"><Start>1;2</Start><End>3;4</End></Line></Drawings></Keyframe></Keyframes><CoordinateSystem id=\"00000000-0000-4000-8000-000000000003\" name=\"Coordinate system 1\"><Position>5;6</Position></CoordinateSystem><Trackability><TrackableDrawing id=\"00000000-0000-4000-8000-000000000003\" name=\"Coordinate system 1\"><TrackablePoint key=\"0\"><ReferenceTimestamp>0</ReferenceTimestamp><ReferenceValue>5;6</ReferenceValue></TrackablePoint></TrackableDrawing><TrackableDrawing id=\"00000000-0000-4000-8000-000000000002\" name=\"L\"><TrackablePoint key=\"a\"><ReferenceTimestamp>10</ReferenceTimestamp><ReferenceValue>1;2</ReferenceValue></TrackablePoint></TrackableDrawing></Trackability></KinoveaVideoAnalysis>";

fn trackability_names(m: &Metadata) -> Vec<String> {
    let doc = crate::kva::write_kva(m, "0.0", false, crate::kva::ExportFlags::FULL).to_document();
    let block = doc
        .split("<Trackability>")
        .nth(1)
        .unwrap()
        .split("</Trackability>")
        .next()
        .unwrap();
    block
        .lines()
        .filter_map(|l| l.split("name=\"").nth(1))
        .map(|n| n.split('"').next().unwrap().to_owned())
        .collect()
}

#[test]
fn merged_coordinate_system_leaves_its_setup_slot_free_for_later_drawings() {
    // `UpdateId` drops the setup registration, so the first slot is free: the next tracker added in the
    // session is enumerated first, before the drawings read from the file.
    let mut m = crate::kva::header_metadata(MERGED);
    m.post_setup_video();
    read_kva(&mut m, MERGED).unwrap();
    assert_eq!(trackability_names(&m), ["L", "Coordinate system 1"]);
    m.trackability.add(
        uuid::Uuid::new_v4(),
        "Late",
        0,
        &[("0".into(), kinovea_geometry::PointF::EMPTY)],
    );
    assert_eq!(trackability_names(&m), ["Late", "L", "Coordinate system 1"]);
}

const CS_TRACKABLE: &str = "<TrackableDrawing id=\"00000000-0000-4000-8000-000000000003\" name=\"Coordinate system 1\"><TrackablePoint key=\"0\"><ReferenceTimestamp>0</ReferenceTimestamp><ReferenceValue>5;6</ReferenceValue></TrackablePoint></TrackableDrawing>";

#[test]
fn coordinate_system_filed_without_its_trackable_is_written_after_a_second_setup() {
    // Without a TrackableDrawing for the coordinate system, `UpdateId` files a copy under the file id. The copy
    // keeps the setup `drawingId` (DrawingTracker.cs:130), so the writer skips it (TrackabilityManager.cs:403-420).
    // A second `PostSetupVideo` registers the coordinate system again: `Add` reassigns the known entry
    // (TrackabilityManager.cs:75-78, DrawingTracker.cs:261), which makes it writable under the current id.
    let xml = MERGED.replace(CS_TRACKABLE, "");
    assert!(
        xml.len() < MERGED.len(),
        "the coordinate system trackable is removed"
    );
    let mut m = crate::kva::header_metadata(&xml);
    m.post_setup_video();
    read_kva(&mut m, &xml).unwrap();
    assert_eq!(trackability_names(&m), ["L"]);
    m.post_setup_video();
    assert_eq!(trackability_names(&m), ["L", "Coordinate system 1"]);
}
