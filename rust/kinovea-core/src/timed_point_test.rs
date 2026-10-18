use crate::timed_point::TimedPoint;

#[test]
fn four_field_round_trip() {
    let p: TimedPoint = "1.5;-2.25;120;3".parse().unwrap();
    assert_eq!((p.x, p.y, p.t, p.r), (1.5, -2.25, 120, 3.0));
    assert_eq!(p.to_string(), "1.5;-2.25;120;3");
}

#[test]
fn three_field_legacy_form() {
    let p: TimedPoint = "1;2;3".parse().unwrap();
    assert_eq!(p, TimedPoint::new(1.0, 2.0, 3));
    assert_eq!(p.r, 0.0);
}

#[test]
fn malformed_text_is_an_error() {
    assert!("1;2".parse::<TimedPoint>().is_err());
    assert!("a;2;3".parse::<TimedPoint>().is_err());
}

#[test]
fn content_hash_is_stable_and_sensitive() {
    let a = TimedPoint::with_radius(1.5, -2.25, 120, 3.0);
    assert_eq!(
        a.content_hash(),
        TimedPoint::with_radius(1.5, -2.25, 120, 3.0).content_hash()
    );
    assert_ne!(
        a.content_hash(),
        TimedPoint::with_radius(1.5, -2.25, 121, 3.0).content_hash()
    );
    // Same rule as .NET: float bits xor-ed with the folded long.
    assert_eq!(TimedPoint::new(0.0, 0.0, 0).content_hash(), 0);
    assert_eq!(
        TimedPoint::new(1.0, 0.0, 0).content_hash(),
        1.0f32.to_bits() as i32
    );
}
