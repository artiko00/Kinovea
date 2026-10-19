//! Loads the `tracking` oracle cases and replays them on a Rust tracker the way
//! `Kinovea.FixtureGen/Areas/TrackingArea.cs` drives the original: reference point at frame 0 (the
//! path point divided by 4 with integer division), then one `track_step` per frame on the growing
//! timeline. Rows have the `track.csv` columns.

#[cfg(test)]
mod mod_test;

use kinovea_core::timed_point::TimedPoint;
use kinovea_fixtures::fixtures_root;
use kinovea_geometry::Size;
use kinovea_metadata::drawing::tracking_parameters::TrackingParameters;
use kinovea_tracking::synthetic::Scene;
use kinovea_tracking::tracker::Tracker;
use serde_json::Value;

pub struct Case {
    pub scene: Scene,
    /// Object centers per frame, in quarter pixels.
    pub path: Vec<(i64, i64)>,
    pub ts_per_frame: i64,
    pub circle: bool,
    pub parameters: TrackingParameters,
}

/// One `track.csv` row: frame, t, x, y, r, score, matched.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub frame: usize,
    pub t: i64,
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub score: f64,
    pub matched: bool,
}

/// The case names, from `tests/cases/tracking/*.json`.
pub fn names() -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(fixtures_root().join("../cases/tracking"))
        .unwrap()
        .filter_map(|e| e.unwrap().file_name().into_string().ok())
        .filter_map(|n| n.strip_suffix(".json").map(str::to_owned))
        .collect();
    names.sort();
    names
}

pub fn load(name: &str) -> Case {
    let path = fixtures_root().join(format!("../cases/tracking/{name}.json"));
    let root: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    case_from(&root["inputs"])
}

pub fn case_from(i: &Value) -> Case {
    let int = |k: &str, d: i64| i[k].as_i64().unwrap_or(d);
    let triple = |k: &str, d: [i32; 3]| {
        i[k].as_array()
            .map(|a| [0, 1, 2].map(|n| a[n].as_i64().unwrap() as i32))
            .unwrap_or(d)
    };
    let size = |k: &str, d: i32| {
        let a = i[k].as_array();
        a.map(|a| Size::new(a[0].as_i64().unwrap() as i32, a[1].as_i64().unwrap() as i32))
            .unwrap_or(Size::new(d, d))
    };
    let mut parameters = TrackingParameters::default();
    // Same order as the oracle: search window, then block window (each setter fits the other).
    parameters.set_search_window(size("searchWindow", 100));
    parameters.set_block_window(size("blockWindow", 20));
    parameters.similarity_threshold = i["similarity"]
        .as_f64()
        .unwrap_or(parameters.similarity_threshold);
    parameters.template_update_threshold = i["templateUpdate"]
        .as_f64()
        .unwrap_or(parameters.template_update_threshold);
    parameters.use_mask = i["useMask"].as_bool().unwrap_or(false);
    Case {
        scene: Scene {
            width: int("width", 160) as u32,
            height: int("height", 120) as u32,
            background: triple("background", [40, 60, 80]),
            gx: int("gx", 8) as i32,
            gy: int("gy", 4) as i32,
            color: triple("color", [230, 230, 230]),
            disc: i["shape"].as_str() == Some("disc"),
            size: int("size", 8),
        },
        path: i["path"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| (p[0].as_i64().unwrap(), p[1].as_i64().unwrap()))
            .collect(),
        ts_per_frame: int("tsPerFrame", 1),
        circle: i["tracker"].as_str() == Some("circle"),
        parameters,
    }
}

/// Runs `tracker` over the case frames. The score column is the step score for templates (1 for the
/// reference) and -1 for circles, as the oracle writes it.
pub fn run(case: &Case, tracker: &mut dyn Tracker) -> Vec<Row> {
    let mut timeline: Vec<TimedPoint> = Vec::new();
    let mut rows = Vec::new();
    for (frame, &center) in case.path.iter().enumerate() {
        let t = frame as i64 * case.ts_per_frame;
        let image = case.scene.render(center);
        let (point, matched, score) = if frame == 0 {
            let mut point = TimedPoint::new((center.0 / 4) as f32, (center.1 / 4) as f32, t);
            tracker.create_reference_point(&mut point, &image);
            (point, true, 1.0)
        } else {
            let step = tracker.track_step(&timeline, t, Some(&image));
            (step.point, step.matched, step.score)
        };
        timeline.push(point);
        let score = if case.circle { -1.0 } else { f64::from(score) };
        rows.push(Row {
            frame,
            t,
            x: f64::from(point.x),
            y: f64::from(point.y),
            r: f64::from(point.r),
            score,
            matched,
        });
    }
    rows
}

/// The original's `track.csv` for `name`.
pub fn expected(name: &str) -> Vec<Row> {
    let text = std::fs::read_to_string(fixtures_root().join(format!("tracking/{name}/track.csv")))
        .unwrap();
    text.lines()
        .skip(1)
        .map(|l| {
            let c: Vec<&str> = l.split(',').collect();
            let f = |n: usize| c[n].parse::<f64>().unwrap();
            Row {
                frame: c[0].parse().unwrap(),
                t: c[1].parse().unwrap(),
                x: f(2),
                y: f(3),
                r: f(4),
                score: f(5),
                matched: c[6] == "1",
            }
        })
        .collect()
}
