#[test]
fn style_module_is_declared() {
    assert!(module_path!().contains("style"));
}
