//! Element mapping of the `Player` group, in the original write order.

use super::dom::Element;
use super::fields::xml_fields;
use super::helpers::XmlField;
use super::player::PlayerPreferences;

xml_fields!(PlayerPreferences {
    decimal_places => "DecimalPlaces", timecode_format => "TimecodeFormat", speed_unit => "SpeedUnit",
    acceleration_unit => "AccelerationUnit", angle_unit => "AngleUnit",
    angular_velocity_unit => "AngularVelocityUnit", angular_acceleration_unit => "AngularAccelerationUnit",
    custom_length_unit => "CustomLengthUnit", custom_length_abbreviation => "CustomLengthAbbreviation",
    cadence_unit => "CadenceUnit", csv_decimal_separator => "CSVDecimalSeparator", export_space => "ExportSpace",
    export_images_in_documents => "ExportImagesInDocuments", aspect_ratio => "AspectRatio",
    deinterlace_by_default => "DeinterlaceByDefault", interactive_frame_tracker => "InteractiveFrameTracker",
    working_zone_memory => "WorkingZoneMemory", show_cache_in_timeline => "ShowCacheInTimeline",
    sync_lock_speed => "SyncLockSpeed", sync_by_motion => "SyncByMotion", image_format => "ImageFormat",
    video_format => "VideoFormat", default_fading => "InfoFading", enable_pixel_filtering => "EnablePixelFiltering",
    draw_on_play => "DrawOnPlay", max_recent_colors => "MaxRecentColors",
    tracking_parameters => "TrackingParameters", enable_filtering => "EnableFiltering",
    enable_custom_tools_debug_mode => "EnableCustomToolsDebugMode", detect_image_sequences => "DetectImageSequences",
    preload_keyframes => "PreloadKeyframes", playback_kva => "PlaybackKVA", export_profile => "ExportProfile",
    kinogram => "Kinogram", lens_calibration => "LensCalibration", camera_motion => "CameraMotion",
    keyframe_presets => "KeyframePresets", pandoc_path => "PandocPath",
    side_by_side_horizontal => "SideBySideHorizontal", enable_frame_skipping => "EnableFrameSkipping",
    timeline_jump_small_size => "TimelineJumpSmallSize", timeline_jump_small_unit => "TimelineJumpSmallUnit",
    timeline_jump_large_size => "TimelineJumpLargeSize", timeline_jump_large_unit => "TimelineJumpLargeUnit",
    speed_label_factor => "SpeedLabelFactor", speed_label_framerate => "SpeedLabelFramerate",
    speed_label_interval => "SpeedLabelInterval", loop_playback => "LoopPlayback",
    enable_hardware_decoding => "EnableHardwareDecoding", enable_preview_scaling => "EnablePreviewScaling",
    enable_hardware_scaling => "EnableHardwareScaling",
});

impl XmlField for PlayerPreferences {
    fn write(&self, name: &str) -> Option<Element> {
        let mut e = Element::new(name);
        self.write_fields(&mut e);
        if let Some(colors) = self.recent_colors.write(self.max_recent_colors) {
            // The original writes the list right before MaxRecentColors.
            let at = e
                .children
                .iter()
                .position(|c| c.name == "MaxRecentColors")?;
            e.children.insert(at, colors);
        }
        Some(e)
    }
    fn read(&mut self, element: &Element) {
        self.read_fields(element);
        if let Some(c) = element.child("RecentColors") {
            self.recent_colors.read(c);
        }
    }
}
