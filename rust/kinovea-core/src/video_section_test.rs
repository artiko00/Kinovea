use crate::video_section::VideoSection;

#[test]
fn containment() {
    let s = VideoSection::new(10, 20);
    assert!(s.contains(10));
    assert!(s.contains(20));
    assert!(!s.contains(21));
    assert!(s.contains_section(VideoSection::new(12, 18)));
    assert!(s.contains_section(VideoSection::new(10, 20)));
    assert!(!s.contains_section(VideoSection::new(5, 18)));
    assert!(!VideoSection::empty().contains(0));
}

#[test]
fn constructors_and_states() {
    assert_eq!(VideoSection::default(), VideoSection::new(0, 99));
    let empty = VideoSection::empty();
    assert!(empty.is_empty());
    assert_eq!(empty.length(), 0);
    let wrapped = VideoSection::new(5, 2);
    assert!(wrapped.wrapped());
    assert!(!wrapped.is_empty());
    assert_eq!(wrapped.length(), -3);
    assert_eq!(VideoSection::new(10, 20).length(), 10);
}

#[test]
fn equality_ignores_empty_sections() {
    assert_ne!(VideoSection::empty(), VideoSection::empty());
    assert_eq!(VideoSection::new(1, 2), VideoSection::new(1, 2));
    assert_ne!(VideoSection::new(1, 2), VideoSection::new(1, 3));
}

#[test]
fn ordering_follows_compare_to() {
    use std::cmp::Ordering;
    let outer = VideoSection::new(0, 100);
    let inner = VideoSection::new(10, 20);
    assert_eq!(inner.partial_cmp(&outer), Some(Ordering::Less));
    assert_eq!(outer.partial_cmp(&inner), Some(Ordering::Greater));
    assert_eq!(outer.partial_cmp(&outer), Some(Ordering::Equal));
    assert!(inner < outer);
}

#[test]
fn text_form() {
    assert_eq!(VideoSection::new(5, 2).to_string(), "? --> 2] [5 --> ?");
    assert_eq!(VideoSection::new(0, 99).to_string(), "[0 --> 99]");
}
