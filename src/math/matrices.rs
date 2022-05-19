//! Linear algebra matrix operations and utilities.

use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut, Mul};

use crate::math::tuple;

use super::ApproxEq;
use super::Tuple;

pub type Matrix2x2 = Matrix<2, 4>;
pub type Matrix3x3 = Matrix<3, 9>;
pub type Matrix4x4 = Matrix<4, 16>;

/// A rectangular matrix of N by N elements, with the given row stride.
///
/// S = Stride of the matrix; how far between each row.
/// L = Length of the matrix; total number of elements.
#[derive(Copy, Clone)]
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

  /// Transposes the matrix.
  pub fn transpose(&self) -> Self {
    let mut result = Self::new();

    for i in 0..S {
      for j in 0..S {
        result[(i, j)] = self[(j, i)];
      }
    }

    result
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

impl<const S: usize, const L: usize> Mul for Matrix<S, L> {
  type Output = Self;

  /// Multiplies two matrices together.
  fn mul(self, rhs: Self) -> Self::Output {
    let mut result = Self::ZERO;

    for y in 0..S {
      for x in 0..S {
        let mut sum = 0.;

        for i in 0..S {
          sum += self[(y, i)] * rhs[(i, x)];
        }

        result[(y, x)] = sum;
      }
    }

    result
  }
}

impl Mul<Tuple> for Matrix4x4 {
  type Output = Tuple;

  /// Multiplies a 4x4 matrix by a tuple.
  fn mul(self, rhs: Tuple) -> Self::Output {
    let mut result = tuple(0., 0., 0., 0.);

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

  /// Computes the sub-matrix of this matrix by removing the given row and column.
  pub fn submatrix(&self, row: usize, column: usize) -> Matrix2x2 {
    todo!()
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

  /// Computes the determinant of the matrix.
  ///
  /// A determinant 'determines' whether a system has a solution.
  pub fn determinant(&self) -> f32 {
    // TODO: make this work across all dimensions.

    let a = self[(0, 0)];
    let b = self[(0, 1)];
    let c = self[(1, 0)];
    let d = self[(1, 1)];

    a * d - b * c
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

  #[test]
  fn matrices_can_multiply_by_other_matrices() {
    let a = Matrix4x4::from_elements(&[
      1., 2., 3., 4.,
      5., 6., 7., 8.,
      9., 8., 7., 6.,
      5., 4., 3., 2.,
    ]);

    let b = Matrix4x4::from_elements(&[
      -2., 1., 2., 3.,
      3., 2., 1., -1.,
      4., 3., 6., 5.,
      1., 2., 7., 8.,
    ]);

    assert_eq!(a * b, Matrix4x4::from_elements(&[
      20., 22., 50., 48.,
      44., 54., 114., 108.,
      40., 58., 110., 102.,
      16., 26., 46., 42.,
    ]));
  }

  #[test]
  fn matrices_should_multiply_by_tuples() {
    let a = Matrix4x4::from_elements(&[
      1., 2., 3., 4.,
      2., 4., 4., 2.,
      8., 6., 4., 1.,
      0., 0., 0., 1.,
    ]);

    let result = a * tuple(1., 2., 3., 1.);

    assert_eq!(result, tuple(18., 24., 33., 1.));
  }

  #[test]
  fn matrix_multiplication_by_identity_should_be_inert() {
    let a = Matrix4x4::from_elements(&[
      0., 1., 2., 4.,
      1., 2., 4., 8.,
      2., 4., 8., 16.,
      4., 8., 16., 32.,
    ]);

    assert_eq!(a * Matrix4x4::IDENTITY, a);
  }

  #[test]
  fn matrix_multiplication_by_tuple_should_be_inert() {
    let a = tuple(1., 2., 3., 4.);

    assert_eq!(Matrix4x4::IDENTITY * a, a);
  }

  #[test]
  fn matrix_transpose_should_work_correctly() {
    let a = Matrix4x4::from_elements(&[
      0., 9., 3., 0.,
      9., 8., 0., 8.,
      1., 8., 5., 3.,
      0., 0., 5., 8.,
    ]);

    assert_eq!(a.transpose(), Matrix4x4::from_elements(&[
      0., 9., 1., 0.,
      9., 8., 8., 0.,
      3., 0., 5., 5.,
      0., 8., 3., 8.,
    ]));
  }

  #[test]
  fn matrix_transpose_of_identity_is_identity() {
    assert_eq!(Matrix4x4::IDENTITY.transpose(), Matrix4x4::IDENTITY);
  }

  #[test]
  fn determinant_should_be_calculated_correctly() {
    let a = Matrix2x2::from_elements(&[
      1., 5.,
      -3., 2.,
    ]);

    assert_eq!(a.determinant(), 17.);
  }

  #[test]
  fn submatrix_of_3x3_is_valid_2x2() {
    let a = Matrix3x3::from_elements(&[
      1., 5., 0.,
      -3., 2., 7.,
      0., 6., -3.,
    ]);

    assert_eq!(a.submatrix(0, 2), Matrix2x2::from_elements(&[
      -3., 2.,
      0., 6.,
    ]));
  }
}