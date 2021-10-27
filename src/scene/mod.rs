/// Objects forming part of a renderable scene.
mod sphere;

// Exports.
pub use sphere::Sphere;

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::*;
}
