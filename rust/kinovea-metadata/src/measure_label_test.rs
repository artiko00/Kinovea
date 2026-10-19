use crate::kva_io::Scale;
use crate::measure_label::{measure_label_block, MeasureLabel, MeasureLabelType};
use kinovea_core::preferences::dom::Element;
use kinovea_geometry::PointF;

#[test]
fn label_round_trip_and_attached_write() {
    let e = Element::parse("<MeasureLabel><SpacePosition>-287;-130</SpacePosition><TimePosition>5</TimePosition></MeasureLabel>").unwrap();
    let mut label = MeasureLabel::read_kva(&e, Scale::IDENTITY);
    assert_eq!(label.position, PointF::new(-287.0, -130.0));
    assert_eq!(label.timestamp, 5);
    let block = measure_label_block(label.write_kva());
    assert_eq!(block.child_text("SpacePosition"), Some("-287;-130"));
    label.set_attach(PointF::new(10.0, 10.0));
    let moved = measure_label_block(label.write_kva_attached(PointF::new(20.0, 0.0)));
    assert_eq!(moved.child_text("SpacePosition"), Some("-277;-140"));
    assert_eq!(MeasureLabel::read_kva(&e, Scale { x: 0.5, y: 0.5 }).position, PointF::new(-143.5, -65.0));
    assert_ne!(label.content_hash(), MeasureLabel::default().content_hash());
}

#[test]
fn measure_label_types_match_the_csharp_names() {
    assert_eq!(MeasureLabelType::ALL.len(), 37);
    assert_eq!("TravelDistance".parse::<MeasureLabelType>(), Ok(MeasureLabelType::TravelDistance));
    assert_eq!(MeasureLabelType::TotalVerticalDisplacement.name(), "TotalVerticalDisplacement");
}
