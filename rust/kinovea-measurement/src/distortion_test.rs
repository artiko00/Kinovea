use crate::distortion::{DistortionHelper, DistortionParameters};
use kinovea_geometry::Size;

#[test]
fn re_exports_are_usable_together() {
    let size = Size::new(640, 480);
    let mut helper = DistortionHelper::default();
    helper.initialize(DistortionParameters::defaults(size), size);
    assert!(helper.is_initialized());
}
