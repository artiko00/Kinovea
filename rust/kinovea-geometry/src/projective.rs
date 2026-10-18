//! Homography between a plane quadrilateral and its image (`ProjectiveMapper`, after
//! Heckbert's "Projective Mappings for Image Warping" via AForge).

mod ellipse;
#[cfg(test)]
mod ellipse_test;

use crate::point::PointF;
use crate::quadrilateral::Quadrilateral;
use crate::vector3::Vector3;
use std::fmt;

type Mat = [[f64; 3]; 3];

const TOLERANCE: f64 = 1e-13;
const IDENTITY: Mat = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

/// A quadrilateral has three collinear vertices; no square-to-quad mapping exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DegenerateQuadrilateral;

impl fmt::Display for DegenerateQuadrilateral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "degenerate quadrilateral: no projective mapping")
    }
}

impl std::error::Error for DegenerateQuadrilateral {}

/// Forward maps plane → image, backward maps image → plane. Defaults to the identity.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectiveMapper {
    map: Mat,
    unmap: Mat,
}

impl Default for ProjectiveMapper {
    fn default() -> Self {
        ProjectiveMapper {
            map: IDENTITY,
            unmap: IDENTITY,
        }
    }
}

impl ProjectiveMapper {
    pub fn new(
        plane: &Quadrilateral,
        image: &Quadrilateral,
    ) -> Result<Self, DegenerateQuadrilateral> {
        let mut m = ProjectiveMapper::default();
        m.update(plane, image)?;
        Ok(m)
    }

    /// Recomputes the homography; on failure the previous mapping is kept.
    pub fn update(
        &mut self,
        plane: &Quadrilateral,
        image: &Quadrilateral,
    ) -> Result<(), DegenerateQuadrilateral> {
        let square_to_input = square_to_quad(plane).ok_or(DegenerateQuadrilateral)?;
        let square_to_output = square_to_quad(image).ok_or(DegenerateQuadrilateral)?;
        self.map = multiply(&square_to_output, &adjugate(&square_to_input));
        self.unmap = adjugate(&self.map);
        Ok(())
    }

    pub(crate) fn unmap_matrix(&self) -> &Mat {
        &self.unmap
    }

    /// Plane → image.
    pub fn forward(&self, p: PointF) -> PointF {
        apply(&self.map, p)
    }

    /// Image → plane.
    pub fn backward(&self, p: PointF) -> PointF {
        apply(&self.unmap, p)
    }

    pub fn forward_quad(&self, q: &Quadrilateral) -> Quadrilateral {
        Quadrilateral {
            vertices: q.vertices.map(|p| self.forward(p)),
        }
    }

    pub fn backward_quad(&self, q: &Quadrilateral) -> Quadrilateral {
        Quadrilateral {
            vertices: q.vertices.map(|p| self.backward(p)),
        }
    }

    /// Homogeneous plane → image.
    pub fn forward_vector3(&self, p: Vector3) -> Vector3 {
        apply_homogeneous(&self.map, &self.map, p)
    }

    /// Homogeneous image → plane. Reproduces the original: the `y` row takes its third
    /// coefficient from the forward matrix.
    pub fn backward_vector3(&self, p: Vector3) -> Vector3 {
        apply_homogeneous(&self.unmap, &self.map, p)
    }
}

fn apply(m: &Mat, p: PointF) -> PointF {
    let (x, y) = (f64::from(p.x), f64::from(p.y));
    let factor = m[2][0] * x + m[2][1] * y + m[2][2];
    let nx = (m[0][0] * x + m[0][1] * y + m[0][2]) / factor;
    let ny = (m[1][0] * x + m[1][1] * y + m[1][2]) / factor;
    PointF::new(nx as f32, ny as f32)
}

fn apply_homogeneous(m: &Mat, y_z_source: &Mat, p: Vector3) -> Vector3 {
    let (x, y, z) = (f64::from(p.x), f64::from(p.y), f64::from(p.z));
    Vector3::new(
        (m[0][0] * x + m[0][1] * y + m[0][2] * z) as f32,
        (m[1][0] * x + m[1][1] * y + y_z_source[1][2] * z) as f32,
        (m[2][0] * x + m[2][1] * y + m[2][2] * z) as f32,
    )
}

fn det2(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * d - b * c
}

/// Transform from the unit square to `quad`; `None` when the quadrilateral is degenerate.
fn square_to_quad(quad: &Quadrilateral) -> Option<Mat> {
    let q: Vec<(f64, f64)> = quad
        .vertices
        .iter()
        .map(|p| (f64::from(p.x), f64::from(p.y)))
        .collect();
    let px = q[0].0 - q[1].0 + q[2].0 - q[3].0;
    let py = q[0].1 - q[1].1 + q[2].1 - q[3].1;
    if px.abs() < TOLERANCE && py.abs() < TOLERANCE {
        return Some([
            [q[1].0 - q[0].0, q[2].0 - q[1].0, q[0].0],
            [q[1].1 - q[0].1, q[2].1 - q[1].1, q[0].1],
            [0.0, 0.0, 1.0],
        ]);
    }
    let (dx1, dx2) = (q[1].0 - q[2].0, q[3].0 - q[2].0);
    let (dy1, dy2) = (q[1].1 - q[2].1, q[3].1 - q[2].1);
    let del = det2(dx1, dx2, dy1, dy2);
    if del == 0.0 {
        return None;
    }
    let g = det2(px, dx2, py, dy2) / del;
    let h = det2(dx1, px, dy1, py) / del;
    Some([
        [
            q[1].0 - q[0].0 + g * q[1].0,
            q[3].0 - q[0].0 + h * q[3].0,
            q[0].0,
        ],
        [
            q[1].1 - q[0].1 + g * q[1].1,
            q[3].1 - q[0].1 + h * q[3].1,
            q[0].1,
        ],
        [g, h, 1.0],
    ])
}

fn multiply(a: &Mat, b: &Mat) -> Mat {
    let mut c = [[0.0; 3]; 3];
    for (r, row) in c.iter_mut().enumerate() {
        for (k, x) in row.iter_mut().enumerate() {
            *x = a[r][0] * b[0][k] + a[r][1] * b[1][k] + a[r][2] * b[2][k];
        }
    }
    c
}

fn adjugate(a: &Mat) -> Mat {
    [
        [
            det2(a[1][1], a[1][2], a[2][1], a[2][2]),
            det2(a[2][1], a[2][2], a[0][1], a[0][2]),
            det2(a[0][1], a[0][2], a[1][1], a[1][2]),
        ],
        [
            det2(a[1][2], a[1][0], a[2][2], a[2][0]),
            det2(a[2][2], a[2][0], a[0][2], a[0][0]),
            det2(a[0][2], a[0][0], a[1][2], a[1][0]),
        ],
        [
            det2(a[1][0], a[1][1], a[2][0], a[2][1]),
            det2(a[2][0], a[2][1], a[0][0], a[0][1]),
            det2(a[0][0], a[0][1], a[1][0], a[1][1]),
        ],
    ]
}
