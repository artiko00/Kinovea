#[test]
fn recording_module_is_declared() {
    assert!(module_path!().contains("recording"));
}
