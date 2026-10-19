use crate::drawing_tracker_test::{bound_drawing, track};
use crate::trim::trim_attached_tracks;
use kinovea_metadata::{Drawing, Uuid};

/// Samples at 0, 10, 20 and 30 on two tracks: `bound` is attached to the drawing, `other` is not.
fn tracks_for(bound: Uuid, other: Uuid) -> Vec<Drawing> {
    let samples = [
        (0, 0.0, 0.0),
        (10, 1.0, 0.0),
        (20, 2.0, 0.0),
        (30, 3.0, 0.0),
    ];
    vec![track(bound, &samples), track(other, &samples)]
}

fn len(d: &Drawing) -> usize {
    match d {
        Drawing::Track(t) => t.positions.len(),
        _ => 0,
    }
}

#[test]
fn trim_cuts_only_the_tracks_bound_to_the_drawing() {
    // Metadata.cs:1251-1260 and DrawingTrack.cs:1540-1565: the closest sample to 12 is 10.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let data = bound_drawing(id, bound);
    let mut tracks = tracks_for(bound, Uuid::new_v4());
    trim_attached_tracks(&data, &mut tracks, id, 12, |_, _| {});
    assert_eq!(len(&tracks[0]), 2);
    assert_eq!(len(&tracks[1]), 4);
}

#[test]
fn the_tracker_of_a_cut_track_is_trimmed_at_its_closest_sample() {
    // DrawingTrack.cs:1562: tracker.Trim(positions[drawPointIndex].T), the sample closest to 12.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let data = bound_drawing(id, bound);
    let mut tracks = tracks_for(bound, Uuid::new_v4());
    let mut trimmed = Vec::new();
    trim_attached_tracks(&data, &mut tracks, id, 12, |track, t| {
        trimmed.push((track, t))
    });
    assert_eq!(trimmed, vec![(bound, 10)]);
}

#[test]
fn a_track_that_loses_no_samples_keeps_its_tracker() {
    // DrawingTrack.cs:1555: the tracker is trimmed only when samples after the closest one are removed.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let data = bound_drawing(id, bound);
    let mut tracks = tracks_for(bound, Uuid::new_v4());
    let mut trimmed = Vec::new();
    trim_attached_tracks(&data, &mut tracks, id, 40, |track, t| {
        trimmed.push((track, t))
    });
    assert!(trimmed.is_empty());
    assert_eq!(len(&tracks[0]), 4);
}
