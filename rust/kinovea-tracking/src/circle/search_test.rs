use super::{closest, hough_params, Scan};
use crate::hough::{hough_circles, HoughCircle};
use crate::pixels::{to_gray, Roi};
use crate::synthetic::Scene;

#[test]
fn highest_vote_level_with_circles_wins() {
    let frame = Scene {
        width: 160,
        height: 120,
        background: [40, 60, 80],
        gx: 8,
        gy: 4,
        color: [230, 230, 230],
        disc: true,
        size: 10,
    }
    .render((200, 200));
    let gray = to_gray(&frame, Roi::new(0, 0, 160, 120));
    let scan = Scan {
        gray: &gray,
        origin: (0, 0),
        last: (50.0, 50.0),
    };
    let found = scan.best_at(9.0, 10.0, 4).expect("the disc has circles");
    assert!((found.x - 50.0).abs() <= 1.0 && (found.y - 50.0).abs() <= 1.0);
    assert!(found.votes % 2 == 0 && found.votes <= 50);
    for votes in (found.votes + 2..=50).step_by(2) {
        let params = hough_params(gray.width, votes, 9.0, 10.0);
        let circles = hough_circles(&gray.data, 160, 120, &params);
        assert!(circles.is_empty(), "level {votes} has circles");
    }
}

#[test]
fn nearest_circle_wins_in_image_coordinates() {
    let circles = [
        HoughCircle {
            x: 10.0,
            y: 10.0,
            radius: 5.0,
        },
        HoughCircle {
            x: 40.0,
            y: 30.0,
            radius: 7.0,
        },
    ];
    let found = closest(&circles, (20, 20), (62.0, 49.0));
    assert_eq!(
        found,
        Some(HoughCircle {
            x: 60.0,
            y: 50.0,
            radius: 7.0
        })
    );
}
