//! Pixel steps of the original trackers, ported from the OpenCV and GDI+ calls they make:
//! `cvtColor(BGR2GRAY)` (`color`), `medianBlur` (`filter`), and the elliptical
//! template mask drawn with `Graphics.FillEllipse` (`mask`). Each module names the behavior
//! it reproduces; gray and median outputs are bit-exact with OpenCV 4.10, the version behind
//! the `OpenCvSharp4` 4.10.0 of the original.

pub mod color;
#[cfg(test)]
mod color_test;
pub mod filter;
#[cfg(test)]
mod filter_test;
pub mod image;
#[cfg(test)]
mod image_test;
pub mod mask;
#[cfg(test)]
mod mask_test;

pub use color::to_gray;
pub use filter::median_blur;
pub use image::{GrayImage, Roi};
pub use mask::ellipse_mask;
