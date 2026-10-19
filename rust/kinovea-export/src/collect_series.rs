//! Stopwatches (`times`) and trajectories (`timeseries`), each sorted as the original sorts them.
//! Timelines of trackable drawings are not collected yet (they arrive with `add-tracking`).

use crate::measure::{Scope, TimeBase};
use crate::measured_data::{TimeEntry, TimeSection, Timeseries};
use kinovea_geometry::PointF;
use kinovea_metadata::drawing::{Chrono, ChronoColumn, ChronoMulti, Track};
use kinovea_metadata::{Drawing, StyleValue};
use std::cmp::Ordering;

/// Stopwatches sorted by the start of their first section. Advanced stopwatches without a closed
/// section are left out.
pub fn times(scope: &Scope) -> Vec<TimeEntry> {
    let mut entries: Vec<TimeEntry> = scope
        .metadata
        .chronos
        .iter()
        .filter_map(|d| match d {
            Drawing::Chrono(c) => Some(chrono_entry(scope, c)),
            Drawing::ChronoMulti(c) => {
                Some(multi_entry(scope, c)).filter(|e| !e.sections.is_empty())
            }
            _ => None,
        })
        .collect();
    entries.sort_by(|a, b| {
        a.sections[0]
            .start
            .partial_cmp(&b.sections[0].start)
            .unwrap_or(Ordering::Equal)
    });
    entries
}

/// Trajectories sorted by their first timestamp.
pub fn timeseries(scope: &Scope) -> Vec<Timeseries> {
    let mut keyed: Vec<(i64, Timeseries)> = scope
        .metadata
        .tracks
        .iter()
        .filter_map(|d| match d {
            Drawing::Track(t) => Some(track_series(scope, t)),
            _ => None,
        })
        .collect();
    keyed.sort_by_key(|(first, _)| *first);
    keyed.into_iter().map(|(_, series)| series).collect()
}

fn chrono_entry(scope: &Scope, c: &Chrono) -> TimeEntry {
    let mut section = TimeSection::default();
    let clock = is_clock(c);
    if !clock && c.start_counting != i64::MAX && c.stop_counting != i64::MAX {
        section.start = scope.time(c.start_counting, TimeBase::UserOrigin);
        section.stop = scope.time(c.stop_counting, TimeBase::UserOrigin);
        section.duration = scope.time(c.stop_counting - c.start_counting, TimeBase::Absolute);
        section.cumul = section.duration;
    } else if clock {
        // A clock reports the time of its origin on the global axis.
        section.start = if c.clock_origin == i64::MAX {
            scope.time(0, TimeBase::Absolute)
        } else {
            scope.time(c.clock_origin, TimeBase::UserOrigin)
        };
    }
    TimeEntry {
        name: c.header.name.clone(),
        sections: vec![section],
        has_tags: false,
        visible_columns: vec![ChronoColumn::Duration],
    }
}

fn multi_entry(scope: &Scope, c: &ChronoMulti) -> TimeEntry {
    let mut cumul_timestamps = 0;
    let mut sections = Vec::new();
    for (i, s) in c.sections.iter().enumerate() {
        if s.section.end == i64::MAX {
            continue;
        }
        let elapsed = s.section.end - s.section.start;
        cumul_timestamps += elapsed;
        sections.push(TimeSection {
            name: if s.name.is_empty() {
                (i + 1).to_string()
            } else {
                s.name.clone()
            },
            tag: s.tag.clone(),
            start: scope.time(s.section.start, TimeBase::UserOrigin),
            stop: scope.time(s.section.end, TimeBase::UserOrigin),
            duration: scope.time(elapsed, TimeBase::Absolute),
            cumul: scope.time(cumul_timestamps, TimeBase::Absolute),
        });
    }
    let has_tags = sections.iter().any(|s| !s.tag.is_empty());
    TimeEntry {
        name: c.header.name.clone(),
        sections,
        has_tags,
        visible_columns: c.visible_columns.clone(),
    }
}

/// The `clock` toggle of the style (`is_clock` in the metadata crate, not exported).
fn is_clock(c: &Chrono) -> bool {
    matches!(
        c.header.style.get("clock"),
        Some(StyleValue::Toggle { value: true, .. })
    )
}

fn track_series(scope: &Scope, t: &Track) -> (i64, Timeseries) {
    let times = t
        .positions
        .iter()
        .map(|p| scope.time(p.t, TimeBase::UserOrigin))
        .collect();
    let points: Vec<PointF> = t
        .positions
        .iter()
        .map(|p| scope.point(PointF::new(p.x, p.y)))
        .collect();
    let first = t.positions.first().map_or(0, |p| p.t);
    (
        first,
        Timeseries {
            name: t.header.name.clone(),
            times,
            data: vec![("0".to_owned(), points)],
        },
    )
}
