use super::{Color, Material};
use crate::surfaces::Intersection;
use crate::types::{Ray, Vect3};
use rand_distr::{Distribution, Uniform};

fn refract(incident: Vect3, normal: Vect3, ratio: f32) -> Vect3 {
    let cos_theta = incident.dot(-normal).min(1.0);
    if cos_theta < 0.0 {
        refract(incident, -normal, ratio.recip())
    } else {
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflection = incident - 2.0 * incident.dot(normal) * normal;
        let orthogonal = ratio * (incident + cos_theta * normal);
        let parallel = -(1.0 - orthogonal.dot(orthogonal)).abs().sqrt() * normal;
        let refraction = orthogonal + parallel;
        let reflectance = {
            // Schlick's approximation
            let r0 = (1.0 - ratio) / (1.0 + ratio);
            (r0 * r0) + (1.0 - r0 * r0) * (1.0 - cos_theta).powi(5)
        };
        let p = Uniform::new(0.0, 1.0).sample(&mut rand::thread_rng());
        if (ratio * sin_theta > 1.0) || (reflectance > p) {
            reflection
        } else {
            refraction
        }
    }
}

/// A reflective metal-like material.
#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    attenuation: Color,
    refraction: f32,
}

impl Dielectric {
    /// Construct a metal material with a given attenuation.
    pub fn new(color: Color, refraction: f32) -> Dielectric {
        Dielectric {
            attenuation: color,
            refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter_at(&self, ray: &Ray, intersection: &Intersection) -> Vec<(Ray, Color)> {
        let normal = intersection.normal();
        let incident = ray.direction();
        let ratio = self.refraction.recip();
        let refracted = refract(incident, normal, ratio);
        vec![(Ray::new(intersection.point(), refracted), self.attenuation)]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_abs_diff_eq;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_refraction_0_deg() {
        let incident = Vect3(0.0, 0.0, 1.0);
        let normal = Vect3(0.0, 0.0, -1.0);
        let ratio = 2.0_f32.sqrt();

        let result = refract(incident, normal, ratio);
        assert_eq!(result, Vect3(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_refraction_45_deg() {
        let incident = Vect3(0.0, 1.0, 1.0).normalize();
        let normal = Vect3(0.0, 0.0, -1.0);
        let ratio_1 = 2.0_f32.sqrt();
        let ratio_2 = 0.0_f32;
        let ratio_3 = 0.9_f32;
        let ratio_4 = 1.0_f32;

        let result_1 = refract(incident, normal, ratio_1);
        let result_2 = refract(incident, normal, ratio_2);
        let result_3 = refract(incident, normal, ratio_3);
        let result_4 = refract(incident, normal, ratio_4);
        assert_abs_diff_eq!(result_1, Vect3(0.0, 1.0, 0.0), epsilon = 0.001);
        assert_abs_diff_eq!(result_2, Vect3(0.0, 1.0, -1.0).normalize(), epsilon = 0.001);
        assert_abs_diff_eq!(result_3, Vect3(0.0, 0.636396, 0.771362), epsilon = 0.001);
        assert_abs_diff_eq!(result_4, incident, epsilon = 0.001);
    }
}
