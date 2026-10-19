//! Kinovea `export` crate: importers (OpenPose, TRC, SRT) and the measured-data collector with the
//! writers that serialize it.

pub mod rtf;

pub mod collect;
mod collect_drawings;
mod collect_series;
pub mod json;
mod json_render;
mod json_series;
pub mod measure;
pub mod measured_data;

#[cfg(test)]
mod case;
#[cfg(test)]
mod case_test;
#[cfg(test)]
mod collect_drawings_test;
#[cfg(test)]
mod collect_series_test;
#[cfg(test)]
mod collect_test;
#[cfg(test)]
mod json_render_test;
#[cfg(test)]
mod json_series_test;
#[cfg(test)]
mod json_test;
#[cfg(test)]
mod lib_test;
#[cfg(test)]
mod measure_angle_test;
#[cfg(test)]
mod measure_test;
#[cfg(test)]
mod measured_data_test;

pub mod import;
#[cfg(test)]
mod import_test;
pub mod openpose;
mod openpose_files;
#[cfg(test)]
mod openpose_files_test;
mod openpose_sort;
#[cfg(test)]
mod openpose_sort_test;
#[cfg(test)]
mod openpose_test;
pub mod srt;
#[cfg(test)]
mod srt_test;
mod srt_timecode;
#[cfg(test)]
mod srt_timecode_test;
pub mod trc;
#[cfg(test)]
mod trc_test;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
