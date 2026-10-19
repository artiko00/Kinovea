use crate::posture::{custom_postures, parse_line_position};
use kinovea_geometry::PointLinePosition;

#[test]
fn module_re_exports_catalog_and_helpers() {
    assert_eq!(custom_postures().len(), 16);
    assert_eq!(parse_line_position("OnSegment"), PointLinePosition::OnSegment);
    assert_eq!(parse_line_position("?"), PointLinePosition::Anywhere);
}
