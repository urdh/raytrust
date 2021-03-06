use auto_ops::*;
use std::ops::{Index, IndexMut};
use std::slice::{ChunksExact, ChunksExactMut};

/// Pixels are represented using three floating-point color channels,
/// with range from `0.0` to `1.0`. There is no alpha channel.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Pixel(pub f32, pub f32, pub f32);

impl Pixel {
    /// The red channel of the pixel.
    pub fn red(&self) -> f32 {
        self.0
    }

    /// The green channel of the pixel.
    pub fn green(&self) -> f32 {
        self.1
    }

    /// The blue channel of the pixel.
    pub fn blue(&self) -> f32 {
        self.2
    }
}

impl_op_ex!(+= |a: &mut Pixel, b: &Pixel| { *a = *a + b; });
impl_op_ex!(+|a: &Pixel, b: &Pixel| -> Pixel { Pixel(a.0 + b.0, a.1 + b.1, a.2 + b.2) });

impl_op_ex!(-= |a: &mut Pixel, b: &Pixel| { *a = *a - b; });
impl_op_ex!(-|a: &Pixel, b: &Pixel| -> Pixel { Pixel(a.0 - b.0, a.1 - b.1, a.2 - b.2) });

impl_op_ex!(*= |a: &mut Pixel, b: &Pixel| { *a = *a * b; });
impl_op_ex!(*|a: &Pixel, b: &Pixel| -> Pixel { Pixel(a.0 * b.0, a.1 * b.1, a.2 * b.2) });

impl_op_ex!(*= |a: &mut Pixel, b: &f32| { *a = *a * b; });
impl_op_ex_commutative!(*|a: &Pixel, b: &f32| -> Pixel { Pixel(a.0 * b, a.1 * b, a.2 * b) });

impl_op_ex!(/= |a: &mut Pixel, b: &f32| { *a = *a / b; });
impl_op_ex!(/|a: &Pixel, b: &f32| -> Pixel { Pixel(a.0 / b, a.1 / b, a.2 / b) });

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
    /// ```ignore
    /// use image::Image;
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
    /// ```ignore
    /// use image::Image;
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
    /// ```ignore
    /// use image::{Image, Pixel};
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
        self.iter().nth(index).expect("Index out of range")
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
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_pixel_default_is_black() {
        let expected = Pixel(0.0, 0.0, 0.0);
        assert_eq!(Pixel::default(), expected);
    }

    #[test]
    fn test_image_size_accessors() {
        let image = Image::new(32, 8);
        assert_eq!(image.width(), 32);
        assert_eq!(image.height(), 8);
    }

    #[test]
    fn test_image_index() {
        let gray = |v: f32| Pixel(v, v, v);
        let mut image = Image::new(2, 2);
        for idx in 0..image.pixels.len() {
            image.pixels[idx] = gray((idx as f32) / 10.0);
        }
        assert_eq!(image[0], vec![gray(0.0), gray(0.1)]);
        assert_eq!(image[1], vec![gray(0.2), gray(0.3)]);
    }

    #[test]
    fn test_image_iter() {
        let image = Image::new(32, 8);
        assert_eq!(image.iter().count(), image.height());
        assert!(image.iter().all(|row| row.len() == image.width()));
    }

    #[test]
    fn test_image_into_iter() {
        let image = Image::new(32, 8);
        for row in &image {
            for pixel in row {
                assert_eq!(*pixel, Pixel::default());
            }
        }
    }
}
