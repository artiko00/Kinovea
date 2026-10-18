//! Kinovea `geometry` crate: the 2D/3D toolbox shared by drawings, calibration and tracking.
//!
//! Module map:
//! - [`point`]: `PointF`, `SizeF`, `RectangleF` with `System.Drawing` semantics.
//! - [`vector`], [`vector3`]: single precision vectors.
//! - [`matrix3x3`], [`svd`]: 3×3 matrix (rotations, inverse) and the Numerical Recipes SVD.
//! - [`helpers`]: distances, oriented angles, rotations, snapping, projections and mixing.
//! - [`quadrilateral`], [`shapes`]: quadrilaterals, circles, ellipses, segments, named points.
//! - [`clipping`]: Liang-Barsky clipping of segments, rays and infinite lines.
//! - [`projective`]: homography between two quadrilaterals and its unit-circle ellipse.
//! - [`circle_fit`]: linear least-squares circle fit.

#[cfg(test)]
pub(crate) mod lib_test;

/// Declares a public module together with its `#[cfg(test)]` sibling.
macro_rules! modules {
    ($($m:ident / $t:ident),* $(,)?) => { $(pub mod $m; #[cfg(test)] mod $t;)* };
}

modules!(
    circle_fit / circle_fit_test,
    clipping / clipping_test,
    helpers / helpers_test,
    matrix3x3 / matrix3x3_test,
    point / point_test,
    projective / projective_test,
    quadrilateral / quadrilateral_test,
    rect / rect_test,
    shapes / shapes_test,
    svd / svd_test,
    vector / vector_test,
    vector3 / vector3_test,
);

pub use circle_fit::fit_circle;
pub use clipping::{clip_infinite_line, clip_line, clip_segment};
pub use matrix3x3::Matrix3x3;
pub use point::{PointF, SizeF};
pub use projective::ProjectiveMapper;
pub use quadrilateral::Quadrilateral;
pub use rect::RectangleF;
pub use shapes::{Circle, ClipResult, Ellipse, NamedPoint, PointLinePosition, Segment};
pub use vector::Vector;
pub use vector3::Vector3;

/// Returns the crate name as declared in `Cargo.toml`.
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
