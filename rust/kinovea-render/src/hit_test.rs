#[test]
fn hit_module_is_declared() {
    assert!(module_path!().contains("hit"));
}
