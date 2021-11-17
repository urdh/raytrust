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
    /// * `target` - the point at which the camera is looking
    /// * `vertical` - the upward direction of the camera
    /// * `focal_length` - the focal length of the camera
    /// * `viewport` - width and height of the viewport
    pub fn new(
        origin: Point3,
        target: Point3,
        vertical: Vect3,
        focal_length: f32,
        viewport: (f32, f32),
    ) -> Camera {
        let optical = (origin - target).normalize();
        let horiz = viewport.0 * vertical.normalize().cross(optical).normalize();
        let vert = viewport.1 * optical.cross(horiz.normalize());
        // Finally, construct the camera.
        let corner = origin - (horiz / 2.0) - (vert / 2.0) - (optical * focal_length);
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
