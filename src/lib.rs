use std::io;

mod image;

pub use image::Image;

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
