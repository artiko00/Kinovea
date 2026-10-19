#[test]
fn cpu_module_is_declared() {
    assert!(module_path!().contains("cpu"));
}
