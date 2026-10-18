//! Plane (or line-derived square) calibration: homography between the world rectangle and its
//! image, with the coordinate system origin and offset (`CalibratorPlane`).

mod quad;
#[cfg(test)]
mod quad_test;
mod xml;
#[cfg(test)]
mod xml_test;

pub use quad::make_quad;

use crate::calibration_axis::CalibrationAxis;
use kinovea_geometry::{PointF, ProjectiveMapper, Quadrilateral, SizeF, Vector3};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalibratorPlane {
    /// Real world size of the reference rectangle.
    pub size: SizeF,
    /// Origin of the coordinate system in grid space.
    origin: PointF,
    /// Offset added to world values.
    pub offset: PointF,
    axis: CalibrationAxis,
    quad_world: Quadrilateral,
    quad_image: Quadrilateral,
    mapping: ProjectiveMapper,
    initialized: bool,
    valid: bool,
}

impl CalibratorPlane {
    pub fn quad_world(&self) -> Quadrilateral {
        self.quad_world
    }

    pub fn quad_image(&self) -> Quadrilateral {
        self.quad_image
    }

    pub fn mapper(&self) -> &ProjectiveMapper {
        &self.mapping
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn axis(&self) -> CalibrationAxis {
        self.axis
    }

    pub fn origin(&self) -> PointF {
        self.origin
    }

    /// Rectified image point → world, with the stored origin.
    pub fn transform(&self, p: PointF) -> PointF {
        self.transform_with_origin_grid(p, self.origin)
    }

    /// Rectified image point → world, with the origin given in rectified image space.
    pub fn transform_with_origin(&self, p: PointF, origin_in_image: PointF) -> PointF {
        if !self.initialized {
            return p;
        }
        self.transform_with_origin_grid(p, self.mapping.backward(origin_in_image))
    }

    fn transform_with_origin_grid(&self, p: PointF, origin: PointF) -> PointF {
        if !self.initialized {
            return p;
        }
        self.grid_to_world_with_origin(self.mapping.backward(p), origin)
    }

    /// World → rectified image, with the stored origin.
    pub fn untransform(&self, p: PointF) -> PointF {
        self.untransform_with_origin(p, self.origin)
    }

    /// World → rectified image, with an origin given in grid space.
    pub fn untransform_with_origin(&self, p: PointF, origin: PointF) -> PointF {
        if !self.initialized {
            return p;
        }
        self.mapping.forward(world_to_grid(p, origin))
    }

    /// World → homogeneous image coordinates.
    pub fn project(&self, p: PointF) -> Vector3 {
        if !self.initialized {
            return Vector3::new(p.x, p.y, 1.0);
        }
        let c = world_to_grid(p, self.origin);
        self.mapping.forward_vector3(Vector3::new(c.x, c.y, 1.0))
    }

    pub fn project_vector3(&self, v: Vector3) -> Vector3 {
        if !self.initialized {
            return Vector3::default();
        }
        self.mapping.forward_vector3(v)
    }

    /// Moves the origin to a rectified image point.
    pub fn set_origin(&mut self, p: PointF) {
        if self.initialized {
            self.origin = self.mapping.backward(p);
        }
    }

    /// Origin back at vertex D of the image quad.
    pub fn reset_origin(&mut self) {
        self.origin = self.mapping.backward(self.quad_image.d());
    }

    pub fn initialize_plane(&mut self, size_world: SizeF, quad_image: Quadrilateral) {
        self.size = size_world;
        self.quad_world = Quadrilateral::from_size(size_world.width, size_world.height);
        self.quad_image = quad_image;
        let _ = self.mapping.update(&self.quad_world, &quad_image);
        self.origin = self.mapping.backward(quad_image.d());
        self.offset = PointF::EMPTY;
        self.valid = quad_image.is_convex();
        self.initialized = true;
    }

    pub fn initialize_line(&mut self, length_world: f32, start: PointF, end: PointF, axis: CalibrationAxis) {
        self.axis = axis;
        let quad = make_quad(start, end, axis);
        self.initialize_plane(SizeF::new(length_world, length_world), quad);
    }

    /// New image quad, same world size and origin.
    pub fn update_quad(&mut self, quad_image: Quadrilateral) {
        if !self.initialized || self.size.width <= 0.0 || self.size.height <= 0.0 {
            self.valid = false;
            return;
        }
        self.quad_image = quad_image;
        let _ = self.mapping.update(&self.quad_world, &quad_image);
        self.valid = quad_image.is_convex();
    }

    pub fn update_line(&mut self, start: PointF, end: PointF) {
        self.update_quad(make_quad(start, end, self.axis));
    }

    /// The original `Clone`: re-initialized copy keeping origin and axis (offset is dropped).
    pub fn clone_calibrator(&self) -> CalibratorPlane {
        let mut c = CalibratorPlane::default();
        c.initialize_plane(self.size, self.quad_image);
        c.origin = self.origin;
        c.axis = self.axis;
        c
    }

    /// Grid point → world with the stored origin and offset.
    pub fn grid_to_world_with_offset(&self, p: PointF) -> PointF {
        self.grid_to_world_with_origin(p, self.origin)
    }

    fn grid_to_world_with_origin(&self, p: PointF, origin: PointF) -> PointF {
        PointF::new(p.x - origin.x + self.offset.x, -p.y + origin.y + self.offset.y)
    }
}

fn world_to_grid(p: PointF, origin: PointF) -> PointF {
    PointF::new(origin.x + p.x, origin.y - p.y)
}
