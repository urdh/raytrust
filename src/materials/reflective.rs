use super::{Color, Material};
use crate::surfaces::Intersection;
use crate::types::{Ray, Vect3};
use rand_distr::{Distribution, UnitDisc};

/// Pick a random point on a disk orthogonal to `normal`.
fn rand_point_on_disk(normal: &Vect3, radius: f32) -> Vect3 {
    let vec: [f32; 2] = UnitDisc.sample(&mut rand::thread_rng());
    let horiz = Vect3(1.0, 0.0, 0.0);
    let x = (horiz - normal.project(horiz)).normalize();
    let y = normal.cross(x);
    (x * vec[0] * radius) + (y * vec[1] * radius)
}

/// A reflective metal-like material.
#[derive(Debug, Clone, Copy)]
pub struct Metal {
    attenuation: Color,
    pertubation: f32,
}

impl Metal {
    /// Construct a metal material with a given attenuation.
    pub fn new(color: Color, fuzziness: f32) -> Metal {
        Metal {
            attenuation: color,
            pertubation: fuzziness,
        }
    }
}

impl Material for Metal {
    fn scatter_at(&self, ray: &Ray, intersection: &Intersection) -> Vec<(Ray, Color)> {
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
