use crate::json_render::render;
use crate::json_series::{series, time_entry};
use crate::measured_data::{TimeEntry, TimeSection, Timeseries};
use kinovea_geometry::PointF;

fn section(name: &str, start: f32) -> TimeSection {
    TimeSection {
        name: name.to_owned(),
        start,
        ..Default::default()
    }
}

#[test]
fn stopwatch_sections_repeat_their_members_under_the_same_names() {
    let entry = TimeEntry {
        name: "Chrono".to_owned(),
        sections: vec![section("a", 1.0), section("", 2.0)],
        has_tags: false,
        visible_columns: Vec::new(),
    };
    let text = render(&time_entry(&entry, "s"));
    assert_eq!(text.matches("\"name\"").count(), 2);
    assert!(text.contains("\"name\": \"Chrono a\""));
    assert!(text.contains("\"name\": \"Chrono\""));
    assert_eq!(text.matches("\"start\"").count(), 2);
}

#[test]
fn trajectory_is_a_time_array_and_one_pair_list_per_point() {
    let s = Timeseries {
        name: "Track".to_owned(),
        times: vec![0.0, 0.5],
        data: vec![("0".to_owned(), vec![PointF::new(1.0, 2.0)])],
    };
    let text = render(&series(&s, "px", "s"));
    assert!(
        text.contains("\"time\": [\n    0.0,\n    0.5\n  ]"),
        "{text}"
    );
    assert!(
        text.contains(
            "\"data\": {\n    \"0\": [\n      [\n        1.0,\n        2.0\n      ]\n    ]\n  }"
        ),
        "{text}"
    );
}
