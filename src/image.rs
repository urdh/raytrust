use std::ops::{Index, IndexMut};
use std::slice::{ChunksExact, ChunksExactMut};

/// Pixels are represented using three floating-point color channels,
/// with range from `0.0` to `1.0`. There is no alpha channel.
#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Default for Pixel {
    /// Default-constructs a completely black pixel.
    fn default() -> Pixel {
        Pixel {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

/// An image is a two-dimensional matrix of pixels, with its origin
/// in the top left corner.
#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Box<[Pixel]>,
}

impl Image {
    /// Contstructs an image with a given size.
    ///
    /// # Arguments
    ///
    /// * `width` - the width of the image (number of columns)
    /// * `height` - the height of the image (number of rows)
    ///
    /// # Example
    ///
    /// ```
    /// use raytrust::image::Image;
    /// let image = Image::new(32, 32);
    /// assert_eq!(image.width(), 32);
    /// assert_eq!(image.height(), 32);
    /// ```
    pub fn new(width: usize, height: usize) -> Image {
        let pixels = vec![Pixel::default(); width * height].into_boxed_slice();
        Image {
            width,
            height,
            pixels,
        }
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns an iterator over rows of the image.
    ///
    /// # Example
    ///
    /// ```
    /// use raytrust::image::Image;
    /// let image = Image::new(32, 32);
    /// assert_eq!(image.iter().count(), image.height());
    /// ```
    pub fn iter(&self) -> ChunksExact<'_, Pixel> {
        self.pixels.chunks_exact(self.width)
    }

    /// Returns an iterator that allows modifying each row.
    ///
    /// # Example
    ///
    /// ```
    /// use raytrust::image::{Image, Pixel};
    /// let mut image = Image::new(4, 4);
    /// for row in image.iter_mut() {
    ///     row[1] = Pixel { r: 0.5, g: 0.5, b: 0.5 };
    /// }
    /// assert_eq!(image[0][1].r, 0.5);
    /// ```
    pub fn iter_mut(&mut self) -> ChunksExactMut<'_, Pixel> {
        self.pixels.chunks_exact_mut(self.width)
    }
}

impl Index<usize> for Image {
    type Output = [Pixel];

    fn index(&self, index: usize) -> &Self::Output {
        &self.iter().nth(index).expect("Index out of range")
    }
}

impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.iter_mut().nth(index).expect("Index out of range")
    }
}

impl<'a> IntoIterator for &'a Image {
    type Item = &'a [Pixel];
    type IntoIter = ChunksExact<'a, Pixel>;

    fn into_iter(self) -> ChunksExact<'a, Pixel> {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Image {
    type Item = &'a mut [Pixel];
    type IntoIter = ChunksExactMut<'a, Pixel>;

    fn into_iter(self) -> ChunksExactMut<'a, Pixel> {
        self.iter_mut()
    }
}

#[cfg(test)]
mod test {}
