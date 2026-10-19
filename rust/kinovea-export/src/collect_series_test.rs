use crate::collect_series::{times, timeseries};
use crate::measure::Scope;
use crate::measured_data::ExportPrefs;
use kinovea_core::preferences::enums::ExportSpace;
use kinovea_core::timecode::TimecodeFormat;
use kinovea_core::timed_point::TimedPoint;
use kinovea_core::video_section::VideoSection;
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Chrono, ChronoColumn, ChronoMulti, Drawing, Track};
use kinovea_metadata::Metadata;

fn prefs() -> ExportPrefs {
    ExportPrefs {
        export_space: ExportSpace::ImageSpace,
        timecode_format: TimecodeFormat::Frames,
        producer: String::new(),
    }
}

fn chrono(name: &str, start: i64) -> Chrono {
    let mut c = Chrono::new(PointF::new(0.0, 0.0));
    c.header.name = name.to_owned();
    c.start_counting = start;
    c.stop_counting = start + 10;
    c
}

fn track(name: &str, first: i64) -> Track {
    let points = vec![
        TimedPoint {
            x: 1.0,
            y: 2.0,
            t: first,
            r: 0.0,
        },
        TimedPoint {
            x: 3.0,
            y: 4.0,
            t: first + 1,
            r: 0.0,
        },
    ];
    let mut t = Track::new(points);
    t.header.name = name.to_owned();
    t
}

#[test]
fn stopwatches_are_sorted_by_their_first_section_start() {
    let m = Metadata {
        chronos: vec![
            Drawing::Chrono(chrono("late", 50)),
            Drawing::Chrono(chrono("early", 20)),
        ],
        ..Default::default()
    };
    let p = prefs();
    let entries = times(&Scope {
        metadata: &m,
        prefs: &p,
    });
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["early", "late"]);
    assert_eq!(entries[0].sections[0].duration, 10.0);
    assert_eq!(entries[0].visible_columns, [ChronoColumn::Duration]);
    assert!(!entries[0].has_tags);
}

#[test]
fn advanced_stopwatch_without_closed_sections_is_skipped() {
    let m = Metadata {
        chronos: vec![Drawing::ChronoMulti(ChronoMulti::new(PointF::new(
            0.0, 0.0,
        )))],
        ..Default::default()
    };
    let p = prefs();
    assert!(times(&Scope {
        metadata: &m,
        prefs: &p
    })
    .is_empty());
}

#[test]
fn advanced_stopwatch_sections_are_cumulative() {
    let mut multi = ChronoMulti::new(PointF::new(0.0, 0.0));
    multi.insert_section(VideoSection { start: 0, end: 2 }, "", "");
    multi.insert_section(VideoSection { start: 4, end: 7 }, "second", "tag");
    let columns = multi.visible_columns.clone();
    let m = Metadata {
        chronos: vec![Drawing::ChronoMulti(multi)],
        ..Default::default()
    };
    let p = prefs();
    let entry = &times(&Scope {
        metadata: &m,
        prefs: &p,
    })[0];
    let sections = &entry.sections;
    assert!(entry.has_tags);
    assert_eq!(entry.visible_columns, columns);
    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].name, "1");
    assert_eq!(sections[1].name, "second");
    assert_eq!(sections[1].tag, "tag");
    assert_eq!(sections[1].duration, 3.0);
    assert_eq!(sections[1].cumul, 5.0);
}

#[test]
fn tracks_are_sorted_by_first_timestamp_with_their_samples() {
    let m = Metadata {
        tracks: vec![
            Drawing::Track(track("second", 500)),
            Drawing::Track(track("first", 100)),
        ],
        ..Default::default()
    };
    let p = prefs();
    let series = timeseries(&Scope {
        metadata: &m,
        prefs: &p,
    });
    assert_eq!(series[0].name, "first");
    assert_eq!(series[0].times, vec![100.0, 101.0]);
    assert_eq!(series[0].data[0].0, "0");
    assert_eq!(
        series[0].data[0].1,
        vec![PointF::new(1.0, 2.0), PointF::new(3.0, 4.0)]
    );
}
