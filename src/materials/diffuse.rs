use super::Material;
use crate::image::Pixel;
use crate::surfaces::Intersection;
use crate::types::{Point3, Ray, Vect3};
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

/// Pick a random point on a sphere centered on `origin`.
///
/// See <https://mathworld.wolfram.com/SpherePointPicking.html>.
fn rand_point_on_sphere(origin: &Point3, radius: f32) -> Point3 {
    let mut rng = thread_rng();
    let vec = Vect3 {
        x: rng.sample(StandardNormal),
        y: rng.sample(StandardNormal),
        z: rng.sample(StandardNormal),
    };
    let norm = vec.norm();
    if norm == 0.0 {
        rand_point_on_sphere(origin, radius)
    } else {
        origin + (vec * (radius / norm))
    }
}

/// A lambertian diffuse material.
#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    attenuation: Pixel,
}

impl Lambertian {
    /// Construct a colored diffuse material with lambertian reflection.
    pub fn new(r: f32, g: f32, b: f32) -> Lambertian {
        Lambertian {
            attenuation: Pixel { r, g, b },
        }
    }
}

impl Material for Lambertian {
    fn scatter_at(&self, _ray: &Ray, intersection: &Intersection) -> (Ray, Pixel) {
        let origin = intersection.point();
        let center = origin + intersection.normal();
        let direction = rand_point_on_sphere(&center, 1.0) - origin;
        (Ray::new(origin, direction), self.attenuation)
    }
}

/// A hemispherical diffuse material.
#[derive(Debug, Clone, Copy)]
pub struct Hemispherical {
    attenuation: Pixel,
}

impl Hemispherical {
    /// Construct a colored diffuse material with hemispherical reflection.
    pub fn new(r: f32, g: f32, b: f32) -> Hemispherical {
        Hemispherical {
            attenuation: Pixel { r, g, b },
        }
    }
}

impl Material for Hemispherical {
    fn scatter_at(&self, _ray: &Ray, intersection: &Intersection) -> (Ray, Pixel) {
        let origin = intersection.point();
        let direction = rand_point_on_sphere(&origin, 1.0) - origin;
        if direction.dot(intersection.normal()) > 0.0 {
            (Ray::new(origin, direction), self.attenuation)
        } else {
            (Ray::new(origin, -direction), self.attenuation)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lambertian_reflects_outward() {
        let ray = Ray::new(
            Point3::zero(),
            Vect3 {
                z: 1.0,
                ..Vect3::zero()
            },
        );
        let intersection = Intersection::new(
            Point3::zero(),
            Vect3 {
                z: 1.0,
                ..Vect3::zero()
            },
        );
        let lambertian = Lambertian::new(1.0, 1.0, 1.0);
        let (reflection, _) = lambertian.scatter_at(&ray, &intersection);

        assert_eq!(reflection.origin(), intersection.point());
        assert!(reflection.direction().dot(intersection.normal()) > 0.0);
    }

    #[test]
    fn test_hemispherical_reflects_outward() {
        let ray = Ray::new(
            Point3::zero(),
            Vect3 {
                z: 1.0,
                ..Vect3::zero()
            },
        );
        let intersection = Intersection::new(
            Point3::zero(),
            Vect3 {
                z: 1.0,
                ..Vect3::zero()
            },
        );
        let hemispherical = Hemispherical::new(1.0, 1.0, 1.0);
        let (reflection, _) = hemispherical.scatter_at(&ray, &intersection);

        assert_eq!(reflection.origin(), intersection.point());
        assert!(reflection.direction().dot(intersection.normal()) > 0.0);
    }
}
