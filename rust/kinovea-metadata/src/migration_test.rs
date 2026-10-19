use crate::migration::{migrate_legacy, needs_migration};
use kinovea_core::preferences::dom::Element;

#[test]
fn version_window() {
    assert!(needs_migration(1.3) && needs_migration(1.5) && needs_migration(1.99));
    assert!(!needs_migration(1.2) && !needs_migration(2.0) && !needs_migration(2.1));
}

#[test]
fn header_calibration_and_fixed_sections() {
    let xml = "<KinoveaVideoAnalysis><FormatVersion>1.5</FormatVersion><OriginalFilename>a.avi</OriginalFilename><ImageSize>10;10</ImageSize><CalibrationHelp><PixelToUnit>0.5</PixelToUnit><LengthUnit UserUnitLength=\"cm\">0</LengthUnit><CoordinatesOrigin>-1;-1</CoordinatesOrigin></CalibrationHelp><Keyframes><Keyframe><Position>3</Position><Comment>dropped without title</Comment></Keyframe></Keyframes><Chronos><Chrono/></Chronos></KinoveaVideoAnalysis>";
    let out = migrate_legacy(&Element::parse(xml).unwrap());
    let names: Vec<&str> = out.children.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["FormatVersion", "Producer", "OriginalFilename", "ImageSize", "Calibration", "Keyframes", "Chronos", "Trackability"]);
    let cal = out.child("Calibration").unwrap();
    assert!(cal.child("CalibrationLine").unwrap().child("Origin").is_none(), "-1;-1 origin is dropped");
    assert_eq!(cal.child("CalibrationLine").unwrap().child_text("Scale"), Some("0.5"));
    assert_eq!(cal.child("Unit").unwrap().text, "Centimeters");
    assert_eq!(cal.child("Unit").unwrap().attribute("Abbreviation"), Some("cm"));
    let kf = &out.child("Keyframes").unwrap().children[0];
    assert!(kf.child("Comment").is_none() && kf.child("Drawings").is_none());
    assert_eq!(kf.child_text("Position"), Some("3"));
}

#[test]
fn unknown_units_and_missing_values_follow_the_stylesheet() {
    let xml = "<KinoveaVideoAnalysis><CalibrationHelp><LengthUnit>9</LengthUnit></CalibrationHelp></KinoveaVideoAnalysis>";
    let out = migrate_legacy(&Element::parse(xml).unwrap());
    let cal = out.child("Calibration").unwrap();
    assert_eq!(cal.child("Unit").unwrap().text, "Pixels");
    assert_eq!(cal.child("CalibrationLine").unwrap().child_text("Origin"), Some(""), "value-of a missing node is empty");
}
