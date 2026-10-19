use super::{case_from, expected, load, names};
use serde_json::json;

#[test]
fn every_case_parses_and_has_one_expected_row_per_frame() {
    let names = names();
    assert!(names.contains(&"template-square".to_owned()), "{names:?}");
    for name in names {
        assert_eq!(expected(&name).len(), load(&name).path.len(), "{name}");
    }
}

#[test]
fn windows_are_set_in_the_oracle_order() {
    // Each setter fits the search window to the block (FitBlockWindow): 10x10 grows to the default
    // 20x20 block, and stays 20x20 when the block becomes 20x12.
    let case =
        case_from(&json!({ "path": [[0, 0]], "searchWindow": [10, 10], "blockWindow": [20, 12] }));
    assert_eq!(
        (
            case.parameters.search_window().width,
            case.parameters.search_window().height
        ),
        (20, 20)
    );
    assert!(!case.circle && !case.scene.disc);
}

/// Never moves: the replay itself is what is under test.
struct Still(kinovea_metadata::drawing::tracking_parameters::TrackingParameters);

impl kinovea_tracking::tracker::Tracker for Still {
    fn parameters(&self) -> &kinovea_metadata::drawing::tracking_parameters::TrackingParameters {
        &self.0
    }
    fn is_ready(&self, _last: &kinovea_core::timed_point::TimedPoint) -> bool {
        true
    }
    fn track_step(
        &mut self,
        timeline: &[kinovea_core::timed_point::TimedPoint],
        time: i64,
        _image: Option<&kinovea_video::frame::FrameBuffer>,
    ) -> kinovea_tracking::tracker::StepResult {
        let mut point = timeline[timeline.len() - 1];
        point.t = time;
        kinovea_tracking::tracker::StepResult {
            point,
            matched: false,
            score: 0.0,
        }
    }
    fn create_reference_point(
        &mut self,
        _point: &mut kinovea_core::timed_point::TimedPoint,
        _image: &kinovea_video::frame::FrameBuffer,
    ) {
    }
    fn trim(&mut self, _time: i64) {}
    fn clear(&mut self) {}
    fn update_image(
        &mut self,
        _time: i64,
        _image: &kinovea_video::frame::FrameBuffer,
        _timeline: &[kinovea_core::timed_point::TimedPoint],
    ) {
    }
}

#[test]
fn the_replay_starts_at_the_integer_path_point_and_steps_every_frame() {
    let case =
        case_from(&json!({ "path": [[163, 202], [170, 202], [177, 202]], "tsPerFrame": 10 }));
    let rows = super::run(&case, &mut Still(case.parameters.clone()));
    assert_eq!(rows.len(), 3);
    // 163 / 4 = 40 and 202 / 4 = 50 with integer division, as `(int)center[0] / 4` in the oracle.
    assert_eq!(
        (
            rows[0].x,
            rows[0].y,
            rows[0].t,
            rows[0].score,
            rows[0].matched
        ),
        (40.0, 50.0, 0, 1.0, true)
    );
    assert_eq!(
        (rows[2].x, rows[2].t, rows[2].score, rows[2].matched),
        (40.0, 20, 0.0, false)
    );
}
