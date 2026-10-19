//! Owner: P2. Painter ported from `DrawingTrack.Draw` for the view without an active tracking: the
//! trajectory (past, and future when shown), its steps, the cursor cross, and the labels of
//! `track/labels.rs`. Not ported: the rotation circle, the tracked circle and the tracker help,
//! which only show while tracking, and the edit and configuring states.

mod labels;
#[cfg(test)]
mod labels_test;

use super::PaintContext;
use crate::renderer::{Brush, LineCap, LineJoin, Pen, Renderer};
use crate::style::colors::with_alpha;
use crate::style::StyleData;
use kinovea_geometry::{PointF, RectangleF};
use kinovea_metadata::drawing::Track;
use kinovea_metadata::measure_label::MeasureLabelType;

/// Tension of the trajectory curve (`enableFiltering` on, the preference's default).
const TENSION: f32 = 0.5;
/// Radius of the steps: the `TrackPointSize` default, which the model does not keep.
const STEP_RADIUS: f32 = 3.0;
/// `defaultCrossRadius`.
const CROSS_RADIUS: f32 = 4.0;
/// `opacityNormal`: the past trail and the labels of past keyframes.
const PAST_OPACITY: f64 = 0.9;
/// `opacityFuture`: the future trail and the labels of future keyframes.
const FUTURE_OPACITY: f64 = 0.25;

/// Port of `DrawingTrack.Draw` in its interactive view.
pub fn paint_track(d: &Track, ctx: &PaintContext, r: &mut dyn Renderer) {
    if !d.is_visible || ctx.timestamp < d.visible {
        return;
    }
    let base = if ctx.timestamp > d.invisible {
        d.header
            .fading
            .opacity_factor(ctx.timestamp, ctx.default_fading)
    } else {
        1.0
    };
    if base <= 0.0 || d.positions.is_empty() {
        return;
    }
    let style = StyleData::from_elements(&d.header.style);
    let index = d.closest_point(ctx.timestamp);
    let points = image_points(d, ctx);
    if points.len() > 1 {
        draw_trajectory(r, ctx, &style, &points[..=index], base * PAST_OPACITY);
        if d.see_future {
            draw_trajectory(r, ctx, &style, &points[index..], base * FUTURE_OPACITY);
        }
        if d.show_keyframe_labels && d.measure_label_type != MeasureLabelType::None {
            labels::paint_keyframe_labels(d, ctx, &style, &points, base, r);
        }
        if d.show_track_label && base == 1.0 {
            labels::paint_main_label(d, ctx, &style, &points, r);
        }
    }

    // Before the first point the cursor is at the future opacity, so it is not drawn.
    let opacity = if ctx.timestamp < d.positions[0].t {
        if d.see_future {
            base * FUTURE_OPACITY
        } else {
            0.0
        }
    } else {
        base
    };
    if opacity == 1.0 && ctx.timestamp <= d.positions[d.positions.len() - 1].t {
        draw_cross(r, ctx, &style, d.positions[index].x, d.positions[index].y);
    }
}

/// Positions as seen by the frame: distortion, then camera motion when they are available.
fn image_points(d: &Track, ctx: &PaintContext) -> Vec<PointF> {
    d.positions
        .iter()
        .map(|p| {
            let mut q = PointF::new(p.x, p.y);
            if let Some(distorter) = ctx.distorter {
                q = distorter.distort(q);
            }
            if let Some(camera) = ctx.camera {
                q = camera.transform(q, ctx.timestamp);
            }
            q
        })
        .collect()
}

/// `DrawTrajectory` in the interactive view: the curve at `opacity`, then the steps on each point.
fn draw_trajectory(
    r: &mut dyn Renderer,
    ctx: &PaintContext,
    style: &StyleData,
    points: &[PointF],
    opacity: f64,
) {
    let view: Vec<PointF> = points
        .iter()
        .map(|p| ctx.viewport.transform_point_f(*p).to_point_f())
        .collect();
    if view.len() <= 1 {
        return;
    }
    let pen = Pen {
        start_cap: LineCap::Round,
        end_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..style.pen((opacity * 255.0) as i32, 1.0)
    };
    r.stroke_curve(&view, TENSION, &pen);
    if style.track_shape.steps {
        let brush = Brush::solid(pen.color);
        for p in &view {
            let rect = RectangleF::new(
                p.x - STEP_RADIUS,
                p.y - STEP_RADIUS,
                STEP_RADIUS * 2.0,
                STEP_RADIUS * 2.0,
            );
            r.fill_ellipse(rect, &brush);
        }
    }
}

/// `DrawMarker`: a cross of radius 4 on the current point, in the track colour.
fn draw_cross(r: &mut dyn Renderer, ctx: &PaintContext, style: &StyleData, x: f32, y: f32) {
    let c = ctx
        .viewport
        .transform_point_f(PointF::new(x, y))
        .to_point_f();
    let pen = Pen::new(with_alpha(style.color, 255), 1.0);
    let radius = CROSS_RADIUS;
    r.stroke_line(
        PointF::new(c.x, c.y - radius),
        PointF::new(c.x, c.y + radius),
        &pen,
    );
    r.stroke_line(
        PointF::new(c.x - radius, c.y),
        PointF::new(c.x + radius, c.y),
        &pen,
    );
}
