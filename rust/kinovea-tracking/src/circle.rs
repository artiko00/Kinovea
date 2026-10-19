//! Port of `Kinovea.ScreenManager/Tracking/Tracking/Circle/TrackerCircle.cs`: the Hough circle
//! tracker. The state is the set of reference times (not stored to KVA): it bounds the averaging
//! window of the reference radius, so a point set by the user is used exactly. Apart from that
//! the tracker is stateless.

#[cfg(test)]
#[path = "circle_test.rs"]
mod circle_test;

mod matching;
mod search;

use crate::tracker::{StepResult, Tracker};
use kinovea_core::timed_point::TimedPoint;
use kinovea_metadata::drawing::tracking_parameters::TrackingParameters;
use kinovea_video::frame::FrameBuffer;
use std::collections::BTreeSet;

/// Samples averaged for the reference radius (`averagingWindow`).
const AVERAGING_WINDOW: usize = 4;

pub struct CircleTracker {
    parameters: TrackingParameters,
    reference_times: BTreeSet<i64>,
}

impl CircleTracker {
    pub fn new(parameters: TrackingParameters) -> Self {
        Self {
            parameters,
            reference_times: BTreeSet::new(),
        }
    }

    /// Mean radius of the last samples, never before the last reference (`TrackStep`). `None` when
    /// the tracker is not ready or no sample follows the last reference (the original throws there).
    fn reference_radius(&self, timeline: &[TimedPoint]) -> Option<f32> {
        let last = timeline.last()?;
        if !self.is_ready(last) {
            return None;
        }
        let last_reference = *self.reference_times.last()?;
        let after = timeline.iter().skip_while(|p| p.t < last_reference).count();
        let samples = AVERAGING_WINDOW.min(after);
        if samples == 0 {
            return None;
        }
        let window = &timeline[timeline.len() - samples..];
        let sum: f64 = window.iter().map(|p| f64::from(p.r)).sum();
        Some((sum / samples as f64) as f32)
    }
}

impl Tracker for CircleTracker {
    fn parameters(&self) -> &TrackingParameters {
        &self.parameters
    }

    fn is_ready(&self, last: &TimedPoint) -> bool {
        last.r != 0.0 && !self.reference_times.is_empty()
    }

    fn track_step(
        &mut self,
        timeline: &[TimedPoint],
        time: i64,
        image: Option<&FrameBuffer>,
    ) -> StepResult {
        let last = *timeline
            .last()
            .expect("the timeline holds the starting point");
        let kept = StepResult {
            point: TimedPoint::with_radius(last.x, last.y, time, last.r),
            matched: false,
            score: 0.0,
        };
        let (Some(image), Some(radius)) = (image, self.reference_radius(timeline)) else {
            return kept;
        };
        match matching::match_circle(
            image,
            radius,
            (last.x, last.y),
            self.parameters.search_window(),
        ) {
            Some(found) => StepResult {
                point: TimedPoint::with_radius(found.x, found.y, time, found.radius),
                matched: true,
                score: found.votes as f32,
            },
            None => kept,
        }
    }

    fn create_reference_point(&mut self, point: &mut TimedPoint, _image: &FrameBuffer) {
        let block = self.parameters.block_window();
        point.r = (block.width as f32 / 2.0).max(block.height as f32 / 2.0);
        self.reference_times.insert(point.t);
    }

    fn trim(&mut self, time: i64) {
        self.reference_times.retain(|&t| t <= time);
    }

    fn clear(&mut self) {
        self.reference_times.clear();
    }

    fn update_image(&mut self, _time: i64, _image: &FrameBuffer, _timeline: &[TimedPoint]) {}
}
