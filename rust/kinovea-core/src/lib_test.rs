#[test]
fn crate_name_matches_package() {
    assert_eq!(crate::crate_name(), "kinovea-core");
}

crate::text_enum!(Sample [One, Two]);

#[test]
fn text_enum_round_trips_member_names() {
    assert_eq!(Sample::ALL, &[Sample::One, Sample::Two]);
    assert_eq!("Two".parse::<Sample>(), Ok(Sample::Two));
    assert_eq!(Sample::One.name(), "One");
    assert!("Three".parse::<Sample>().is_err());
}
