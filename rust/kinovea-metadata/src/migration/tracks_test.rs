use crate::migration::migrate_legacy;
use kinovea_core::preferences::dom::Element;

#[test]
fn legacy_tracks_become_track_point_lists_with_a_main_label() {
    let xml = "<KinoveaVideoAnalysis><Tracks><Track><TimePosition>10</TimePosition><Mode>Complete</Mode><TrackPositionList Count=\"2\" UserUnitLength=\"cm\"><TrackPosition UserX=\"0\" UserTime=\"0\">1;2;0</TrackPosition><TrackPosition>3;4;1</TrackPosition></TrackPositionList><Label><Text>old</Text></Label><MainLabel><KeyframeLabel><SpacePosition>5;6</SpacePosition><TimePosition>0</TimePosition></KeyframeLabel></MainLabel><TrackLine><LineStyle><Size>2</Size><ColorRGB>1;2;3</ColorRGB></LineStyle></TrackLine></Track></Tracks></KinoveaVideoAnalysis>";
    let out = migrate_legacy(&Element::parse(xml).unwrap());
    let track = &out.child("Tracks").unwrap().children[0];
    assert_eq!(track.attribute("name"), Some("old"));
    let names: Vec<&str> = track.children.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["TimePosition", "Mode", "TrackPointList", "MainLabel", "DrawingStyle"]);
    let list = track.child("TrackPointList").unwrap();
    assert_eq!((list.attribute("Count"), list.attribute("UserUnitLength")), (Some("2"), Some("cm")));
    assert_eq!(list.children[0].text, "1;2;0");
    assert_eq!(list.children[0].attribute("UserX"), Some("0"));
    assert_eq!(list.children[1].attribute("UserY"), Some(""), "missing attributes are written empty");
    let main = track.child("MainLabel").unwrap();
    assert_eq!((main.attribute("Text"), main.child_text("SpacePosition")), (Some("old"), Some("5;6")));
    let style = track.child("DrawingStyle").unwrap();
    assert_eq!(style.children[2].child_text("Value"), Some("Solid;false"));
}
