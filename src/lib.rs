use rand::{thread_rng, Rng};
use std::io;

mod camera;
mod image;
mod scene;
mod surfaces;
mod types;

use camera::Camera;
pub use image::Image;
use scene::{intersects, Scene};
use surfaces::Sphere;
use types::{Point3, Ray, Vect3};

/// Render the color for a specific pixel.
fn render_ray(ray: &Ray, scene: &Scene) -> image::Pixel {
    if let Some(intersection) = intersects(ray, scene, 0.0..f32::INFINITY) {
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

/// Get a sample scene containing sample surfaces.
pub fn get_scene() -> Scene {
    Scene {
        surfaces: vec![
            Box::new(Sphere {
                center: Point3 {
                    z: -1.0,
                    ..Point3::zero()
                },
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Point3 {
                    y: -100.5,
                    z: -1.0,
                    ..Point3::zero()
                },
                radius: 100.0,
            }),
        ],
    }
}

/// Render an image by raytracing.
///
/// # Arguments
///
/// * `scene` - scene to render
/// * `width` - output image width
/// * `height` - output image height
/// * `samples` - samples per pixel
/// * `callback` - callback called when a row has been rendered
pub fn render<F>(
    scene: &Scene,
    width: usize,
    height: usize,
    samples: usize,
    mut callback: F,
) -> Image
where
    F: FnMut(usize),
{
    let mut image = Image::new(width, height);
    let mut rng = thread_rng();

    // Viewport & focal length
    let aspect_ratio = width as f32 / height as f32;
    let viewport = (2.0 * aspect_ratio, 2.0);
    let focal_length = 1.0;

    // Camera definition
    let direction = Vect3 {
        z: 1.0,
        ..Vect3::zero()
    };
    let camera = Camera::new(Point3::zero(), direction, focal_length, viewport);

    // Render the image!
    for (y, row) in image.iter_mut().rev().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let acc = (0..samples)
                .map(|_| {
                    let u = ((x as f32) + rng.gen_range(0.0..1.0)) / ((width as f32) - 1.0);
                    let v = ((y as f32) + rng.gen_range(0.0..1.0)) / ((height as f32) - 1.0);
                    render_ray(&camera.ray(u, v), scene)
                })
                .fold(image::Pixel::default(), |acc, pixel| image::Pixel {
                    r: acc.r + pixel.r,
                    g: acc.g + pixel.g,
                    b: acc.b + pixel.b,
                });
            *pixel = image::Pixel {
                r: acc.r / (samples as f32),
                g: acc.g / (samples as f32),
                b: acc.b / (samples as f32),
            };
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
/// * `callback` - callback called when a row has been rendered
///
/// # Example
///
/// ```
/// use raytrust::{Image, write_pgm};
/// let image = Image::new(8, 8);
/// write_pgm(&mut std::io::stdout(), &image, |_: usize| ());
/// ```
pub fn write_pgm<F>(
    stream: &mut (dyn io::Write),
    image: &Image,
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
                ((pixel.r * 255.0).round() as u8),
                ((pixel.g * 255.0).round() as u8),
                ((pixel.b * 255.0).round() as u8)
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
        image[0][0] = image::Pixel {
            r: 1.0,
            g: 0.5,
            b: 0.0,
        };
        image[1][0] = image::Pixel {
            r: 1.25,
            g: -1.25,
            b: 0.0,
        };

        let mut vec: Vec<u8> = Vec::new();
        write_pgm(&mut vec, &image, |_: usize| ())?;

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
