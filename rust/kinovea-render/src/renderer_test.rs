#[test]
fn renderer_module_is_declared() {
    assert!(module_path!().contains("renderer"));
}
