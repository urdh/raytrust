/// Materials used to render surfaces.
mod dielectric;
mod diffuse;
mod reflective;

// Exports.
pub use dielectric::Dielectric;
pub use diffuse::{Hemispherical, Lambertian};
pub use reflective::Metal;

// Imports.
use crate::surfaces::Intersection;
use crate::types::Ray;
use std::vec::Vec;

/// A color with red/green/blue components.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    /// The red channel of the color.
    pub fn red(&self) -> f32 {
        self.0
    }

    /// The green channel of the color.
    pub fn green(&self) -> f32 {
        self.1
    }

    /// The blue channel of the color.
    pub fn blue(&self) -> f32 {
        self.2
    }
}

/// A (possibly reflecting) material.
pub trait Material {
    /// Reflect a ray at an intersection point.
    ///
    /// # Arguments
    ///
    /// * `ray` - ray to reflect
    /// * `intersection` - intersection to reflect at
    fn scatter_at(&self, ray: &Ray, intersection: &Intersection) -> Vec<(Ray, Color)>;
}
