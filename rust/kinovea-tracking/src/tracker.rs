//! The tracker contract (`AbstractTracker.cs`), without the drawing of the gizmo: the player renders
//! it from the tracker state. The image is the BGRA frame the original converts to a `Mat`
//! (`BitmapConverter.ToMat` of a `Format32bppPArgb` bitmap, so four channels).

use kinovea_core::timed_point::TimedPoint;
use kinovea_metadata::drawing::tracking_parameters::TrackingParameters;
use kinovea_video::frame::FrameBuffer;

/// The outcome of `TrackStep`: the new point (the previous one when not matched), whether the target
/// was found, and the similarity or vote score behind it (0 when not matched).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepResult {
    pub point: TimedPoint,
    pub matched: bool,
    pub score: f32,
}

pub trait Tracker {
    fn parameters(&self) -> &TrackingParameters;
    /// `IsReady`: whether a step can run from `last`.
    fn is_ready(&self, last: &TimedPoint) -> bool;
    /// `TrackStep`: finds the target at `time` from the points tracked so far (the last one is the
    /// starting point). Without an image the previous position is kept and the step is not matched.
    fn track_step(
        &mut self,
        timeline: &[TimedPoint],
        time: i64,
        image: Option<&FrameBuffer>,
    ) -> StepResult;
    /// `CreateReferenceTrackPoint`: a point placed by the user. The circle tracker sets its radius.
    fn create_reference_point(&mut self, point: &mut TimedPoint, image: &FrameBuffer);
    /// `Trim`: forgets what was learned after `time`.
    fn trim(&mut self, time: i64);
    fn clear(&mut self);
    /// `UpdateImage`: the image at `time` changed (a filter was applied, for example).
    fn update_image(&mut self, time: i64, image: &FrameBuffer, timeline: &[TimedPoint]);
}
