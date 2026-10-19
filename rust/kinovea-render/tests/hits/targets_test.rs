use super::*;
use crate::load::metadata;

#[test]
fn keyframes_come_first_at_their_own_time_then_the_singletons_in_manager_order() {
    let m = metadata("tests/cases/render/inputs/geometry.kva");
    let all = entries(&m, 7);
    let first = &all[0];
    assert_eq!(first.key.container, "k0");
    assert_eq!(first.key.element, "CrossMark");
    assert_eq!(first.timestamp, m.keyframes[0].timestamp);
    let singletons: Vec<(&str, usize, &str, i64)> = all[all.len() - 4..]
        .iter()
        .map(|e| {
            (
                e.key.container.as_str(),
                e.key.index,
                e.key.element.as_str(),
                e.timestamp,
            )
        })
        .collect();
    assert_eq!(
        singletons,
        vec![
            ("singletons", 0, "Spotlights", 7),
            ("singletons", 1, "AutoNumbers", 7),
            ("singletons", 2, "CoordinateSystem", 7),
            ("singletons", 3, "TestGrid", 7),
        ]
    );
}

#[test]
fn singletons_are_always_ported() {
    let m = metadata("tests/cases/render/inputs/geometry.kva");
    let all = entries(&m, 0);
    assert!(all.iter().rev().take(4).all(|e| e.target.is_ported()));
}
