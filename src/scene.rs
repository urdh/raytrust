// Imports.
use crate::image;
use crate::surfaces::*;
use crate::types::Ray;
use std::cmp::Ordering;
use std::ops::Range;

/// A full, renderable "scene".
pub struct Scene {
    pub surfaces: Vec<Box<dyn Surface>>,
}

impl Ray {
    /// Check whether a ray intersects any surface in a scene.
    ///
    /// # Arguments
    ///
    /// * `ray` - the ray to trace along
    /// * `scene` - the scene to intersect in
    /// * `filter` - a distance range in which to intersect
    fn intersects(&self, scene: &Scene, filter: Range<f32>) -> Option<Intersection> {
        scene
            .surfaces
            .iter()
            .filter_map(|surface| surface.intersected_by(self))
            .map(|match_| (match_, (match_.point() - self.origin()).norm()))
            .filter(|(_, distance)| filter.contains(distance))
            .min_by(|(_, a), (_, b)| match (a.is_nan(), b.is_nan()) {
                (true, true) => Ordering::Equal,
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                _ => a.partial_cmp(b).unwrap(),
            })
            .map(|(match_, _)| match_)
    }
}

impl Scene {
    /// Render the color for a specific ray.
    ///
    /// # Arguments
    ///
    /// * `ray` - the ray to trace along
    /// * `depth` - max number of reflections
    pub fn render_ray(&self, ray: &Ray, depth: usize) -> image::Pixel {
        if depth == 0 {
            // We reached the recusion depth. Return a black pixel.
            return image::Pixel::default();
        }
        if let Some(intersection) = ray.intersects(self, 0.0..f32::INFINITY) {
            // We have an intersection! Map the normal to colors.
            image::Pixel {
                r: 0.5 * (intersection.normal().x + 1.0),
                g: 0.5 * (intersection.normal().y + 1.0),
                b: 0.5 * (intersection.normal().z + 1.0),
            }
        } else {
            // Fall-back: fancy blue-ish gradient
            let t = 0.5 * (ray.direction().y + 1.0);
            image::Pixel {
                r: (1.0 - t) * 1.0 + t * 0.5,
                g: (1.0 - t) * 1.0 + t * 0.7,
                b: (1.0 - t) * 1.0 + t * 1.0,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{Point3, Vect3};
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_intersection_filter() {
        let sphere = Sphere {
            center: Point3 {
                z: 2.0,
                ..Point3::zero()
            },
            radius: 1.0,
        };
        let ray = Ray::new(
            Point3::zero(),
            Vect3 {
                z: 1.0,
                ..Vect3::zero()
            },
        );

        let scene = Scene {
            surfaces: vec![Box::new(sphere)],
        };
        assert_ne!(ray.intersects(&scene, 0.0..f32::INFINITY), None);
        assert_eq!(ray.intersects(&scene, 0.0..0.5), None);
        assert_eq!(ray.intersects(&scene, 1.5..2.0), None);
    }

    #[test]
    fn test_multiple_objects() {
        let sphere_a = Sphere {
            center: Point3 {
                z: 2.0,
                ..Point3::zero()
            },
            radius: 1.0,
        };
        let sphere_b = Sphere {
            center: Point3 {
                z: 4.0,
                ..Point3::zero()
            },
            radius: 1.0,
        };
        let ray = Ray::new(
            Point3::zero(),
            Vect3 {
                z: 1.0,
                ..Vect3::zero()
            },
        );

        let scene = Scene {
            surfaces: vec![Box::new(sphere_a), Box::new(sphere_b)],
        };
        assert_eq!(
            ray.intersects(&scene, 0.0..f32::INFINITY)
                .map(|match_| match_.point()),
            Some(Point3 {
                z: 1.0,
                ..Point3::zero()
            })
        );
        assert_eq!(
            ray.intersects(&scene, 2.0..f32::INFINITY)
                .map(|match_| match_.point()),
            Some(Point3 {
                z: 3.0,
                ..Point3::zero()
            })
        );
    }
}
