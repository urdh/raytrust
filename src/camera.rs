use crate::types::{Point3, Ray, Vect3};
use rand::{thread_rng, Rng};
use rand_distr::Uniform;
use std::f32::consts::PI;

/// Pick a random point in an isosceles triangle that is
/// symmetric around the horizontal axis.
///
/// See <https://mathworld.wolfram.com/TrianglePointPicking.html>.
fn rand_point_in_triangle(angle: f32) -> Vect3 {
    let mut rng = thread_rng();
    let up = Vect3((angle / 2.0).cos(), (angle / 2.0).sin(), 0.0);
    let down = Vect3(up.x(), -up.y(), up.z());
    let u = rng.sample(Uniform::new_inclusive(0.0, 1.0));
    let v = rng.sample(Uniform::new_inclusive(0.0, 1.0));
    let point = u * up + v * down;
    if point.x() > up.x() {
        Vect3(up.x(), 0.0, 0.0) - point
    } else {
        point
    }
}

/// A camera abstraction.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Point3,
    corner: Point3,
    camera_cs: (Vect3, Vect3, Vect3),
    image_plane: (Vect3, Vect3),
    radius: f32,
}

impl Camera {
    /// Create a new camera.
    ///
    /// # Arguments
    ///
    /// * `origin` - the origin of the camera
    /// * `target` - the point at which the camera is focused
    /// * `vertical` - the upward direction of the camera
    /// * `focal_length` - the focal length of the camera
    /// * `aperture` - the aperture of the camera, in f-stops
    /// * `viewport` - width and height of the viewport
    pub fn new(
        origin: Point3,
        target: Point3,
        vertical: Vect3,
        focal_length: f32,
        aperture: f32,
        viewport: (f32, f32),
    ) -> Camera {
        // Assume we want the field-of-view as if the image plane was at distance
        // `a`, but we need to place the image at distance `b` for focusing. This
        // is essentially equivalent to scaling up the size of the image plane by
        // a factor `b/a` while still aiming the rays at `b`.
        let focus_dist = (origin - target).norm();
        let image_scale = (focus_dist / focal_length).abs();
        // The radius of the lens is given by the f-stop aperture.
        let radius = (focal_length / aperture) / 2.0;
        // Calculate the three base vectors of the camera coordinate system.
        let z = (origin - target).normalize();
        let x = vertical.normalize().cross(z).normalize();
        let y = z.cross(x);
        // Compute the lower left corner of the image plane.
        let horiz = image_scale * viewport.0 * x;
        let vert = image_scale * viewport.1 * y;
        let corner = origin - (horiz / 2.0) - (vert / 2.0) - (z * focus_dist);
        // Construct the camera!
        Camera {
            origin,
            corner,
            camera_cs: (x, y, z),
            image_plane: (horiz, vert),
            radius,
        }
    }

    /// Sample a singe point for a regular polygon aperture.
    fn sample_aperture(&self, sides: u32) -> Vect3 {
        let mut rng = thread_rng();
        let angle = 2.0 * PI * (sides as f32).recip();
        // Genetare a random point on an isosceles triangle with angle
        // 2π / N between the legs. Then, rotate this triangle by 2πn / N,
        // where `n` is a random integer in the range [0, N), to get a
        // random point on the regular N-polygon.
        let segment = rng.sample(Uniform::new(0, sides));
        let point = rand_point_in_triangle(angle);
        let s = ((segment as f32) * angle).sin();
        let c = ((segment as f32) * angle).cos();
        let (x, y, _) = &self.camera_cs;
        // In order to keep the area equal to the corresponding perfectly
        // circular aperture, the radius must be scaled. The relative area
        // of the polygon is given by N/2π sin(2π/N), so the radius must
        // be scaled by the square root of the reciprocal of that value.
        let radius = self.radius * (angle / angle.sin()).sqrt();
        radius * ((x * (point.x() * c - point.y() * s)) + (y * (point.x() * s + point.y() * c)))
    }

    /// Get a ray pointing through a specific viewport position.
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let (w, h) = &self.image_plane;
        let offset = self.sample_aperture(5);
        let direction = self.corner + (w * u) + (h * v) - self.origin;
        Ray::new(self.origin + offset, direction - offset)
    }
}
