//! Kinovea `export` crate. Behavior is added by later OpenSpec changes.

pub mod rtf;

#[cfg(test)]
mod lib_test;

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
