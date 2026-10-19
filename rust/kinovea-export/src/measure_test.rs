use crate::measure::{Scope, TimeBase};
use crate::measured_data::ExportPrefs;
use kinovea_core::preferences::enums::ExportSpace;
use kinovea_core::timecode::TimecodeFormat;
use kinovea_geometry::{PointF, Size};
use kinovea_metadata::Metadata;

pub(crate) fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

pub(crate) fn prefs(format: TimecodeFormat, space: ExportSpace) -> ExportPrefs {
    ExportPrefs {
        export_space: space,
        timecode_format: format,
        producer: "test".to_owned(),
    }
}

/// 640x480 video with `per_frame` timestamps per frame and 40 ms frames.
pub(crate) fn metadata(per_frame: f64, origin: i64) -> Metadata {
    let mut m = Metadata::new(Size::new(640, 480));
    m.video.average_timestamps_per_frame = per_frame;
    m.video.baseline_frame_interval_ms = 40.0;
    m.video.time_origin = origin;
    m
}

#[test]
fn classic_time_rounds_frames_to_even() {
    let m = metadata(1000.0, 0);
    let p = prefs(TimecodeFormat::ClassicTime, ExportSpace::WorldSpace);
    let s = Scope {
        metadata: &m,
        prefs: &p,
    };
    assert!(
        close(s.time(1500, TimeBase::Absolute), 0.08),
        "1.5 frames is 2 frames"
    );
    assert!(
        close(s.time(500, TimeBase::Absolute), 0.0),
        "0.5 frames is 0 frames"
    );
    assert!(
        close(s.time(2500, TimeBase::Absolute), 0.08),
        "2.5 frames is 2 frames"
    );
}

#[test]
fn user_origin_subtracts_the_time_origin_but_absolute_does_not() {
    let m = metadata(1000.0, 1000);
    let p = prefs(TimecodeFormat::ClassicTime, ExportSpace::WorldSpace);
    let s = Scope {
        metadata: &m,
        prefs: &p,
    };
    assert!(close(s.time(2000, TimeBase::UserOrigin), 0.04));
    assert!(close(s.time(2000, TimeBase::Absolute), 0.08));
}

#[test]
fn normalized_format_is_read_as_classic_time() {
    let m = metadata(1000.0, 0);
    let p = prefs(TimecodeFormat::Normalized, ExportSpace::WorldSpace);
    let s = Scope {
        metadata: &m,
        prefs: &p,
    };
    assert!(close(s.time(1000, TimeBase::Absolute), 0.04));
}

#[test]
fn other_formats_scale_frames_and_milliseconds() {
    let m = metadata(1000.0, 0);
    let time = |format| {
        let p = prefs(format, ExportSpace::WorldSpace);
        Scope {
            metadata: &m,
            prefs: &p,
        }
        .time(2000, TimeBase::Absolute)
    };
    assert!(close(time(TimecodeFormat::Frames), 2.0));
    assert!(close(time(TimecodeFormat::Milliseconds), 80.0));
    assert!(close(time(TimecodeFormat::Microseconds), 80_000.0));
    assert!(close(
        time(TimecodeFormat::TenThousandthOfHours),
        80.0 / 360.0
    ));
    assert!(close(
        time(TimecodeFormat::HundredthOfMinutes),
        80.0 / 600.0
    ));
    assert!(
        close(time(TimecodeFormat::Timestamps), 2000.0),
        "raw timestamps ignore the origin"
    );
}

#[test]
fn world_space_maps_the_image_centre_to_the_origin() {
    let m = metadata(1000.0, 0);
    let world = prefs(TimecodeFormat::ClassicTime, ExportSpace::WorldSpace);
    let image = prefs(TimecodeFormat::ClassicTime, ExportSpace::ImageSpace);
    let centre = PointF::new(320.0, 240.0);
    let w = Scope {
        metadata: &m,
        prefs: &world,
    }
    .point(centre);
    assert!(close(w.x, 0.0) && close(w.y, 0.0));
    assert_eq!(
        Scope {
            metadata: &m,
            prefs: &image
        }
        .point(centre),
        centre
    );
}
