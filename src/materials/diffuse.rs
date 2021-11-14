use super::{Color, Material};
use crate::surfaces::Intersection;
use crate::types::{Point3, Ray, Vect3};
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

/// Pick a random point on a sphere centered on `origin`.
///
/// See <https://mathworld.wolfram.com/SpherePointPicking.html>.
fn rand_point_on_sphere(origin: &Point3, radius: f32) -> Point3 {
    let mut rng = thread_rng();
    let vec = Vect3(
        rng.sample(StandardNormal),
        rng.sample(StandardNormal),
        rng.sample(StandardNormal),
    );
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
    attenuation: Color,
}

impl Lambertian {
    /// Construct a colored diffuse material with lambertian reflection.
    pub fn new(color: Color) -> Lambertian {
        Lambertian { attenuation: color }
    }
}

impl Material for Lambertian {
    fn scatter_at(&self, _ray: &Ray, intersection: &Intersection) -> Vec<(Ray, Color)> {
        let origin = intersection.point();
        let center = origin + intersection.normal();
        let direction = rand_point_on_sphere(&center, 1.0) - origin;
        if direction.norm() > 0.0 {
            vec![(Ray::new(origin, direction), self.attenuation)]
        } else {
            vec![(Ray::new(origin, intersection.normal()), self.attenuation)]
        }
    }
}

/// A hemispherical diffuse material.
#[derive(Debug, Clone, Copy)]
pub struct Hemispherical {
    attenuation: Color,
}

impl Hemispherical {
    /// Construct a colored diffuse material with hemispherical reflection.
    pub fn new(color: Color) -> Hemispherical {
        Hemispherical { attenuation: color }
    }
}

impl Material for Hemispherical {
    fn scatter_at(&self, _ray: &Ray, intersection: &Intersection) -> Vec<(Ray, Color)> {
        let origin = intersection.point();
        let direction = rand_point_on_sphere(&origin, 1.0) - origin;
        if direction.dot(intersection.normal()) > 0.0 {
            vec![(Ray::new(origin, direction), self.attenuation)]
        } else {
            vec![(Ray::new(origin, -direction), self.attenuation)]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lambertian_reflects_outward() {
        let ray = Ray::new(Point3::zero(), Vect3(0.0, 0.0, 1.0));
        let intersection = Intersection::new(Point3::zero(), Vect3(0.0, 0.0, 1.0));
        let lambertian = Lambertian::new(Color(1.0, 1.0, 1.0));
        let scatters = lambertian.scatter_at(&ray, &intersection);

        for (reflection, _) in scatters {
            assert_eq!(reflection.origin(), intersection.point());
            assert!(reflection.direction().dot(intersection.normal()) > 0.0);
        }
    }

    #[test]
    fn test_hemispherical_reflects_outward() {
        let ray = Ray::new(Point3::zero(), Vect3(0.0, 0.0, 1.0));
        let intersection = Intersection::new(Point3::zero(), Vect3(0.0, 0.0, 1.0));
        let hemispherical = Hemispherical::new(Color(1.0, 1.0, 1.0));
        let scatters = hemispherical.scatter_at(&ray, &intersection);

        for (reflection, _) in scatters {
            assert_eq!(reflection.origin(), intersection.point());
            assert!(reflection.direction().dot(intersection.normal()) > 0.0);
        }
    }
}
