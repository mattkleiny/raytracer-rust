//! Ray types and arithmetic.

use std::ops::Mul;

use crate::maths::Matrix4x4;

use super::{Point, Vector};

/// A ray is a line segment in 3-space with a starting point and a direction.
#[derive(Copy, Clone, Debug)]
pub struct Ray {
  pub origin: Point,
  pub direction: Vector,
}

impl Ray {
  /// Creates a new ray.
  pub fn new(origin: Point, direction: Vector) -> Self {
    Self {
      origin,
      direction,
    }
  }

  /// Computes the position of the ray at a given distance from it's origin.
  pub fn position(&self, distance: f32) -> Vector {
    self.origin + self.direction * distance
  }
}

impl Mul<Ray> for Matrix4x4 {
  type Output = Ray;

  /// Transforms a ray by a 4x4 matrix.
  fn mul(self, rhs: Ray) -> Self::Output {
    Ray {
      origin: self * rhs.origin,
      direction: self * rhs.direction,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{Matrix4x4, point, Ray, vec3};

  #[test]
  fn ray_should_expose_basic_properties() {
    let origin = point(1., 2., 3.);
    let direction = vec3(4., 5., 6.);

    let ray = Ray::new(origin, direction);

    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
  }

  #[test]
  fn ray_should_compute_position() {
    let origin = point(2., 3., 4.);
    let direction = vec3(1., 0., 0.);

    let ray = Ray::new(origin, direction);

    assert_eq!(ray.position(0.), origin);
    assert_eq!(ray.position(1.), point(3., 3., 4.));
    assert_eq!(ray.position(-1.), point(1., 3., 4.));
    assert_eq!(ray.position(2.5), point(4.5, 3., 4.));
  }

  #[test]
  fn ray_should_translate() {
    let ray = Ray::new(point(1., 2., 3.), vec3(0., 1., 0.));
    let transform = Matrix4x4::translate(3., 4., 5.);

    let translated_ray = transform * ray;

    assert_eq!(translated_ray.origin, point(4., 6., 8.));
    assert_eq!(translated_ray.direction, vec3(0., 1., 0.));
  }

  #[test]
  fn ray_should_scale() {
    let ray = Ray::new(point(1., 2., 3.), vec3(0., 1., 0.));
    let transform = Matrix4x4::scale(2., 3., 4.);

    let scaled_ray = transform * ray;

    assert_eq!(scaled_ray.origin, point(2., 6., 12.));
    assert_eq!(scaled_ray.direction, vec3(0., 3., 0.));
  }
}