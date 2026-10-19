use crate::drawing_tracker_test::{bound_drawing, track};
use crate::manager::trim_attached_tracks;
use kinovea_metadata::{Drawing, Uuid};

#[test]
fn trim_cuts_only_the_tracks_bound_to_the_drawing() {
    // Metadata.cs:1251-1260 and DrawingTrack.cs:1540-1565: the closest sample to 12 is 10.
    let (id, bound) = (Uuid::new_v4(), Uuid::new_v4());
    let other = Uuid::new_v4();
    let data = bound_drawing(id, bound);
    let mut tracks: Vec<Drawing> = vec![
        track(
            bound,
            &[
                (0, 0.0, 0.0),
                (10, 1.0, 0.0),
                (20, 2.0, 0.0),
                (30, 3.0, 0.0),
            ],
        ),
        track(
            other,
            &[
                (0, 0.0, 0.0),
                (10, 1.0, 0.0),
                (20, 2.0, 0.0),
                (30, 3.0, 0.0),
            ],
        ),
    ];
    trim_attached_tracks(&data, &mut tracks, id, 12);
    let len = |d: &Drawing| match d {
        Drawing::Track(t) => t.positions.len(),
        _ => 0,
    };
    assert_eq!(len(&tracks[0]), 2);
    assert_eq!(len(&tracks[1]), 4);
}
