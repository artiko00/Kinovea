#[test]
fn helpers_are_declared() {
    assert!(module_path!().contains("helpers"));
}
