//! Tests of the seeded generator and the XML builders.

use super::gen::{drawing, Rng};
use kinovea_metadata::ids::parse_id;

#[test]
fn same_seed_gives_same_sequence() {
    let (mut a, mut b) = (Rng::new(7), Rng::new(7));
    let first: Vec<u32> = (0..32).map(|_| a.bits()).collect();
    let second: Vec<u32> = (0..32).map(|_| b.bits()).collect();
    assert_eq!(first, second);
}

#[test]
fn different_seeds_diverge() {
    assert_ne!(Rng::new(1).bits(), Rng::new(2).bits());
}

#[test]
fn int_stays_in_range() {
    let mut r = Rng::new(3);
    for _ in 0..1000 {
        assert!((-1..=9000).contains(&r.int(-1, 9000)));
    }
}

#[test]
fn drawing_builder_sets_id_and_name() {
    let e = drawing("Line", &mut Rng::new(5), Vec::new());
    assert_eq!(e.name, "Line");
    assert!(parse_id(e.attribute("id").unwrap()).is_some());
    assert_eq!(e.attribute("name"), Some("Drawing <1> & co"));
}
