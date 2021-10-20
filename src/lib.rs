use std::io;

pub mod image;

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
/// use raytrust::{image, write_pgm};
/// let image = image::Image::new(8, 8);
/// write_pgm(&mut std::io::stdout(), &image);
/// ```
pub fn write_pgm(stream: &mut (dyn io::Write), image: &image::Image) -> Result<(), io::Error> {
    writeln!(stream, "P3")?;
    writeln!(stream, "{} {}", image.width(), image.height())?;
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
