use crate::drawing_tracker::{point_at_time, track_point_at, Trackable};
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::Track;
use kinovea_metadata::trackability::TrackabilityManager;
use kinovea_metadata::{Drawing, Uuid};

/// Records the values a drawing receives.
#[derive(Default)]
pub(crate) struct FakeTrackable {
    pub(crate) values: Vec<(String, PointF, i64)>,
    pub(crate) finished: usize,
}

impl Trackable for FakeTrackable {
    fn set_trackable_point_value(&mut self, key: &str, value: PointF, age: i64) {
        self.values.push((key.to_owned(), value, age));
    }

    fn after_all_points_set(&mut self) {
        self.finished += 1;
    }
}

/// A track drawing with the given `(t, x, y)` positions.
pub(crate) fn track(id: Uuid, positions: &[(i64, f32, f32)]) -> Drawing {
    let points = positions
        .iter()
        .map(|&(t, x, y)| TimedPoint::new(x, y, t))
        .collect();
    let mut track = Track::new(points);
    track.header.id = id;
    Drawing::Track(track)
}

/// A drawing `id` with one point `a` at (1, 1) at time 0, bound to track `bound`.
pub(crate) fn bound_drawing(id: Uuid, bound: Uuid) -> TrackabilityManager {
    let mut data = TrackabilityManager::default();
    data.add(id, "Line", 0, &[("a".into(), PointF::new(1.0, 1.0))]);
    data.bind_track(id, "a", bound);
    data
}

/// The track of `bound` moves from (0, 0) at 0 to (7, 8) at 40.
pub(crate) fn moving(bound: Uuid) -> [Drawing; 1] {
    [track(bound, &[(0, 0.0, 0.0), (40, 7.0, 8.0)])]
}

#[test]
fn point_at_time_follows_the_bound_track() {
    // DrawingTracker.cs:684-689
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let data = bound_drawing(id, bound);
    let tracker = data.tracker(id).unwrap();
    assert_eq!(
        point_at_time(tracker, &moving(bound), "a", 40),
        PointF::new(7.0, 8.0)
    );
}

#[test]
fn unbound_and_unknown_keys_are_empty_while_object_tracking() {
    // DrawingTracker.cs:684-692: a key without a track is EMPTY when object tracking is initialized.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let mut data = bound_drawing(id, bound);
    data.add(
        id,
        "Line",
        0,
        &[
            ("a".into(), PointF::new(1.0, 1.0)),
            ("b".into(), PointF::new(3.0, 3.0)),
        ],
    );
    let tracker = data.tracker(id).unwrap();
    assert_eq!(
        point_at_time(tracker, &moving(bound), "b", 40),
        PointF::EMPTY
    );
    assert_eq!(
        point_at_time(tracker, &moving(bound), "zz", 40),
        PointF::EMPTY
    );
}

#[test]
fn a_gap_in_the_track_is_empty() {
    // DrawingTracker.cs:688 dereferences GetTimedPoint(), which is null in a gap (DrawingTrack.cs:1677);
    // the original throws there, this returns EMPTY.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let data = bound_drawing(id, bound);
    let gap = [track(bound, &[(0, 0.0, 0.0), (100, 7.0, 8.0)])];
    assert_eq!(
        point_at_time(data.tracker(id).unwrap(), &gap, "a", 50),
        PointF::EMPTY
    );
}

#[test]
fn without_object_tracking_the_reference_value_is_returned() {
    // DrawingTracker.cs:695-699: the reference value, camera motion not applied.
    let id = Uuid::new_v4();
    let mut data = TrackabilityManager::default();
    data.add(id, "Line", 0, &[("a".into(), PointF::new(1.0, 1.0))]);
    let tracker = data.tracker(id).unwrap();
    assert_eq!(point_at_time(tracker, &[], "a", 50), PointF::new(1.0, 1.0));
    assert_eq!(point_at_time(tracker, &[], "zz", 50), PointF::EMPTY);
}

#[test]
fn track_positions_are_clamped_at_the_ends_and_absent_in_gaps() {
    // DrawingTrack.cs:1663-1679
    let id = Uuid::new_v4();
    let Drawing::Track(t) = track(id, &[(10, 3.0, 0.0), (20, 4.0, 0.0), (50, 5.0, 0.0)]) else {
        unreachable!()
    };
    let at = |time| track_point_at(&t, time).map(|p| (p.x, p.t));
    assert_eq!(at(5), Some((3.0, 10)));
    assert_eq!(at(20), Some((4.0, 20)));
    assert_eq!(at(30), None);
    assert_eq!(at(90), Some((5.0, 50)));
}
