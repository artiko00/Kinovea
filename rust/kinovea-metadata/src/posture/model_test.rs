use crate::posture::model::*;

#[test]
fn defaults_match_the_original_constructors() {
    let stroke = Stroke::default();
    assert_eq!((stroke.style, stroke.width, stroke.arrow_begin, stroke.arrow_end), (SegmentLineStyle::Solid, 2, false, false));
    assert_eq!(stroke.color, TRANSPARENT);
    assert_eq!("Dash".parse::<SegmentLineStyle>(), Ok(SegmentLineStyle::Dash));
    assert_eq!("Ellipse".parse::<HandleType>(), Ok(HandleType::Ellipse));
    assert!(GenericPosture::default().point_list.is_empty());
}
