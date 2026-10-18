//! Butterworth cutoff selection and filtered series against the kinematics fixtures.

mod support;
use support::*;

use kinovea_kinematics::butterworth_filter;
use kinovea_kinematics::Kinematics::*;

#[test]
fn cutoff_indices_and_filtered_series_match() {
    for name in ["linear", "circular", "winter"] {
        let (header, rows) = series(name, "linear.csv");
        let json = filtering_json(name);
        let fps: f64 = json_value(&json, "fps").parse().unwrap();
        assert_eq!(json_value(&json, "can_filter"), "true");
        for (raw, filtered, key) in [(XRaw, X, "x_cutoff_index"), (YRaw, Y, "y_cutoff_index")] {
            let want_index: usize = json_value(&json, key).parse().unwrap();
            let f = butterworth_filter(&column(&header, &rows, raw), fps, 100).unwrap();
            assert_eq!(f.best_cutoff_index, Some(want_index), "{name} {key}");
            let want = column(&header, &rows, filtered);
            for (i, (g, w)) in f.results[want_index].data.iter().zip(&want).enumerate() {
                assert_cell(*g, *w, &format!("{name} {filtered}[{i}]"));
            }
        }
    }
}
