use crate::container::{CacheAddResult, FramesContainer};
use crate::frame::VideoFrame;
use crate::prebuffer::{PreBuffer, PreBufferConfig};

fn frame(ts: i64, prev: i64) -> VideoFrame {
    VideoFrame::new(None, ts, prev)
}

#[test]
fn trait_methods_delegate_to_the_prebuffer() {
    let mut pb = PreBuffer::new(PreBufferConfig::default());
    assert_eq!(
        FramesContainer::add(&mut pb, frame(0, 0)),
        CacheAddResult::Added
    );
    FramesContainer::acquire_closest(&mut pb, 0);
    assert_eq!(FramesContainer::current(&pb).map(|f| f.timestamp), Some(0));
    assert_eq!(FramesContainer::snapshot(&pb).version, 1);
    assert_eq!(FramesContainer::timestamps(&pb), vec![0]);
}
