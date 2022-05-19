//! Linear algebra matrix operations and utilities.

use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

pub type Matrix2x2 = Matrix<2, 4>;
pub type Matrix3x3 = Matrix<3, 9>;
pub type Matrix4x4 = Matrix<4, 16>;

/// A rectangular matrix of N by N elements, with the given row stride.
#[derive(Clone)]
pub struct Matrix<const STRIDE: usize, const LENGTH: usize> {
  elements: [f32; LENGTH],
}

impl<const STRIDE: usize, const LENGTH: usize> Matrix<STRIDE, LENGTH> {
  pub const ZERO: Self = Self::new();

  /// Constructs a new empty matrix.
  pub const fn new() -> Self {
    Self { elements: [0.; LENGTH] }
  }

  /// Constructs a matrix from the given elements.
  pub const fn from_elements(elements: &[f32; LENGTH]) -> Self {
    Self { elements: *elements }
  }
}

impl<const STRIDE: usize, const LENGTH: usize> Debug for Matrix<STRIDE, LENGTH> {
  /// Formats the matrix in a semi-readable manner.
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    for y in 0..STRIDE {
      write!(formatter, "[ ")?;

      for x in 0..STRIDE {
        write!(formatter, "{:.2} ", self.elements[x + y * STRIDE])?;
      }

      write!(formatter, "]\n")?
    }

    Ok(())
  }
}

impl<const STRIDE: usize, const LENGTH: usize> Index<(usize, usize)> for Matrix<STRIDE, LENGTH> {
  type Output = f32;

  /// Accesses a single element of the matrix.
  ///
  /// N.B: This is column-major order.
  #[inline]
  fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
    &self.elements[x + y * STRIDE]
  }
}

impl<const STRIDE: usize, const LENGTH: usize> IndexMut<(usize, usize)> for Matrix<STRIDE, LENGTH> {
  /// Mutably accesses a single element of the matrix.
  ///
  /// N.B: This is column-major order.
  #[inline]
  fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
    &mut self.elements[x + y * STRIDE]
  }
}

impl Matrix2x2 {
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 2x2 identity matrix (1 along the left to right diagonal).
  pub const fn identity() -> Self {
    Self::from_elements(&[
      1., 0.,
      0., 1.,
    ])
  }
}

impl Matrix3x3 {
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 3x3 identity matrix (1 along the left to right diagonal).
  pub const fn identity() -> Self {
    Self::from_elements(&[
      1., 0., 0.,
      0., 1., 0.,
      0., 0., 1.,
    ])
  }
}

impl Matrix4x4 {
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 4x4 identity matrix (1 along the left to right diagonal).
  pub const fn identity() -> Self {
    Self::from_elements(&[
      1., 0., 0., 0.,
      0., 1., 0., 0.,
      0., 0., 1., 0.,
      0., 0., 0., 1.,
    ])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn matrix2x2_should_construct_from_elements() {
    let matrix = Matrix2x2::from_elements(&[
      -3., 5.,
      1., -2.,
    ]);

    assert_eq!(matrix[(0, 0)], -3.);
    assert_eq!(matrix[(0, 1)], 5.);
    assert_eq!(matrix[(1, 0)], 1.);
    assert_eq!(matrix[(1, 1)], -2.);
  }

  #[test]
  fn matrix3x3_should_construct_from_elements() {
    let matrix = Matrix3x3::from_elements(&[
      -3., 5., 0.,
      1., -2., -7.,
      0., 1., 1.,
    ]);

    assert_eq!(matrix[(0, 0)], -3.);
    assert_eq!(matrix[(1, 1)], -2.);
    assert_eq!(matrix[(2, 2)], 1.);
  }

  #[test]
  fn matrix4x4_should_construct_from_elements() {
    let matrix = Matrix4x4::from_elements(&[
      1., 2., 3., 4.,
      5.5, 6.5, 7.5, 8.5,
      9., 10., 11., 12.,
      13.5, 14.5, 15.5, 16.5,
    ]);

    assert_eq!(matrix[(0, 0)], 1.);
    assert_eq!(matrix[(0, 3)], 4.);
    assert_eq!(matrix[(1, 0)], 5.5);
    assert_eq!(matrix[(1, 2)], 7.5);
    assert_eq!(matrix[(2, 2)], 11.);
    assert_eq!(matrix[(3, 0)], 13.5);
    assert_eq!(matrix[(3, 2)], 15.5);
  }
}