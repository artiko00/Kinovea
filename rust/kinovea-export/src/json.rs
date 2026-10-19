//! The JSON writer: the layout of `ExporterJSON` (Kinetics Toolkit model) over the measured data.

use crate::json_render::{render, Json};
use crate::json_series::{series, time_entry};
use crate::measured_data::{MeasuredData, Scalar};

/// Writes `metadata` then `data`, indented. `rtf_to_text` converts keyframe comments, which are
/// stored as RTF, to plain text.
pub fn write_json(md: &MeasuredData, rtf_to_text: fn(&str) -> String) -> String {
    render(&Json::object(vec![
        ("metadata", metadata(md)),
        ("data", data(md, rtf_to_text)),
    ]))
}

fn metadata(md: &MeasuredData) -> Json {
    let size = Json::object(vec![
        ("width", Json::Int(i64::from(md.image_size.width))),
        ("height", Json::Int(i64::from(md.image_size.height))),
    ]);
    Json::object(vec![
        ("producer", Json::text(&md.producer)),
        ("fullPath", Json::text(&md.full_path)),
        ("originalFilename", Json::text(&md.original_filename)),
        ("imageSize", size),
        ("captureFramerate", Json::Float(md.capture_framerate)),
        ("videoFramerate", Json::Float(md.video_framerate)),
    ])
}

fn data(md: &MeasuredData, rtf_to_text: fn(&str) -> String) -> Json {
    let time_unit = md.units.time_symbol;
    let keyframes = md
        .keyframes
        .iter()
        .map(|k| {
            Json::object(vec![
                ("name", Json::text(&k.name)),
                ("timeUnit", Json::text(time_unit)),
                ("time", Json::Float(k.time)),
                ("comment", Json::text(&rtf_to_text(&k.comment))),
            ])
        })
        .collect();
    let positions = md
        .positions
        .iter()
        .map(|p| {
            let value = Json::Array(vec![Json::Float(p.point.x), Json::Float(p.point.y)]);
            measure(&p.name, p.time, md.units.length_symbol, time_unit, value)
        })
        .collect();
    let times = md.times.iter().map(|t| time_entry(t, time_unit)).collect();
    let timeseries = md
        .timeseries
        .iter()
        .map(|s| series(s, md.units.length_symbol, time_unit))
        .collect();
    Json::object(vec![
        ("keyframes", Json::Array(keyframes)),
        ("positions", Json::Array(positions)),
        (
            "distances",
            Json::Array(scalars(&md.distances, md.units.length_symbol, time_unit)),
        ),
        (
            "angles",
            Json::Array(scalars(&md.angles, md.units.angle_symbol, time_unit)),
        ),
        ("times", Json::Array(times)),
        ("timeseries", Json::Array(timeseries)),
    ])
}

fn scalars(list: &[Scalar], data_unit: &str, time_unit: &str) -> Vec<Json> {
    list.iter()
        .map(|s| measure(&s.name, s.time, data_unit, time_unit, Json::Float(s.value)))
        .collect()
}

fn measure(name: &str, time: f32, data_unit: &str, time_unit: &str, value: Json) -> Json {
    Json::object(vec![
        ("name", Json::text(name)),
        ("timeUnit", Json::text(time_unit)),
        ("dataUnit", Json::text(data_unit)),
        ("time", Json::Float(time)),
        ("value", value),
    ])
}
