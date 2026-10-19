use super::producer::{Frame, ImageDescriptor, ImageFormat};

fn descriptor(width: usize) -> ImageDescriptor {
    ImageDescriptor {
        format: ImageFormat::Rgb32,
        width,
        height: 2,
        top_down: true,
        buffer_size: 8,
    }
}

#[test]
fn fill_copies_the_payload_when_it_fits() {
    let mut frame = Frame::try_new(4).expect("allocation");
    frame.fill(&[1, 2, 3], 3);
    assert_eq!(frame.payload(), &[1, 2, 3]);
}

#[test]
fn fill_keeps_the_previous_payload_when_the_new_one_is_oversized() {
    let mut frame = Frame::try_new(4).expect("allocation");
    frame.fill(&[1, 2], 2);
    frame.fill(&[9, 9, 9, 9, 9], 5);
    assert_eq!(frame.payload(), &[1, 2]);
}

#[test]
fn try_new_reports_an_impossible_allocation() {
    assert!(Frame::try_new(usize::MAX).is_err());
}

#[test]
fn compatible_accepts_identical_descriptors() {
    let a = descriptor(4);
    assert!(ImageDescriptor::compatible(Some(&a), Some(&descriptor(4))));
}

#[test]
fn compatible_rejects_a_different_width() {
    let a = descriptor(4);
    assert!(!ImageDescriptor::compatible(Some(&a), Some(&descriptor(5))));
}

#[test]
fn compatible_rejects_invalid_and_missing_descriptors() {
    let valid = descriptor(4);
    let invalid = ImageDescriptor::invalid();
    assert!(!ImageDescriptor::compatible(Some(&invalid), Some(&invalid)));
    assert!(!ImageDescriptor::compatible(Some(&valid), Some(&invalid)));
    assert!(!ImageDescriptor::compatible(Some(&valid), None));
}
