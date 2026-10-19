use crate::geometry::{VideoGeometry, VideoGeometryRequest};
use kinovea_core::image_options::{Demosaicing, ImageRotation};
use kinovea_core::preferences::enums::ImageAspectRatio;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::Size;

#[test]
fn default_geometry_is_unscaled_and_empty() {
    let geometry = VideoGeometry::default();
    assert_eq!(geometry.reference_size, Size::default());
    assert_eq!(geometry.output_size, Size::default());
    assert!(!geometry.is_pre_scaled);
    assert_eq!(geometry.scale, 1.0);
    assert_eq!(geometry.image_aspect_ratio, ImageAspectRatio::Auto);
    assert_eq!(geometry.image_rotation, ImageRotation::Rotate0);
    assert_eq!(geometry.demosaicing, Demosaicing::None);
    assert!(!geometry.deinterlacing);
    assert!(!geometry.stabilization_applied);
}

#[test]
fn geometry_constructor_stores_reference_size_as_given() {
    let geometry = VideoGeometry::new(
        Size::new(1920, 1080),
        Size::new(960, 540),
        true,
        0.5,
        ImageAspectRatio::Force169,
        ImageRotation::Rotate90,
        Demosaicing::RGGB,
        true,
        true,
    );
    assert_eq!(geometry.reference_size, Size::new(1920, 1080));
    assert_eq!(geometry.output_size, Size::new(960, 540));
    assert!(geometry.is_pre_scaled);
    assert_eq!(geometry.scale, 0.5);
    assert_eq!(geometry.image_rotation, ImageRotation::Rotate90);
    assert_eq!(geometry.demosaicing, Demosaicing::RGGB);
    assert!(geometry.deinterlacing && geometry.stabilization_applied);
}

#[test]
fn request_constructor_keeps_every_field() {
    let points = vec![TimedPoint::new(1.0, 2.0, 0), TimedPoint::new(3.0, 4.0, 40)];
    let request = VideoGeometryRequest::new(
        Size::new(640, 480),
        0.25,
        0.75,
        false,
        ImageAspectRatio::Force43,
        ImageRotation::Rotate270,
        Demosaicing::GBRG,
        true,
        points.clone(),
    );
    assert_eq!(request.presentation_size, Size::new(640, 480));
    assert_eq!(request.presentation_scale, 0.25);
    assert_eq!(request.viewport_fit_scale, 0.75);
    assert!(!request.allow_pre_scaling);
    assert_eq!(request.aspect_ratio, ImageAspectRatio::Force43);
    assert_eq!(request.rotation, ImageRotation::Rotate270);
    assert_eq!(request.demosaicing, Demosaicing::GBRG);
    assert!(request.deinterlace);
    assert_eq!(request.stabilization_data, points);
}
