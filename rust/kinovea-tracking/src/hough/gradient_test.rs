use super::sobel;

#[test]
fn vertical_step_gives_horizontal_derivative_with_replicated_border() {
    // 5x3 image: columns 0..=1 dark, columns 2..=4 bright.
    let data: Vec<u8> = (0..15).map(|i| if i % 5 >= 2 { 100 } else { 0 }).collect();
    let g = sobel(&data, 5, 3);
    assert_eq!(&g.dx[5..10], &[0, 400, 400, 0, 0]);
    assert!(g.dy.iter().all(|&v| v == 0));
}

#[test]
fn horizontal_step_gives_vertical_derivative() {
    // 3x5 image: rows 0..=1 dark, rows 2..=4 bright.
    let data: Vec<u8> = (0..15).map(|i| if i / 3 >= 2 { 100 } else { 0 }).collect();
    let g = sobel(&data, 3, 5);
    assert_eq!(g.dy[4], 400);
    assert!(g.dx.iter().all(|&v| v == 0));
}
