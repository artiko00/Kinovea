use crate::ncc::Map;
use crate::ncc_map::MinMax;

#[test]
fn min_max_loc_takes_the_first_extreme_in_row_major_order() {
    let map = Map {
        width: 2,
        height: 2,
        values: vec![0.5, 0.9, 0.9, 0.1],
    };
    assert_eq!(
        map.min_max_loc(),
        MinMax {
            min: 0.1,
            max: 0.9,
            min_loc: (1, 1),
            max_loc: (1, 0)
        }
    );
}

#[test]
fn min_max_loc_never_selects_nan() {
    let map = Map {
        width: 2,
        height: 1,
        values: vec![f32::NAN, 0.2],
    };
    let found = map.min_max_loc();
    assert_eq!((found.max, found.max_loc), (0.2, (1, 0)));
    assert_eq!((found.min, found.min_loc), (0.2, (1, 0)));
}

#[test]
fn at_reads_row_major_values() {
    let map = Map {
        width: 3,
        height: 2,
        values: vec![0.0, 0.1, 0.2, 1.0, 1.1, 1.2],
    };
    assert_eq!(map.at(2, 1), 1.2);
}
