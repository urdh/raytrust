// Imports.
use crate::image;
use crate::materials::*;
use crate::surfaces::*;
use crate::types::Ray;
use std::cmp::Ordering;
use std::ops::Range;

/// An object, defined as a surface with a material.
pub struct Object {
    pub surface: Box<dyn Surface>,
    pub material: Box<dyn Material>,
}

/// A full, renderable "scene".
pub struct Scene {
    pub objects: Vec<Object>,
}

impl Ray {
    /// Check whether a ray intersects any surface in a scene.
    ///
    /// # Arguments
    ///
    /// * `ray` - the ray to trace along
    /// * `scene` - the scene to intersect in
    /// * `filter` - a distance range in which to intersect
    fn intersects<'a>(
        &self,
        scene: &'a Scene,
        filter: Range<f32>,
    ) -> Option<(Intersection, &'a dyn Material)> {
        scene
            .objects
            .iter()
            .flat_map(|object| {
                object
                    .surface
                    .intersected_by(self, filter.clone())
                    .into_iter()
                    .map(move |intersection| (intersection, &*object.material))
            })
            .map(|match_| (match_, (match_.0.point() - self.origin()).norm()))
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
        if let Some((intersection, material)) = ray.intersects(self, 0.001..f32::INFINITY) {
            // We have an intersection! Scatter the ray, then average the attenuated
            // color of each scattered ray to get the color of the pixel.
            let scatters = material.scatter_at(ray, &intersection);
            let acc = scatters
                .iter()
                .map(|(reflected, attenuation)| {
                    self.render_ray(reflected, depth - 1)
                        * image::Pixel(attenuation.red(), attenuation.green(), attenuation.blue())
                })
                .fold(image::Pixel::default(), |acc, pixel| acc + pixel);
            if !scatters.is_empty() {
                acc / (scatters.len() as f32)
            } else {
                image::Pixel::default()
            }
        } else {
            // Fall-back: fancy blue-ish gradient
            let t = 0.5 * (ray.direction().y() + 1.0);
            ((1.0 - t) * image::Pixel(1.0, 1.0, 1.0)) + (t * image::Pixel(0.5, 0.7, 1.0))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{Point3, Vect3};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_intersection_filter() {
        let material = Lambertian::new(Color(1.0, 1.0, 1.0));
        let sphere = Sphere {
            center: Point3(0.0, 0.0, 2.0),
            radius: 1.0,
        };
        let ray = Ray::new(Point3::zero(), Vect3(0.0, 0.0, 1.0));

        let scene = Scene {
            objects: vec![Object {
                surface: Box::new(sphere),
                material: Box::new(material),
            }],
        };
        assert!(ray.intersects(&scene, 0.0..f32::INFINITY).is_some());
        assert!(ray.intersects(&scene, 0.0..0.5).is_none());
        assert!(ray.intersects(&scene, 1.5..2.0).is_none());
    }

    #[test]
    fn test_multiple_objects() {
        let material = Lambertian::new(Color(1.0, 1.0, 1.0));
        let sphere_a = Sphere {
            center: Point3(0.0, 0.0, 2.0),
            radius: 1.0,
        };
        let sphere_b = Sphere {
            center: Point3(0.0, 0.0, 4.0),
            radius: 1.0,
        };
        let ray = Ray::new(Point3::zero(), Vect3(0.0, 0.0, 1.0));

        let scene = Scene {
            objects: vec![
                Object {
                    surface: Box::new(sphere_a),
                    material: Box::new(material),
                },
                Object {
                    surface: Box::new(sphere_b),
                    material: Box::new(material),
                },
            ],
        };
        assert_eq!(
            ray.intersects(&scene, 0.0..f32::INFINITY)
                .map(|(intersection, _)| intersection.point()),
            Some(Point3(0.0, 0.0, 1.0))
        );
        assert_eq!(
            ray.intersects(&scene, 2.0..f32::INFINITY)
                .map(|(intersection, _)| intersection.point()),
            Some(Point3(0.0, 0.0, 3.0))
        );
    }
}
