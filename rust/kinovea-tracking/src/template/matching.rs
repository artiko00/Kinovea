//! The correlation step of `TrackerTemplateMatching.MatchTemplate`: the search window around the
//! last point (rounded, then clamped to the image), the masked correlation with a retry without the
//! mask when the peak is infinite or above 1, and the sub-pixel refinement of the peak.

use kinovea_metadata::drawing::tracking_parameters::TrackingParameters;
use kinovea_video::frame::FrameBuffer;

use crate::ncc::{match_template, Plane};
use crate::pixels::image::{GrayImage, Roi};
use crate::pixels::mask::ellipse_mask;
use crate::refine::peak_offset;
use crate::template::capture::{aligned, crop_bgra, Patch};

/// Center of the best location, in image coordinates, and its similarity. The similarity is 0 when
/// the correlation failed, in which case the location is the last point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Found {
    pub similarity: f32,
    pub x: f32,
    pub y: f32,
}

pub fn match_block(
    parameters: &TrackingParameters,
    image: &FrameBuffer,
    template: &Patch,
    last: (f32, f32),
    mask: &mut Option<GrayImage>,
) -> Found {
    let search = parameters.search_window();
    let roi = Roi::new(
        aligned(last.0) as i32 - search.width / 2,
        aligned(last.1) as i32 - search.height / 2,
        search.width,
        search.height,
    )
    .clamp_to(image.width(), image.height());
    if roi.size.width < template.width as i32 || roi.size.height < template.height as i32 {
        return lost(last);
    }
    let pixels = crop_bgra(image, roi);
    let search_plane = Plane {
        data: &pixels,
        width: roi.size.width as usize,
        height: roi.size.height as usize,
        channels: 4,
    };
    let template_plane = Plane {
        data: &template.pixels,
        width: template.width,
        height: template.height,
        channels: 4,
    };
    // `cvMaskGray` (`TrackerTemplateMatching.cs:452-480`): built when masking is on and kept after it.
    if parameters.use_mask && !mask.as_ref().is_some_and(|m| fits(m, template)) {
        *mask = Some(ellipse_mask(template.width as u32, template.height as u32));
    }
    let mask = mask
        .as_ref()
        .filter(|m| fits(m, template))
        .map(|m| m.data.as_slice());
    let mut map = match_template(search_plane, template_plane, mask);
    let mut best = map.min_max_loc();
    if best.max.is_infinite() || best.max > 1.0 {
        map = match_template(search_plane, template_plane, None);
        best = map.min_max_loc();
    }
    if best.max.is_infinite() {
        return lost(last);
    }
    let (mx, my) = best.max_loc;
    let (dx, dy) = peak_offset(&map, mx, my);
    let half = (template.width / 2, template.height / 2);
    Found {
        similarity: best.max,
        x: roi.x as f32 + (mx as f32 + dx) + half.0 as f32,
        y: roi.y as f32 + (my as f32 + dy) + half.1 as f32,
    }
}

/// Whether a kept mask has the template size. OpenCV rejects another size; the port applies no mask.
fn fits(mask: &GrayImage, template: &Patch) -> bool {
    mask.width as usize == template.width && mask.height as usize == template.height
}

fn lost(last: (f32, f32)) -> Found {
    Found {
        similarity: 0.0,
        x: last.0,
        y: last.1,
    }
}
