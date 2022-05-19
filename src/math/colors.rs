//! Defines basic color operations in floating point linear color space.

use std::ops::{Add, Mul, Sub};

use super::ApproxEq;

/// Creates a new color with the given RGB values.
#[inline]
pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
  Color { r, g, b }
}

/// Describes a color in floating point linear color space.
#[derive(Copy, Clone, Debug)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl Color {
  pub const BLACK: Self = rgb(0., 0., 0.);
  pub const RED: Self = rgb(1., 0., 0.);
  pub const GREEN: Self = rgb(0., 1., 0.);
  pub const BLUE: Self = rgb(0., 0., 1.);
  pub const WHITE: Self = rgb(1., 1., 1.);
}

impl PartialEq for Color {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    // equality for colors is approximate by default for the floating point fields.
    let r = self.r.is_approx(other.r);
    let g = self.g.is_approx(other.g);
    let b = self.b.is_approx(other.b);

    r && g && b
  }
}

impl ApproxEq for Color {
  #[inline]
  fn is_approx(&self, rhs: Self) -> bool {
    *self == rhs // approximate equality by default
  }
}

impl Add for Color {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      r: self.r + rhs.r,
      g: self.g + rhs.g,
      b: self.b + rhs.b,
    }
  }
}

impl Sub for Color {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      r: self.r - rhs.r,
      g: self.g - rhs.g,
      b: self.b - rhs.b,
    }
  }
}

impl Mul for Color {
  type Output = Self;

  #[inline]
  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      r: self.r * rhs.r,
      g: self.g * rhs.g,
      b: self.b * rhs.b,
    }
  }
}

impl Mul<f32> for Color {
  type Output = Self;

  #[inline]
  fn mul(self, rhs: f32) -> Self::Output {
    Self {
      r: self.r * rhs,
      g: self.g * rhs,
      b: self.b * rhs,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn colors_are_red_green_blue_tuples() {
    let color = rgb(-0.5, 0.4, 1.7);

    assert_eq!(color.r, -0.5);
    assert_eq!(color.g, 0.4);
    assert_eq!(color.b, 1.7);
  }

  #[test]
  fn colors_should_add() {
    let a = rgb(0.9, 0.6, 0.75);
    let b = rgb(0.7, 0.1, 0.25);

    assert_eq!(a + b, rgb(1.6, 0.7, 1.0));
  }

  #[test]
  fn colors_should_subtract() {
    let a = rgb(0.9, 0.6, 0.75);
    let b = rgb(0.7, 0.1, 0.25);

    assert_eq!(a - b, rgb(0.2, 0.5, 0.5));
  }

  #[test]
  fn colors_should_multiply() {
    let a = rgb(1., 0.2, 0.4);
    let b = rgb(0.9, 1., 0.1);

    assert_eq!(a * b, rgb(0.9, 0.2, 0.04));
  }

  #[test]
  fn colors_should_multiply_by_scalar() {
    let a = rgb(0.2, 0.3, 0.4);

    assert_eq!(a * 2., rgb(0.4, 0.6, 0.8));
  }
}