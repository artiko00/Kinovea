#[test]
fn compose_module_is_declared() {
    assert!(module_path!().contains("compose"));
}
