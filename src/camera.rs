use crate::types::{Point3, Ray, Vect3};

/// A camera abstraction.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Point3,
    corner: Point3,
    horiz: Vect3,
    vert: Vect3,
}

impl Camera {
    /// Create a new camera.
    ///
    /// # Arguments
    ///
    /// * `origin` - the origin of the camera
    /// * `direction` - the direction in which the camera is pointing
    /// * `focal_length` - the focal length of the camera
    /// * `viewport` - width and height of the viewport
    pub fn new(
        origin: Point3,
        direction: Vect3,
        focal_length: f32,
        viewport: (f32, f32),
    ) -> Camera {
        // Assume the camera is horizontal, e.g. the horizontal
        // axis of the viewport is parallel to the X axis.
        let x_axis = Vect3 {
            x: 1.0,
            ..Vect3::zero()
        };
        let horiz = viewport.0 * (x_axis - direction.project(x_axis)).normalize();
        let vert = viewport.1 * direction.normalize().cross(horiz.normalize());
        // Finally, construct the camera.
        let corner = origin - (horiz / 2.0) - (vert / 2.0) - (direction.normalize() * focal_length);
        Camera {
            origin,
            corner,
            horiz,
            vert,
        }
    }

    /// Get a ray pointing through a specific viewport position.
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.corner + (u * self.horiz) + (v * self.vert) - self.origin;
        Ray::new(self.origin, direction)
    }
}
