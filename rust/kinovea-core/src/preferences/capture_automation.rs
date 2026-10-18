//! Photofinish and trigger automation settings of the `Capture` group.

use super::enums::CaptureTriggerAction;
use super::fields::xml_struct;
use super::helpers::Fixed3;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhotofinishConfiguration {
    pub threshold_height: i32,
    pub consolidation_height: i32,
    pub output_height: i32,
    pub waterfall: bool,
    pub waterfall_flush_height: i32,
}

impl Default for PhotofinishConfiguration {
    fn default() -> Self {
        Self {
            threshold_height: 16,
            consolidation_height: 2,
            output_height: 1000,
            waterfall: true,
            waterfall_flush_height: 100,
        }
    }
}

xml_struct!(PhotofinishConfiguration {
    threshold_height => "ThresholdHeight", consolidation_height => "ConsolidationHeight",
    output_height => "OutputHeight", waterfall => "Waterfall", waterfall_flush_height => "WaterfallFlushHeight",
});

#[derive(Debug, Clone, PartialEq)]
pub struct CaptureAutomationConfiguration {
    pub enable_audio_trigger: bool,
    pub audio_input_device: String,
    pub audio_trigger_threshold: Fixed3,
    pub enable_udp_trigger: bool,
    pub udp_port: i32,
    pub trigger_quiet_period: Fixed3,
    pub trigger_action: CaptureTriggerAction,
    pub enable_auto_numbering: bool,
    pub ignore_overwrite_warning: bool,
    pub default_trigger_armed: bool,
}

impl Default for CaptureAutomationConfiguration {
    fn default() -> Self {
        Self {
            enable_audio_trigger: false,
            audio_input_device: Uuid::nil().to_string(),
            audio_trigger_threshold: Fixed3(0.9),
            enable_udp_trigger: false,
            udp_port: 8875,
            trigger_quiet_period: Fixed3(0.0),
            trigger_action: CaptureTriggerAction::RecordVideo,
            enable_auto_numbering: true,
            ignore_overwrite_warning: false,
            default_trigger_armed: true,
        }
    }
}

xml_struct!(CaptureAutomationConfiguration {
    enable_audio_trigger => "EnableAudioTrigger", audio_input_device => "AudioInputDevice",
    audio_trigger_threshold => "AudioTriggerThreshold", enable_udp_trigger => "EnableUDPTrigger",
    udp_port => "UDPPort", trigger_quiet_period => "AudioQuietPeriod", trigger_action => "TriggerAction",
    enable_auto_numbering => "EnableAutoNumbering", ignore_overwrite_warning => "IgnoreOverwriteWarning",
    default_trigger_armed => "DefaultTriggerArmed",
});
