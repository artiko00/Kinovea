//! Drawing helpers shared by several painters.

pub mod angle_helper;
#[cfg(test)]
mod angle_helper_test;
pub mod arrow;
#[cfg(test)]
mod arrow_test;
pub mod bounding_box;
#[cfg(test)]
mod bounding_box_test;
pub mod decimal;
#[cfg(test)]
mod decimal_test;
pub mod frame;
#[cfg(test)]
mod frame_test;
pub mod mini_label;
#[cfg(test)]
mod mini_label_test;
pub mod rounded_rect;
#[cfg(test)]
mod rounded_rect_test;
pub mod squiggly;
#[cfg(test)]
mod squiggly_test;
#[cfg(test)]
pub(crate) mod support_test;
pub mod tick_mark;
#[cfg(test)]
mod tick_mark_test;
