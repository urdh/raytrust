/// Surfaces forming part of a renderable scene.
mod sphere;

// Exports.
pub use sphere::Sphere;

// Imports.
use crate::types::{Point3, Ray, Vect3};
use std::ops::Range;

/// An intersectable surface.
pub trait Surface {
    /// Return all intersectiona between a ray and this surface.
    ///
    /// # Arguments
    ///
    /// * `ray` - ray to trace along
    /// * `filter` - a distance range in which to intersect
    fn intersected_by(&self, ray: &Ray, filter: Range<f32>) -> Vec<Intersection>;
}

/// An intersection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    point: Point3,
    normal: Vect3,
}

impl Intersection {
    /// Construct an intersection.
    pub fn new(point: Point3, normal: Vect3) -> Intersection {
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
