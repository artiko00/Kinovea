use super::*;

#[test]
fn the_kva_of_a_case_loads_from_the_repository_root() {
    let m = metadata("tests/cases/render/inputs/geometry.kva");
    assert!(!m.keyframes.is_empty());
}
