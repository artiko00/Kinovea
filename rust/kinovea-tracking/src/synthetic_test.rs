use crate::synthetic::Scene;

fn scene(disc: bool) -> Scene {
    Scene {
        width: 8,
        height: 6,
        background: [40, 60, 80],
        gx: 8,
        gy: 4,
        color: [230, 230, 230],
        disc,
        size: 2,
    }
}

#[test]
fn background_follows_the_integer_gradient() {
    let frame = scene(false).render((-100, -100));
    // (x * 8 + y * 4) / 16 at (7, 5) is 76 / 16 = 4.
    assert_eq!(frame.pixel(7, 5), Some([44, 64, 84, 255]));
    assert_eq!(frame.pixel(0, 0), Some([40, 60, 80, 255]));
}

#[test]
fn a_square_on_the_pixel_grid_covers_whole_pixels() {
    // Center (4, 3) px: the 2 px square covers pixels 3 and 4 on both axes.
    let frame = scene(false).render((16, 12));
    assert_eq!(frame.pixel(3, 2), Some([230, 230, 230, 255]));
    assert_eq!(frame.pixel(4, 3), Some([230, 230, 230, 255]));
    // Background at (5, 3): (5 * 8 + 3 * 4) / 16 = 3.
    assert_eq!(frame.pixel(5, 3), Some([43, 63, 83, 255]));
}

#[test]
fn a_half_pixel_offset_blends_the_edge_pixels() {
    // Center (4.5, 3) px: pixel 5 is half covered, (bg * 8 + 230 * 8) / 16.
    let frame = scene(false).render((18, 12));
    let [b, _, _, _] = frame.pixel(5, 3).unwrap();
    assert_eq!(i32::from(b), (43 * 8 + 230 * 8) / 16);
}

#[test]
fn a_disc_excludes_the_corners_of_its_box() {
    let frame = scene(true).render((16, 12));
    assert_eq!(frame.pixel(4, 3), Some([230, 230, 230, 255]));
    assert_ne!(frame.pixel(2, 1), Some([230, 230, 230, 255]));
}
