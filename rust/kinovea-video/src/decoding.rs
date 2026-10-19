//! Port of `Kinovea.Video/Caching/DecoderRelocation.cs`, `FrameSkippingPolicy.cs` and
//! `DecodingJobPlan.cs`. The planning logic that fills these lives with the decoders.

use std::fmt;

kinovea_core::text_enum!(
    /// `DecoderRelocation`: how the decoder must move before serving the next frame.
    DecoderRelocation [None, Advance, Seek]
);

kinovea_core::text_enum!(
    /// `FrameSkippingPolicy`: how far behind playback the decoder may skip frames.
    FrameSkippingPolicy [Normal, Behind, FarBehind]
);

/// `DecodingJobPlan`: what the decoder decided for one job.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodingJobPlan {
    pub target_timestamp: i64,
    pub target_acquired: bool,
    pub request_fulfilled_in_planning: bool,
    pub decoder_relocation: DecoderRelocation,
    pub resubmit_pending: bool,
}

impl Default for DecodingJobPlan {
    fn default() -> Self {
        Self {
            target_timestamp: -1,
            target_acquired: false,
            request_fulfilled_in_planning: false,
            decoder_relocation: DecoderRelocation::None,
            resubmit_pending: false,
        }
    }
}

impl fmt::Display for DecodingJobPlan {
    /// Same layout as the C# `ToString`, with `True`/`False` for the flags.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DecodingJobPlan: Target: [~{}]. Acquired: {}. FulfilledInPlanning: {}. \
             DecoderInitAction: {}. Resubmit pending: {}.",
            self.target_timestamp,
            csharp_bool(self.target_acquired),
            csharp_bool(self.request_fulfilled_in_planning),
            self.decoder_relocation.name(),
            csharp_bool(self.resubmit_pending)
        )
    }
}

fn csharp_bool(value: bool) -> &'static str {
    if value {
        "True"
    } else {
        "False"
    }
}
