//! Owner: P1. Painters ported from `DrawingGenericPosture.Draw`, in its drawing order.

mod angles;
#[cfg(test)]
mod angles_test;
#[cfg(test)]
mod computed_points_test;
#[cfg(test)]
mod entry_test;
#[cfg(test)]
mod fixture_test;
mod frame;
#[cfg(test)]
mod frame_test;
mod points;
#[cfg(test)]
mod points_test;
mod strokes;
#[cfg(test)]
mod strokes_test;
mod texts;
#[cfg(test)]
mod texts_test;

use super::PaintContext;
use crate::renderer::Renderer;
use angles::paint_angles;
use frame::PostureFrame;
use kinovea_metadata::drawing::GenericPostureDrawing;
use points::{paint_computed_points, paint_points};
use strokes::{paint_circles, paint_polylines, paint_segments};
use texts::{paint_distances, paint_positions};

/// Port of `DrawingGenericPosture.Draw`: segments, circles, angles, polylines, points, computed
/// points, distances and positions. Nothing is drawn when fully faded or without a tool description.
pub fn paint_posture(d: &GenericPostureDrawing, ctx: &PaintContext, r: &mut dyn Renderer) {
    let Some(frame) = PostureFrame::new(d, ctx) else {
        return;
    };
    paint_segments(&frame, r);
    paint_circles(&frame, r);
    paint_angles(&frame, r);
    paint_polylines(&frame, r);
    paint_points(&frame, r);
    paint_computed_points(&frame, r);
    paint_distances(&frame, r);
    paint_positions(&frame, r);
}
