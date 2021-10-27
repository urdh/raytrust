use std::io;

mod image;
mod scene;
mod types;

pub use image::Image;
use scene::{intersects, Object, Sphere};
use types::{Point3, Ray, Vect3};

/// Render the color for a specific pixel.
fn render_ray(ray: &Ray, object: &Object) -> image::Pixel {
    if intersects(ray, object) {
        return image::Pixel {
            r: 1.0,
            ..image::Pixel::default()
        };
    }
    let t = 0.5 * (ray.direction().y + 1.0);
    image::Pixel {
        r: (1.0 - t) * 1.0 + t * 0.5,
        g: (1.0 - t) * 1.0 + t * 0.7,
        b: (1.0 - t) * 1.0 + t * 1.0,
    }
}

/// Render an image by raytracing.
///
/// # Arguments
///
/// * `width` - output image width
/// * `height` - output image height
/// * `callback` - callback called when a row has been rendered
pub fn render<F>(width: usize, height: usize, mut callback: F) -> Image
where
    F: FnMut(usize),
{
    let mut image = Image::new(width, height);

    // Viewport & focal length
    let viewport_height = 2.0;
    let viewport_width = (width as f32 / height as f32) * viewport_height;
    let focal_length = 1.0;

    // Camera position, horizontal & vertical viewport extents
    let origin = Point3::zero();
    let horiz = Vect3 {
        x: viewport_width,
        ..Vect3::zero()
    };
    let vert = Vect3 {
        y: viewport_height,
        ..Vect3::zero()
    };

    // Lower left corner of the camera view
    let lower_left_corner = origin
        - (horiz / 2.0)
        - (vert / 2.0)
        - Vect3 {
            z: focal_length,
            ..Vect3::zero()
        };

    // The scene (just a simple sphere)
    let sphere = Object::Sphere(Sphere {
        center: Point3 {
            z: -1.0,
            ..Point3::zero()
        },
        radius: 0.5,
    });

    // Render the image!
    for (y, row) in image.iter_mut().rev().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let u = (x as f32) / ((width as f32) - 1.0);
            let v = (y as f32) / ((height as f32) - 1.0);
            let dir = lower_left_corner + (u * horiz) + (v * vert) - origin;
            let ray = Ray::new(origin, dir);
            *pixel = render_ray(&ray, &sphere);
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
