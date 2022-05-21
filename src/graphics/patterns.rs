//! Patterns for shape rendering.

use crate::maths::{Color, Vector};

/// Represents a pattern that can produces colors at distinct points on an object.
pub trait ColorPattern {
  /// Samples the color of the pattern at the given point.
  fn sample_at(&self, point: Vector) -> Color;
}

/// A simple striped pattern implementation.
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

#[cfg(test)]
mod tests {
  use crate::maths::point;

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
}