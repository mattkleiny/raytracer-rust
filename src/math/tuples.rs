//! Defines basic tuple operations in floating point vector space.

use std::ops::{Add, Div, Mul, Neg, Sub};

use super::ApproxEq;

/// Creates a (X, Y, Z, W) tuple in floating point 4-space.
#[inline]
pub const fn tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
  Tuple { x, y, z, w }
}

/// Creates a new vector; an (X, Y, Z) tuple with the W component at 0.
#[inline]
pub const fn vec(x: f32, y: f32, z: f32) -> Tuple {
  Tuple { x, y, z, w: 0. }
}

/// Creates a new point; an (X, Y, Z) tuple with the W component at 1.
#[inline]
pub const fn point(x: f32, y: f32, z: f32) -> Tuple {
  Tuple { x, y, z, w: 1. }
}

/// A tuple in floating point 4-space, with basic mathematical operations defined.
#[derive(Copy, Clone, Debug)]
pub struct Tuple {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl Tuple {
  /// Does this tuple represent a vector between two points?
  #[inline]
  pub fn is_vector(&self) -> bool {
    self.w.is_approx(0.)
  }

  /// Does this tuple represent a single point in space?
  #[inline]
  pub fn is_point(&self) -> bool {
    self.w.is_approx(1.)
  }

  /// Computes the magnitude of this vector; the length essentially.
  pub fn magnitude(&self) -> f32 {
    let x2 = self.x * self.x;
    let y2 = self.y * self.y;
    let z2 = self.z * self.z;
    let w2 = self.w * self.w;

    (x2 + y2 + z2 + w2).sqrt()
  }

  /// Normalizes the tuple to the range (-1, 1) for all components.
  pub fn normalize(&self) -> Self {
    let magnitude = self.magnitude();

    Self {
      x: self.x / magnitude,
      y: self.y / magnitude,
      z: self.z / magnitude,
      w: self.w / magnitude,
    }
  }

  /// Computes the dot product of this tuple and another.
  ///
  /// The dot product represents the 'shadow' of the other vector on this one.
  pub fn dot(&self, other: &Self) -> f32 {
    let x = self.x * other.x;
    let y = self.y * other.y;
    let z = self.z * other.z;
    let w = self.w * other.w;

    x + y + z + w
  }

  /// Computes the cross product of this tuple and another.
  ///
  /// The cross product is a vector perpendicular to both vectors.
  pub fn cross(&self, other: &Self) -> Self {
    let x = self.y * other.z - self.z * other.y;
    let y = self.z * other.x - self.x * other.z;
    let z = self.x * other.y - self.y * other.x;

    return vec(x, y, z);
  }
}

impl PartialEq for Tuple {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    // equality for tuples is approximate by default for the floating point fields.
    let x = self.x.is_approx(other.x);
    let y = self.y.is_approx(other.y);
    let z = self.z.is_approx(other.z);
    let w = self.w.is_approx(other.w);

    x && y && z && w
  }
}

impl ApproxEq for Tuple {
  #[inline]
  fn is_approx(&self, rhs: Self) -> bool {
    *self == rhs // approximate equality by default
  }
}

impl Neg for Tuple {
  type Output = Self;

  #[inline]
  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: -self.w,
    }
  }
}

impl Add for Tuple {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
      w: self.w + rhs.w,
    }
  }
}

impl Sub for Tuple {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w,
    }
  }
}

impl Mul<f32> for Tuple {
  type Output = Self;

  #[inline]
  fn mul(self, rhs: f32) -> Self::Output {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
      w: self.w * rhs,
    }
  }
}

impl Div<f32> for Tuple {
  type Output = Self;

  #[inline]
  fn div(self, rhs: f32) -> Self::Output {
    Self {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
      w: self.w / rhs,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tuple_should_create_valid_vectors() {
    let tuple = vec(4.3, -4.2, 3.1);

    assert_eq!(tuple.x, 4.3);
    assert_eq!(tuple.y, -4.2);
    assert_eq!(tuple.z, 3.1);
    assert_eq!(tuple.w, 0.0);
    assert!(tuple.is_vector());
    assert!(!tuple.is_point());
  }

  #[test]
  fn tuple_should_create_valid_points() {
    let tuple = point(4.3, -4.2, 3.1);

    assert_eq!(tuple.x, 4.3);
    assert_eq!(tuple.y, -4.2);
    assert_eq!(tuple.z, 3.1);
    assert_eq!(tuple.w, 1.0);
    assert!(!tuple.is_vector());
    assert!(tuple.is_point());
  }

  #[test]
  fn tuples_should_exhibit_equality() {
    let a = vec(3., -2., 5.);
    let b = vec(-2., 3., 1.);
    let c = vec(3., -2., 5.);
    let d = point(3., -2., 5.);

    assert_eq!(a, c);
    assert_eq!(c, a);
    assert_ne!(a, b);
    assert_ne!(b, a);
    assert_ne!(a, d);
    assert_ne!(b, d);
  }

  #[test]
  fn tuples_should_negate() {
    let a = tuple(1., -2., 3., -4.);

    assert_eq!(-a, tuple(-1., 2., -3., 4.));
  }

  #[test]
  fn tuples_should_add() {
    let a = vec(3., -2., 5.);
    let b = vec(-2., 3., 1.);

    assert_eq!(a + b, vec(1., 1., 6.));
  }

  #[test]
  fn tuples_should_subtract_two_points() {
    let a = point(3., 2., 1.);
    let b = point(5., 6., 7.);

    assert_eq!(a - b, vec(-2., -4., -6.));
  }

  #[test]
  fn tuples_should_subtract_two_vectors() {
    let a = vec(3., 2., 1.);
    let b = vec(5., 6., 7.);

    assert_eq!(a - b, vec(-2., -4., -6.));
  }

  #[test]
  fn tuples_should_subtract_vector_from_point() {
    let a = point(3., 2., 1.);
    let b = vec(5., 6., 7.);

    assert_eq!(a - b, point(-2., -4., -6.));
  }

  #[test]
  fn tuples_should_multiply_by_a_scalar() {
    let a = tuple(1., -2., 3., -4.);

    assert_eq!(a * 3.5, tuple(3.5, -7., 10.5, -14.));
  }

  #[test]
  fn tuples_should_multiply_by_a_fraction() {
    let a = tuple(1., -2., 3., -4.);

    assert_eq!(a * 0.5, tuple(0.5, -1., 1.5, -2.));
  }

  #[test]
  fn tuples_should_divide_by_scalar() {
    let a = tuple(1., -2., 3., -4.);

    assert_eq!(a / 2., tuple(0.5, -1., 1.5, -2.));
  }

  #[test]
  fn tuples_should_compute_magnitude_of_unit_x() {
    assert_eq!(1., vec(1., 0., 0.).magnitude());
  }

  #[test]
  fn tuples_should_compute_magnitude_of_unit_y() {
    assert_eq!(1., vec(0., 1., 0.).magnitude());
  }

  #[test]
  fn tuples_should_compute_magnitude_of_unit_z() {
    assert_eq!(1., vec(0., 0., 1.).magnitude());
  }

  #[test]
  fn tuples_should_compute_magnitude_of_positive_vector() {
    assert_eq!(14f32.sqrt(), vec(1., 2., 3.).magnitude());
  }

  #[test]
  fn tuples_should_compute_magnitude_of_negative_vector() {
    assert_eq!(14f32.sqrt(), vec(-1., -2., -3.).magnitude());
  }

  #[test]
  fn tuples_should_normalize_unit_x() {
    assert_eq!(vec(4., 0., 0.).normalize(), vec(1., 0., 0.));
  }

  #[test]
  fn tuples_should_normalize_unit_y() {
    assert_eq!(vec(0., 20., 0.).normalize(), vec(0., 1., 0.));
  }

  #[test]
  fn tuples_should_normalize_unit_z() {
    assert_eq!(vec(0., 0., -8.).normalize(), vec(0., 0., -1.));
  }

  #[test]
  fn tuples_should_have_unit_length_normalized_vector() {
    assert!(vec(1., 2., 3.).normalize().magnitude().is_approx(1.));
  }

  #[test]
  fn tuples_should_compute_a_dot_product() {
    let a = vec(1., 2., 3.);
    let b = vec(2., 3., 4.);

    assert_eq!(a.dot(&b), 20.);
  }

  #[test]
  fn tuples_should_compute_a_cross_product() {
    let a = vec(1., 2., 3.);
    let b = vec(2., 3., 4.);

    let cross = a.cross(&b);

    assert_eq!(cross, vec(-1., 2., -1.));
  }
}