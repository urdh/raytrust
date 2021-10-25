use std::io;

mod image;
mod types;

pub use image::Image;
use types::{Point3, Ray, Vect3};

/// Render the color for a specific pixel.
fn render_ray(ray: Ray) -> image::Pixel {
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
pub fn render(width: usize, height: usize) -> Image {
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

    // Render the image!
    let pb = indicatif::ProgressBar::new_spinner();
    for (y, row) in image.iter_mut().rev().enumerate() {
        pb.set_message(format!("Rendering line {}/{}", (y + 1), height));
        for (x, pixel) in row.iter_mut().enumerate() {
            let u = (x as f32) / ((width as f32) - 1.0);
            let v = (y as f32) / ((height as f32) - 1.0);
            let dir = lower_left_corner + (u * horiz) + (v * vert) - origin;
            let ray = Ray::new(origin, dir);
            *pixel = render_ray(ray);
        }
        pb.tick();
    }
    pb.finish();

    image
}

/// Serialize an image using the PGM format.
///
/// # Arguments
///
/// * `stream` - writer/sink to serialize image into
/// * `image` - image to serialize
///
/// # Example
///
/// ```
/// use raytrust::{Image, write_pgm};
/// let image = Image::new(8, 8);
/// write_pgm(&mut std::io::stdout(), &image);
/// ```
pub fn write_pgm(stream: &mut (dyn io::Write), image: &Image) -> Result<(), io::Error> {
    let pb = indicatif::ProgressBar::new_spinner().with_message("Saving image");
    writeln!(stream, "P3")?;
    writeln!(stream, "{} {}", image.width(), image.height())?;
    writeln!(stream, "255")?;
    for row in image {
        for pixel in row {
            writeln!(
                stream,
                "{} {} {}",
                ((pixel.r * 255.0).round() as u8),
                ((pixel.g * 255.0).round() as u8),
                ((pixel.b * 255.0).round() as u8)
            )?;
        }
        pb.tick();
    }
    pb.finish();
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
        write_pgm(&mut vec, &image)?;

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
