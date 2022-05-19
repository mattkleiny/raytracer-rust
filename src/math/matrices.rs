//! Linear algebra matrix operations and utilities.

use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

use super::ApproxEq;

pub type Matrix2x2 = Matrix<2, 4>;
pub type Matrix3x3 = Matrix<3, 9>;
pub type Matrix4x4 = Matrix<4, 16>;

/// A rectangular matrix of N by N elements, with the given row stride.
///
/// S = Stride of the matrix; how far between each row.
/// L = Length of the matrix; total number of elements.
#[derive(Clone)]
pub struct Matrix<const S: usize, const L: usize> {
  elements: [f32; L],
}

impl<const S: usize, const L: usize> Matrix<S, L> {
  pub const ZERO: Self = Self::new();

  /// Constructs a new empty matrix.
  pub const fn new() -> Self {
    Self { elements: [0.; L] }
  }

  /// Constructs a matrix from the given elements.
  pub const fn from_elements(elements: &[f32; L]) -> Self {
    Self { elements: *elements }
  }
}

impl<const S: usize, const L: usize> Debug for Matrix<S, L> {
  /// Formats the matrix in a semi-readable manner.
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    for y in 0..S {
      write!(formatter, "[ ")?;

      for x in 0..S {
        write!(formatter, "{:.2} ", self.elements[x + y * S])?;
      }

      write!(formatter, "]\n")?
    }

    Ok(())
  }
}

impl<const S: usize, const L: usize> Index<(usize, usize)> for Matrix<S, L> {
  type Output = f32;

  /// Accesses a single element of the matrix.
  ///
  /// N.B: This is column-major order.
  #[inline]
  fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
    &self.elements[x + y * S]
  }
}

impl<const S: usize, const L: usize> IndexMut<(usize, usize)> for Matrix<S, L> {
  /// Mutably accesses a single element of the matrix.
  ///
  /// N.B: This is column-major order.
  #[inline]
  fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
    &mut self.elements[x + y * S]
  }
}

impl<const S: usize, const L: usize> PartialEq for Matrix<S, L> {
  /// Standard per-element equality.
  fn eq(&self, other: &Self) -> bool {
    for i in 0..self.elements.len() {
      if !self.elements[i].is_approx(other.elements[i]) {
        return false;
      }
    }
    return true;
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

  #[test]
  fn matrix_equality_should_work() {
    let a = Matrix3x3::from_elements(&[
      1., 2., 3.,
      4., 5., 6.,
      7., 8., 9.,
    ]);

    let b = Matrix3x3::from_elements(&[
      1., 2., 3.,
      4., 5., 6.,
      7., 8., 9.,
    ]);

    assert_eq!(a, b);
  }

  #[test]
  fn matrix_inequality_should_work() {
    let a = Matrix3x3::from_elements(&[
      1., 2., 3.,
      4., 5., 6.,
      7., 8., 9.,
    ]);

    let b = Matrix3x3::from_elements(&[
      2., 3., 4.,
      5., 6., 7.,
      8., 9., 10.,
    ]);

    assert_ne!(a, b);
  }
}