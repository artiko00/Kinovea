//! Port of `TrackerTemplateMatching.cs` without the gizmo: the template timeline, readiness, the
//! track step with its failure semantics and the template update policy. Each track owns one
//! tracker. Templates are live state, rebuilt when the track is re-read, never stored in KVA.

pub mod capture;
#[cfg(test)]
mod capture_test;
pub mod matching;
#[cfg(test)]
mod matching_mask_test;
#[cfg(test)]
mod matching_test;
#[cfg(test)]
mod policy_test;

use kinovea_core::timed_point::TimedPoint;
use kinovea_metadata::drawing::tracking_parameters::TrackingParameters;
use kinovea_video::frame::FrameBuffer;

use crate::pixels::image::GrayImage;
use crate::timeline::Timeline;
use crate::tracker::{StepResult, Tracker};
pub use capture::TrackingTemplate;
use matching::match_block;

pub struct TemplateTracker {
    parameters: TrackingParameters,
    templates: Timeline<TrackingTemplate>,
    /// `cvMaskGray`: built once with masking on, then kept and used even when masking is turned off.
    mask: Option<GrayImage>,
}

impl TemplateTracker {
    pub fn new(parameters: TrackingParameters) -> Self {
        TemplateTracker {
            parameters,
            templates: Timeline::new(),
            mask: None,
        }
    }

    /// The parameters are shared with the user in the original: changing them makes the tracker
    /// not ready when the block window changes.
    pub fn parameters_mut(&mut self) -> &mut TrackingParameters {
        &mut self.parameters
    }

    /// Templates in time order, for the gizmo's score readout.
    pub fn templates(&self) -> &Timeline<TrackingTemplate> {
        &self.templates
    }

    /// The last template when it has the size of the block window.
    fn ready_template(&self) -> Option<&TrackingTemplate> {
        let block = self.parameters.block_window();
        self.templates.last().filter(|t| {
            t.patch.width == block.width as usize && t.patch.height == block.height as usize
        })
    }

    /// `CreateTrackPoint`: keeps the previous template when the match is very good or very bad,
    /// otherwise captures a new one at the rounded point.
    fn create_track_point(
        &mut self,
        (x, y): (f32, f32),
        score: f32,
        time: i64,
        image: &FrameBuffer,
        previous: &[TimedPoint],
    ) -> TimedPoint {
        let score_f = f64::from(score);
        let keep = !previous.is_empty()
            && (score_f > self.parameters.template_update_threshold
                || score_f < self.parameters.similarity_threshold);
        let patch = match self.templates.last() {
            Some(template) if keep => template.patch.clone(),
            _ => capture::capture(image, (x, y), self.parameters.block_window()),
        };
        self.templates
            .insert(time, TrackingTemplate { score, patch });
        TimedPoint::new(x, y, time)
    }
}

impl Tracker for TemplateTracker {
    fn parameters(&self) -> &TrackingParameters {
        &self.parameters
    }

    fn is_ready(&self, _last: &TimedPoint) -> bool {
        self.ready_template().is_some()
    }

    /// `TrackStep`. Without an image, or when not ready, the previous point is kept and nothing is
    /// recorded; the original throws there, which the callers never reach.
    fn track_step(
        &mut self,
        timeline: &[TimedPoint],
        time: i64,
        image: Option<&FrameBuffer>,
    ) -> StepResult {
        let last = *timeline.last().expect("a track step starts from a point");
        let previous = StepResult {
            point: TimedPoint::new(last.x, last.y, time),
            matched: false,
            score: 0.0,
        };
        let (Some(image), Some(template)) = (image, self.ready_template().map(|t| t.patch.clone()))
        else {
            return previous;
        };
        let found = match_block(
            &self.parameters,
            image,
            &template,
            (last.x, last.y),
            &mut self.mask,
        );
        let failed = found.similarity == 0.0
            || f64::from(found.similarity) < self.parameters.similarity_threshold;
        let (location, score) = if failed {
            ((last.x, last.y), 0.0)
        } else {
            ((found.x, found.y), found.similarity)
        };
        let point = self.create_track_point(location, score, time, image, timeline);
        StepResult {
            point,
            matched: !failed,
            score,
        }
    }

    /// `CreateReferenceTrackPoint`: always captures a template, with a score of 1.
    fn create_reference_point(&mut self, point: &mut TimedPoint, image: &FrameBuffer) {
        let patch = capture::capture(image, (point.x, point.y), self.parameters.block_window());
        self.templates
            .insert(point.t, TrackingTemplate { score: 1.0, patch });
    }

    fn trim(&mut self, time: i64) {
        self.templates.trim(time);
    }

    fn clear(&mut self) {
        self.templates.clear();
    }

    fn update_image(&mut self, _time: i64, _image: &FrameBuffer, _timeline: &[TimedPoint]) {}
}
