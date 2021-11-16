use super::{Intersection, Surface};
use crate::types::{Point3, Ray};
use std::ops::Range;

/// An intersectable sphere.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Surface for Sphere {
    fn intersected_by(&self, ray: &Ray, filter: Range<f32>) -> Vec<Intersection> {
        let offset = ray.origin() - self.center;
        // Solving ax² + 2bx + c = r², where the constants are derived
        // from expanding `(ray.at(x) - self.center)² = self.radius²`.
        let a = ray.direction().dot(ray.direction());
        let b = offset.dot(ray.direction());
        let c = offset.dot(offset) - (self.radius * self.radius);
        // If the there are any positive non-imaginary solutions,
        // we have intersected with the shpere. Pick the closest
        // intersection point for the caller.
        let distances = [
            (-b - ((b * b) - (a * c)).sqrt()) / a,
            (-b + ((b * b) - (a * c)).sqrt()) / a,
        ];
        IntoIterator::into_iter(distances)
            .filter(|distance| filter.contains(distance))
            .map(|distance| {
                // Intersection! Return a point and normal.
                let point = ray.at(distance);
                let normal = point - self.center;
                Intersection::new(point, normal / self.radius)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::types::Vect3;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_no_intersections() {
        let sphere = Sphere {
            center: Point3(0.0, 0.0, 2.0),
            radius: 1.0,
        };
        let ray_z = Ray::new(Point3::zero(), Vect3(0.0, 0.0, -1.0));
        let ray_x = Ray::new(Point3::zero(), Vect3(-1.0, 0.0, 0.0));

        assert_eq!(sphere.intersected_by(&ray_z, 0.0..f32::INFINITY), vec![]);
        assert_eq!(sphere.intersected_by(&ray_x, 0.0..f32::INFINITY), vec![]);
    }

    #[test]
    fn test_one_intersection() {
        let sphere = Sphere {
            center: Point3(0.0, 0.0, 2.0),
            radius: 1.0,
        };
        let ray = Ray::new(Point3(1.0, 0.0, 0.0), Vect3(0.0, 0.0, 1.0));

        let expected = vec![
            Intersection::new(Point3(1.0, 0.0, 2.0), Vect3(1.0, 0.0, 0.0)),
            Intersection::new(Point3(1.0, 0.0, 2.0), Vect3(1.0, 0.0, 0.0)),
        ];
        assert_eq!(sphere.intersected_by(&ray, 0.0..f32::INFINITY), expected);
    }

    #[test]
    fn test_two_intersections() {
        let sphere = Sphere {
            center: Point3(0.0, 0.0, 2.0),
            radius: 1.0,
        };
        let ray = Ray::new(Point3::zero(), Vect3(0.0, 0.0, 1.0));

        let expected = vec![
            Intersection::new(Point3(0.0, 0.0, 1.0), Vect3(0.0, 0.0, -1.0)),
            Intersection::new(Point3(0.0, 0.0, 3.0), Vect3(0.0, 0.0, 1.0)),
        ];
        assert_eq!(sphere.intersected_by(&ray, 0.0..f32::INFINITY), expected);
    }
}
