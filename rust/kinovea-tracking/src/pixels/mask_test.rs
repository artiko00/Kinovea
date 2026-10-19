use crate::pixels::mask::ellipse_mask;
use kinovea_fixtures::fixtures_root;

/// Rows of `template-mask/mask.csv` (`row,values`), 255 where the original mask is set.
fn fixture_rows() -> Vec<Vec<u8>> {
    let path = fixtures_root().join("tracking/template-mask/mask.csv");
    let text = std::fs::read_to_string(path).unwrap();
    text.lines()
        .skip(1)
        .map(|line| {
            let (_, values) = line.split_once(',').unwrap();
            values
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn mask_has_the_block_size_and_binary_values() {
    let m = ellipse_mask(21, 17);
    assert_eq!((m.width, m.height), (21, 17));
    assert!(m.data.iter().all(|&v| v == 0 || v == 255));
}

#[test]
fn mask_matches_the_gdi_fixture_except_the_known_pixel() {
    let want = fixture_rows();
    assert_eq!(want.len(), 17);
    let got = ellipse_mask(21, 17);
    let mut differences = Vec::new();
    for (y, row) in want.iter().enumerate() {
        assert_eq!(row.len(), 21);
        for (x, &value) in row.iter().enumerate() {
            if got.at(x as u32, y as u32) != value {
                differences.push((x, y));
            }
        }
    }
    // GDI+ leaves (4, 2) empty although the exact ellipse covers its corner (boundary x = 3.73).
    assert_eq!(differences, vec![(4, 2)]);
}
