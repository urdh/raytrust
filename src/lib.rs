use rand::{thread_rng, Rng};
use std::io;

mod camera;
mod image;
mod materials;
mod scene;
mod surfaces;
mod types;

use camera::Camera;
pub use image::Image;
use materials::{Color, Dielectric, Hemispherical, Lambertian, Metal};
use scene::{Object, Scene};
use surfaces::Sphere;
use types::{Point3, Vect3};

pub fn get_scene(aspect_ratio: f32) -> (Camera, Scene) {
    // Viewport size.
    let viewport = (2.0 * aspect_ratio, 2.0_f32);

    // The angle of view can be determined using the diagonal image plane dimension `d`
    // and the focal length `f`, using the relation `aov = 2 * arctan(d / 2f)`. Working
    // backwards, we can calculate `f` as `f = d / (2 * tan(aov / 2))`.
    // Note: Since we use diagonal field-of-view, exact angles will differ compared to
    // the book (depending on aspect ratio).
    let angle_of_view = 40.0_f32.to_radians();
    let diagonal = (viewport.0.powi(2) + viewport.1.powi(2)).sqrt();
    let focal_length = (diagonal / 2.0) / (angle_of_view / 2.0).tan();
    let aperture = 16.0;

    // Camera definition
    let origin = Point3(-2.0, 2.0, 1.0);
    let target = Point3(0.0, 0.0, -1.0);
    let vertical = Vect3(0.0, 1.0, 0.0);
    let camera = Camera::new(origin, target, vertical, focal_length, aperture, viewport);

    // Small sample scene containing sample surfaces.
    let scene = Scene {
        objects: vec![
            // Left side hollow dielectric sphere.
            Object {
                surface: Box::new(Sphere {
                    center: Point3(-1.0, 0.0, -1.0),
                    radius: 0.5,
                }),
                material: Box::new(Dielectric::new(Color(1.0, 1.0, 1.0), 1.5)),
            },
            Object {
                surface: Box::new(Sphere {
                    center: Point3(-1.0, 0.0, -1.0),
                    radius: -0.4,
                }),
                material: Box::new(Dielectric::new(Color(1.0, 1.0, 1.0), 1.5)),
            },
            // Center diffuse sphere.
            Object {
                surface: Box::new(Sphere {
                    center: Point3(0.0, 0.0, -1.0),
                    radius: 0.5,
                }),
                material: Box::new(Lambertian::new(Color(0.1, 0.2, 0.5))),
            },
            // Right side metal sphere.
            Object {
                surface: Box::new(Sphere {
                    center: Point3(1.0, 0.0, -1.0),
                    radius: 0.5,
                }),
                material: Box::new(Metal::new(Color(0.8, 0.6, 0.2), 0.0)),
            },
            // "Ground" sphere.
            Object {
                surface: Box::new(Sphere {
                    center: Point3(0.0, -100.5, -1.0),
                    radius: 100.0,
                }),
                material: Box::new(Hemispherical::new(Color(0.8, 0.8, 0.0))),
            },
        ],
    };

    // Return the camera & scene.
    (camera, scene)
}

/// Render an image by raytracing.
///
/// # Arguments
///
/// * `scene` - scene to render
/// * `width` - output image width
/// * `height` - output image height
/// * `samples` - samples per pixel
/// * `depth` - recursion depth
/// * `callback` - callback called when a row has been rendered
pub fn render<F>(
    scene: &Scene,
    camera: &Camera,
    width: usize,
    height: usize,
    samples: usize,
    depth: usize,
    mut callback: F,
) -> Image
where
    F: FnMut(usize),
{
    let mut image = Image::new(width, height);
    let mut rng = thread_rng();

    // Render the image!
    for (y, row) in image.iter_mut().rev().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let acc = (0..samples)
                .map(|_| {
                    let u = ((x as f32) + rng.gen_range(0.0..1.0)) / ((width as f32) - 1.0);
                    let v = ((y as f32) + rng.gen_range(0.0..1.0)) / ((height as f32) - 1.0);
                    scene.render_ray(&camera.ray(u, v), depth)
                })
                .fold(image::Pixel::default(), |acc, pixel| acc + pixel);
            *pixel = acc / (samples as f32);
        }
        callback(height - y);
    }

    image
}

/// Serialize an image using the PGM format.
///
/// # Arguments
///
/// * `stream` - writer/sink to serialize image into
/// * `image` - image to serialize
/// * `gamma` - gamma correction to apply
/// * `callback` - callback called when a row has been rendered
///
/// # Example
///
/// ```
/// use raytrust::{Image, write_pgm};
/// let image = Image::new(8, 8);
/// write_pgm(&mut std::io::stdout(), &image, 2.2, |_: usize| ());
/// ```
pub fn write_pgm<F>(
    stream: &mut (dyn io::Write),
    image: &Image,
    gamma: f32,
    mut callback: F,
) -> Result<(), io::Error>
where
    F: FnMut(usize),
{
    writeln!(stream, "P3")?;
    writeln!(stream, "{} {}", image.width(), image.height())?;
    writeln!(stream, "255")?;
    for (y, row) in image.iter().enumerate() {
        for pixel in row {
            writeln!(
                stream,
                "{} {} {}",
                ((pixel.red().powf(gamma.recip()) * 255.0).round() as u8),
                ((pixel.green().powf(gamma.recip()) * 255.0).round() as u8),
                ((pixel.blue().powf(gamma.recip()) * 255.0).round() as u8)
            )?;
        }
        callback(y + 1);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_write_pgm() -> Result<(), io::Error> {
        let mut image = Image::new(1, 2);
        image[0][0] = image::Pixel(1.0, 0.5, 0.0);
        image[1][0] = image::Pixel(1.25, -1.25, 0.0);

        let mut vec: Vec<u8> = Vec::new();
        write_pgm(&mut vec, &image, 1.0, |_: usize| ())?;

        let expected = indoc::indoc! {"
            P3
            1 2
            255
            255 128 0
            255 0 0
        "};

        assert_eq!(expected, std::str::from_utf8(&vec).unwrap());
        Ok(())
    }
}
