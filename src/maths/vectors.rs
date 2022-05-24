//! Tuple types for points and vectors.

use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::maths::Matrix4x4;

use super::ApproxEq;

pub type Point = Vector;

/// Creates a new point; an (X, Y, Z) tuple with the W component at 1.
pub const fn point(x: f64, y: f64, z: f64) -> Vector {
  Vector { x, y, z, w: 1. }
}

/// Creates a new vector; an (X, Y, Z) tuple with the W component at 0.
pub const fn vec3(x: f64, y: f64, z: f64) -> Vector {
  Vector { x, y, z, w: 0. }
}

/// Creates a (X, Y, Z, W) tuple in floating point 4-space.
pub const fn vec4(x: f64, y: f64, z: f64, w: f64) -> Vector {
  Vector { x, y, z, w }
}

/// A tuple in floating point 4-space, with basic mathematical operations defined.
#[derive(Copy, Clone, Debug)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}

impl Vector {
  /// Creates a new vector with the given components.
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
    Self { x, y, z, w }
  }

  /// Does this vector represent a vector between two points?
  pub fn is_vector(&self) -> bool {
    self.w.is_approx(0.)
  }

  /// Does this vector represent a single point in space?
  pub fn is_point(&self) -> bool {
    self.w.is_approx(1.)
  }

  /// Computes the magnitude of this vector; the length essentially.
  pub fn magnitude(&self) -> f64 {
    let x2 = self.x * self.x;
    let y2 = self.y * self.y;
    let z2 = self.z * self.z;
    let w2 = self.w * self.w;

    (x2 + y2 + z2 + w2).sqrt()
  }

  /// Normalizes the vector to the range (-1, 1) for all components.
  pub fn normalize(&self) -> Self {
    let magnitude = self.magnitude();

    Self {
      x: self.x / magnitude,
      y: self.y / magnitude,
      z: self.z / magnitude,
      w: self.w / magnitude,
    }
  }

  /// Computes the dot product of this vector and another.
  ///
  /// The dot product represents the 'shadow' of the other vector on this one.
  pub fn dot(&self, other: Self) -> f64 {
    let x = self.x * other.x;
    let y = self.y * other.y;
    let z = self.z * other.z;
    let w = self.w * other.w;

    x + y + z + w
  }

  /// Computes the cross product of this vector and another.
  ///
  /// The cross product is a vector perpendicular to both vectors.
  pub fn cross(&self, other: Self) -> Self {
    let x = self.y * other.z - self.z * other.y;
    let y = self.z * other.x - self.x * other.z;
    let z = self.x * other.y - self.y * other.x;

    return vec3(x, y, z);
  }

  /// Reflects a vector about the given normal.
  pub fn reflect(self, normal: Self) -> Self {
    self - normal * 2. * self.dot(normal)
  }
}

impl PartialEq for Vector {
  fn eq(&self, other: &Self) -> bool {
    // equality for vectors is approximate by default for the floating point fields.
    let x = self.x.is_approx(other.x);
    let y = self.y.is_approx(other.y);
    let z = self.z.is_approx(other.z);
    let w = self.w.is_approx(other.w);

    x && y && z && w
  }
}

impl Index<usize> for Vector {
  type Output = f64;

  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      3 => &self.w,
      _ => panic!("Index out of range!")
    }
  }
}

impl IndexMut<usize> for Vector {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      3 => &mut self.w,
      _ => panic!("Index out of range!")
    }
  }
}

impl Neg for Vector {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: -self.w,
    }
  }
}

impl Add for Vector {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
      w: self.w + rhs.w,
    }
  }
}

impl Sub for Vector {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w,
    }
  }
}

impl Mul<f64> for Vector {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
      w: self.w * rhs,
    }
  }
}

impl Div<f64> for Vector {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    Self {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
      w: self.w / rhs,
    }
  }
}

impl Mul<Vector> for Matrix4x4 {
  type Output = Vector;

  /// Transforms a vector by a 4x4 matrix.
  fn mul(self, rhs: Vector) -> Self::Output {
    let mut result = vec4(0., 0., 0., 0.);

    for row in 0..4 {
      let x = self[(row, 0)] * rhs.x;
      let y = self[(row, 1)] * rhs.y;
      let z = self[(row, 2)] * rhs.z;
      let w = self[(row, 3)] * rhs.w;

      result[row] = x + y + z + w;
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vector_should_create_valid_vectors() {
    let tuple = vec3(4.3, -4.2, 3.1);

    assert_eq!(tuple.x, 4.3);
    assert_eq!(tuple.y, -4.2);
    assert_eq!(tuple.z, 3.1);
    assert_eq!(tuple.w, 0.0);
    assert!(tuple.is_vector());
    assert!(!tuple.is_point());
  }

  #[test]
  fn vector_should_create_valid_points() {
    let tuple = point(4.3, -4.2, 3.1);

    assert_eq!(tuple.x, 4.3);
    assert_eq!(tuple.y, -4.2);
    assert_eq!(tuple.z, 3.1);
    assert_eq!(tuple.w, 1.0);
    assert!(!tuple.is_vector());
    assert!(tuple.is_point());
  }

  #[test]
  fn vectors_should_exhibit_equality() {
    let a = vec3(3., -2., 5.);
    let b = vec3(-2., 3., 1.);
    let c = vec3(3., -2., 5.);
    let d = point(3., -2., 5.);

    assert_eq!(a, c);
    assert_eq!(c, a);
    assert_ne!(a, b);
    assert_ne!(b, a);
    assert_ne!(a, d);
    assert_ne!(b, d);
  }

  #[test]
  fn vectors_should_negate() {
    let a = vec4(1., -2., 3., -4.);

    assert_eq!(-a, vec4(-1., 2., -3., 4.));
  }

  #[test]
  fn vectors_should_add() {
    let a = vec3(3., -2., 5.);
    let b = vec3(-2., 3., 1.);

    assert_eq!(a + b, vec3(1., 1., 6.));
  }

  #[test]
  fn vectors_should_subtract_two_points() {
    let a = point(3., 2., 1.);
    let b = point(5., 6., 7.);

    assert_eq!(a - b, vec3(-2., -4., -6.));
  }

  #[test]
  fn vectors_should_subtract_two_vectors() {
    let a = vec3(3., 2., 1.);
    let b = vec3(5., 6., 7.);

    assert_eq!(a - b, vec3(-2., -4., -6.));
  }

  #[test]
  fn vectors_should_subtract_vector_from_point() {
    let a = point(3., 2., 1.);
    let b = vec3(5., 6., 7.);

    assert_eq!(a - b, point(-2., -4., -6.));
  }

  #[test]
  fn vectors_should_multiply_by_a_scalar() {
    let a = vec4(1., -2., 3., -4.);

    assert_eq!(a * 3.5, vec4(3.5, -7., 10.5, -14.));
  }

  #[test]
  fn vectors_should_multiply_by_a_fraction() {
    let a = vec4(1., -2., 3., -4.);

    assert_eq!(a * 0.5, vec4(0.5, -1., 1.5, -2.));
  }

  #[test]
  fn vectors_should_divide_by_scalar() {
    let a = vec4(1., -2., 3., -4.);

    assert_eq!(a / 2., vec4(0.5, -1., 1.5, -2.));
  }

  #[test]
  fn vectors_should_compute_magnitude_of_unit_x() {
    assert_eq!(1., vec3(1., 0., 0.).magnitude());
  }

  #[test]
  fn vectors_should_compute_magnitude_of_unit_y() {
    assert_eq!(1., vec3(0., 1., 0.).magnitude());
  }

  #[test]
  fn vectors_should_compute_magnitude_of_unit_z() {
    assert_eq!(1., vec3(0., 0., 1.).magnitude());
  }

  #[test]
  fn vectors_should_compute_magnitude_of_positive_vector() {
    assert_eq!(14f64.sqrt(), vec3(1., 2., 3.).magnitude());
  }

  #[test]
  fn vectors_should_compute_magnitude_of_negative_vector() {
    assert_eq!(14f64.sqrt(), vec3(-1., -2., -3.).magnitude());
  }

  #[test]
  fn vectors_should_normalize_unit_x() {
    assert_eq!(vec3(4., 0., 0.).normalize(), vec3(1., 0., 0.));
  }

  #[test]
  fn vectors_should_normalize_unit_y() {
    assert_eq!(vec3(0., 20., 0.).normalize(), vec3(0., 1., 0.));
  }

  #[test]
  fn vectors_should_normalize_unit_z() {
    assert_eq!(vec3(0., 0., -8.).normalize(), vec3(0., 0., -1.));
  }

  #[test]
  fn vectors_should_have_unit_length_normalized_vector() {
    assert!(vec3(1., 2., 3.).normalize().magnitude().is_approx(1.));
  }

  #[test]
  fn vectors_should_compute_a_dot_product() {
    let a = vec3(1., 2., 3.);
    let b = vec3(2., 3., 4.);

    assert_eq!(a.dot(b), 20.);
  }

  #[test]
  fn vectors_should_compute_a_cross_product() {
    let a = vec3(1., 2., 3.);
    let b = vec3(2., 3., 4.);

    let cross = a.cross(b);

    assert_eq!(cross, vec3(-1., 2., -1.));
  }

  #[test]
  fn reflect_a_vector_approaching_at_45_degrees() {
    let vector = vec3(1., -1., 0.);
    let normal = vec3(0., 1., 0.);

    let reflection = vector.reflect(normal);

    assert_eq!(reflection, vec3(1., 1., 0.));
  }

  #[test]
  fn reflect_a_vector_off_a_slanted_surface() {
    let vector = vec3(0., -1., 0.);
    let normal = vec3(2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.);

    let reflection = vector.reflect(normal);

    assert_eq!(reflection, vec3(1., 0., 0.));
  }
}