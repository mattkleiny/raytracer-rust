//! Patterns for shape rendering.

use crate::maths::{Color, Matrix4x4, Vector};

/// A pattern that can be independently transformed.
#[derive(Clone, Debug, PartialEq)]
pub struct TransformPattern<P> {
  pattern: P,
  transform: Matrix4x4,
}

impl<P> TransformPattern<P> {
  /// Creates a new color pattern that is transformed in space.
  pub fn new(pattern: P) -> Self {
    Self {
      pattern,
      transform: Matrix4x4::identity(),
    }
  }

  /// Modifies the pattern with the given transform.
  pub fn with_transform(self, transform: Matrix4x4) -> Self {
    Self { transform: self.transform * transform, ..self }
  }
}

impl<P> ColorPattern for TransformPattern<P> where P: ColorPattern {
  fn sample_at(&self, mut point: Vector) -> Color {
    if let Ok(inverse) = self.transform.invert() {
      point = inverse * point;
    }

    self.pattern.sample_at(point)
  }
}

/// Represents a pattern that can produces colors at distinct points on an object.
pub trait ColorPattern {
  /// Samples the color of the pattern at the given point.
  fn sample_at(&self, point: Vector) -> Color;
}

/// A simple striped color pattern.
#[derive(Clone, Debug, PartialEq)]
pub struct StripedPattern {
  a: Color,
  b: Color,
}

impl StripedPattern {
  /// Creates a new striped pattern with the given colors.
  pub fn new(a: Color, b: Color) -> Self {
    Self { a, b }
  }
}

impl ColorPattern for StripedPattern {
  fn sample_at(&self, point: Vector) -> Color {
    if (point.x.floor() % 2.) == 0. {
      self.a
    } else {
      self.b
    }
  }
}

/// A simple gradient color pattern.
#[derive(Clone, Debug, PartialEq)]
pub struct GradientPattern {
  a: Color,
  b: Color,
}

impl GradientPattern {
  /// Creates a new gradient pattern with the given colors.
  pub fn new(a: Color, b: Color) -> Self {
    Self { a, b }
  }
}

impl ColorPattern for GradientPattern {
  fn sample_at(&self, point: Vector) -> Color {
    let distance = self.b - self.a;
    let fraction = point.x - point.x.floor();

    self.a + distance * fraction
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{point, rgb};

  use super::*;

  #[test]
  fn striped_pattern_is_constant_in_y() {
    let pattern = StripedPattern::new(Color::WHITE, Color::BLACK);

    assert_eq!(pattern.sample_at(point(0., 0., 0.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(0., 1., 0.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(0., 2., 0.)), Color::WHITE);
  }

  #[test]
  fn striped_pattern_is_constant_in_z() {
    let pattern = StripedPattern::new(Color::WHITE, Color::BLACK);

    assert_eq!(pattern.sample_at(point(0., 0., 0.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(0., 0., 1.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(0., 0., 2.)), Color::WHITE);
  }

  #[test]
  fn striped_pattern_alternates_in_x() {
    let pattern = StripedPattern::new(Color::WHITE, Color::BLACK);

    assert_eq!(pattern.sample_at(point(0., 0., 0.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(0.9, 0., 0.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(1., 0., 0.)), Color::BLACK);
    assert_eq!(pattern.sample_at(point(-0.1, 0., 0.)), Color::BLACK);
    assert_eq!(pattern.sample_at(point(-1., 0., 0.)), Color::BLACK);
    assert_eq!(pattern.sample_at(point(-1.1, 0., 0.)), Color::WHITE);
  }

  #[test]
  fn gradient_pattern_linearly_interpolates_between_colors() {
    let pattern = GradientPattern::new(Color::WHITE, Color::BLACK);

    assert_eq!(pattern.sample_at(point(0., 0., 0.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(0.25, 0., 0.)), rgb(0.75, 0.75, 0.75));
    assert_eq!(pattern.sample_at(point(0.5, 0., 0.)), rgb(0.5, 0.5, 0.5));
    assert_eq!(pattern.sample_at(point(0.75, 0., 0.)), rgb(0.25, 0.25, 0.25));
  }
}