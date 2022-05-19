//! Matrix types and utilities.

use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut, Mul};

use super::{ApproxEq, tuple, Tuple};

pub type Matrix2x2 = Matrix<2, 4>;
pub type Matrix3x3 = Matrix<3, 9>;
pub type Matrix4x4 = Matrix<4, 16>;

/// A rectangular matrix of N by N elements, with the given row stride.
///
/// S = Stride of the matrix; how many columns between each row.
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
  pub const fn create(elements: &[f32; L]) -> Self {
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
        write!(formatter, "{: >5.2} ", self.elements[x + y * S])?;
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
  fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
    &self.elements[column + row * S]
  }
}

impl<const S: usize, const L: usize> IndexMut<(usize, usize)> for Matrix<S, L> {
  /// Mutably accesses a single element of the matrix.
  ///
  /// N.B: This is column-major order.
  #[inline]
  fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
    &mut self.elements[column + row * S]
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

    for row in 0..S {
      for column in 0..S {
        let mut sum = 0.;

        for i in 0..S {
          sum += self[(row, i)] * rhs[(i, column)];
        }

        result[(row, column)] = sum;
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

/// Specializations for 4x4 matrices.
impl Matrix4x4 {
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 4x4 identity matrix (1 along the left to right diagonal).
  pub const fn identity() -> Self {
    Self::create(&[
      1., 0., 0., 0.,
      0., 1., 0., 0.,
      0., 0., 1., 0.,
      0., 0., 0., 1.,
    ])
  }

  /// Computes the sub-matrix of this matrix by removing the given row and column.
  pub fn to_sub_matrix(&self, row: usize, column: usize) -> Matrix3x3 {
    let mut result = Matrix3x3::new();
    let mut i = 0;

    for y in 0..4 {
      if y == row {
        continue;
      }

      for x in 0..4 {
        if x == column {
          continue;
        }

        result[(i / 3, i % 3)] = self[(y, x)];
        i += 1;
      }
    }

    result
  }

  /// Computes the determinant of the sub-matrix with the given row and column removed.
  pub fn minor(&self, row: usize, column: usize) -> f32 {
    self.to_sub_matrix(row, column).determinant()
  }

  /// Calculates the cofactor of the matrix with the given row and column removed.
  pub fn cofactor(&self, row: usize, column: usize) -> f32 {
    let minor = self.minor(row, column);

    if (row + column) % 2 == 0 {
      minor
    } else {
      -minor
    }
  }

  /// Computes the determinant of the matrix.
  ///
  /// A determinant 'determines' whether a system of equations has a solution.
  pub fn determinant(&self) -> f32 {
    let mut result = 0.;

    for i in 0..4 {
      result += self[(0, i)] * self.cofactor(0, i);
    }

    result
  }

  /// Inverts this matrix.
  pub fn invert(&self) -> anyhow::Result<Self> {
    let determinant = self.determinant();
    if determinant == 0. {
      return Err(anyhow!("Cannot invert a matrix with a determinant of 0"));
    }

    let mut result = Self::new();

    for row in 0..4 {
      for column in 0..4 {
        result[(column, row)] = self.cofactor(row, column) / determinant;
      }
    }

    Ok(result)
  }
}

/// Specializations for 3x3 matrices.
impl Matrix3x3 {
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 3x3 identity matrix (1 along the left to right diagonal).
  pub const fn identity() -> Self {
    Self::create(&[
      1., 0., 0.,
      0., 1., 0.,
      0., 0., 1.,
    ])
  }

  /// Computes the sub-matrix of this matrix by removing the given row and column.
  pub fn to_sub_matrix(&self, row: usize, column: usize) -> Matrix2x2 {
    let mut result = Matrix2x2::new();
    let mut i = 0;

    for y in 0..3 {
      if y == row {
        continue;
      }

      for x in 0..3 {
        if x == column {
          continue;
        }

        result[(i / 2, i % 2)] = self[(y, x)];
        i += 1;
      }
    }

    result
  }

  /// Computes the determinant of the sub-matrix with the given row and column removed.
  pub fn minor(&self, row: usize, column: usize) -> f32 {
    self.to_sub_matrix(row, column).determinant()
  }

  /// Calculates the cofactor of the matrix with the given row and column removed.
  pub fn cofactor(&self, row: usize, column: usize) -> f32 {
    let minor = self.minor(row, column);

    if (row + column) % 2 == 0 {
      minor
    } else {
      -minor
    }
  }

  /// Computes the determinant of the matrix.
  ///
  /// A determinant 'determines' whether a system has a solution.
  pub fn determinant(&self) -> f32 {
    let mut result = 0.;

    for i in 0..3 {
      result += self[(0, i)] * self.cofactor(0, i);
    }

    result
  }
}

/// Specializations for 2x2 matrices.
impl Matrix2x2 {
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 2x2 identity matrix (1 along the left to right diagonal).
  pub const fn identity() -> Self {
    Self::create(&[
      1., 0.,
      0., 1.,
    ])
  }

  /// Computes the determinant of the matrix.
  ///
  /// A determinant 'determines' whether a system has a solution.
  pub fn determinant(&self) -> f32 {
    // TODO: make this work across all dimensions.
    let [a, b, c, d] = self.elements;

    a * d - b * c
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn matrix2x2_should_construct_from_elements() {
    let matrix = Matrix2x2::create(&[
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
    let matrix = Matrix3x3::create(&[
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
    let matrix = Matrix4x4::create(&[
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
    let a = Matrix3x3::create(&[
      1., 2., 3.,
      4., 5., 6.,
      7., 8., 9.,
    ]);

    let b = Matrix3x3::create(&[
      1., 2., 3.,
      4., 5., 6.,
      7., 8., 9.,
    ]);

    assert_eq!(a, b);
  }

  #[test]
  fn matrix_inequality_should_work() {
    let a = Matrix3x3::create(&[
      1., 2., 3.,
      4., 5., 6.,
      7., 8., 9.,
    ]);

    let b = Matrix3x3::create(&[
      2., 3., 4.,
      5., 6., 7.,
      8., 9., 10.,
    ]);

    assert_ne!(a, b);
  }

  #[test]
  fn matrices_can_multiply_by_other_matrices() {
    let a = Matrix4x4::create(&[
      1., 2., 3., 4.,
      5., 6., 7., 8.,
      9., 8., 7., 6.,
      5., 4., 3., 2.,
    ]);

    let b = Matrix4x4::create(&[
      -2., 1., 2., 3.,
      3., 2., 1., -1.,
      4., 3., 6., 5.,
      1., 2., 7., 8.,
    ]);

    assert_eq!(a * b, Matrix4x4::create(&[
      20., 22., 50., 48.,
      44., 54., 114., 108.,
      40., 58., 110., 102.,
      16., 26., 46., 42.,
    ]));
  }

  #[test]
  fn matrices_should_multiply_by_tuples() {
    let a = Matrix4x4::create(&[
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
    let a = Matrix4x4::create(&[
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
    let a = Matrix4x4::create(&[
      0., 9., 3., 0.,
      9., 8., 0., 8.,
      1., 8., 5., 3.,
      0., 0., 5., 8.,
    ]);

    assert_eq!(a.transpose(), Matrix4x4::create(&[
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
    let a = Matrix2x2::create(&[
      1., 5.,
      -3., 2.,
    ]);

    assert_eq!(a.determinant(), 17.);
  }

  #[test]
  fn sub_matrix_of_3x3_is_valid_2x2() {
    let a = Matrix3x3::create(&[
      1., 5., 0.,
      -3., 2., 7.,
      0., 6., -3.,
    ]);

    assert_eq!(a.to_sub_matrix(0, 2), Matrix2x2::create(&[
      -3., 2.,
      0., 6.,
    ]));
  }

  #[test]
  fn sub_matrix_of_4x4_is_valid_3x3() {
    let a = Matrix4x4::create(&[
      -6., 1., 1., 6.,
      -8., 5., 8., 6.,
      -1., 0., 8., 2.,
      -7., 1., -1., 1.,
    ]);

    assert_eq!(a.to_sub_matrix(2, 1), Matrix3x3::create(&[
      -6., 1., 6.,
      -8., 8., 6.,
      -7., -1., 1.,
    ]));
  }

  #[test]
  fn matrix3x3_should_calculate_minor() {
    let a = Matrix3x3::create(&[
      3., 5., 0.,
      2., -1., -7.,
      6., -1., 5.,
    ]);

    let b = a.to_sub_matrix(1, 0);

    assert_eq!(b.determinant(), 25.);
    assert_eq!(a.minor(1, 0), 25.);
  }

  #[test]
  fn matrix3x3_should_calculate_cofactor() {
    let a = Matrix3x3::create(&[
      3., 5., 0.,
      2., -1., -7.,
      6., -1., 5.,
    ]);

    assert_eq!(a.minor(0, 0), -12.);
    assert_eq!(a.cofactor(0, 0), -12.);
    assert_eq!(a.minor(1, 0), 25.);
    assert_eq!(a.cofactor(1, 0), -25.);
  }

  #[test]
  fn matrix3x3_should_calculate_cofactor_and_determinant() {
    let a = Matrix3x3::create(&[
      1., 2., 6.,
      -5., 8., -4.,
      2., 6., 4.
    ]);

    assert_eq!(a.cofactor(0, 0), 56.);
    assert_eq!(a.cofactor(0, 1), 12.);
    assert_eq!(a.cofactor(0, 2), -46.);
    assert_eq!(a.determinant(), -196.);
  }

  #[test]
  fn matrix4x4_should_calculate_cofactor_and_determinant() {
    let a = Matrix4x4::create(&[
      -2., -8., 3., 5.,
      -3., 1., 7., 3.,
      1., 2., -9., 6.,
      -6., 7., 7., -9.,
    ]);

    assert_eq!(a.cofactor(0, 0), 690.);
    assert_eq!(a.cofactor(0, 1), 447.);
    assert_eq!(a.cofactor(0, 2), 210.);
    assert_eq!(a.cofactor(0, 3), 51.);
    assert_eq!(a.determinant(), -4071.);
  }

  #[test]
  fn matrix4x4_inversion_should_fail_if_not_possible() {
    let a = Matrix4x4::create(&[
      -4., 2., -2., -3.,
      9., 6., 2., 6.,
      0., -5., 1., -5.,
      0., 0., 0., 0.,
    ]);

    assert_eq!(a.determinant(), 0.);
    assert!(a.invert().is_err());
  }

  #[test]
  fn matrix4x4_inversion_should_work_if_possible() {
    let a = Matrix4x4::create(&[
      -5., 2., 6., -8.,
      1., -5., 1., 8.,
      7., 7., -6., -7.,
      1., -3., 7., 4.
    ]);

    let b = a.invert().expect("Failed to invert matrix");

    assert_eq!(a.determinant(), 532.);

    assert_eq!(a.cofactor(2, 3), -160.);
    assert_eq!(b[(3, 2)], -160. / 532.);

    assert_eq!(a.cofactor(3, 2), 105.);
    assert_eq!(b[(2, 3)], 105. / 532.);

    assert_eq!(b, Matrix4x4::create(&[
      0.21805, 0.45113, 0.24060, -0.04511,
      -0.80827, -1.45677, -0.44361, 0.52068,
      -0.07895, -0.22368, -0.05263, 0.19737,
      -0.52256, -0.81391, -0.30075, 0.30639,
    ]));
  }

  #[test]
  fn matrix_inversion_results_in_original_matrix() {
    let a = Matrix4x4::create(&[
      3., -9., 7., 3.,
      3., -8., 2., -9.,
      -4., 4., 4., 1.,
      -6., 5., -1., 1.,
    ]);

    let b = Matrix4x4::create(&[
      8., 2., 2., 2.,
      3., -1., 7., 0.,
      7., 0., 5., 4.,
      6., -2., 0., 5.,
    ]);

    let c = a * b;
    let inverse = b.invert().expect("Failed to invert matrix");

    assert_eq!(c * inverse, a);
  }
}