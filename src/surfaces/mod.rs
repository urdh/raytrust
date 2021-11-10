/// Surfaces forming part of a renderable scene.
mod sphere;

// Exports.
pub use sphere::Sphere;

// Imports.
use crate::types::{Point3, Ray, Vect3};

/// An intersectable surface.
pub trait Surface {
    /// Check whether a ray intersects this surface.
    ///
    /// # Arguments
    ///
    /// * `ray` - ray to trace along
    fn intersected_by(&self, ray: &Ray) -> Option<Intersection>;
}

/// An intersection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    point: Point3,
    normal: Vect3,
}

impl Intersection {
    /// Construct an intersection.
    fn new(point: Point3, normal: Vect3) -> Intersection {
        Intersection {
            point,
            normal: normal.normalize(),
        }
    }

    /// Get the point of this intersection.
    pub fn point(&self) -> Point3 {
        self.point
    }
    /// Get the normal of this intersection.
    pub fn normal(&self) -> Vect3 {
        self.normal
    }
}
