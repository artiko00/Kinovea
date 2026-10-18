use crate::drawing::{Drawing, Line};
use crate::metadata::Metadata;
use kinovea_geometry::PointF;

#[test]
fn automatic_names_skip_taken_ones() {
    let mut m = Metadata::default();
    let mut named = Drawing::Line(Line::new(PointF::EMPTY, PointF::new(1.0, 1.0)));
    named.header_mut().name = "Line 1".into();
    m.chronos.push(named);
    assert_eq!(m.next_name("Line", &[]), "Line 2");
    assert_eq!(m.next_name("Line", &["Line 2".into()]), "Line 3");
    let mut untitled = vec![Drawing::Line(Line::new(PointF::EMPTY, PointF::EMPTY)), Drawing::Line(Line::new(PointF::EMPTY, PointF::EMPTY))];
    m.name_untitled(&mut untitled);
    assert_eq!(untitled[0].header().name, "Line 2");
    assert_eq!(untitled[1].header().name, "Line 3");
}
