use crate::case::{check_optional_file, measured};
use crate::csv::CsvFormat;
use crate::csv_trajectory::write_csv_trajectory;
use crate::measured_data::Timeseries;
use kinovea_geometry::PointF;

#[test]
fn matches_the_oracle_byte_for_byte_or_writes_no_file() {
    check_optional_file("trajectory.csv", |md| {
        write_csv_trajectory(md, &CsvFormat::new('.'))
    });
}

#[test]
fn comma_decimal_uses_semicolons_between_cells() {
    let text = write_csv_trajectory(&measured("complete"), &CsvFormat::new(','));
    assert!(text.unwrap().starts_with(concat!(
        "\"Time\";\"Trajectory 1/0/X\";\"Trajectory 1/0/Y\"\r\n",
        "\"0\";\"-181\";\"-113\"\r\n",
        "\"0,04\";\"161\";\"-121\"\r\n",
    )));
}

#[test]
fn series_with_other_times_leave_empty_cells() {
    let mut md = measured("minimal");
    md.timeseries = vec![
        series("A", vec![0.5, 1.0], vec![point(1.0, 2.0), point(3.0, 4.0)]),
        series("B", vec![1.0], vec![point(5.0, 6.0)]),
    ];
    let text = write_csv_trajectory(&md, &CsvFormat::new('.'));
    assert_eq!(
        text.as_deref(),
        Some(concat!(
            "\"Time\",\"A/0/X\",\"A/0/Y\",\"B/0/X\",\"B/0/Y\"\r\n",
            "\"0.5\",\"1\",\"2\",,\r\n",
            "\"1\",\"3\",\"4\",\"5\",\"6\"\r\n",
        ))
    );
}

fn series(name: &str, times: Vec<f32>, points: Vec<PointF>) -> Timeseries {
    Timeseries {
        name: name.to_owned(),
        times,
        data: vec![("0".to_owned(), points)],
    }
}

fn point(x: f32, y: f32) -> PointF {
    PointF { x, y }
}
