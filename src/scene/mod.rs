/// Surfaces forming part of a renderable scene.
mod sphere;

// Exports.
pub use sphere::Sphere;

// Imports.
use crate::types::{Point3, Ray, Vect3};

/// An intersectable surface.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Surface {
    Sphere(Sphere),
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

/// Check whether a ray intersects a surface.
///
/// # Arguments
///
/// * `ray` - the ray to trace along
/// * `surface` - the surface to intersect
pub fn intersects(ray: &Ray, surface: &Surface) -> Option<Intersection> {
    match surface {
        Surface::Sphere(s) => s.intersected_by(ray),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::*;
}
