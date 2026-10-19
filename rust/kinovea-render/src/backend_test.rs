#[test]
fn backend_module_is_declared() {
    assert!(module_path!().contains("backend"));
}
