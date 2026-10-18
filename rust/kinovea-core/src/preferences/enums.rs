//! Enumerations stored by name in `Preferences.xml` (ports of `Kinovea.Services/Types/*`).

use super::names::named_enum;
pub use super::names::{NamedEnum, UnknownName};
use std::fmt;
use std::str::FromStr;

named_enum!(
    ExplorerThumbSize,
    "thumbnail size",
    [ExtraLarge, Large, Medium, Small, ExtraSmall]
);
named_enum!(FileSortAxis, "file sort axis", [Name, Date, Size]);
named_enum!(
    FileProperty,
    "file property",
    [Size, Framerate, Duration, CreationTime, HasKva, IsImage]
);
named_enum!(
    CsvDecimalSeparator,
    "CSV decimal separator",
    [System, Point, Comma]
);
named_enum!(ExportSpace, "export space", [WorldSpace, ImageSpace]);
named_enum!(
    ImageAspectRatio,
    "aspect ratio",
    [Auto, Force43, Force169, ForcedSquarePixels]
);
named_enum!(ImageFormat, "image format", [JPG, PNG, BMP]);
named_enum!(VideoContainer, "video container", [MP4, MKV, AVI]);
named_enum!(VideoCodec, "video codec", [MJPEG, H264, H265]);
named_enum!(
    TimelineJumpUnit,
    "timeline jump unit",
    [Second, Millisecond, Frame, SnapPercent]
);
named_enum!(CaptureRecordingMode, "recording mode", [Delay, Scheduled]);
named_enum!(
    EncodingQuality,
    "encoding quality",
    [PerceptuallyLossless, High, Good, Medium]
);
named_enum!(EncodingSpeed, "encoding speed", [Fast, Medium, Slow]);
named_enum!(
    /// `[Flags]` in C#; only the named combinations are supported as text.
    KvaExportFlags, "KVA export flags",
    [Basics, Drawings, Calibration, VideoSpecific, Full, DefaultCaptureRecording]
);
named_enum!(
    TrackingAlgorithm,
    "tracking algorithm",
    [Correlation, Circle]
);
named_enum!(CameraMotionFeatureType, "feature type", [ORB, SIFT]);
named_enum!(CaptureCodec, "capture codec", [MJPEG, RAW]);
named_enum!(
    CaptureTriggerAction,
    "trigger action",
    [RecordVideo, SaveSnapshot]
);
named_enum!(
    MeasureLabelType,
    "measure label type",
    [
        None,
        Name,
        Clock,
        RelativeTime,
        Frame,
        Position,
        RelativePosition,
        Distance,
        TravelDistance,
        HorizontalTravel,
        VerticalTravel,
        AngleToHorizontal,
        AngleToVertical,
        Speed,
        HorizontalVelocity,
        VerticalVelocity,
        Acceleration,
        HorizontalAcceleration,
        VerticalAcceleration,
        Center,
        Radius,
        Diameter,
        Circumference,
        Count,
        CountReverse,
        CadenceInstant,
        CadenceAverage,
        CadenceVariation,
        PeriodInstant,
        PeriodAverage,
        PointDeviation,
        LineDeviation,
        HorizontalDeviation,
        VerticalDeviation,
        CircularDeviation,
        TotalHorizontalDisplacement,
        TotalVerticalDisplacement
    ]
);
