use std::ops::{Index, IndexMut};
use std::slice::{ChunksExact, ChunksExactMut};

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Box<[Pixel]>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let pixels = vec![Pixel::default(); width * height].into_boxed_slice();
        Image {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter(&self) -> ChunksExact<'_, Pixel> {
        self.pixels.chunks_exact(self.width)
    }

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
