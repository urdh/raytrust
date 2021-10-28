use super::Intersection;
use crate::types::{Point3, Ray};

/// An intersectable sphere.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    /// Check whether a ray intersects this sphere.
    ///
    /// # Arguments
    ///
    /// * `ray` - ray to trace along
    pub fn intersected_by(&self, ray: &Ray) -> Option<Intersection> {
        let offset = ray.origin() - self.center;
        // Solving ax² + 2bx + c = r², where the constants are derived
        // from expanding `(ray.at(x) - self.center)² = self.radius²`.
        let a = ray.direction().dot(ray.direction());
        let b = offset.dot(ray.direction());
        let c = offset.dot(offset) - (self.radius * self.radius);
        // If the smallest non-imaginary solution is positive, we have
        // intersected with the outside of the shpere.
        let distance = (-b - ((b * b) - (a * c)).sqrt()) / a;
        if distance > 0.0 {
            // Intersection! Return a point and normal.
            let point = ray.at(distance);
            let normal = point - self.center;
            Some(Intersection::new(point, normal))
        } else {
            // No intersection :(
            None
        }
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
            center: Point3 {
                z: 2.0,
                ..Point3::zero()
            },
            radius: 1.0,
        };
        let ray_z = Ray::new(
            Point3::zero(),
            Vect3 {
                z: -1.0,
                ..Vect3::zero()
            },
        );
        let ray_x = Ray::new(
            Point3::zero(),
            Vect3 {
                x: -1.0,
                ..Vect3::zero()
            },
        );

        assert_eq!(sphere.intersected_by(&ray_z), None);
        assert_eq!(sphere.intersected_by(&ray_x), None);
    }

    #[test]
    fn test_one_intersection() {
        let sphere = Sphere {
            center: Point3 {
                z: 2.0,
                ..Point3::zero()
            },
            radius: 1.0,
        };
        let ray = Ray::new(
            Point3 {
                x: 1.0,
                ..Point3::zero()
            },
            Vect3 {
                z: 1.0,
                ..Vect3::zero()
            },
        );

        let expected = Intersection::new(
            Point3 {
                x: 1.0,
                y: 0.0,
                z: 2.0,
            },
            Vect3 {
                x: 1.0,
                ..Vect3::zero()
            },
        );
        assert_eq!(sphere.intersected_by(&ray), Some(expected));
    }

    #[test]
    fn test_two_intersections() {
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

        let expected = Intersection::new(
            Point3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vect3 {
                z: -1.0,
                ..Vect3::zero()
            },
        );
        assert_eq!(sphere.intersected_by(&ray), Some(expected));
    }
}
