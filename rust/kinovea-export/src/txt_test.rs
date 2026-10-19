use crate::case::{cases_with_fixture, fixture_text, measured};
use crate::csv::CsvFormat;
use crate::measured_data::Timeseries;
use crate::txt::write_txt_trajectory;
use kinovea_geometry::PointF;

#[test]
fn matches_the_oracle_byte_for_byte() {
    let cases = cases_with_fixture("trajectory.txt");
    assert!(!cases.is_empty());
    for name in cases {
        let want = fixture_text(&name, "trajectory.txt");
        let got = write_txt_trajectory(&measured(&name), &CsvFormat::new('.'));
        assert_eq!(got, want, "{name}");
    }
}

#[test]
fn comma_decimal_and_nan_as_text() {
    let mut md = measured("minimal");
    md.timeseries = vec![Timeseries {
        name: "A".to_owned(),
        times: vec![0.5],
        data: vec![(
            "0".to_owned(),
            vec![PointF {
                x: f32::NAN,
                y: 2.0,
            }],
        )],
    }];
    let got = write_txt_trajectory(&md, &CsvFormat::new(','));
    assert_eq!(
        got,
        concat!(
            "#Kinovea Trajectory data export\r\n",
            "#T X Y\r\n",
            "\r\n",
            "# A\r\n",
            "0,5 NaN 2\r\n",
            "\r\n",
        )
    );
}
