use crate::drawing::{Chrono, Drawing, Line, Track};
use crate::keyframe::Keyframe;
use crate::metadata::edit::{EditError, MultiItem};
use crate::metadata::{Manager, Metadata};
use crate::multi::Spotlight;
use kinovea_core::timed_point::TimedPoint;
use kinovea_geometry::PointF;

fn line(name: &str) -> Drawing {
    let mut d = Drawing::Line(Line::new(PointF::EMPTY, PointF::new(1.0, 2.0)));
    d.header_mut().name = name.to_owned();
    d
}

fn keyframe_with_line(m: &mut Metadata) -> (uuid::Uuid, uuid::Uuid) {
    let kf = Keyframe::new(250, "k");
    let kid = kf.id;
    m.merge_insert_keyframe(kf);
    m.add_drawing(Manager::Keyframe(kid), line("a")).unwrap();
    m.register_trackables();
    (kid, m.keyframes[0].drawings[0].id())
}

/// A track from 0 to 200 with one keyframe label, as a keyframe at 100 leaves it.
fn track_with_label_at(m: &mut Metadata, keyframe: i64) -> uuid::Uuid {
    let mut track = Track::new(vec![TimedPoint::new(0.0, 0.0, 0), TimedPoint::new(1.0, 1.0, 200)]);
    track.update_keyframe_labels(&[keyframe]);
    let id = track.header.id;
    m.tracks.push(Drawing::Track(track));
    id
}

fn label_count(m: &Metadata) -> usize {
    match &m.tracks[0] {
        Drawing::Track(t) => t.keyframe_labels.len(),
        _ => 0,
    }
}

#[test]
fn keyframe_drawing_takes_the_keyframe_time_as_reference() {
    let mut m = Metadata::default();
    let (kid, id) = keyframe_with_line(&mut m);
    let d = m.drawing(Manager::Keyframe(kid), id).unwrap();
    assert_eq!(d.header().reference_timestamp, 250);
    assert_eq!(d.header().fading.reference_timestamp, 250);
}

#[test]
fn delete_drawing_forgets_its_tracker_and_returns_the_drawing() {
    let mut m = Metadata::default();
    let (kid, id) = keyframe_with_line(&mut m);
    assert!(m.trackability.tracker(id).is_some());

    let removed = m.delete_drawing(Manager::Keyframe(kid), id).unwrap();

    assert_eq!(removed.map(|d| d.id()), Some(id));
    assert!(m.keyframes[0].drawings.is_empty());
    assert!(m.trackability.tracker(id).is_none());
    assert!(m.delete_drawing(Manager::Keyframe(kid), id).unwrap().is_none());
}

#[test]
fn chronos_are_found_and_edited_in_place_but_singletons_are_not_drawing_lists() {
    let mut m = Metadata::default();
    m.add_drawing(Manager::Chronos, Drawing::Chrono(Chrono::new(PointF::EMPTY))).unwrap();
    let id = m.chronos[0].id();

    if let Some(d) = m.drawing_mut(Manager::Chronos, id) {
        d.header_mut().name = "renamed".to_owned();
    }

    assert_eq!(m.drawing(Manager::Chronos, id).map(|d| d.header().name.as_str()), Some("renamed"));
    assert!(m.drawing(Manager::Singletons, id).is_none());
    assert!(m.drawing(Manager::Tracks, id).is_none());
}

#[test]
fn singletons_cannot_be_added_or_deleted() {
    let mut m = Metadata::default();
    let id = m.coordinate_system.header.id;
    assert_eq!(m.add_drawing(Manager::Singletons, line("x")), Err(EditError::Singleton));
    assert_eq!(m.delete_drawing(Manager::Singletons, id), Err(EditError::Singleton));
}

#[test]
fn a_drawing_already_in_its_container_is_not_added_again() {
    let mut m = Metadata::default();
    let (kid, _) = keyframe_with_line(&mut m);
    let again = m.keyframes[0].drawings[0].clone();
    m.add_drawing(Manager::Keyframe(kid), again).unwrap();
    assert_eq!(m.keyframes[0].drawings.len(), 1);

    let chrono = Drawing::Chrono(Chrono::new(PointF::EMPTY));
    m.add_drawing(Manager::Chronos, chrono.clone()).unwrap();
    m.add_drawing(Manager::Chronos, chrono).unwrap();
    assert_eq!(m.chronos.len(), 1);
}

#[test]
fn delete_keyframe_forgets_the_trackers_of_its_drawings() {
    let mut m = Metadata::default();
    let (kid, id) = keyframe_with_line(&mut m);

    assert!(m.delete_keyframe(kid).is_some());

    assert!(m.keyframes.is_empty());
    assert!(m.trackability.tracker(id).is_none());
    assert!(m.delete_keyframe(kid).is_none());
}

#[test]
fn delete_keyframe_updates_the_keyframe_labels_of_tracks() {
    let mut m = Metadata::default();
    let kf = Keyframe::new(100, "k");
    let kid = kf.id;
    m.merge_insert_keyframe(kf);
    track_with_label_at(&mut m, 100);
    assert_eq!(label_count(&m), 1);

    m.delete_keyframe(kid);

    assert_eq!(label_count(&m), 0);
}

#[test]
fn deleting_a_track_forgets_the_tracking_bound_to_it() {
    let mut m = Metadata::default();
    m.add_multi_item(MultiItem::Spotlight(Spotlight::new(5, PointF::EMPTY, 3)));
    m.register_spotlights();
    let spotlight_id = m.spotlights.items[0].id;
    let track = Track::new(vec![TimedPoint::new(1.0, 2.0, 0)]);
    let track_id = track.header.id;
    m.tracks.push(Drawing::Track(track));
    m.trackability.trackers.get_mut(spotlight_id).unwrap().points[0].1.track_id = Some(track_id);

    m.delete_drawing(Manager::Tracks, track_id).unwrap();

    assert_eq!(m.trackability.tracker(spotlight_id).unwrap().points[0].1.track_id, None);
}
