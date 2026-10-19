use crate::image_options::{Demosaicing, ImageRotation};

#[test]
fn rotation_names_are_member_names() {
    assert_eq!(
        ImageRotation::ALL,
        &[
            ImageRotation::Rotate0,
            ImageRotation::Rotate90,
            ImageRotation::Rotate180,
            ImageRotation::Rotate270
        ]
    );
    assert_eq!(ImageRotation::Rotate90.name(), "Rotate90");
    assert_eq!(
        "Rotate270".parse::<ImageRotation>(),
        Ok(ImageRotation::Rotate270)
    );
    assert!("Rotate45".parse::<ImageRotation>().is_err());
}

#[test]
fn demosaicing_names_are_member_names() {
    assert_eq!(Demosaicing::None.name(), "None");
    assert_eq!("GBRG".parse::<Demosaicing>(), Ok(Demosaicing::GBRG));
    assert!("rggb".parse::<Demosaicing>().is_err());
}
