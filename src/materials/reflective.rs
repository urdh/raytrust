use super::Material;
use crate::image::Pixel;
use crate::surfaces::Intersection;
use crate::types::{Ray, Vect3};
use rand::{thread_rng, Rng};
use rand_distr::Uniform;
use std::f32::consts::PI;

/// Pick a random point on a disk orthogonal to `normal`.
///
/// See <https://mathworld.wolfram.com/DiskPointPicking.html>.
fn rand_point_on_disk(normal: &Vect3, radius: f32) -> Vect3 {
    let mut rng = thread_rng();
    let r: f32 = rng.sample(Uniform::new_inclusive(0.0, radius));
    let phi: f32 = rng.sample(Uniform::new(0.0, 2.0 * PI));
    let horiz = Vect3(1.0, 0.0, 0.0);
    let x = (horiz - normal.project(horiz)).normalize();
    let y = normal.cross(x);
    (x * r.sqrt() * phi.cos()) + (y * r.sqrt() * phi.sin())
}

/// A reflective metal-like material.
#[derive(Debug, Clone, Copy)]
pub struct Metal {
    attenuation: Pixel,
    pertubation: f32,
}

impl Metal {
    /// Construct a metal material with a given attenuation.
    pub fn new(r: f32, g: f32, b: f32, fuzziness: f32) -> Metal {
        Metal {
            attenuation: Pixel { r, g, b },
            pertubation: fuzziness,
        }
    }
}

impl Material for Metal {
    fn scatter_at(&self, ray: &Ray, intersection: &Intersection) -> Vec<(Ray, Pixel)> {
        let normal = intersection.normal();
        let incident = ray.direction();
        let reflection = incident - 2.0 * incident.dot(normal) * normal;
        let direction = reflection + rand_point_on_disk(&reflection, self.pertubation);
        if direction.dot(intersection.normal()) > 0.0 {
            vec![(Ray::new(intersection.point(), direction), self.attenuation)]
        } else {
            vec![]
        }
    }
}
