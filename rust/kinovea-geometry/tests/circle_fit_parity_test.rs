//! Circle fit parity against `best_fit_circle` of the kinematics circular trajectory.

use kinovea_fixtures::fixtures_root;
use kinovea_geometry::{fit_circle, PointF};

#[test]
fn fit_of_the_filtered_circular_trajectory_matches_best_fit_circle() {
    let dir = fixtures_root().join("kinematics/circular");
    let csv = std::fs::read_to_string(dir.join("linear.csv")).unwrap();
    let mut lines = csv.lines();
    let header: Vec<&str> = lines.next().unwrap().split(',').collect();
    let (ix, iy) = (
        header.iter().position(|h| *h == "X").unwrap(),
        header.iter().position(|h| *h == "Y").unwrap(),
    );
    let points: Vec<PointF> = lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let cols: Vec<&str> = l.split(',').collect();
            PointF::new(cols[ix].parse().unwrap(), cols[iy].parse().unwrap())
        })
        .collect();
    assert_eq!(points.len(), 180);

    let json = std::fs::read_to_string(dir.join("filtering.json")).unwrap();
    let value = |key: &str| -> f64 {
        let start = json.find(key).unwrap() + key.len();
        let rest = &json[start..];
        let num: String = rest
            .trim_start_matches([':', ' ', '"'])
            .chars()
            .take_while(|c| c.is_ascii_digit() || matches!(c, '-' | '.' | 'E' | 'e' | '+'))
            .collect();
        num.parse().unwrap()
    };
    let circle = fit_circle(&points);
    assert!(
        (f64::from(circle.center.x) - value("\"cx\"")).abs() < 1e-3,
        "{circle:?}"
    );
    assert!(
        (f64::from(circle.center.y) - value("\"cy\"")).abs() < 1e-3,
        "{circle:?}"
    );
    assert!(
        (f64::from(circle.radius) - value("\"radius\"")).abs() < 1e-3,
        "{circle:?}"
    );
}
