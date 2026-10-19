use super::match_circle;
use crate::synthetic::Scene;
use kinovea_geometry::Size;

fn scene(size: i64) -> Scene {
    Scene {
        width: 160,
        height: 120,
        background: [40, 60, 80],
        gx: 8,
        gy: 4,
        color: [230, 230, 230],
        disc: true,
        size,
    }
}

#[test]
fn disc_is_matched_near_its_center_with_votes() {
    let frame = scene(10).render((200, 200));
    let found = match_circle(&frame, 10.0, (48.0, 48.0), Size::new(80, 80)).expect("a disc");
    assert!((found.x - 50.0).abs() <= 1.0 && (found.y - 50.0).abs() <= 1.0);
    assert!(found.votes > 0);
}

#[test]
fn uniform_background_gives_no_candidate() {
    let frame = scene(0).render((0, 0));
    assert_eq!(
        match_circle(&frame, 10.0, (50.0, 50.0), Size::new(80, 80)),
        None
    );
}
