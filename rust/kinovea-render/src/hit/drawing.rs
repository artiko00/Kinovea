//! Dispatcher of `HitTest` over the drawing kinds (one group module per kind family).

use super::angle::hit_angle;
use super::chrono::{hit_chrono, hit_chrono_multi, hit_counter};
use super::context::{Hit, HitContext};
use super::label::hit_label;
use super::media::hit_bitmap;
use super::plane::{hit_distortion_grid, hit_plane};
use super::posture::hit_posture;
use super::shapes::{hit_circle, hit_crossmark, hit_rectangle};
use super::strokes::{hit_line, hit_pencil, hit_polyline};
use super::time::{hit_time_polyline, hit_time_segment};
use super::track::hit_track;
use kinovea_geometry::PointF;
use kinovea_metadata::Drawing;

/// Whether `hit_test` answers for this kind. The others always miss: no port yet, see the crate notes.
pub fn is_ported(d: &Drawing) -> bool {
    !matches!(d, Drawing::Svg(_))
}

/// `AbstractDrawing.HitTest` for any attached or detached drawing. Kinds not ported miss.
pub fn hit_test(d: &Drawing, p: PointF, ctx: &HitContext) -> Hit {
    match d {
        Drawing::Circle(x) => hit_circle(x, p, ctx),
        Drawing::CrossMark(x) => hit_crossmark(x, p, ctx),
        Drawing::Rectangle(x) => hit_rectangle(x, p, ctx),
        Drawing::Angle(x) => hit_angle(x, p, ctx),
        Drawing::Track(x) => hit_track(x, p, ctx),
        Drawing::Chrono(x) => hit_chrono(x, p, ctx),
        Drawing::ChronoMulti(x) => hit_chrono_multi(x, p, ctx),
        Drawing::Counter(x) => hit_counter(x, p, ctx),
        Drawing::GenericPostureDrawing(x) => hit_posture(x, p, ctx),
        Drawing::Label(x) => hit_label(x, p, ctx),
        Drawing::Bitmap(x) => hit_bitmap(x, p, ctx),
        Drawing::Plane(x) => hit_plane(x, p, ctx),
        Drawing::DistortionGrid(x) => hit_distortion_grid(x, p, ctx),
        Drawing::Line(x) => hit_line(x, p, ctx),
        Drawing::Pencil(x) => hit_pencil(x, p, ctx),
        Drawing::Polyline(x) => hit_polyline(x, p, ctx),
        Drawing::TimePolyline(x) => hit_time_polyline(x, p, ctx),
        Drawing::TimeSegment(x) => hit_time_segment(x, p, ctx),
        _ => Hit::Miss,
    }
}
