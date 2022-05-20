//! Graphics abstractions and tools.

use image::{ImageBuffer, ImageFormat, ImageResult, Rgba, RgbaImage};

use crate::maths::Color;

/// A canvas is a 2D array of pixels that can be drawn to.
pub struct Canvas {
  width: u32,
  height: u32,
  pixels: Vec<Color>,
}

impl Canvas {
  /// Creates a new canvas with the given width and height with a default black color.
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height,
      pixels: vec![Color::BLACK; (width * height) as usize],
    }
  }

  /// The width of the canvas in pixels.
  pub fn width(&self) -> usize {
    self.width as usize
  }

  /// The height of the canvas in pixels.
  pub fn height(&self) -> usize {
    self.height as usize
  }

  /// Retrieves the pixels at the given (x, y) position in the canvas.
  pub fn get_pixel(&mut self, x: usize, y: usize) -> Color {
    self.pixels[x + y * self.width as usize]
  }

  /// Sets the pixels at the given (x, y) position in the canvas.
  pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
    self.pixels[x + y * self.width as usize] = color;
  }

  /// Fills the canvas with the given color.
  pub fn fill(&mut self, color: Color) {
    self.pixels.fill(color);
  }

  /// Accesses the pixels as a slice of colors.
  pub fn as_slice(&self) -> &[Color] {
    &self.pixels
  }

  /// Converts the canvas to an image of RGBA pixels.
  pub fn to_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut image = RgbaImage::new(self.width, self.height);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
      let color = self.pixels[x as usize + y as usize * self.width as usize];

      *pixel = Rgba([
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
        255,
      ]);
    }

    image
  }

  /// Saves the image to the given path as a .png file.
  pub fn save_to_png(&self, path: &str) -> ImageResult<()> {
    let image = self.to_image();

    image.save_with_format(path, ImageFormat::Png)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn canvas_should_default_to_black_color() {
    let canvas = Canvas::new(10, 20);

    assert_eq!(canvas.width(), 10);
    assert_eq!(canvas.height(), 20);

    for pixel in canvas.as_slice() {
      assert_eq!(*pixel, Color::BLACK);
    }
  }

  #[test]
  fn canvas_should_read_and_write_pixels() {
    let mut canvas = Canvas::new(10, 20);

    canvas.set_pixel(2, 3, Color::RED);

    assert_eq!(canvas.get_pixel(2, 3), Color::RED);
  }

  #[test]
  fn canvas_should_convert_to_image() {
    let canvas = Canvas::new(10, 20);
    let image = canvas.to_image();

    assert_eq!(image.width(), 10);
    assert_eq!(image.height(), 20);

    assert_eq!(image.pixels().len(), 10 * 20);
  }
}
