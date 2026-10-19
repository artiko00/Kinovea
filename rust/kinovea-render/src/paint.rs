//! One painter per drawing kind, ported from each `Draw` override (design D2). Painters work in
//! image coordinates and map them through `ctx.viewport`, like the original `transformer`.
//! Ownership during the parallel port: P1 = geometric kinds, arrows, squiggles, singletons,
//! postures; P2 = labels, time drawings, tracks, multi-drawings, media, mini labels, boxes.

/// Declares each painter module with its sibling test module.
macro_rules! painters {
    ($($m:ident / $t:ident),+ $(,)?) => {
        $(pub mod $m; #[cfg(test)] mod $t;)+
    };
}

painters!(
    angle / angle_test,
    chrono / chrono_test,
    circle / circle_test,
    counter / counter_test,
    crossmark / crossmark_test,
    helpers / helpers_test,
    label / label_test,
    line / line_test,
    media / media_test,
    multi / multi_test,
    paths / paths_test,
    plane / plane_test,
    posture / posture_test,
    rectangle / rectangle_test,
    singletons / singletons_test,
    time_segment / time_segment_test,
    track / track_test,
);

mod context;
#[cfg(test)]
mod context_test;

pub use context::{CameraTransform, Distorter, PaintContext};

use crate::renderer::Renderer;
use kinovea_metadata::Drawing;

/// Paints one attached or detached drawing (`AbstractDrawing.Draw`).
pub fn paint_drawing(d: &Drawing, ctx: &PaintContext, r: &mut dyn Renderer) {
    match d {
        Drawing::Line(x) => line::paint_line(x, ctx, r),
        Drawing::Angle(x) => angle::paint_angle(x, ctx, r),
        Drawing::CrossMark(x) => crossmark::paint_crossmark(x, ctx, r),
        Drawing::Circle(x) => circle::paint_circle(x, ctx, r),
        Drawing::Rectangle(x) => rectangle::paint_rectangle(x, ctx, r),
        Drawing::Pencil(x) => paths::paint_pencil(x, ctx, r),
        Drawing::Polyline(x) => paths::paint_polyline(x, ctx, r),
        Drawing::TimePolyline(x) => paths::paint_time_polyline(x, ctx, r),
        Drawing::Label(x) => label::paint_label(x, ctx, r),
        Drawing::Plane(x) => plane::paint_plane(x, ctx, r),
        Drawing::DistortionGrid(x) => plane::paint_distortion_grid(x, ctx, r),
        Drawing::TimeSegment(x) => time_segment::paint_time_segment(x, ctx, r),
        Drawing::Chrono(x) => chrono::paint_chrono(x, ctx, r),
        Drawing::ChronoMulti(x) => chrono::paint_chrono_multi(x, ctx, r),
        Drawing::Counter(x) => counter::paint_counter(x, ctx, r),
        Drawing::Track(x) => track::paint_track(x, ctx, r),
        Drawing::Bitmap(x) => media::paint_bitmap(x, ctx, r),
        Drawing::Svg(x) => media::paint_svg(x, ctx, r),
        Drawing::GenericPostureDrawing(x) => posture::paint_posture(x, ctx, r),
    }
}
