//! Elementary rotations and yaw/pitch/roll composition, as in the original `Matrix3x3`.

use super::Matrix3x3;

impl Matrix3x3 {
    /// Rotation about the X axis.
    pub fn rotation_x(radians: f32) -> Matrix3x3 {
        let (sin, cos) = sin_cos(radians);
        Matrix3x3::from_array([[1.0, 0.0, 0.0], [0.0, cos, -sin], [0.0, sin, cos]])
    }

    /// Rotation about the Y axis.
    pub fn rotation_y(radians: f32) -> Matrix3x3 {
        let (sin, cos) = sin_cos(radians);
        Matrix3x3::from_array([[cos, 0.0, sin], [0.0, 1.0, 0.0], [-sin, 0.0, cos]])
    }

    /// Rotation about the Z axis.
    pub fn rotation_z(radians: f32) -> Matrix3x3 {
        let (sin, cos) = sin_cos(radians);
        Matrix3x3::from_array([[cos, -sin, 0.0], [sin, cos, 0.0], [0.0, 0.0, 1.0]])
    }

    /// `RotationY(yaw) · RotationX(pitch) · RotationZ(roll)`.
    pub fn from_yaw_pitch_roll(yaw: f32, pitch: f32, roll: f32) -> Matrix3x3 {
        (Matrix3x3::rotation_y(yaw) * Matrix3x3::rotation_x(pitch)) * Matrix3x3::rotation_z(roll)
    }

    /// Extracts `(yaw, pitch, roll)` from a matrix built by [`Matrix3x3::from_yaw_pitch_roll`].
    pub fn yaw_pitch_roll(&self) -> (f32, f32, f32) {
        let v = &self.v;
        let yaw = f64::from(v[0][2]).atan2(f64::from(v[2][2])) as f32;
        let pitch = f64::from(-v[1][2]).asin() as f32;
        let roll = f64::from(v[1][0]).atan2(f64::from(v[1][1])) as f32;
        (yaw, pitch, roll)
    }
}

/// Sine and cosine computed in double precision then narrowed, like `(float)Math.Cos`.
fn sin_cos(radians: f32) -> (f32, f32) {
    let (s, c) = f64::from(radians).sin_cos();
    (s as f32, c as f32)
}
