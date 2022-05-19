//! Ray projection and arithmetic.

use super::Tuple;

/// A ray is a line segment in 3-space with a starting point and a direction.
#[derive(Copy, Clone, Debug)]
pub struct Ray {
  pub origin: Tuple,
  pub direction: Tuple,
}

impl Ray {
  /// Creates a new ray.
  #[inline]
  pub fn new(origin: Tuple, direction: Tuple) -> Self {
    Self {
      origin,
      direction,
    }
  }

  /// Computes the position of the ray at a given distance from it's origin.
  pub fn position(&self, distance: f32) -> Tuple {
    self.origin + self.direction * distance
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{point, Ray, vec};

  #[test]
  fn ray_should_expose_basic_properties() {
    let origin = point(1., 2., 3.);
    let direction = vec(4., 5., 6.);

    let ray = Ray::new(origin, direction);

    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
  }

  #[test]
  fn ray_should_compute_position() {
    let origin = point(2., 3., 4.);
    let direction = vec(1., 0., 0.);

    let ray = Ray::new(origin, direction);

    assert_eq!(ray.position(0.), origin);
    assert_eq!(ray.position(1.), point(3., 3., 4.));
    assert_eq!(ray.position(-1.), point(1., 3., 4.));
    assert_eq!(ray.position(2.5), point(4.5, 3., 4.));
  }
}