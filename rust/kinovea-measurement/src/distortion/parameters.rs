//! Intrinsics and distortion coefficients (`DistortionParameters`).

use kinovea_geometry::Size;

pub const DEFAULT_FOCAL_LENGTH_MM: f64 = 2.77;
pub const DEFAULT_SENSOR_WIDTH_MM: f64 = 6.160;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DistortionParameters {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub k1: f64,
    pub k2: f64,
    pub k3: f64,
    pub p1: f64,
    pub p2: f64,
    pub pixels_per_millimeter: f64,
    /// File the parameters came from, written back as `Path`.
    pub path: String,
}

impl DistortionParameters {
    #[allow(clippy::too_many_arguments)]
    pub fn new(k1: f64, k2: f64, k3: f64, p1: f64, p2: f64, fx: f64, fy: f64, cx: f64, cy: f64, pixels_per_millimeter: f64) -> Self {
        DistortionParameters {
            fx,
            fy,
            cx,
            cy,
            k1,
            k2,
            k3,
            p1,
            p2,
            pixels_per_millimeter,
            path: String::new(),
        }
    }

    /// `pixels_per_millimeter` derived from the default sensor width.
    #[allow(clippy::too_many_arguments)]
    pub fn for_image(k1: f64, k2: f64, k3: f64, p1: f64, p2: f64, fx: f64, fy: f64, cx: f64, cy: f64, image_size: Size) -> Self {
        Self::new(k1, k2, k3, p1, p2, fx, fy, cx, cy, f64::from(image_size.width) / DEFAULT_SENSOR_WIDTH_MM)
    }

    /// No distortion, default focal length, principal point at the image centre.
    pub fn defaults(image_size: Size) -> Self {
        let ppmm = f64::from(image_size.width) / DEFAULT_SENSOR_WIDTH_MM;
        let f = DEFAULT_FOCAL_LENGTH_MM * ppmm;
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, f, f, f64::from(image_size.width) / 2.0, f64::from(image_size.height) / 2.0, ppmm)
    }

    /// Row-major camera matrix `[[fx, 0, cx], [0, fy, cy], [0, 0, 1]]`.
    pub fn camera_matrix(&self) -> [[f64; 3]; 3] {
        [[self.fx, 0.0, self.cx], [0.0, self.fy, self.cy], [0.0, 0.0, 1.0]]
    }

    /// OpenCV layout `[k1, k2, p1, p2, k3]`.
    pub fn dist_coeffs(&self) -> [f64; 5] {
        [self.k1, self.k2, self.p1, self.p2, self.k3]
    }

    /// XOR of the `.NET` `double.GetHashCode()` of every parameter.
    pub fn content_hash(&self) -> i32 {
        [self.k1, self.k2, self.k3, self.p1, self.p2, self.fx, self.fy, self.cx, self.cy]
            .iter()
            .fold(0, |h, v| h ^ double_hash(*v))
    }
}

fn double_hash(v: f64) -> i32 {
    let bits = v.to_bits();
    (bits as i32) ^ ((bits >> 32) as i32)
}
