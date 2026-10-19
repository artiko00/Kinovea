//! Inputs of `HitTest` (`point`, `timestamp`, `distorter`, `transformer`) and its result codes.

use crate::paint::{CameraTransform, Distorter};
use crate::renderer::Renderer;
use crate::viewport::ImageToViewport;
use kinovea_metadata::{Fading, Metadata};

/// What a point hits: nothing, the drawing as a whole, or one of its handles (numbered from 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hit {
    Miss,
    Body,
    Handle(u32),
}

impl Hit {
    /// The integers of the original `HitTest`: -1 miss, 0 body, n for handle n.
    pub fn code(self) -> i32 {
        match self {
            Hit::Miss => -1,
            Hit::Body => 0,
            Hit::Handle(n) => n as i32,
        }
    }
}

/// `HitTest` parameters plus what the original reads from its canvas: the text metrics that
/// `MeasureString` gives for labels, through `text`.
pub struct HitContext<'a> {
    pub viewport: &'a ImageToViewport,
    pub timestamp: i64,
    pub metadata: &'a Metadata,
    pub default_fading: &'a Fading,
    pub distorter: Option<&'a dyn Distorter>,
    pub camera: Option<&'a dyn CameraTransform>,
    pub text: &'a dyn Renderer,
}

#[cfg(test)]
#[path = "context_test.rs"]
mod context_test;
