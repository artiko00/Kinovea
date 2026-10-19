use crate::tracker::{StepResult, Tracker};
use kinovea_core::timed_point::TimedPoint;
use kinovea_metadata::drawing::tracking_parameters::TrackingParameters;
use kinovea_video::frame::FrameBuffer;

/// A tracker that never finds anything: the contract can be used as a trait object.
struct Blind(TrackingParameters);

impl Tracker for Blind {
    fn parameters(&self) -> &TrackingParameters {
        &self.0
    }
    fn is_ready(&self, _last: &TimedPoint) -> bool {
        true
    }
    fn track_step(
        &mut self,
        timeline: &[TimedPoint],
        time: i64,
        _image: Option<&FrameBuffer>,
    ) -> StepResult {
        let last = timeline[timeline.len() - 1];
        StepResult {
            point: TimedPoint::with_radius(last.x, last.y, time, last.r),
            matched: false,
            score: 0.0,
        }
    }
    fn create_reference_point(&mut self, _point: &mut TimedPoint, _image: &FrameBuffer) {}
    fn trim(&mut self, _time: i64) {}
    fn clear(&mut self) {}
    fn update_image(&mut self, _time: i64, _image: &FrameBuffer, _timeline: &[TimedPoint]) {}
}

#[test]
fn an_unmatched_step_keeps_the_previous_position_at_the_new_time() {
    let mut tracker: Box<dyn Tracker> = Box::new(Blind(TrackingParameters::default()));
    let result = tracker.track_step(&[TimedPoint::new(3.0, 4.0, 0)], 7, None);
    assert_eq!(result.point, TimedPoint::new(3.0, 4.0, 7));
    assert!(!result.matched);
}
