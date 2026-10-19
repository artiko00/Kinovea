//! Stopwatch and trajectory objects of the JSON writer (the `times` and `timeseries` entries).

use crate::json_render::Json;
use crate::measured_data::{TimeEntry, Timeseries};
use kinovea_geometry::PointF;

/// One object per stopwatch. Each section repeats the same members, as the original does, so a
/// stopwatch with several sections yields duplicate names.
pub fn time_entry(entry: &TimeEntry, time_unit: &str) -> Json {
    let mut members = Vec::new();
    for s in &entry.sections {
        let name = if s.name.is_empty() {
            entry.name.clone()
        } else {
            format!("{} {}", entry.name, s.name)
        };
        members.push(("name", Json::text(&name)));
        members.push(("timeUnit", Json::text(time_unit)));
        members.push(("duration", Json::Float(s.duration)));
        members.push(("cumul", Json::Float(s.cumul)));
        members.push(("start", Json::Float(s.start)));
        members.push(("stop", Json::Float(s.stop)));
    }
    Json::object(members)
}

/// The Kinetics Toolkit shape: a `time` array, then a `data` object with one `[x, y]` array per
/// tracked point.
pub fn series(s: &Timeseries, length_unit: &str, time_unit: &str) -> Json {
    let data = s
        .data
        .iter()
        .map(|(key, points)| (key.clone(), Json::Array(points.iter().map(point).collect())))
        .collect();
    Json::object(vec![
        ("name", Json::text(&s.name)),
        ("timeUnit", Json::text(time_unit)),
        ("dataUnit", Json::text(length_unit)),
        (
            "time",
            Json::Array(s.times.iter().map(|t| Json::Float(*t)).collect()),
        ),
        ("data", Json::Object(data)),
    ])
}

fn point(p: &PointF) -> Json {
    Json::Array(vec![Json::Float(p.x), Json::Float(p.y)])
}
