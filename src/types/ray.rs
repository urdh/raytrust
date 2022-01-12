use super::{Point3, Vect3};

/// The proverbial ray of the raytracer.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vect3,
}

impl Ray {
    /// Contstruct a ray.
    pub fn new(origin: Point3, direction: Vect3) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Get a specific point along the half-line.
    pub fn at(&self, distance: f32) -> Point3 {
        self.origin + (distance * self.direction)
    }

    /// Get the origin of this ray.
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    /// Get the direction of this ray.
    pub fn direction(&self) -> Vect3 {
        self.direction
    }

    /// Generate a random ray.
    #[cfg(test)]
    pub fn sample<R: rand::Rng>(rng: &mut R) -> Ray {
        let origin = Point3::sample(rng);
        let direction = Vect3::sample(rng);
        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_ulps_eq;

    #[test]
    fn test_ray_at() {
        let origin = Point3(1.0, 0.0, -1.0);
        let direction = Vect3(0.0, 1.0, 1.0);

        let ray = Ray::new(origin, direction);
        assert_ulps_eq!(ray.at(0.0), origin);
        assert_ulps_eq!(ray.at(1.0), origin + direction.normalize());
        assert_ulps_eq!(ray.at(direction.norm()), origin + direction);
    }
}
