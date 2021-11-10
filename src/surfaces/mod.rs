/// Surfaces forming part of a renderable scene.
mod sphere;

// Exports.
pub use sphere::Sphere;

/// An intersectable surface.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Surface {
    Sphere(Sphere),
}
