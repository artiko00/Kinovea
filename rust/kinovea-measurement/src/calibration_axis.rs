//! Which image axis a line calibration defines (`CalibrationAxis`).

crate::text_enum!(
    /// `LineHorizontal`: the line is the +X axis; `LineVertical`: the +Y axis; `ImageAxes`:
    /// only the length is used and axes follow the image.
    CalibrationAxis [LineHorizontal, LineVertical, ImageAxes]
);

#[allow(clippy::derivable_impls)]
impl Default for CalibrationAxis {
    fn default() -> Self {
        CalibrationAxis::LineHorizontal
    }
}
