//! Port of `Kinovea.Pipeline/Frame.cs`, `ImageDescriptor.cs`, `Interfaces/IFrameProducer.cs`
//! and `Events/FrameProducedEventArgs.cs`. The C# event becomes a handler installed on the
//! producer; like the event, it runs on the producer thread. Arrival instants are passed in
//! so frequency measurements can be reproduced.

use std::collections::TryReserveError;
use std::time::Instant;

/// Receives each produced frame: payload bytes, payload length and arrival instant.
pub type FrameHandler = Box<dyn FnMut(&[u8], usize, Instant) + Send>;

/// Camera side of the pipeline. The producer calls its handler once per frame.
pub trait FrameProducer {
    /// Installs the handler (`Some`) or removes it (`None`), like `+=` and `-=` on the C# event.
    fn set_frame_handler(&mut self, handler: Option<FrameHandler>);
}

/// Pre-allocated byte buffer for one frame. The payload may be shorter than the buffer.
#[derive(Debug)]
pub struct Frame {
    buffer: Vec<u8>,
    payload_length: usize,
}

impl Frame {
    /// Allocates a zeroed buffer, reporting allocation failure instead of aborting.
    pub fn try_new(buffer_size: usize) -> Result<Self, TryReserveError> {
        let mut buffer = Vec::new();
        buffer.try_reserve_exact(buffer_size)?;
        buffer.resize(buffer_size, 0);
        Ok(Self {
            buffer,
            payload_length: 0,
        })
    }

    pub fn payload(&self) -> &[u8] {
        &self.buffer[..self.payload_length]
    }

    /// Copies `payload_length` bytes when they fit. An oversized payload leaves the frame
    /// unchanged, as `WriteSlot` does in C#.
    pub fn fill(&mut self, bytes: &[u8], payload_length: usize) {
        if payload_length <= self.buffer.len() {
            self.buffer[..payload_length].copy_from_slice(&bytes[..payload_length]);
            self.payload_length = payload_length;
        }
    }
}

/// Pixel format of a camera image. Mirrors `Kinovea.Services.ImageFormat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    None,
    Rgb24,
    Rgb32,
    Y800,
    Jpeg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageDescriptor {
    pub format: ImageFormat,
    pub width: usize,
    pub height: usize,
    pub top_down: bool,
    pub buffer_size: usize,
}

impl ImageDescriptor {
    /// The C# `Invalid` singleton. Equal by value, which only matches that singleton in practice.
    pub fn invalid() -> Self {
        Self {
            format: ImageFormat::None,
            width: 0,
            height: 0,
            top_down: true,
            buffer_size: 0,
        }
    }

    /// True when both descriptors exist, neither is invalid, and they describe the same layout.
    pub fn compatible(a: Option<&Self>, b: Option<&Self>) -> bool {
        let invalid = Self::invalid();
        match (a, b) {
            (Some(a), Some(b)) if *a != invalid && *b != invalid => a == b,
            _ => false,
        }
    }
}
