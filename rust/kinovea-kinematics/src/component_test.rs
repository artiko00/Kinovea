use crate::component::{Component, Kinematics, UnknownKinematics};

#[test]
fn names_round_trip_as_csv_headers() {
    assert_eq!(Kinematics::ALL.len(), 22);
    for k in Kinematics::ALL {
        assert_eq!(k.name().parse::<Kinematics>().unwrap(), *k);
        assert_eq!(k.to_string(), k.name());
    }
    assert_eq!(Kinematics::LinearHorizontalVelocity.name(), "LinearHorizontalVelocity");
    assert_eq!("Nope".parse::<Kinematics>(), Err(UnknownKinematics("Nope".into())));
    assert_eq!(UnknownKinematics("Nope".into()).to_string(), "unknown kinematics component: Nope");
}

#[test]
fn components_are_distinct() {
    assert_ne!(Component::Magnitude, Component::Horizontal);
    assert_ne!(Component::Horizontal, Component::Vertical);
}
