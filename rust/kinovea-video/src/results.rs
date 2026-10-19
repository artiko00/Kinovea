//! Port of `Kinovea.Video/OpenVideoResult.cs`, `RecordingResult.cs` and `VideoExportResult.cs`.

/// `OpenVideoResult`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenVideoResult {
    Success,
    UnknownError,
    NotSupported,
    /// C# `FileNotOpenned` (sic).
    FileNotOpened,
    StreamInfoNotFound,
    VideoStreamNotFound,
    CodecNotFound,
    CodecNotOpened,
    CodecNotSupported,
    Cancelled,
    EmptyWatcher,
}

/// `RecordingResult`, returned by capture recording.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingResult {
    Success,
    UnknownError,
    MuxerNotFound,
    MuxerParametersNotAllocated,
    EncoderNotFound,
    VideoStreamNotCreated,
    EncoderParametersNotSet,
    EncoderNotOpened,
    FileNotOpened,
    FileHeaderNotWritten,
    InputFrameNotAllocated,
}

/// `VideoExportResult`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoExportResult {
    Success,
    UnknownError,
    Cancelled,
    InputError,
    /// C# `FFMpegNotFound`.
    FfmpegNotFound,
    /// C# `FFMpegNotStarted`.
    FfmpegNotStarted,
    /// C# `FFMpegError`.
    FfmpegError,
}
