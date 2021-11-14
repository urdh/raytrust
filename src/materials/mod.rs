/// Materials used to render surfaces.
mod diffuse;
mod reflective;

// Exports.
pub use diffuse::{Hemispherical, Lambertian};
pub use reflective::Metal;

// Imports.
use crate::image::Pixel;
use crate::surfaces::Intersection;
use crate::types::Ray;
use std::vec::Vec;

/// A (possibly reflecting) material.
pub trait Material {
    /// Reflect a ray at an intersection point.
    ///
    /// # Arguments
    ///
    /// * `ray` - ray to reflect
    /// * `intersection` - intersection to reflect at
    fn scatter_at(&self, ray: &Ray, intersection: &Intersection) -> Vec<(Ray, Pixel)>;
}
