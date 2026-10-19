//! Port of `Kinovea.Video/Caching/PlayerState.cs`, `PlayerAction.cs`, `SeekProgress.cs`,
//! `WorkingZoneRequest.cs`, `OpenVideoResult.cs`, `RecordingResult.cs` and `VideoExportResult.cs`.

use crate::capabilities::CacheLoadMode;
use kinovea_core::video_section::VideoSection;
use std::fmt;

kinovea_core::text_enum!(
    /// `PlayerAction`: the kind of presentation request the player makes.
    PlayerAction [Playback, Timestamp, StepForward, StepBackward]
);

/// `PlayerState`: immutable snapshot of what the player asked for and when.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    /// Generation counter, incremented on each player state change.
    pub id: i32,
    pub action: PlayerAction,
    /// Whether the decoder must be relocated before control returns to the player.
    pub synchronous_fulfill: bool,
    /// Playback start, timestamp target, or current frame, depending on `action`.
    pub reference_timestamp: i64,
    /// Interval between frames during playback, in milliseconds.
    pub playback_frame_interval: f64,
    /// Interval between presented frames during playback, in milliseconds.
    pub refresh_interval: f64,
}

impl PlayerState {
    pub fn new(
        id: i32,
        action: PlayerAction,
        synchronous_fulfill: bool,
        reference_timestamp: i64,
        playback_frame_interval: f64,
        refresh_interval: f64,
    ) -> Self {
        Self {
            id,
            action,
            synchronous_fulfill,
            reference_timestamp,
            playback_frame_interval,
            refresh_interval,
        }
    }

    /// `PlayerState.MakeInvalid()`.
    pub fn make_invalid() -> Self {
        Self::new(-1, PlayerAction::Timestamp, true, 0, 0.0, 0.0)
    }
}

impl fmt::Display for PlayerState {
    /// Same layout as the C# `ToString`, including `True`/`False` for the flag.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sync = if self.synchronous_fulfill {
            "True"
        } else {
            "False"
        };
        write!(
            f,
            "#{} Mode: {}. Sync: {}. Ref: [~{}].",
            self.id,
            self.action.name(),
            sync,
            self.reference_timestamp
        )
    }
}

/// `SeekProgress`: feedback on a seek, covering the section decoded so far.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeekProgress {
    pub version: i64,
    pub section: VideoSection,
}

impl SeekProgress {
    pub fn new(version: i64, section: VideoSection) -> Self {
        Self { version, section }
    }

    /// `SeekProgress.MakeEmpty()`.
    pub fn empty() -> Self {
        Self::new(-1, VideoSection::empty())
    }

    pub fn in_progress(&self) -> bool {
        !self.section.is_empty()
    }
}

/// `WorkingZoneRequest`: the zone to load and the policy for the full cache.
#[derive(Debug, Clone, Copy)]
pub struct WorkingZoneRequest {
    pub working_zone: VideoSection,
    pub cache_load_mode: CacheLoadMode,
    /// Maximum memory allowed for the full cache.
    pub max_memory_mb: i32,
}

impl WorkingZoneRequest {
    pub fn new(
        working_zone: VideoSection,
        cache_load_mode: CacheLoadMode,
        max_memory_mb: i32,
    ) -> Self {
        Self {
            working_zone,
            cache_load_mode,
            max_memory_mb,
        }
    }
}
