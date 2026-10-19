//! Port of `Kinovea.Video/VideoGeometry/VideoGeometry.cs` and `VideoGeometryRequest.cs`. The
//! constructors store their arguments as-is; no computation is done here.

use kinovea_core::image_options::{Demosaicing, ImageRotation};
use kinovea_core::preferences::enums::ImageAspectRatio;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::Size;

/// `VideoGeometry`: the geometry the reader actually produced.
#[derive(Debug, Clone, PartialEq)]
pub struct VideoGeometry {
    /// Canonical image size, before any scaling. Used for drawing coordinates.
    pub reference_size: Size,
    /// Actual bitmap size produced by the reader.
    pub output_size: Size,
    /// True when the output size matches the requested presentation size.
    pub is_pre_scaled: bool,
    /// Scale applied to the reference size to produce the output size.
    pub scale: f64,
    pub image_aspect_ratio: ImageAspectRatio,
    pub image_rotation: ImageRotation,
    pub demosaicing: Demosaicing,
    pub deinterlacing: bool,
    pub stabilization_applied: bool,
}

impl VideoGeometry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        reference_size: Size,
        output_size: Size,
        is_pre_scaled: bool,
        scale: f64,
        image_aspect_ratio: ImageAspectRatio,
        image_rotation: ImageRotation,
        demosaicing: Demosaicing,
        deinterlacing: bool,
        stabilization_applied: bool,
    ) -> Self {
        Self {
            reference_size,
            output_size,
            is_pre_scaled,
            scale,
            image_aspect_ratio,
            image_rotation,
            demosaicing,
            deinterlacing,
            stabilization_applied,
        }
    }
}

impl Default for VideoGeometry {
    fn default() -> Self {
        Self::new(
            Size::default(),
            Size::default(),
            false,
            1.0,
            ImageAspectRatio::Auto,
            ImageRotation::Rotate0,
            Demosaicing::None,
            false,
            false,
        )
    }
}

/// `VideoGeometryRequest`: the image geometry the player asks the reader for.
#[derive(Debug, Clone, PartialEq)]
pub struct VideoGeometryRequest {
    /// Size of the rectangle where the image is rendered. May be empty initially.
    pub presentation_size: Size,
    /// Scale from the reference size to the presentation size.
    pub presentation_scale: f64,
    /// Scale from the reference size to fit the viewport.
    pub viewport_fit_scale: f64,
    /// Whether the reader may pre-scale the image.
    pub allow_pre_scaling: bool,
    pub aspect_ratio: ImageAspectRatio,
    pub rotation: ImageRotation,
    pub demosaicing: Demosaicing,
    pub deinterlace: bool,
    pub stabilization_data: Vec<TimedPoint>,
}

impl VideoGeometryRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        presentation_size: Size,
        presentation_scale: f64,
        viewport_fit_scale: f64,
        allow_pre_scaling: bool,
        aspect_ratio: ImageAspectRatio,
        rotation: ImageRotation,
        demosaicing: Demosaicing,
        deinterlace: bool,
        stabilization_data: Vec<TimedPoint>,
    ) -> Self {
        Self {
            presentation_size,
            presentation_scale,
            viewport_fit_scale,
            allow_pre_scaling,
            aspect_ratio,
            rotation,
            demosaicing,
            deinterlace,
            stabilization_data,
        }
    }
}
