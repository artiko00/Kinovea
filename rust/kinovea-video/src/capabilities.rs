//! Port of `Kinovea.Video/VideoReaders/VideoCapabilities.cs` (flags), `Caching/VideoDecodingMode.cs`,
//! `Caching/CacheLoadMode.cs`, the capability shortcuts and `CanSwitchDecodingMode` of `VideoReader.cs`.

use std::ops::BitOr;

/// One `const fn` per shortcut, each reading its flag with `has`.
macro_rules! accessors {
    ($($name:ident => $flag:ident),* $(,)?) => {
        $(pub const fn $name(self) -> bool {
            self.has(Self::$flag)
        })*
    };
}

/// `[Flags] enum VideoCapabilities`. Values match the C# constants so they can be compared with
/// the oracle output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VideoCapabilities(u32);

impl VideoCapabilities {
    pub const NONE: Self = Self(0);
    pub const CAN_DECODE_ON_DEMAND: Self = Self(1);
    pub const CAN_PRE_BUFFER: Self = Self(2);
    pub const CAN_CACHE: Self = Self(4);
    pub const CAN_CHANGE_WORKING_ZONE: Self = Self(8);
    pub const CAN_CHANGE_ASPECT_RATIO: Self = Self(16);
    pub const CAN_CHANGE_DEINTERLACING: Self = Self(32);
    pub const CAN_CHANGE_VIDEO_DURATION: Self = Self(64);
    pub const CAN_CHANGE_FRAME_RATE: Self = Self(128);
    pub const CAN_CHANGE_IMAGE_ROTATION: Self = Self(256);
    pub const CAN_CHANGE_DEMOSAICING: Self = Self(512);
    pub const CAN_STABILIZE: Self = Self(1024);

    pub const fn bits(self) -> u32 {
        self.0
    }

    /// `(Flags & flag) != 0`.
    const fn has(self, flag: Self) -> bool {
        self.0 & flag.0 != 0
    }

    accessors! {
        can_change_working_zone => CAN_CHANGE_WORKING_ZONE,
        can_decode_on_demand => CAN_DECODE_ON_DEMAND,
        can_pre_buffer => CAN_PRE_BUFFER,
        can_cache => CAN_CACHE,
        can_change_aspect_ratio => CAN_CHANGE_ASPECT_RATIO,
        can_change_image_rotation => CAN_CHANGE_IMAGE_ROTATION,
        can_change_demosaicing => CAN_CHANGE_DEMOSAICING,
        can_change_deinterlacing => CAN_CHANGE_DEINTERLACING,
        can_stabilize => CAN_STABILIZE,
    }

    /// `VideoReader.CanSwitchDecodingMode`. Not-initialized is always allowed.
    pub const fn can_switch_decoding_mode(self, mode: VideoDecodingMode) -> bool {
        match mode {
            VideoDecodingMode::NotInitialized => true,
            VideoDecodingMode::OnDemand => self.can_decode_on_demand(),
            VideoDecodingMode::PreBuffering => self.can_pre_buffer(),
            VideoDecodingMode::Caching => self.can_cache(),
        }
    }
}

impl BitOr for VideoCapabilities {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// `VideoDecodingMode`: how the reader currently serves frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoDecodingMode {
    NotInitialized,
    OnDemand,
    PreBuffering,
    Caching,
}

/// `CacheLoadMode`: full-cache policy when the working zone changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheLoadMode {
    Keep,
    Reload,
    DoNotLoad,
}
