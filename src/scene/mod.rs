/// Surfaces forming part of a renderable scene.
mod sphere;

// Exports.
pub use sphere::Sphere;

// Imports.
use crate::types::{Point3, Ray, Vect3};
use std::ops::Range;

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
/// * `filter` - a distance range in which to intersect
pub fn intersects(ray: &Ray, surface: &Surface, filter: Range<f32>) -> Option<Intersection> {
    let intersection = match surface {
        Surface::Sphere(s) => s.intersected_by(ray),
    };
    intersection.filter(|i| filter.contains(&(i.point() - ray.origin()).norm()))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_intersection_filter() {
        let sphere = Sphere {
            center: Point3 {
                z: 2.0,
                ..Point3::zero()
            },
            radius: 1.0,
        };
        let ray = Ray::new(
            Point3::zero(),
            Vect3 {
                z: 1.0,
                ..Vect3::zero()
            },
        );

        assert_ne!(
            intersects(&ray, &Surface::Sphere(sphere), 0.0..f32::INFINITY),
            None
        );
        assert_eq!(intersects(&ray, &Surface::Sphere(sphere), 0.0..0.5), None);
        assert_eq!(intersects(&ray, &Surface::Sphere(sphere), 1.5..2.0), None);
    }
}
