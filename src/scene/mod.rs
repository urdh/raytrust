/// Objects forming part of a renderable scene.
mod sphere;

// Exports.
pub use sphere::Sphere;

// Imports.
use crate::types::Ray;

/// An intersectable object.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Object {
    Sphere(Sphere),
}

/// Check whether a ray intersects an object.
///
/// # Arguments
///
/// * `ray` - the ray to trace along
/// * `object` - the object to intersect
pub fn intersects(ray: &Ray, object: &Object) -> bool {
    match object {
        Object::Sphere(s) => s.intersected_by(ray),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::*;
}
