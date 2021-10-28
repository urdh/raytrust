/// Surfaces forming part of a renderable scene.
mod sphere;

// Exports.
pub use sphere::Sphere;

// Imports.
use crate::types::Ray;

/// An intersectable surface.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Surface {
    Sphere(Sphere),
}

/// Check whether a ray intersects a surface.
///
/// # Arguments
///
/// * `ray` - the ray to trace along
/// * `surface` - the surface to intersect
pub fn intersects(ray: &Ray, surface: &Surface) -> bool {
    match surface {
        Surface::Sphere(s) => s.intersected_by(ray),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::*;
}
