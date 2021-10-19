use std::io;

pub mod image;

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
