use super::Material;
use crate::image::Pixel;
use crate::surfaces::Intersection;
use crate::types::Ray;

/// A reflective metal-like material.
#[derive(Debug, Clone, Copy)]
pub struct Metal {
    attenuation: Pixel,
}

impl Metal {
    /// Construct a metal material with a given attenuation.
    pub fn new(r: f32, g: f32, b: f32) -> Metal {
        Metal {
            attenuation: Pixel { r, g, b },
        }
    }
}

impl Material for Metal {
    fn scatter_at(&self, ray: &Ray, intersection: &Intersection) -> Vec<(Ray, Pixel)> {
        let normal = intersection.normal();
        let incident = ray.direction();
        let reflection = incident - 2.0 * incident.dot(normal) * normal;
        vec![(Ray::new(intersection.point(), reflection), self.attenuation)]
    }
}
