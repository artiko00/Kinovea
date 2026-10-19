use crate::paint::PaintContext;
use crate::viewport::ImageToViewport;
use kinovea_metadata::{Fading, Metadata};

#[test]
fn context_holds_the_draw_parameters() {
    let viewport = ImageToViewport::new(crate::viewport::Point { x: 0, y: 0 }, 1.0);
    let metadata = Metadata::default();
    let fading = Fading::default();
    let ctx = PaintContext {
        viewport: &viewport,
        timestamp: 7,
        selected: false,
        metadata: &metadata,
        default_fading: &fading,
        distorter: None,
        camera: None,
    };
    assert_eq!(ctx.timestamp, 7);
    assert!(ctx.distorter.is_none() && ctx.camera.is_none());
}
