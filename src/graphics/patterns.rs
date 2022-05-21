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

/// A simple ring color pattern.
#[derive(Clone, Debug, PartialEq)]
pub struct RingPattern {
  a: Color,
  b: Color,
}

impl RingPattern {
  /// Creates a new ring pattern with the given colors.
  pub fn new(a: Color, b: Color) -> Self {
    Self { a, b }
  }
}


impl ColorPattern for RingPattern {
  fn sample_at(&self, point: Vector) -> Color {
    let x2 = point.x * point.x;
    let z2 = point.z * point.z;

    if (x2 + z2).sqrt().floor() % 2. == 0. {
      self.a
    } else {
      self.b
    }
  }
}

/// A simple checker color pattern.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckerPattern {
  a: Color,
  b: Color,
}

impl CheckerPattern {
  /// Creates a new checker pattern with the given colors.
  pub fn new(a: Color, b: Color) -> Self {
    Self { a, b }
  }
}

impl ColorPattern for CheckerPattern {
  fn sample_at(&self, point: Vector) -> Color {
    if (point.x.floor() + point.y.floor() + point.z.floor()) % 2. == 0. {
      self.a
    } else {
      self.b
    }
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

  #[test]
  fn ring_pattern_should_extend_in_both_x_and_z() {
    let pattern = RingPattern::new(Color::WHITE, Color::BLACK);

    assert_eq!(pattern.sample_at(point(0., 0., 0.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(1., 0., 0.)), Color::BLACK);
    assert_eq!(pattern.sample_at(point(0., 0., 1.)), Color::BLACK);
    assert_eq!(pattern.sample_at(point(0.708, 0., 1.)), Color::BLACK);
  }

  #[test]
  fn checker_pattern_should_repeat_in_x() {
    let pattern = CheckerPattern::new(Color::WHITE, Color::BLACK);

    assert_eq!(pattern.sample_at(point(0., 0., 0.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(0., 0.99, 0.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(0., 1.01, 0.)), Color::BLACK);
  }

  #[test]
  fn checker_pattern_should_repeat_in_z() {
    let pattern = CheckerPattern::new(Color::WHITE, Color::BLACK);

    assert_eq!(pattern.sample_at(point(0., 0., 0.)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(0., 0., 0.99)), Color::WHITE);
    assert_eq!(pattern.sample_at(point(0., 0., 1.01)), Color::BLACK);
  }
}