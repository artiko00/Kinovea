use super::{FrameHandler, FramePipeline, FrameProducer, RingBufferError};

#[derive(Default)]
struct IdleProducer {
    handler: Option<FrameHandler>,
}

impl FrameProducer for IdleProducer {
    fn set_frame_handler(&mut self, handler: Option<FrameHandler>) {
        self.handler = handler;
    }
}

#[test]
fn rejects_a_capacity_that_is_not_a_power_of_two_without_binding_the_producer() {
    let mut producer = IdleProducer::default();
    let error = FramePipeline::new(&mut producer, vec![], 6, 8).err();
    assert_eq!(error, Some(RingBufferError::NotPowerOfTwo(6)));
    assert!(producer.handler.is_none());
}

#[test]
fn a_new_pipeline_has_no_drops_and_no_frequency_yet() {
    let mut producer = IdleProducer::default();
    let pipeline = FramePipeline::new(&mut producer, vec![], 4, 8).expect("pipeline");
    assert_eq!(pipeline.drops(), 0);
    assert_eq!(pipeline.frequency(), 0.0);
    pipeline.teardown(&mut producer);
    assert!(producer.handler.is_none());
}
