//! Tests for `decoding.rs`: the decoder enums and `DecodingJobPlan` with its C# text form.

use crate::decoding::{DecoderRelocation, DecodingJobPlan, FrameSkippingPolicy};

#[test]
fn enum_names_match_csharp_members() {
    assert_eq!(DecoderRelocation::Advance.name(), "Advance");
    assert_eq!(DecoderRelocation::None.name(), "None");
    assert_eq!(FrameSkippingPolicy::FarBehind.name(), "FarBehind");
}

#[test]
fn default_plan_targets_nothing() {
    let plan = DecodingJobPlan::default();
    assert_eq!(plan.target_timestamp, -1);
    assert!(!plan.target_acquired);
    assert!(!plan.request_fulfilled_in_planning);
    assert_eq!(plan.decoder_relocation, DecoderRelocation::None);
    assert!(!plan.resubmit_pending);
}

#[test]
fn plan_text_matches_csharp_tostring() {
    let plan = DecodingJobPlan {
        target_timestamp: 42,
        target_acquired: true,
        request_fulfilled_in_planning: false,
        decoder_relocation: DecoderRelocation::Seek,
        resubmit_pending: true,
    };
    assert_eq!(
        plan.to_string(),
        "DecodingJobPlan: Target: [~42]. Acquired: True. FulfilledInPlanning: False. \
         DecoderInitAction: Seek. Resubmit pending: True."
    );
}
